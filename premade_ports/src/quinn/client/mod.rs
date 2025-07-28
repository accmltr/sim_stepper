use framework::event_port::{ClientPort, Port};

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
