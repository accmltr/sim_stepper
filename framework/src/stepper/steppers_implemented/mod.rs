pub mod forward_stepper;
// ---------------------------------------------------------------------
// /// Runs one simulation authentically, while another one diverges by
// /// stepping over local events emediatly. Both simulations step over
// /// authentic events received from the server emediatly.
// ///
// /// When the local events make it to the server and are received back
// /// or rejected by the server, then the local events should be deleted
// /// from the history of the diverging timeline; either via undoing and
// /// replaying the history without the events, or via resetting to the
// /// state before the events and replaying without them included in the
// /// history.
// pub mod dual_stepper;
// ---------------------------------------------------------------------
//
// // Other steppers:
// //
// // rewind runner: applies locally generated messages in the next tick,
// // then
// //

// /// Applies
// pub struct RewindStepper {}

// /// Caches recent simulation states and then replays with valid messages
// /// when valid or invalid response for locally applied messages received,
// /// or when timout happens.
// pub struct ResetStepper {}

// // struct ComboStepper;
// ---------------------------------------------------------------------
