// Copyright 2023 Raven Industries inc.

use std::sync::mpsc::channel;

#[cfg(feature = "peak")]
use ag_iso_stack::driver::PeakDriver;
#[cfg(feature = "socketcan")]
use ag_iso_stack::driver::SocketcanDriver;
use ag_iso_stack::driver::{Driver, Frame};
use ag_iso_stack::tracing;
use clap::{Parser, ValueEnum};
#[cfg(feature = "peak")]
use pcan_basic::bus::UsbBus;

fn parse_usb_bus(s: &str) -> Option<UsbBus> {
    let s = s.to_uppercase();
    match s.as_str() {
        "USB1" => Some(UsbBus::USB1),
        "USB2" => Some(UsbBus::USB2),
        "USB3" => Some(UsbBus::USB3),
        "USB4" => Some(UsbBus::USB4),
        "USB5" => Some(UsbBus::USB5),
        "USB6" => Some(UsbBus::USB6),
        "USB7" => Some(UsbBus::USB7),
        "USB8" => Some(UsbBus::USB8),
        "USB9" => Some(UsbBus::USB9),
        "USB10" => Some(UsbBus::USB10),
        "USB11" => Some(UsbBus::USB11),
        "USB12" => Some(UsbBus::USB12),
        "USB13" => Some(UsbBus::USB13),
        "USB14" => Some(UsbBus::USB14),
        "USB15" => Some(UsbBus::USB15),
        "USB16" => Some(UsbBus::USB16),
        _ => None,
    }
}

#[derive(Debug, Clone, ValueEnum)]
enum CanDriver {
    #[cfg(feature = "socketcan")]
    Socketcan,
    #[cfg(feature = "peak")]
    Pcan,
}

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

    #[clap(short, long)]
    pub driver: CanDriver,
}

fn create_driver(iface: &str, driver: CanDriver) -> Box<dyn Driver> {
    match driver {
        #[cfg(feature = "socketcan")]
        CanDriver::Socketcan => {
            if let Ok(index) = iface.parse::<u32>() {
                Box::new(SocketcanDriver::new_by_index(index))
            } else {
                Box::new(SocketcanDriver::new_by_name(iface))
            }
        }
        #[cfg(feature = "peak")]
        CanDriver::Pcan => {
            let bus = parse_usb_bus(iface).unwrap();
            let baud = ag_iso_stack::driver::Baudrate::Baud250K;
            Box::new(PeakDriver::new(bus, baud))
        }
        #[allow(unreachable_patterns)]
        _ => unreachable!(),
    }
}

fn main() {
    let opts = Options::parse();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(opts.log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_err| eprintln!("Unable to set global default subscriber"))
        .unwrap();

    tracing::info!(
        "Forwarding CAN traffic from {} to {}",
        opts.input_interface,
        opts.output_interface
    );

    let mut input = create_driver(&opts.input_interface, opts.driver.clone());
    let mut output = create_driver(&opts.output_interface, opts.driver);

    input.open().unwrap();
    output.open().unwrap();

    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(true).unwrap()).unwrap();

    loop {
        if rx.try_recv().is_ok() {
            break;
        }

        let mut frame = Frame::default();

        #[allow(clippy::collapsible_if)]
        if input.read_nonblocking(&mut frame).is_ok() {
            if output.write_nonblocking(&frame).is_err() {
                break;
            }
        }
    }
}
