use framework::{
    event::Event,
    event_port::{Port, ServerPort},
};
use quinn::Endpoint;
use std::{error::Error, marker::PhantomData, thread};

pub struct QuinnServerPort<E, EventSourceID>
where
    E: Event,
    EventSourceID: Eq,
{
    event_type: PhantomData<E>,
    event_source_id_type: PhantomData<EventSourceID>,
}

impl<E, EventSourceID> QuinnServerPort<E, EventSourceID>
where
    E: Event,
    EventSourceID: Eq,
{
    pub fn new(endpoint: Endpoint) -> Self {
        let (tx, rx) = crossbeam::channel::unbounded::<(EventSourceID, E)>();

        // Spawn seperate thread.
        thread::spawn(move || {
            // Start a tokio async runtime.
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build tokio runtime.");

            // Use the runtime to wait for the async entry point to exit.
            rt.block_on(async {
                match handle_endpoint(endpoint).await {
                    Err(err) => eprintln!("Error occurred on server port:\n{err:?}"),
                    _ => (),
                }
            });
        });

        Self {
            event_type: PhantomData,
            event_source_id_type: PhantomData,
        }
    }
}

async fn handle_endpoint(endpoint: Endpoint) -> Result<(), Box<dyn Error>> {
    while let Some(inc) = endpoint.accept().await {
        match inc.await {
            Ok(conn) => {
                let remote_addr = conn.remote_address();
                let rtt = conn.rtt();
                println!("New connection established with client addr: {remote_addr:?}.");
                println!("Ping is: {rtt:?}");

                let mut send_stream = conn.open_uni().await?;
                println!("Writing message to client.");
                send_stream.write_all(b"hello from server").await?;
                send_stream.finish()?;
            }
            Err(conn_err) => {
                println!("Error when accepting inc. client connection: {conn_err:?}");
            }
        }
    }
    println!("Server endpoint closed.");

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
