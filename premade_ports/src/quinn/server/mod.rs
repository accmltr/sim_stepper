mod connection;

use connection::ClientConnection;
use crossbeam::channel::RecvError;
use framework::{
    event::Event,
    event_port::{Port, ServerPort},
};
use quinn::Endpoint;
use std::{
    error::Error,
    marker::PhantomData,
    thread::{self, JoinHandle},
};

pub struct QuinnServerPort<E, EventSourceID>
where
    E: Event,
    EventSourceID: Eq,
{
    pub thread_join_handle: JoinHandle<()>,
    slave_thread_receiver: crossbeam::channel::Receiver<ClientConnection<EventSourceID, E>>,
    event_type: PhantomData<E>,
    event_source_id_type: PhantomData<EventSourceID>,
}

impl<E, EventSourceID> QuinnServerPort<E, EventSourceID>
where
    E: Event + 'static,
    EventSourceID: Eq + Send + 'static,
{
    pub fn new(endpoint: Endpoint) -> Self {
        let (beam_sen, beam_recv) =
            crossbeam::channel::unbounded::<ClientConnection<EventSourceID, E>>();

        // Spawn seperate thread.
        let thread_join_handle = thread::spawn(move || {
            // Start a tokio async runtime.
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build tokio runtime.");

            // Use the runtime to wait for the async entry point to exit.
            rt.block_on(async {
                if let Err(err) = async {
                    println!("Waiting for incoming connections");
                    println!("Server listening on: {:?}", endpoint.local_addr()?);
                    while let Some(incoming) = endpoint.accept().await {
                        println!("New connection accepted.");
                        let conn = incoming.await.unwrap();
                        println!("Connection established.");
                        let (tokio_sen, tokio_recv) = tokio::sync::mpsc::channel::<E>(100);
                        let remote_addr = conn.remote_address();
                        let rtt = conn.rtt();
                        let port_connection = ClientConnection {
                            event_source_id: conn.stable_id(),
                            event_receiver: tokio_recv,
                        };
                        // let _ = tx.send(port_connection);
                        println!("New connection established with client addr: {remote_addr:?}.");
                        println!("Ping is: {rtt:?}");

                        let mut send_stream = conn.open_uni().await.unwrap();
                        println!("Writing message to client.");
                        send_stream.write_all(b"hello from server").await.unwrap();
                        send_stream.finish().unwrap();
                    }

                    println!("Server endpoint closed.");

                    Ok::<(), Box<dyn Error>>(())
                }
                .await
                {
                    eprintln!("[Server Port]\n{err:?}")
                }
            });
        });

        Self {
            thread_join_handle,
            slave_thread_receiver: beam_recv,
            event_type: PhantomData,
            event_source_id_type: PhantomData,
        }
    }

    pub fn read_received(&self) -> Result<ClientConnection<EventSourceID, E>, RecvError> {
        self.slave_thread_receiver.recv()
    }
}

impl<E, EventSourceID> Port<E, EventSourceID> for QuinnServerPort<E, EventSourceID>
where
    E: Event,
    EventSourceID: Eq,
{
    fn read_events(&mut self) -> std::collections::HashMap<EventSourceID, Vec<E>> {
        todo!()
    }

    fn step(&mut self) {
        todo!()
    }
}

impl<E, EventSourceID> ServerPort<E, EventSourceID> for QuinnServerPort<E, EventSourceID>
where
    E: Event,
    EventSourceID: Eq,
{
    fn queue_runtime_events(
        &mut self,
        _step_input: framework::simulation::StepInput<EventSourceID, E>,
    ) {
        todo!()
    }

    fn queue_server_step_input_to_clients(
        &mut self,
        _step_input: framework::simulation::StepInput<EventSourceID, E>,
    ) {
        todo!()
    }
}
