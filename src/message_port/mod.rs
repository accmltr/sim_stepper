pub trait ClientPort<M> {
    /// Send message to server, reliably and ordered.
    fn send(&self, message: M);

    /// Returns messages received since last `read()`.
    fn read(&mut self) -> Vec<M>;
}

pub trait ServerPort<M, I> {
    fn send(&self, message: M, agent_id: I) -> Result<(), ServerPortSendError>;

    fn read(&mut self) -> Vec<(I, M)>;
}

pub enum ServerPortSendError {
    NotConnected,
}
