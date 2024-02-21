// Copyright 2023 Raven Industries inc.

use clap::Parser;

#[cfg(target_os = "unix")]
use socketcan::{CanSocket, Socket};

/// Forward CAN traffic from one interface to another
#[derive(Debug, Parser)]
#[clap(name = "forward", verbatim_doc_comment)]
struct Options {
    /// The log level
    #[clap(short, long, default_value_t = tracing::Level::DEBUG)]
    pub log_level: tracing::Level,

    /// The interface to read traffic from
    ///
    /// Can be either a string interface name, or an integer interface index
    #[clap(short, long, default_value_t = String::from("vcan0"))]
    pub input_interface: String,

    /// The interface to write traffic to
    ///
    /// Can be either a string interface name, or an integer interface index
    #[clap(short, long, default_value_t = String::from("vcan1"))]
    pub output_interface: String,
}

fn open_socket_can_interface() -> (CanSocket, CanSocket) {
    let mut input = CanSocket::open(&opts.input_interface)
        .expect("The given input interface cannot be opened!");

    let mut output = CanSocket::open(&opts.output_interface)
        .expect("The given output interface cannot be opened!");

    (input, output)
}

fn main() {
    let opts = Options::parse();

    let subscriber = tracing_subscriber::fmt()
        // ... add configuration
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_err| eprintln!("Unable to set global default subscriber"))
        .unwrap();

    tracing::info!(
        "Forwarding CAN traffic from {} to {}",
        opts.input_interface,
        opts.output_interface
    );

    let (input, output) = open_can_interface();

    input
        .set_nonblocking(true)
        .expect("Could not set input bus to non-blocking!");
    output
        .set_nonblocking(true)
        .expect("Could not set output bus to non-blocking!");

    loop {
        match input.receive() {
            Ok(frame) => {
                output
                    .transmit(&frame)
                    .expect("Could not forward received message!");
            }
            Err(_err) => continue,
        }
    }
}
