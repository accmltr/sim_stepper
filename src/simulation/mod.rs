use crate::{message::Message, message_data::MessageData};

pub trait Simulation<D: MessageData> {
    /// Process simulation one step and returns consequential messages.
    fn step(&self, messages: Vec<Message<D>>) -> Vec<Message<D>>;

    /// The current tick count.
    fn tick_count(&self) -> u64;
}
