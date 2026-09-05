#![forbid(unsafe_code)]

mod account_issuer;
mod account_issuer_rpc;
mod authority;
mod custody;
mod error;

#[cfg(windows)]
mod windows_ipc;

pub const BROKER_SERVICE_NAME: &str =
    ocentra_protected_capability_custody_protocol::constants::BROKER_SERVICE_NAME;

pub fn run_service() -> Result<(), BrokerError> {
    #[cfg(windows)]
    {
        windows_ipc::run_service()
    }
    #[cfg(not(windows))]
    {
        Err(BrokerError::UnsupportedPlatform)
    }
}

#[derive(Debug)]
pub enum BrokerError {
    InvalidLaunch,
    Transport,
    PeerAuthentication,
    Protocol(ocentra_protected_capability_custody_protocol::types::ProtocolError),
    Request,
    DeploymentRequired,
    UnsupportedPlatform,
}

impl std::error::Error for BrokerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Protocol(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ocentra_protected_capability_custody_protocol::types::ProtocolError> for BrokerError {
    fn from(error: ocentra_protected_capability_custody_protocol::types::ProtocolError) -> Self {
        Self::Protocol(error)
    }
}
