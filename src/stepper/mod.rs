use crate::{Simulation, simulation::StepResult};

pub mod steppers_implemented;

/// The direct interface between the runtime and this library.
/// [[Runner]]  manages a [[Simulation]] by collecting all messages
/// for the next tick, then running the next tick.
///
/// Usually you would include the auther of the event as a field in
/// your event struct.
pub trait Stepper<Event, R, State, S>
where
    R: StepResult<Event>,
    S: Simulation<Event, State, R>,
{
    fn step(&mut self, local_events: &[Event]);
    fn simulation(&self) -> &S;
}
