/// Connects to described server, sends locally generated events and
/// reads server event consequential events.
///
/// DOES:
/// - Convert incoming batched events into simulation events.
/// - Convert local generated, outgoing events into raw events.
pub mod client;

/// Accepts all attempted connections and opens channel for incoming and
/// outgoing events with clients.
///
/// DOES:
/// - Assume events have unique sources.
/// - Assume unique sources are identified by connection UUID.
/// - Batch outgoing source events by source ID.
/// - How to convert raw incoming events to simulation events by adding
///   source ID.
///
/// DOES NOT:
/// - Auth connections before opening event streams.
pub mod server;
