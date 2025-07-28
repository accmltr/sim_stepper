use std::thread;

use framework::{
    event::Event,
    event_port::{Port, ServerPort},
};

pub struct QuinnServerPort {}

impl QuinnServerPort {
    pub fn new() {
        let (tx, rx) = std::sync::mpsc::channel();

        thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build tokio runtime.");

            runtime.block_on(async {
                run_networking().await;
            });
        });
    }
}

async fn run_networking() {}

impl<E, EventSourceID> Port<E, EventSourceID> for QuinnServerPort
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

impl<E, EventSourceID> ServerPort<E, EventSourceID> for QuinnServerPort
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
