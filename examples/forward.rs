// Copyright 2023 Raven Industries inc.

use clap::Parser;

#[cfg(target_os = "linux")]
use socketcan::{BlockingCan, CanSocket, Socket};

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

#[cfg(target_os = "linux")]
fn open_can_interface(input_name: &str, output_name: &str) -> (CanSocket, CanSocket) {
    let mut input =
        CanSocket::open(input_name).expect("The given input interface cannot be opened!");

    let mut output =
        CanSocket::open(output_name).expect("The given output interface cannot be opened!");

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

    #[cfg(target_os = "linux")]
    |opts: Options| {
        let (input, output) = open_can_interface(&opts.input_interface, &opts.output_interface);
        input
            .set_nonblocking(true)
            .expect("Could not set input bus to non-blocking!");
        output
            .set_nonblocking(true)
            .expect("Could not set output bus to non-blocking!");

        loop {
            match input.borrow().receive() {
                Ok(frame) => {
                    output
                        .borrow()
                        .transmit(&frame)
                        .expect("Could not forward received message!");
                }
                Err(_err) => continue,
            }
        }
    };
}
