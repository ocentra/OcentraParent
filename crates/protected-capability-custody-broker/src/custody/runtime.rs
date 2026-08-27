use ocentra_protected_capability_custody_core::broker_admission::{
    BrokerCustodyRuntime, BrokerPlatformSessionState, BrokerRuntimeError,
};
use ocentra_protected_capability_custody_protocol::response::ResponseStatus;

use crate::BrokerError;

#[cfg(windows)]
pub(super) mod admission;

pub(super) enum RuntimeState {
    Ready { runtime: Box<BrokerCustodyRuntime> },
    FailClosed(ResponseStatus),
}

impl RuntimeState {
    pub(super) fn open() -> Self {
        match open_runtime() {
            Ok(runtime) => Self::Ready {
                runtime: Box::new(runtime),
            },
            Err(status) => Self::FailClosed(status),
        }
    }

    pub(super) fn platform_session_state(&self) -> Result<BrokerPlatformSessionState, BrokerError> {
        match self {
            Self::Ready { runtime } => runtime
                .platform_session_state()
                .map_err(map_currentness_error),
            Self::FailClosed(_) => Err(BrokerError::DeploymentRequired),
        }
    }
}

pub(super) fn runtime_error_status(error: &BrokerRuntimeError) -> ResponseStatus {
    match error {
        BrokerRuntimeError::Custody(
            ocentra_protected_capability_custody_core::custody::CustodyError::UnsupportedPlatform,
        ) => ResponseStatus::UnsupportedPlatform,
        BrokerRuntimeError::InvalidRequest | BrokerRuntimeError::Binding(_) => {
            ResponseStatus::Rejected
        }
        BrokerRuntimeError::InvalidBrokerProcess
        | BrokerRuntimeError::DeploymentRequired
        | BrokerRuntimeError::Unavailable
        | BrokerRuntimeError::Custody(_) => ResponseStatus::Unavailable,
    }
}

fn open_runtime() -> Result<BrokerCustodyRuntime, ResponseStatus> {
    let runtime =
        BrokerCustodyRuntime::start_broker_owned().map_err(|error| runtime_error_status(&error))?;
    runtime
        .platform_session_state()
        .map_err(|error| runtime_error_status(&error))?;
    Ok(runtime)
}

fn map_currentness_error(_error: BrokerRuntimeError) -> BrokerError {
    BrokerError::DeploymentRequired
}
