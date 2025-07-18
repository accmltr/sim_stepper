use std::marker::PhantomData;

/// Generic Params:
/// `RI` - Runtime In. Can be used by runtime to request that the simulation
/// gather certain information during the step process and return it as `RO`.
/// `RO` - Runtime Out. Returned information for runtime.
pub struct Simulation<State, I, O, SI, SO, RI, RO>
where
    I: StepI<SI, RI>,
    O: StepO<SO, RO>,
    State: StepLogic<I, O, SI, SO, RI, RO>,
{
    step_count: u64,
    state: State,
    input_type: PhantomData<SI>,
    output_type: PhantomData<SO>,
    runtime_in_type: PhantomData<RI>,
    runtime_out_type: PhantomData<RO>,
    step_in_type: PhantomData<I>,
    step_out_type: PhantomData<O>,
}

impl<State, I, O, SI, SO, RI, RO> Simulation<State, I, O, SI, SO, RI, RO>
where
    I: StepI<SI, RI>,
    O: StepO<SO, RO>,
    State: StepLogic<I, O, SI, SO, RI, RO>,
{
    pub fn new(state: State) -> Self {
        Self {
            step_count: 0,
            state,
            input_type: PhantomData,
            output_type: PhantomData,
            runtime_in_type: PhantomData,
            runtime_out_type: PhantomData,
            step_in_type: PhantomData,
            step_out_type: PhantomData,
        }
    }

    pub fn from_save(step_count: u64, state: State) -> Self {
        Self {
            step_count,
            state,
            input_type: PhantomData,
            output_type: PhantomData,
            runtime_in_type: PhantomData,
            runtime_out_type: PhantomData,
            step_in_type: PhantomData,
            step_out_type: PhantomData,
        }
    }

    pub fn step_count(&self) -> u64 {
        self.step_count
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn step(&mut self, input: I) -> O {
        // Increment step count.
        self.step_count += 1;
        // Run step logic and return consequential events.
        self.state.step(self.step_count, input)
    }
}

pub trait StepLogic<I, O, SI, SO, RI, RO>
where
    I: StepI<SI, RI>,
    O: StepO<SO, RO>,
{
    fn step(&mut self, step_count: u64, input: I) -> O;
}

pub trait StepI<SI, RI> {
    fn simulation_in(&self) -> &SI;
    fn runtime_in(&self) -> &Option<RI>;
}

pub trait StepO<SO, RO> {
    fn simulation_out(&self) -> &SO;
    fn runtime_out(&self) -> &Option<RO>;
}
