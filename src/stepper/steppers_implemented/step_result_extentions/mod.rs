use crate::simulation::StepResult;

pub trait ResponseStepResult<Message>: StepResult<Message> {
    fn results(&self) -> &[usize];
}

pub trait OutBoxStepResult<Incoming, OutGoing>: StepResult<Incoming> {
    fn outbox(&self) -> Vec<OutGoing>;
}
