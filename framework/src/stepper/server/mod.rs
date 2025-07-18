use crate::{
    event_port::ServerPort,
    simulation::{Simulation, StepLogic},
};

pub struct ServerStepper<StepInput, StepOutput, State, RI, RO>
where
    State: StepLogic<StepInput, StepOutput>,
{
    /// Inner simulation of the stepper, which is never exposed mutably.
    simulation: Simulation<StepInput, StepOutput, State>,
}

impl<StepInput, StepOutput, State, RI, RO> ServerStepper<StepInput, StepOutput, State, RI, RO>
where
    State: StepLogic<StepInput, StepOutput>,
{
    /// Simply creates a new stepper containing a simulation.
    pub fn new(simulation: Simulation<StepInput, StepOutput, State>) -> Self {
        Self { simulation }
    }

    /// Returns an immutable reference to the inner simulation of this stepper.
    pub fn simulation(&self) -> &Simulation<StepInput, StepOutput, State> {
        &self.simulation
    }

    pub fn step<P>(&mut self, runtime_query: RI, event_port: &mut P) -> RO
    where
        P: ServerPort<StepInput, StepOutput>,
    {
        // Fetch inputs from port.
        let input = event_port.stepper_read();

        // Do step and store consequential input.
        let essential_input = self.simulation.step(input);

        // Queue consequential input to port for broadcasting.
        event_port.step_output_to_all(essential_input);
    }
}
