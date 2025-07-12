/// The purpose of this library is to provide a framework for creating
/// deterministic simulations, along with useful implementations for
/// networked solutions.
///
/// The basic idea is that events are stepped over in a simulation.
/// These events will come from participants in the simulation.
/// Participants are observers of all source events that go through
/// the authentic simulation and could send events to the simulation
/// based on how you
///
/// Stepper:
///
/// Contains the Simulation and fetches events from the Port when
/// stepped. Its job is to determine which events will be stepped over
/// and which messages will be sent out to routes in the port.
///
/// Some steppers, for clients, might step over events that are from
/// the local runtime emmediatly, and cache the simulation's state
/// while sending those same events to the server. It will then keep
/// an eye out for whne those events are included in the timeline of
/// the authentic simulation on the server and the revert to the state
/// before the event was stepped over locally and re-step the history
/// from then to the latest tick. This way the local simulation's
/// event history and state are identical to the server's. Rejected
/// events can be sent from the servers runtime, via the server's port
/// and read on the port by the local stepper to rectify the local
/// simulation *1. In this case, the stepper, runtime
/// and port implentations all work together to add a new feature to
/// the lock step model: rolebacks. Rolebacks improve perceived
/// latency for clients, but can be hard to render when the local state
/// is reconciled with the server's. At least the reconciliation part
/// of the roleback solution can be simplified greatly by this
/// framework, and simulations can be deterministic even when a roleback
/// feature is introduced.
///
///
///
/// Port:
///
/// Handles routes
///
/// Messages Vs Events:
///
/// Messages is what the port receives, and sends on demand.
///
/// Events are processed by a simulation to change its state.
///
/// The runtimes owns the port and the stepper. It passes a ref of the
/// port to the stepper when it calls step on it. This way port may be
/// replacable during runtime. The port handles in messages while
/// the stepper handles in events to keep a seperation of concerns, but
/// this also enables the runtime to send and read messages through the
/// port. E.g. now you can send chat messages, or results of invalid
/// events that you cache in your simulation, next to, but not in, the
/// simulation state; so it does not get included in the state *1.
///
///
/// Fetches the messages from a port each time it is stepped.hhhhhhhhhhh
///
/// If you want to create an offline deterministic simulation or game,
/// you only need to use the Simulation trait.
///
/// If you want to create a lock-step simulation over a network, use
/// the SimpleStepper and provide it with a message Port of your
/// choice/creation. When creating or using an pre-made Port, you will
/// need to define how messages directed to certain agents will be
/// sent reliably, and maybe even ordered.
///
/// Simulation
///    OR
/// Simulation || Stepper & Port
///
// Re-exports
pub use simulation::Simulation;

// Mods
/// Manages connections and messages from and to connections.
mod port;
/// Lock step simulation.
mod simulation;
/// Manages simulation and based on message incoming and outgoing messages.
mod stepper;

// Requirements:
//
// - Simulates accross network deterministically.
// - Server auth simulation.
// - Connection authentication plugability.
//
// Impossible features:
// - Scoped messages for agents: This introduces too much complexity.
