use crate::event_port::Port;
use crate::simulation::Simulation;
use crate::simulation::StepLogic;

pub use implemented::ForwardStepper;

mod implemented;

/// Runs your simulation for you based on incoming events, that are read
/// by the stepper from a given [`Port`] reference.
///
pub trait Stepper<Input, Output, State>
where
    State: StepLogic<Input, Output>,
{
    /// Read only access to [`Simulation`].
    fn simulation(&self) -> &Simulation<Input, Output, State>;

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
        P: Port<Input>;
}
