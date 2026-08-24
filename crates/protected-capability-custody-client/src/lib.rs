#![forbid(unsafe_code)]

pub mod admission;

use std::fmt;

#[cfg(windows)]
mod windows_ipc;

use ocentra_protected_capability_custody_protocol::constants;

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
    UnsupportedPlatform,
}

impl fmt::Display for ClientError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::BrokerUnavailable => constants::ERROR_CLIENT_BROKER_UNAVAILABLE,
            Self::Transport => constants::ERROR_CLIENT_TRANSPORT,
            Self::PeerAuthentication => constants::ERROR_CLIENT_PEER_AUTHENTICATION,
            Self::Protocol(_) => constants::ERROR_CLIENT_PROTOCOL,
            Self::UnsupportedPlatform => constants::ERROR_CLIENT_UNSUPPORTED,
        })
    }
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Protocol(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ocentra_protected_capability_custody_protocol::types::ProtocolError> for ClientError {
    fn from(error: ocentra_protected_capability_custody_protocol::types::ProtocolError) -> Self {
        Self::Protocol(error)
    }
}

pub(crate) fn map_transport_error(_error: std::io::Error) -> ClientError {
    ClientError::Transport
}
