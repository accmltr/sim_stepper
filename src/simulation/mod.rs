pub mod state;

pub trait Simulation<Event, Response, State> {
    /// Changes simulation based on tick incrementing and consequential
    /// messages given.
    ///
    /// Returns [`EventResult`] for each given event. So `events` and
    /// returned [`Vec`] need to have same `len()`.
    fn step(&self, events: &[Event]) -> Vec<EventOutcome<Response>>;

    /// The current tick count.
    fn tick_count(&self) -> u64;

    /// Return the inner state of the simulation.
    fn state(&self) -> &State;
}

pub struct EventOutcome<Response> {
    /// The event index as found in the ordered list of events given to
    /// the `Simulation::step` method.
    index: usize,
    /// Whether the event changed the state of the simulation.
    consequential: bool,
    /// Optional response to event for external use.
    response: Option<Response>,
}
