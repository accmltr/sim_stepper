use crate::{
    event_port::EventPort,
    simulation::{Simulation, StepLogic},
    stepper::Stepper,
};

/// Only steps forwards, does not feature roleback functionality. Great
/// for RTS games that usually do not have a need for low perceived
/// latency. *This is a great option to test out this library with.*
pub struct ForwardStepper<Event, State: StepLogic<Event>> {
    simulation: Simulation<Event, State>,
}

impl<Event, State: StepLogic<Event>> Stepper<Event, State> for ForwardStepper<Event, State>
where
    Event: Clone,
{
    fn simulation(&self) -> &Simulation<Event, State> {
        &self.simulation
    }

    fn step<P>(&mut self, event_port: &mut P)
    where
        P: EventPort<Event>,
    {
        // Fetch events from port.
        let events = event_port.read_events();

        // Do step and store consequential event indices.
        let indices = self.simulation.step(events);

        // Find and clone consequential events into vector.
        let consequential = indices.iter().map(|i| events[*i].clone()).collect();

        // Hand over consequential events to port for broadcasting.
        event_port.outbox_consequential(consequential);
    }
}
