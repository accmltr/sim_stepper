mod endpoint;

use framework::event::Event;
use premade_ports::quinn::{QuinnClientPort, QuinnServerPort};
use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    thread,
    time::Duration,
};

const SERVER_NAME: &str = "localhost";
const LOCALHOST_V4: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
const CLIENT_ADDR: SocketAddr = SocketAddr::new(LOCALHOST_V4, 5000);
const SERVER_ADDR: SocketAddr = SocketAddr::new(LOCALHOST_V4, 5001);

mod mock_sim_and_stepper;

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    println!("Hello from 'quinn_simple' example.");

    // Spawn seperate thread.
    let thread_handle = thread::spawn(move || {
        println!("Creating tokio runtime.");
        // Start a tokio async runtime.
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to build tokio runtime.");

        // Use the runtime to wait for the async entry point to exit.
        rt.block_on(async {
            println!("Creating server endpoint.");
            let (server_endpoint, server_cert) =
                endpoint::make_server_endpoint(SERVER_ADDR).unwrap();
            println!("Creating client endpoint.");
            let client_endpoint =
                endpoint::make_client_endpoint(CLIENT_ADDR, &[&server_cert]).unwrap();

            println!("Creating server port.");
            let server_port = QuinnServerPort::<MyEvent, u64>::new(server_endpoint);
            println!("Creating client port.");
            let client_port = QuinnClientPort::<MyEvent, u64>::new(
                client_endpoint,
                SERVER_ADDR,
                SERVER_NAME.to_string(),
            );
        });
    });

    thread_handle.join();

    thread::sleep(Duration::new(1, 0));

    // println!("Sending input event from client to server.");

    // println!("Event read on server port.");

    // println!(
    //     "Mock simulation returned event and caused broadcast from server, event read on client."
    // );

    println!("Simple quinn ports example done.");
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
enum MyEvent {
    PlayerMove { x: u64, y: u64 },
    PlayerJump,
}

impl Event for MyEvent {}
