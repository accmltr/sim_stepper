use crate::{
    event_port::ServerPort,
    simulation::{Simulation, StepLogic},
};

pub struct ServerStepper<State, Event, EventSourceId>
where
    EventSourceId: Eq,
    State: StepLogic<Event, EventSourceId>,
{
    /// Inner simulation of the stepper, which is never exposed mutably.
    simulation: Simulation<State, Event, EventSourceId>,
}

impl<State, Event, EventSourceId> ServerStepper<State, Event, EventSourceId>
where
    EventSourceId: Eq,
    State: StepLogic<Event, EventSourceId>,
{
    /// Simply creates a new stepper containing a simulation.
    pub fn new(simulation: Simulation<State, Event, EventSourceId>) -> Self {
        Self { simulation }
    }

    /// Returns an immutable reference to the inner simulation of this stepper.
    pub fn simulation(&self) -> &Simulation<State, Event, EventSourceId> {
        &self.simulation
    }

    pub fn step<P>(&mut self, event_port: &mut P)
    where
        P: ServerPort<Event, EventSourceId>,
    {
        // Fetch inputs from port.
        let sim_in = event_port.read_events();

        // Do step and capture return values.
        let sim_out = self.simulation.step(sim_in);

        // Give step output to port for broadcasting.
        event_port.queue_server_step_input_to_clients(sim_out);
    }
}
