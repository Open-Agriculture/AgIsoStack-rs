// Copyright 2023 Raven Industries inc.

use std::sync::mpsc::channel;

use ag_iso_stack::driver::{Driver, DriverReadError, Frame, SocketcanDriver};
use clap::Parser;

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
    #[clap(short, long, default_value_t = String::from("can0"))]
    pub input_interface: String,

    /// The interface to write traffic to
    ///
    /// Can be either a string interface name, or an integer interface index
    #[clap(short, long, default_value_t = String::from("can1"))]
    pub output_interface: String,
}

fn create_driver(iface: &str) -> impl Driver {
    if let Ok(index) = iface.parse::<u32>() {
        SocketcanDriver::new_by_index(index)
    } else {
        SocketcanDriver::new_by_name(iface)
    }
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

    let mut input = create_driver(&opts.input_interface);
    let mut output = create_driver(&opts.output_interface);

    input.open().unwrap();
    output.open().unwrap();

    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(true).unwrap()).unwrap();

    loop {
        if rx.try_recv().is_ok() {
            break;
        }

        let mut frame = Frame::default();

        match input.read_nonblocking(&mut frame) {
            Ok(_) => {
                tracing::info!("Read frame: {frame:?}");
                tracing::info!("Attempting to write frame");
                match output.write_nonblocking(&frame) {
                    Ok(_) => tracing::info!("Wrote frame: {frame:?}"),
                    Err(e) => tracing::info!("Failed to write frame: {e:?}"),
                }
            }
            Err(DriverReadError::NoFrameReady) => {}
            Err(e) => tracing::error!("Failed to read frame: {e:?}"),
        }
    }
}
