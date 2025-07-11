use crate::message_data::MessageData;

type AgentNo = u16;

/// Used to associate message data with agent.
pub struct Message<D: MessageData> {
    agent_no: AgentNo,
    data: D,
}

impl<D: MessageData> Message<D> {
    pub fn new(agent_no: AgentNo, data: D) -> Self {
        Self { agent_no, data }
    }

    pub fn agent_no(&self) -> AgentNo {
        self.agent_no
    }

    pub fn data(&self) -> &D {
        &self.data
    }
}
