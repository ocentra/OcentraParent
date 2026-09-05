#![forbid(unsafe_code)]

#[path = "transport/protocol.rs"]
mod protocol;

#[cfg(windows)]
mod operations;
#[cfg(windows)]
mod runtime;
#[cfg(windows)]
mod transport;

use std::process::{ExitCode, Termination};

#[cfg(windows)]
fn main() -> ExitCode {
    runtime::run().report()
}

#[cfg(not(windows))]
fn main() -> ExitCode {
    ExitCode::FAILURE
}
