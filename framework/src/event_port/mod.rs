use std::collections::HashMap;

use crate::{event::Event, simulation::StepInput};

/// 1. Creates an endpoint for either client or server.
/// 2. Accepts QUIC connections and does auth.
/// 3. If auth success, creates port connection.
/// 4. Collects and coalesses messages from senders to simulation, between
///    `send()` calls.
/// Note: Locally "sent" messages are tagged as such.
pub trait Port<E, EventSourceID>
where
    E: Event,
    EventSourceID: Eq,
{
    /// Return client inputs to stepper.
    fn read_events(&mut self) -> HashMap<EventSourceID, Vec<E>>;

    /// Send messages at the end of each step and clear inboxes etc.
    fn step(&mut self);
}

pub trait ServerPort<E, EventSourceID>: Port<E, EventSourceID>
where
    E: Event,
    EventSourceID: Eq,
{
    fn queue_runtime_events(&mut self, step_input: StepInput<EventSourceID, E>);
    fn queue_server_step_input_to_clients(&mut self, step_input: StepInput<EventSourceID, E>);
}

pub trait ClientPort<E, EventSourceID>: Port<E, EventSourceID>
where
    E: Event,
    EventSourceID: Eq,
{
    fn queue_client_events_to_server(&mut self, events: Vec<E>);
}
