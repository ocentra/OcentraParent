#![forbid(unsafe_code)]

use std::process::ExitCode;

#[cfg(windows)]
use std::ffi::OsString;

#[cfg(windows)]
use windows_service::service_dispatcher;

#[cfg(windows)]
use ocentra_protected_capability_custody_broker::{run_service, BROKER_SERVICE_NAME};

#[cfg(windows)]
windows_service::define_windows_service!(ffi_service_main, service_main);

#[cfg(windows)]
fn service_main(_arguments: Vec<OsString>) {
    let _ = run_service();
}

#[cfg(windows)]
fn main() -> ExitCode {
    match service_dispatcher::start(BROKER_SERVICE_NAME, ffi_service_main) {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

#[cfg(not(windows))]
fn main() -> ExitCode {
    ExitCode::FAILURE
}
