#![forbid(unsafe_code)]

use std::process::ExitCode;

#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(windows)]
use windows_service::service_dispatcher;

#[cfg(windows)]
use ocentra_protected_capability_custody_broker::{run_service, BROKER_SERVICE_NAME};

#[cfg(windows)]
windows_service::define_windows_service!(ffi_service_main, service_main);

#[cfg(windows)]
static SERVICE_FAILED: AtomicBool = AtomicBool::new(false);

#[cfg(windows)]
fn service_main(_arguments: Vec<OsString>) {
    // `run_service` publishes a terminal non-zero SCM status only after its
    // preflight permits status registration. Retain an independent process-exit
    // signal for preflight failures and for rejected final status updates.
    if run_service().is_err() {
        SERVICE_FAILED.store(true, Ordering::Release);
    }
}

#[cfg(windows)]
fn main() -> ExitCode {
    match service_dispatcher::start(BROKER_SERVICE_NAME, ffi_service_main) {
        Ok(()) if !SERVICE_FAILED.load(Ordering::Acquire) => ExitCode::SUCCESS,
        Ok(()) => ExitCode::FAILURE,
        Err(_) => ExitCode::FAILURE,
    }
}

#[cfg(not(windows))]
fn main() -> ExitCode {
    ExitCode::FAILURE
}
