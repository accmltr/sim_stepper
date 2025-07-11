use crate::Simulation;

mod simple_stepper;

/// The direct interface between the runtime and this library.
/// [[Runner]]  manages a [[Simulation]] by collecting all messages
/// for the next tick, then running the next tick.
pub trait Stepper<Event, Response, State, S>
where
    S: Simulation<Event, Response, State>,
{
    fn step(&mut self, local_events: &Vec<Event>);
    fn simulation(&self) -> &S;
}

// rewind runner: applies locally generated messages in the next tick, then
//

/// Applies
pub struct RewindStepper {}

/// Caches recent simulation states and then replays with valid messages.
pub struct ResetStepper {}

// struct ComboStepper;
