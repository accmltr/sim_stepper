use crate::message::Message;

pub trait Simulation<M: Message, S> {
    /// Process simulation one step and returns consequential messages.
    fn step(&self, messages: &Vec<M>) -> Vec<M>;

    /// The current tick count.
    fn tick_count(&self) -> u64;

    /// Return the inner state of the simulation.
    fn state(&self) -> &S;
}
