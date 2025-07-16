use std::marker::PhantomData;

pub struct Simulation<Input, Output, State: StepLogic<Input, Output>> {
    step_count: u64,
    state: State,
    input_type: PhantomData<Input>,
    output_type: PhantomData<Output>,
}

impl<Input, Output, State: StepLogic<Input, Output>> Simulation<Input, Output, State> {
    pub fn new(state: State) -> Self {
        Self {
            step_count: 0,
            state,
            input_type: PhantomData,
            output_type: PhantomData,
        }
    }

    pub fn from_save(step_count: u64, state: State) -> Self {
        Self {
            step_count,
            state,
            input_type: PhantomData,
            output_type: PhantomData,
        }
    }

    pub fn step_count(&self) -> u64 {
        self.step_count
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn step(&mut self, input: Input) -> Output {
        // Increment step count.
        self.step_count += 1;
        // Run step logic and return consequential events.
        self.state.step(self.step_count, input)
    }
}

/// Implement to define logic for simulation.
///
/// Can contain local state that will not be considered as part of the
/// simulation. The given `state` type should only contain data that
/// needs to be replicated by other viewers/participants in the
/// simulation. **Do not store temporary or machine specific data in
/// the State type, store it the [`Logic`] implementation, e.g. auto-
/// clicker state for player controller.**
///
/// *Seperating simulation state from logic and temporary state makes
/// it much easier to consider simulation replication while developing
/// a project.*
///
/// NB: The `step` method needs to return only events that were used
/// and effected a change in the simulation state. Make sure to include
/// every event that had an effect on the state, otherwise other
/// replications of this simulation will fall out of sync and will need
/// to be reconciled. Including too events that did not have an effect
/// will take a toll on event traffic and simulation history volume.
pub trait StepLogic<Input, Output> {
    fn step(&mut self, step_count: u64, input: Input) -> Output;
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
