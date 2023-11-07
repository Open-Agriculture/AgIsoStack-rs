# AgIsoStack-rs

## About This Library

AgIsoStack-rs is an MIT licensed hardware agnostic ISOBUS (ISO11783) and SAE J1939 CAN stack written in Rust.

**This project is an experimental Work in Progress, and is not suitable for consumption.**

## Compilation

This library is built with Cargo

```sh
cargo build
```

## Tests

Tests for this library are run with Cargo

```sh
cargo test
```

## Features

This crate provides multiple optional features that allow the user to pick and choose what they want
to compile.

| Feature name | Description                                                    | Enabled by default |
|--------------|----------------------------------------------------------------|--------------------|
| `peak`       | Enables the `ag_iso_stack::driver::PeakDriver`                 | No                 |
| `socketcan`  | Enables the `ag_iso_stack::driver::SocketcanDriver`            | No                 |
| `tracing`    | Enables developer diagnostic logging using the `tracing` crate | No                 |

Pass these features to Cargo when building like: `cargo build --features socketcan,tracing`.

### PCAN-Basic
Note that the `peak` CAN driver requires kernel support (should be enabled by default) _and_ the
`pcanbasic` library, for which there does not seem to be support for in the Ubuntu or Fedora package
repositories.

For Linux, you'll have to build and install from source after downloading the library here:
<https://www.peak-system.com/fileadmin/media/linux/index.htm#Section_Driver-Proproetary>

```sh
## Fedora:
# sudo dnf install kernel-devel popt-devel
sudo apt install linux-headers-generic libpopt-dev
tar -xzvf peak-linux-driver-8.16.0.tar.gz
cd peak-linux-driver-8.16.0/
make clean
make all
sudo make install
## Fedora:
# make KERNEL_LOCATION=/usr/src/kernels/6.5.9-200.fc38.x86_64/ clean
# EXTRA_CFLAGS=-Wno-error=incompatible-pointer-types make KERNEL_LOCATION=/usr/src/kernels/6.5.9-200.fc38.x86_64/ NET=NETDEV_SUPPORT all
# sudo make KERNEL_LOCATION=/usr/src/kernels/6.5.9-200.fc38.x86_64/ install
```

For Windows, it appears you can install the driver and PCAN-Basic library from
<https://www.peak-system.com/Drivers.523.0.html?&L=1>.
