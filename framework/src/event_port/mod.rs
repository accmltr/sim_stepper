pub trait ServerPort<SI, SO> {
    /// Return client inputs to stepper.
    fn stepper_read(&mut self) -> SI;

    /// Broadcast the step input used on server for all clients
    /// to replicate.
    fn sim_out_to_all(&mut self, stepper_output: SO);

    /// Send messages at the end of each step.
    fn send(&mut self);
}

pub trait ClientPort<SIFromServer, ClientSIToServer> {
    /// Read stepper input received from server.
    fn stepper_read(&mut self) -> SIFromServer;

    /// Message from client to server.
    fn sim_in_to_server(&mut self, message: ClientSIToServer);

    /// Send messages at the end of each step.
    fn send(&mut self);
}

/// 1. Creates an endpoint for either client or server.
/// 2. Accepts QUIC connections and does auth.
/// 3. If auth success, creates port connection.
/// 4. Collects messages from senders to simulation, between read calls.
/// Note: Locally "sent" messages are tagged as such.
pub trait Port<Id, SI, IO>
where
    Id: Eq,
{
    /// Return messages received since last `send()` call by connection ID.
    fn read(&self) -> Vec<ReceivedMessages<Id, SI>>;

    fn send(&mut self, recipient_id: Id, message: IO) {}
}

pub struct ReceivedMessages<Id, Message> {
    id: Id,
    received_locally: bool,
    messages: Vec<Message>,
}

impl<Id, Message> ReceivedMessages<Id, Message> {
    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn received_locally(&self) -> bool {
        self.received_locally
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }
}
