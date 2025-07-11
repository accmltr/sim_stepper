use super::Stepper;
use crate::{message::Message, simulation::Simulation};

use std::marker::PhantomData;

/// Only steps forward, very simple.
pub struct SimpleStepper<M: Message, T, S: Simulation<M, T>> {
    message_data_type: PhantomData<M>,
    state_type: PhantomData<T>,
    simulation: S,
}

impl<M, T, S> Stepper<M, T, S> for SimpleStepper<M, T, S>
where
    M: Message,
    S: Simulation<M, T>,
{
    fn step(&mut self, local_messages: &Vec<M>) {
        let consq = self.simulation.step(local_messages);

        // Broadcast the `consq` messages to all connected listeners.
    }

    fn simulation(&self) -> &S {
        &self.simulation
    }
}
