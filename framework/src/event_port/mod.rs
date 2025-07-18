pub trait ServerPort<SI, SO> {
    /// Return client inputs to stepper.
    fn stepper_read(&mut self) -> SI;

    /// Broadcast the step input used on server for all clients
    /// to replicate.
    fn sim_out_to_all(&mut self, stepper_output: SO);

    /// Send messages at the end of each step.
    fn send(&mut self);
}

pub trait ClientPort<SI, ToServer> {
    /// Read stepper input received from server.
    fn stepper_read(&mut self) -> SI;

    /// The contribution from this client to the total input on the
    /// server.
    fn client_to_server(&mut self, message: ToServer);

    /// Send messages at the end of each step.
    fn send(&mut self);
}
