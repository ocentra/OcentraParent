use super::BrokerRuntimeError;

pub(super) fn broker_executable(_error: std::io::Error) -> BrokerRuntimeError {
    BrokerRuntimeError::InvalidBrokerProcess
}

pub(super) fn storage_io(_error: std::io::Error) -> BrokerRuntimeError {
    BrokerRuntimeError::Unavailable
}

pub(super) fn path_security(_error: crate::path_security::PathSecurityError) -> BrokerRuntimeError {
    BrokerRuntimeError::Unavailable
}

pub(super) fn broker_platform_admission(
    error: &crate::platform::PlatformError,
) -> BrokerRuntimeError {
    match error {
        crate::platform::PlatformError::DeploymentRequired => {
            BrokerRuntimeError::DeploymentRequired
        }
        _ => BrokerRuntimeError::InvalidBrokerProcess,
    }
}

pub(super) fn authority(_error: crate::authority::AuthorityError) -> BrokerRuntimeError {
    BrokerRuntimeError::InvalidRequest
}

pub(super) fn platform(error: &crate::platform::PlatformError) -> BrokerRuntimeError {
    match error {
        crate::platform::PlatformError::DeploymentRequired => {
            BrokerRuntimeError::DeploymentRequired
        }
        _ => BrokerRuntimeError::Unavailable,
    }
}

pub(super) fn token_platform(error: &crate::platform::PlatformError) -> BrokerRuntimeError {
    match error {
        crate::platform::PlatformError::WrongBinding
        | crate::platform::PlatformError::Rejected
        | crate::platform::PlatformError::Conflict => BrokerRuntimeError::InvalidRequest,
        crate::platform::PlatformError::DeploymentRequired => {
            BrokerRuntimeError::DeploymentRequired
        }
        _ => BrokerRuntimeError::Unavailable,
    }
}

pub(super) fn protocol(
    _error: ocentra_protected_capability_custody_protocol::types::ProtocolError,
) -> BrokerRuntimeError {
    BrokerRuntimeError::InvalidRequest
}
