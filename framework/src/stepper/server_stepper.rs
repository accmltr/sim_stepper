use crate::{
    event_port::ServerPort,
    simulation::{Simulation, StepLogic},
};

pub struct ServerStepper<StepInput, StepOutput, State>
where
    State: StepLogic<StepInput, StepOutput>,
{
    simulation: Simulation<StepInput, StepOutput, State>,
}

impl<StepInput, StepOutput, State> ServerStepper<StepInput, StepOutput, State>
where
    State: StepLogic<StepInput, StepOutput>,
{
    pub fn new(simulation: Simulation<StepInput, StepOutput, State>) -> Self {
        Self { simulation }
    }

    fn simulation(&self) -> &Simulation<StepInput, StepOutput, State> {
        &self.simulation
    }

    fn step<P>(&mut self, event_port: &mut P)
    where
        P: ServerPort<StepInput, StepOutput>,
    {
        // Fetch inputs from port.
        let events = event_port.stepper_read();

        // Do step and store consequential input.
        let cons = self.simulation.step(events);

        // Queue consequential input to port for broadcasting.
        event_port.step_input_to_all(cons);
    }
}
