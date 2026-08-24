use ocentra_protected_capability_custody_core::broker_admission::{
    BrokerCustodyRuntime, BrokerPlatformSessionState, BrokerProcessAdmission, BrokerRuntimeError,
};
use ocentra_protected_capability_custody_protocol::response::ResponseStatus;

pub(super) enum RuntimeState {
    Ready {
        runtime: Box<BrokerCustodyRuntime>,
        platform: BrokerPlatformSessionState,
    },
    FailClosed(ResponseStatus),
}

impl RuntimeState {
    pub(super) fn open() -> Self {
        match open_runtime() {
            Ok((runtime, platform)) => Self::Ready {
                runtime: Box::new(runtime),
                platform,
            },
            Err(status) => Self::FailClosed(status),
        }
    }

    pub(super) fn platform_session_state(&self) -> Option<BrokerPlatformSessionState> {
        match self {
            Self::Ready { platform, .. } => Some(*platform),
            Self::FailClosed(_) => None,
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

fn open_runtime() -> Result<(BrokerCustodyRuntime, BrokerPlatformSessionState), ResponseStatus> {
    let admission = BrokerProcessAdmission::for_current_process()
        .map_err(|error| runtime_error_status(&error))?;
    let runtime = BrokerCustodyRuntime::open_broker_owned(admission)
        .map_err(|error| runtime_error_status(&error))?;
    let platform = runtime
        .platform_session_state()
        .map_err(|error| runtime_error_status(&error))?;
    Ok((runtime, platform))
}
