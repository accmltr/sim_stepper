pub trait ServerPort<StepperInput, StepperOutput> {
    /// Return client inputs to stepper.
    fn stepper_read(&mut self) -> StepperInput;

    /// Broadcast the step input used on server for all clients
    /// to replicate.
    fn step_input_to_all(&mut self, stepper_output: StepperOutput);

    /// Send messages at the end of each step.
    fn send(&mut self);
}

pub trait ClientPort<StepperInput, ToServer> {
    /// Read stepper input received from server.
    fn stepper_read(&mut self) -> StepperInput;

    /// The contribution from this client to the total input on the
    /// server.
    fn client_to_server(&mut self, message: ToServer);

    /// Send messages at the end of each step.
    fn send(&mut self);
}
