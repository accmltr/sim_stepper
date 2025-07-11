/// The direct interface between the runtime and this library.
/// [[Runner]]  manages a [[Simulation]] by collecting all messages
/// for the next tick, then running the next tick.
pub trait Stepper<M> {
    fn process_message(&mut self, msg: M);
    fn process_message_batch(&mut self, msg: Vec<M>);
}

// forward runner: only ticks forward, never reverting simulation
pub struct ForwardStepper {}
// rewind runner: applies locally generated messages in the next tick, then
//

/// Applies
pub struct RewindStepper {}

/// Caches recent simulation states and then replays with valid messages.
pub struct ResetStepper {}

// struct ComboStepper;
