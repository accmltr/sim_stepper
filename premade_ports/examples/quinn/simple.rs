mod endpoint;

use premade_ports::quinn::QuinnServerPort;
use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

const SERVER_NAME: &str = "localhost";
const LOCALHOST_V4: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
const CLIENT_ADDR: SocketAddr = SocketAddr::new(LOCALHOST_V4, 5000);
const SERVER_ADDR: SocketAddr = SocketAddr::new(LOCALHOST_V4, 5001);

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    println!("Hello from 'quinn_simple' example.");

    println!("Creating server endpoint.");
    let (server_endpoint, server_cert) = endpoint::make_server_endpoint(SERVER_ADDR)?;
    println!("Creating client endpoint.");
    let client_endpoint = endpoint::make_client_endpoint(CLIENT_ADDR, &[&server_cert])?;

    println!("Creating server port.");
    // let server_port = QuinnServerPort::<Event, EventSourceID>::new(server_endpoint);
    // println!("Creating client port.");
    // let client_port = QuinnClientPort::build(server_endpoint)?;

    println!("Sending input event from client to server.");

    println!("Event read on server port.");

    println!(
        "Mock simulation returned event and caused broadcast from server, event read on client."
    );

    Ok(())
}
