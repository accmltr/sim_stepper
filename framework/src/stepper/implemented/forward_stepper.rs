use crate::{
    event_port::Port,
    simulation::{Simulation, StepLogic},
    stepper::Stepper,
};

/// Only steps forwards, does not feature roleback functionality. Great
/// for RTS games that usually do not have a need for low perceived
/// latency. *This is a great option to test out this library with.*
pub struct ForwardStepper<Input, Output, State: StepLogic<Input, Output>> {
    simulation: Simulation<Input, Output, State>,
}

impl<Input, Output, State: StepLogic<Input, Output>> ForwardStepper<Input, Output, State> {
    pub fn new(simulation: Simulation<Input, Output, State>) -> Self {
        Self { simulation }
    }
}

impl<Input, Output, State: StepLogic<Input, Output>> Stepper<Input, Output, State>
    for ForwardStepper<Input, Output, State>
{
    fn simulation(&self) -> &Simulation<Input, Output, State> {
        &self.simulation
    }

    fn step<P>(&mut self, event_port: &mut P)
    where
        P: Port<Input, Output>,
    {
        // Fetch events from port.
        let events = event_port.read_events();

        // Do step and store consequential event indices.
        let output = self.simulation.step(events);

        // Hand over consequential events to port for broadcasting.
        event_port.outbox_consequential(output);
    }
}
