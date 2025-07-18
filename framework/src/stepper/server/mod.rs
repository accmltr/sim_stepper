use crate::{
    event_port::ServerPort,
    simulation::{Simulation, StepInput, StepLogic},
};

pub struct ServerStepper<State, SI, SO, RI, RO>
where
    State: StepLogic<SI, SO, RI, RO>,
{
    /// Inner simulation of the stepper, which is never exposed mutably.
    simulation: Simulation<State, SI, SO, RI, RO>,
}

impl<State, SI, SO, RI, RO> ServerStepper<State, SI, SO, RI, RO>
where
    State: StepLogic<SI, SO, RI, RO>,
{
    /// Simply creates a new stepper containing a simulation.
    pub fn new(simulation: Simulation<State, SI, SO, RI, RO>) -> Self {
        Self { simulation }
    }

    /// Returns an immutable reference to the inner simulation of this stepper.
    pub fn simulation(&self) -> &Simulation<State, SI, SO, RI, RO> {
        &self.simulation
    }

    pub fn step<P>(&mut self, runtime_input: Option<RI>, event_port: &mut P) -> RO
    where
        P: ServerPort<SI, SO>,
    {
        // Fetch inputs from port.
        let sim_in = event_port.stepper_read();

        // Construct StepInput instance.
        let step_in = StepInput::new(sim_in, runtime_input);

        // Do step and store returned value.
        let (step_out, run_out) = self.simulation.step(step_in);

        // Queue consequential input to port for broadcasting.
        event_port.step_output_to_all(step_out);
    }
}
