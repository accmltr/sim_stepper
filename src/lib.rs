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
