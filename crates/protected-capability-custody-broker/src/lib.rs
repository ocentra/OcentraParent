#![forbid(unsafe_code)]

mod authority;
mod custody;
mod error;

#[cfg(windows)]
mod windows_ipc;

use ocentra_protected_capability_custody_protocol::transport::pipe::BrokerPipeName;

pub fn run_from_inherited_bootstrap(pipe_name: &BrokerPipeName) -> Result<(), BrokerError> {
    #[cfg(windows)]
    {
        windows_ipc::run(pipe_name)
    }
    #[cfg(not(windows))]
    {
        let _pipe_name = pipe_name;
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
