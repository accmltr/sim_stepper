use crate::simulation::StepOutput;

pub trait ResponseStepResult<Message>: StepOutput<Message> {
    fn results(&self) -> &[usize];
}

pub trait OutBoxStepResult<Incoming, OutGoing>: StepOutput<Incoming> {
    fn outbox(&self) -> Vec<OutGoing>;
}
