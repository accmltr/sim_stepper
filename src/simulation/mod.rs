pub trait Simulation<Event, State, R>
where
    R: StepResult<Event>,
{
    /// Changes simulation based on tick incrementing and consequential
    /// messages given.
    ///
    /// Returns [`EventResult`] for each given event. So `events` and
    /// returned [`Vec`] need to have same `len()`.
    fn step(&self, events: &[Event]) -> R;

    /// The current tick count.
    fn tick_count(&self) -> u64;

    /// Return the inner state of the simulation.
    fn state(&self) -> &State;
}

pub trait StepResult<Event> {
    /// Returns indices of events that lead to a delta in the
    /// simulation state. The indices point to events in the
    /// slice of events that were given as an argument to the
    /// `step` method.
    ///
    /// This set of events can be used to
    /// re-simulate the last step perfectly, wherever another
    /// simulation with the same tick count and state uses them.
    fn consequential(&self) -> Vec<usize>;
}
