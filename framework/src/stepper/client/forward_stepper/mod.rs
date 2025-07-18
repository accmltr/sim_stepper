use crate::simulation::{Simulation, StepLogic};

pub struct ForwardStepper<State, SI, SO, RI, RO>
where
    State: StepLogic<SI, SO, RI, RO>,
{
    simulation: Simulation<State, SI, SO, RI, RO>,
}

impl<State, SI, SO, RI, RO> ForwardStepper<State, SI, SO, RI, RO>
where
    State: StepLogic<SI, SO, RI, RO>,
{
    pub fn new(simulation: Simulation<State, SI, SO, RI, RO>) -> Self {
        Self { simulation }
    }

    pub fn simulation(&self) -> &Simulation<State, SI, SO, RI, RO> {
        &self.simulation
    }
}
