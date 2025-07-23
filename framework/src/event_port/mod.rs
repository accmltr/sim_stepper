/// 1. Creates an endpoint for either client or server.
/// 2. Accepts QUIC connections and does auth.
/// 3. If auth success, creates port connection.
/// 4. Collects and coalesses messages from senders to simulation, between
///    `send()` calls.
/// Note: Locally "sent" messages are tagged as such.
pub trait Port<SI> {
    /// Return client inputs to stepper.
    fn stepper_read(&mut self) -> SI;

    /// Send messages at the end of each step.
    fn send(&mut self);
}

pub trait ClientPort<SIFromServer, ClientSIToServer>: Port<SIFromServer> {
    /// Message from client to server.
    fn sim_in_to_server(&mut self, message: ClientSIToServer);
}

pub trait ServerPort<SI, SO>: Port<SI> {
    /// Broadcast the step input used on server for all clients
    /// to replicate.
    fn sim_out_to_all(&mut self, stepper_output: SO);
}

pub struct TotalStepInput<Id, Event> {
    all_coalessed: Vec<CoalessedEvents<Id, Event>>,
}

impl<Id, Event> TotalStepInput<Id, Event> {
    pub fn all_coalessed(&self) -> &Vec<CoalessedEvents<Id, Event>> {
        &self.all_coalessed
    }
}

pub struct CoalessedEvents<Id, Event> {
    id: Id,
    received_locally: bool,
    events: Vec<Event>,
}

impl<Id, Message> CoalessedEvents<Id, Message> {
    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn received_locally(&self) -> bool {
        self.received_locally
    }

    pub fn events(&self) -> &Vec<Message> {
        &self.events
    }
}
