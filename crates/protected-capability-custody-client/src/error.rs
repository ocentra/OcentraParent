use std::fmt;

use super::ClientError;
use ocentra_protected_capability_custody_protocol::constants;

impl fmt::Display for ClientError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::BrokerUnavailable => constants::ERROR_CLIENT_BROKER_UNAVAILABLE,
            Self::Transport => constants::ERROR_CLIENT_TRANSPORT,
            Self::PeerAuthentication => constants::ERROR_CLIENT_PEER_AUTHENTICATION,
            Self::Protocol(_) => constants::ERROR_CLIENT_PROTOCOL,
            Self::DeploymentRequired => constants::ERROR_CLIENT_BROKER_UNAVAILABLE,
            Self::UnsupportedPlatform => constants::ERROR_CLIENT_UNSUPPORTED,
        })
    }
}

impl std::error::Error for ClientError {}

impl From<ocentra_protected_capability_custody_protocol::types::ProtocolError> for ClientError {
    fn from(error: ocentra_protected_capability_custody_protocol::types::ProtocolError) -> Self {
        Self::Protocol(error)
    }
}
