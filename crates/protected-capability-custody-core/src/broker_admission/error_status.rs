use super::BrokerRuntimeError;

pub(super) fn broker_executable(_error: std::io::Error) -> BrokerRuntimeError {
    BrokerRuntimeError::InvalidBrokerProcess
}

pub(super) fn storage_io(_error: std::io::Error) -> BrokerRuntimeError {
    BrokerRuntimeError::Unavailable
}

pub(super) fn broker_platform_admission(
    _error: &crate::platform::PlatformError,
) -> BrokerRuntimeError {
    BrokerRuntimeError::InvalidBrokerProcess
}

pub(super) fn authority(_error: crate::authority::AuthorityError) -> BrokerRuntimeError {
    BrokerRuntimeError::InvalidRequest
}

pub(super) fn platform(_error: crate::platform::PlatformError) -> BrokerRuntimeError {
    BrokerRuntimeError::Unavailable
}

pub(super) fn token_platform(error: &crate::platform::PlatformError) -> BrokerRuntimeError {
    match error {
        crate::platform::PlatformError::WrongBinding
        | crate::platform::PlatformError::Rejected
        | crate::platform::PlatformError::Conflict => BrokerRuntimeError::InvalidRequest,
        _ => BrokerRuntimeError::Unavailable,
    }
}

pub(super) fn protocol(
    _error: ocentra_protected_capability_custody_protocol::types::ProtocolError,
) -> BrokerRuntimeError {
    BrokerRuntimeError::InvalidRequest
}
