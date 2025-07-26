//! Create a game that is also a deterministic, event-based simulation and get
//! reliable multiplayer functionality, implemented for you.
//!
//! Purpose:
//! - Replicates determistic simulations accross QUIC connections.
//! - Accepts runtime/local connections for local event generation and custom
//!   connection solutions to be handled by runtime.
//! - Provides various simulation managers which handle message passing between
//!   local simulation(s) and the port.
//! - Provides port implementation for networking functionality via QUIC.
pub mod event_port;
pub mod simulation;
pub mod stepper;
