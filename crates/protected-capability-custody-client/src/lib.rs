#![forbid(unsafe_code)]

pub mod admission;

mod error;

#[cfg(windows)]
mod windows_ipc;

pub fn connect() -> Result<admission::AuthenticatedBrokerSession, ClientError> {
    #[cfg(windows)]
    {
        windows_ipc::connect()
    }
    #[cfg(not(windows))]
    {
        Err(ClientError::UnsupportedPlatform)
    }
}

#[derive(Debug)]
pub enum ClientError {
    BrokerUnavailable,
    Transport,
    PeerAuthentication,
    Protocol(ocentra_protected_capability_custody_protocol::types::ProtocolError),
    DeploymentRequired,
    UnsupportedPlatform,
}
