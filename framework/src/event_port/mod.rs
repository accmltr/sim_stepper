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
