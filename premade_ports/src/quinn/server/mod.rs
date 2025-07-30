use crossbeam::channel::{Receiver, RecvError, Sender};
use framework::{
    event::Event,
    event_port::{Port, ServerPort},
};
use quinn::Endpoint;
use std::{
    error::Error,
    marker::PhantomData,
    thread::{self, JoinHandle},
    time::Duration,
};

pub struct QuinnServerPort<E, EventSourceID>
where
    E: Event,
    EventSourceID: Eq,
{
    pub thread_join_handle: JoinHandle<()>,
    slave_thread_receiver: Receiver<&'static str>,
    event_type: PhantomData<E>,
    event_source_id_type: PhantomData<EventSourceID>,
}

impl<E, EventSourceID> QuinnServerPort<E, EventSourceID>
where
    E: Event + 'static,
    EventSourceID: Eq + Send + 'static,
{
    pub fn new(endpoint: Endpoint) -> Self {
        // let (tx, rx) = crossbeam::channel::unbounded::<(EventSourceID, E)>();
        let (tx, rx) = crossbeam::channel::unbounded();

        // Spawn seperate thread.
        let thread_join_handle = thread::spawn(move || {
            // Start a tokio async runtime.
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build tokio runtime.");

            // Use the runtime to wait for the async entry point to exit.
            rt.block_on(async {
                if let Err(err) = handle_endpoint(endpoint, tx).await {
                    eprintln!("[Server Port]\n{err:?}")
                }
            });
        });

        Self {
            thread_join_handle,
            slave_thread_receiver: rx,
            event_type: PhantomData,
            event_source_id_type: PhantomData,
        }
    }

    pub fn read_received(&self) -> Result<&'static str, RecvError> {
        self.slave_thread_receiver.recv()
    }
}

async fn handle_endpoint(
    endpoint: Endpoint,
    master_thread_sender: Sender<&'static str>,
) -> Result<(), Box<dyn Error>> {
    println!("Waiting for incoming connections");
    println!("Server listening on: {:?}", endpoint.local_addr()?);
    let _ = master_thread_sender.send("hi");
    tokio::spawn(async move {
        println!("lsjfsdlkjsdj");
        while let Some(incoming) = endpoint.accept().await {
            println!("New connection accepted.");
            let conn = incoming.await.unwrap();
            println!("Connection established.");
            let remote_addr = conn.remote_address();
            let rtt = conn.rtt();
            println!("New connection established with client addr: {remote_addr:?}.");
            println!("Ping is: {rtt:?}");

            let mut send_stream = conn.open_uni().await.unwrap();
            println!("Writing message to client.");
            send_stream.write_all(b"hello from server").await.unwrap();
            send_stream.finish().unwrap();
        }
        println!("Server endpoint closed.");
    })
    .await;

    thread::sleep(Duration::from_secs(10));

    Ok(())
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
