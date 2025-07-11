use crate::{Simulation, message::Message};

mod simple_stepper;

/// The direct interface between the runtime and this library.
/// [[Runner]]  manages a [[Simulation]] by collecting all messages
/// for the next tick, then running the next tick.
pub trait Stepper<M: Message, T, S: Simulation<M, T>> {
    fn step(&mut self, local_messages: &Vec<M>);
    fn simulation(&self) -> &S;
}

// rewind runner: applies locally generated messages in the next tick, then
//

/// Applies
pub struct RewindStepper {}

/// Caches recent simulation states and then replays with valid messages.
pub struct ResetStepper {}

// struct ComboStepper;
