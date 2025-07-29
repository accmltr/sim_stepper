use framework::{
    event::Event,
    event_port::{Port, ServerPort},
};
use quinn::Endpoint;
use std::{marker::PhantomData, thread};

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

        thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build tokio runtime.");

            runtime.block_on(async {
                run_networking().await;
            });
        });

        Self {
            event_type: PhantomData,
            event_source_id_type: PhantomData,
        }
    }
}

async fn run_networking() {}

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
