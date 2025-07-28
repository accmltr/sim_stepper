use crate::{
    event::Event,
    simulation::{Simulation, StepLogic},
};

pub struct ForwardStepper<State, E, EventSourceID>
where
    E: Event,
    State: StepLogic<E, EventSourceID>,
    EventSourceID: Eq,
{
    simulation: Simulation<State, E, EventSourceID>,
}

impl<State, E, EventSourceID> ForwardStepper<State, E, EventSourceID>
where
    E: Event,
    State: StepLogic<E, EventSourceID>,
    EventSourceID: Eq,
{
    pub fn new(simulation: Simulation<State, E, EventSourceID>) -> Self {
        Self { simulation }
    }

    pub fn simulation(&self) -> &Simulation<State, E, EventSourceID> {
        &self.simulation
    }
}
