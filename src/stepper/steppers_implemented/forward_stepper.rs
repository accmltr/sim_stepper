use crate::{port::Port, simulation::Simulation, stepper::Stepper};

use std::marker::PhantomData;

/// Only steps forwards, does not feature roleback functionality. Great
/// for RTS games that usually do not have a need for low perceived
/// latency. *This is a great option to test out this library with.*
pub struct ForwardStepper<Event, State, S>
where
    S: Simulation<Event, State>,
{
    simulation: S,
    _phantom: PhantomData<(Event, State)>,
}

impl<Event, State, S> Stepper<Event, State, S> for ForwardStepper<Event, State, S>
where
    Event: Clone,
    S: Simulation<Event, State>,
{
    fn simulation(&self) -> &S {
        &self.simulation
    }

    fn step<P>(&mut self, port: &mut P)
    where
        P: Port<Event>,
    {
        // Fetch events from port.
        let events = port.read_events();

        // Do step and store consequential event indices.
        let indices = self.simulation.step(events);

        // Find and clone consequential events into vector.
        let consequential = indices.iter().map(|i| events[*i].clone()).collect();

        // Add consequential events to port event outbox for
        // broadcasting.
        port.write_events(consequential);
    }
}
