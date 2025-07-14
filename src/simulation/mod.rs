use std::marker::PhantomData;

pub struct Simulation<Event, State, L>
where
    L: Logic<Event, State>,
{
    step_count: u64,
    logic: L,
    state: State,
    event_type: PhantomData<Event>,
}

impl<Event, State, L> Simulation<Event, State, L>
where
    L: Logic<Event, State>,
{
    pub fn new(state: State, logic: L) -> Self {
        Self {
            step_count: 0,
            logic,
            state,
            ..Default::default()
        }
    }

    pub fn from_save(step_count: u64, state: State, logic: L) -> Self {
        Self {
            step_count,
            logic,
            state,
            ..Default::default()
        }
    }

    pub fn step_count(&self) -> u64 {
        self.step_count
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn step(&mut self, events: &[Event]) -> Vec<usize> {
        self.step_count += 1;
    }
}

/// Implement to define logic for simulation.
///
/// Can contain local state that will not be considered as part of the
/// simulation. The given `state` type should only contain data that
/// needs to be replicated by other viewers/participants in the
/// simulation. **Do not store temporary or machine specific data in
/// the State type, store it the [`Logic`] implementation.**
pub trait Logic<Event, State> {
    fn step(&mut self, &mut state: State, events: &[Event]) -> Vec<usize> {
        self.step_count += 1;
    }
}

// ----------------------------------------------------------------------
// pub trait StepInput<Event> {
//     fn events(&self) -> &[Event];
// }

// #[cfg_attr(docsrs, cfg(any(feature = "easy_step_io", feature = "easy_step_in")))]
// #[cfg(any(feature = "easy_step_io", feature = "easy_step_in"))]
// impl<Event> StepInput<Event> for &[Event] {
//     fn events(&self) -> &[Event] {
//         *self
//     }
// }

// pub trait StepOutput {
//     /// Returns indices of events that lead to a delta in the
//     /// simulation state. The indices point to events in the
//     /// slice of events that were given as an argument to the
//     /// `step` method.
//     ///
//     /// This set of events can be used to
//     /// re-simulate the last step perfectly, wherever another
//     /// simulation with the same tick count and state uses them.
//     fn consequential(&self) -> &[usize];
// }

// #[cfg_attr(docsrs, cfg(any(feature = "easy_step_io", feature = "easy_step_out")))]
// #[cfg(any(feature = "easy_step_io", feature = "easy_step_out"))]
// impl StepOutput for Vec<usize> {
//     fn consequential(&self) -> &[usize] {
//         &self
//     }
// }
