#![deny(unsafe_code)]

use std::process::ExitCode;

mod provisioning;

fn main() -> ExitCode {
    if std::env::args_os().nth(1).is_some() {
        return provisioning::unexpected_arguments_exit_code();
    }
    match provisioning::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => error.exit_code(),
    }
}
