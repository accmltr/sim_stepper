use crate::simulation::{Simulation, StepLogic};

pub struct ForwardStepper<State, Event, EventSourceID>
where
    State: StepLogic<Event, EventSourceID>,
    EventSourceID: Eq,
{
    simulation: Simulation<State, Event, EventSourceID>,
}

impl<State, Event, EventSourceID> ForwardStepper<State, Event, EventSourceID>
where
    State: StepLogic<Event, EventSourceID>,
    EventSourceID: Eq,
{
    pub fn new(simulation: Simulation<State, Event, EventSourceID>) -> Self {
        Self { simulation }
    }

    pub fn simulation(&self) -> &Simulation<State, Event, EventSourceID> {
        &self.simulation
    }
}
