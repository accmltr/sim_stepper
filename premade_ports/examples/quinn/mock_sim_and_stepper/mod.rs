use framework::{event::Event, simulation::StepLogic};

pub struct MyGame;

impl<E, EventSourceID> StepLogic<E, EventSourceID> for MyGame
where
    E: Event,
    EventSourceID: Eq,
{
    fn step(
        &mut self,
        step_count: u64,
        input: framework::simulation::StepInput<EventSourceID, E>,
    ) -> framework::simulation::StepInput<EventSourceID, E> {
        // Log step count.
        println!("Step no: {step_count}");

        // Simply return all events as received.
        input
    }
}
