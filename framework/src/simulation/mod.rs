use std::marker::PhantomData;

/// Generic Params:
/// `RI` - Runtime In. Can be used by runtime to request that the simulation
/// gather certain information during the step process and return it as `RO`.
/// `RO` - Runtime Out. Returned information for runtime.
pub struct Simulation<State, SI, SO, RI, RO>
where
    State: StepLogic<SI, SO, RI, RO>,
{
    step_count: u64,
    state: State,
    sim_in_type: PhantomData<SI>,
    sim_out_type: PhantomData<SO>,
    runtime_in_type: PhantomData<RI>,
    runtime_out_type: PhantomData<RO>,
}

impl<State, SI, SO, RI, RO> Simulation<State, SI, SO, RI, RO>
where
    State: StepLogic<SI, SO, RI, RO>,
{
    pub fn new(state: State) -> Self {
        Self {
            step_count: 0,
            state,
            sim_in_type: PhantomData,
            sim_out_type: PhantomData,
            runtime_in_type: PhantomData,
            runtime_out_type: PhantomData,
        }
    }

    pub fn from_save(step_count: u64, state: State) -> Self {
        Self {
            step_count,
            state,
            sim_in_type: PhantomData,
            sim_out_type: PhantomData,
            runtime_in_type: PhantomData,
            runtime_out_type: PhantomData,
        }
    }

    pub fn step_count(&self) -> u64 {
        self.step_count
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn step(&mut self, input: StepInput<SI, RI>) -> (SO, RO) {
        // Increment step count.
        self.step_count += 1;
        // Run step logic and return consequential events.
        self.state.step(self.step_count, input)
    }
}

pub trait StepLogic<SI, SO, RI, RO> {
    fn step(&mut self, step_count: u64, input: StepInput<SI, RI>) -> (SO, RO);
}

pub struct StepInput<SI, RI> {
    simulation_input: SI,
    runtime_input: Option<RI>,
}

impl<SI, RI> StepInput<SI, RI> {
    pub fn new(simulation_input: SI, runtime_input: Option<RI>) -> Self {
        Self {
            simulation_input,
            runtime_input,
        }
    }

    pub fn simulation_in(&self) -> &SI {
        &self.simulation_input
    }

    pub fn runtime_in(&self) -> &Option<RI> {
        &self.runtime_input
    }
}
