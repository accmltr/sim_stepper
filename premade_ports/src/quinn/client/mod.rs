use std::{
    error::Error,
    marker::PhantomData,
    net::SocketAddr,
    thread::{self, JoinHandle},
};

use framework::{
    event::Event,
    event_port::{ClientPort, Port},
};
use quinn::Endpoint;

pub struct QuinnClientPort<E, EventSourceID>
where
    E: Event,
    EventSourceID: Eq,
{
    pub thread_join_handle: JoinHandle<()>,
    event_type: PhantomData<E>,
    event_source_id_type: PhantomData<EventSourceID>,
}

impl<E, EventSourceID> QuinnClientPort<E, EventSourceID>
where
    E: Event,
    EventSourceID: Eq,
{
    pub fn new(endpoint: Endpoint, server_addr: SocketAddr, server_name: String) -> Self {
        println!("Starting client port.");
        let (tx, rx) = crossbeam::channel::unbounded::<(EventSourceID, E)>();

        // Spawn seperate thread.
        let handle = thread::spawn(move || {
            // Start a tokio async runtime.
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build tokio runtime.");

            // Use the runtime to wait for the async entry point to exit.
            rt.block_on(async {
                // Run endpoint logic.
                if let Err(err) = handle_endpoint(endpoint, server_addr, server_name).await {
                    eprintln!("Error occurred on client port:\n{err:?}")
                }
            });
        });

        Self {
            thread_join_handle: handle,
            event_type: PhantomData,
            event_source_id_type: PhantomData,
        }
    }
}

async fn handle_endpoint(
    endpoint: Endpoint,
    server_addr: SocketAddr,
    server_name: String,
) -> Result<(), Box<dyn Error>> {
    println!("Attempting to connect to server.");
    println!("Connecting to addre: {:?}", server_addr);
    match endpoint.connect(server_addr, server_name.as_str()) {
        Ok(connecting) => {
            println!("Attempting to establish connection with server.");
            match connecting.await {
                Ok(conn) => {
                    println!("Successfully connected to server: {conn:?}");
                    while let Ok(mut recv) = conn.accept_uni().await {
                        let msg = recv.read_to_end(50).await?;
                        println!("Message received from server:\n{msg:?}")
                    }
                }
                Err(conn_err) => {
                    println!("Could not establish connection with server: {conn_err:?}");
                }
            };
        }
        Err(connect_err) => {
            println!("Could not connect to server: {connect_err:?}");
        }
    }
    println!("Client endpoint closed.");

    Ok(())
}
impl<E, EventSourceID> Port<E, EventSourceID> for QuinnClientPort<E, EventSourceID>
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

impl<E, EventSourceID> ClientPort<E, EventSourceID> for QuinnClientPort<E, EventSourceID>
where
    E: Event,
    EventSourceID: Eq,
{
    fn queue_client_events_to_server(&mut self, _events: Vec<E>) {
        todo!()
    }
}
