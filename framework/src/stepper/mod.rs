use crate::event_port::EventPort;
use crate::simulation::Simulation;
use crate::simulation::StepLogic;

pub mod steppers_implemented;

/// Runs your simulation for you based on incoming events, that are read
/// by the stepper from a given [`Port`] reference.
///
pub trait Stepper<Event, State>
where
    State: StepLogic<Event>,
{
    /// Read only access to [`Simulation`].
    fn simulation(&self) -> &Simulation<Event, State>;

    /// Reads events from port and step simulation with those events.
    ///
    /// ## Note:
    /// This does not call `send()` on the given port. This is so that
    /// other interested parties, like the runtime, still has a chance
    /// to asses the outcome of this step and include their own messages
    /// to the port outbox before sending happens once per frame. The
    /// runtime or port should be responsible for sending after each
    /// step - **only once**.
    ///
    fn step<P>(&mut self, event_port: &mut P)
    where
        P: EventPort<Event>;
}
