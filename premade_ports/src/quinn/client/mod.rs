use framework::{
    event::Event,
    event_port::{ClientPort, Port},
};

pub struct QuinnClientPort {}

impl<E, EventSourceID> Port<E, EventSourceID> for QuinnClientPort
where
    E: Event,
    EventSourceID: Eq,
{
    fn read_events(&mut self) -> std::collections::HashMap<EventSourceID, Vec<E>> {
        todo!()
    }

    fn step(&mut self) {
        todo!()
    }
}

impl<E, EventSourceID> ClientPort<E, EventSourceID> for QuinnClientPort
where
    E: Event,
    EventSourceID: Eq,
{
    fn queue_client_events_to_server(&mut self, _events: Vec<E>) {
        todo!()
    }
}
