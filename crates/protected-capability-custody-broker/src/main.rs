#![forbid(unsafe_code)]

use std::process::ExitCode;

use ocentra_protected_capability_custody_broker::{run_from_inherited_bootstrap, BrokerError};
use ocentra_protected_capability_custody_protocol::constants::BROKER_PIPE_ARGUMENT;
use ocentra_protected_capability_custody_protocol::transport::pipe::BrokerPipeName;

fn main() -> ExitCode {
    match exact_pipe_argument() {
        Ok(pipe_name) if run_from_inherited_bootstrap(&pipe_name).is_ok() => ExitCode::SUCCESS,
        _ => ExitCode::FAILURE,
    }
}

fn exact_pipe_argument() -> Result<BrokerPipeName, BrokerError> {
    let mut arguments = std::env::args_os();
    arguments.next().ok_or(BrokerError::InvalidLaunch)?;
    let argument = arguments.next().ok_or(BrokerError::InvalidLaunch)?;
    if argument.to_str().ok_or(BrokerError::InvalidLaunch)? != BROKER_PIPE_ARGUMENT {
        return Err(BrokerError::InvalidLaunch);
    }
    let pipe_name = match arguments
        .next()
        .ok_or(BrokerError::InvalidLaunch)?
        .into_string()
    {
        Ok(value) => value,
        Err(error) => {
            drop(error);
            return Err(BrokerError::InvalidLaunch);
        }
    };
    if pipe_name.is_empty() || arguments.next().is_some() {
        return Err(BrokerError::InvalidLaunch);
    }
    BrokerPipeName::try_from_untrusted(pipe_name).map_err(map_pipe_error)
}

fn map_pipe_error(
    error: ocentra_protected_capability_custody_protocol::types::ProtocolError,
) -> BrokerError {
    BrokerError::Protocol(error)
}
