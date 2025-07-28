use framework::event_port::{ClientPort, Port, ServerPort};

pub mod quinn;

pub struct QuinnServerPort {}

impl<Event, EventSourceID> Port<Event, EventSourceID> for QuinnServerPort
where
    EventSourceID: Eq,
{
    fn read_events(&mut self) -> std::collections::HashMap<EventSourceID, Vec<Event>> {
        todo!()
    }

    fn step(&mut self) {
        todo!()
    }
}

impl<Event, EventSourceID> ServerPort<Event, EventSourceID> for QuinnServerPort
where
    EventSourceID: Eq,
{
    fn queue_runtime_events(
        &mut self,
        _step_input: framework::simulation::StepInput<EventSourceID, Event>,
    ) {
        todo!()
    }

    fn queue_server_step_input_to_clients(
        &mut self,
        _step_input: framework::simulation::StepInput<EventSourceID, Event>,
    ) {
        todo!()
    }
}

pub struct QuinnClientPort {}

impl<Event, EventSourceID> Port<Event, EventSourceID> for QuinnClientPort
where
    EventSourceID: Eq,
{
    fn read_events(&mut self) -> std::collections::HashMap<EventSourceID, Vec<Event>> {
        todo!()
    }

    fn step(&mut self) {
        todo!()
    }
}

impl<Event, EventSourceID> ClientPort<Event, EventSourceID> for QuinnClientPort
where
    EventSourceID: Eq,
{
    fn queue_client_events_to_server(&mut self, _events: Vec<Event>) {
        todo!()
    }
}
