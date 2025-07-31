use framework::event::Event;
use tokio::sync::mpsc::Receiver;

pub struct ClientConnection<EventSourceID, E>
where
    E: Event,
    EventSourceID: Eq,
{
    pub event_source_id: EventSourceID,
    pub event_receiver: Receiver<E>,
}
