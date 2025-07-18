use crate::event_port::ServerPort;
use crate::simulation::Simulation;
use crate::simulation::StepLogic;

pub use server_stepper::ServerStepper;

mod server_stepper;

pub trait ClientStepper<StepInput, StepOutput, State>
where
    State: StepLogic<StepInput, StepOutput>,
{
    fn simulation(&self) -> &Simulation<StepInput, StepOutput, State>;

    fn step<P>(&mut self, event_port: &mut P) -> StepOutput
    where
        P: ServerPort<StepInput, StepOutput>;
}
