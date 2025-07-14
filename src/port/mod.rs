pub mod connection;

pub trait Port<Event> {
    /// Returns simulation events received since last `send` call.
    fn read_events(&mut self) -> &[Event];

    /// Adds events event outbox for broadcasting to all parties who want
    /// to replicate the simulation state and tick count.
    fn write_events(&mut self, events: Vec<Event>);

    /// Called once at the end of each simulation step. Clears event inbox,
    /// then empties and sends contents of event outbox.
    fn send(&mut self);
}

// /// [`RouteId`] is used to tell the [`Port`] whom to send the message to.
// ///
// /// This way you could use or create a [`Port`] that lets your specify
// /// via [`RouteId`] whether you want to:
// ///  - Broadcast a message.
// ///  - Send a message to a specific recipient, e.g. a chat message or
// ///    a response to an invalid event received earlier for rolebacks.
// ///  - Send a message to the local runtime.
// ///
// /// Ports are can also be used to authenticate connections.
// ///
// /// E.g. the port might only accept events from connections after
// /// verifying player accounts and participant status in lobby. Only then,
// /// converting client events into simulation events with player number
// /// attached included in the type. In this case there will be 3 event
// /// types, client events (verified by port), simulation events (in/out
// /// for simulation), server events (including player number, similar to
// /// simulation event).
// ///
// /// # Generic Params
// /// `Event` - Event type that is passed to local [`Simulation`]. *Usually
// /// contains event source identifier.*
// /// `Incoming` - Incoming message type. *Usually an enum with a variant
// /// for a incoming client events without source identifier, to save on
// /// bandwidth.*
// /// `Outgoing` - Usually an enum with many types of outgoing messages
// /// (like chat messages, lobby status updates etc), which also has a
// /// variant for an outgoing event that includes the source identifier
// /// for the event, so that connections can see the output of the step
// /// call on the server/authentic simulation.
// pub trait UsualPort<Event, Incoming, Outgoing> {
//     /// Returns simulation events received since last `send` call.
//     fn read_events(&mut self) -> &[Event];

//     /// Returns all [`Incoming`] messages received since last `send` call.
//     fn read_all(&mut self) -> &[Incoming];

//     /// Clears inboxes, then empties and sends contents of all outboxes.
//     /// Expected to only get called once at the end of every step.
//     fn send(&mut self);

//     fn add_to_broadcast_outbox(&mut self, messages: Vec<Outgoing>);

//     // /// Read buffer cleared when send is called.
//     // fn send(&mut self, outbox: Vec<OutboxCapsule<Message, RouteId>>);
// }

// /// Outgoing messages are stored in a [`OutboxCapsule`] to differentiate
// /// them from incoming messages, which are stored in an [`InboxCapsule`],
// /// even though both structs contain the same type of data.
// pub struct OutboxCapsule<Message, RouteId> {
//     destination: RouteId,
//     message: Message,
// }

// pub struct InboxCapsule<Message, RouteId> {
//     sender: RouteId,
//     message: Message,
// }
