use std::{collections::HashMap, marker::PhantomData};

pub type StepInput<EventSourceId, Event> = HashMap<EventSourceId, Vec<Event>>;

pub struct Simulation<State, Event, EventSourceID>
where
    EventSourceID: Eq,
    State: StepLogic<Event, EventSourceID>,
{
    step_count: u64,
    state: State,
    event_source_id: PhantomData<EventSourceID>,
    event_type: PhantomData<Event>,
}

impl<State, Event, EventSourceID> Simulation<State, Event, EventSourceID>
where
    EventSourceID: Eq,
    State: StepLogic<Event, EventSourceID>,
{
    pub fn new(state: State) -> Self {
        Self {
            step_count: 0,
            state,
            event_source_id: PhantomData,
            event_type: PhantomData,
        }
    }

    pub fn from_save(step_count: u64, state: State) -> Self {
        Self {
            step_count,
            state,
            event_source_id: PhantomData,
            event_type: PhantomData,
        }
    }

    pub fn step_count(&self) -> u64 {
        self.step_count
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn step(
        &mut self,
        input: StepInput<EventSourceID, Event>,
    ) -> StepInput<EventSourceID, Event> {
        // Increment step count.
        self.step_count += 1;
        // Run step logic and return consequential events.
        self.state.step(self.step_count, input)
    }
}

pub trait StepLogic<Event, EventSourceID>
where
    EventSourceID: Eq,
{
    fn step(
        &mut self,
        step_count: u64,
        input: StepInput<EventSourceID, Event>,
    ) -> StepInput<EventSourceID, Event>;
}
