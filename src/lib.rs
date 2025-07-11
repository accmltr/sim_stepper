// Re-exports
pub use simulation::Simulation;

// Mods
/// Message type going through port > stepper > simulation and back.
mod message;
/// Types to handle communication and verficiation of connections for
/// server and clients.
mod message_port;
/// Deterministic lock step simulation.
mod simulation;
/// Manages simulation and message passing.
mod stepper;

// Requirements:
//
// - Simulates accross network deterministically.
// - Server auth simulation.
// - Connection authentication plugability.
