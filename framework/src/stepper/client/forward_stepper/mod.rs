use crate::simulation::{Simulation, StepLogic};

pub struct ForwardStepper<Input, Output, State>
where
    State: StepLogic<Input, Output>,
{
    simulation: Simulation<Input, Output, State>,
}

impl<Input, Output, State> ForwardStepper<Input, Output, State>
where
    State: StepLogic<Input, Output>,
{
    pub fn new(simulation: Simulation<Input, Output, State>) -> Self {
        Self { simulation }
    }
}
