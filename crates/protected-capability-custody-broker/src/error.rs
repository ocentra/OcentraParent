use std::fmt;

use ocentra_protected_capability_custody_protocol::constants;

use crate::BrokerError;

impl fmt::Display for BrokerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLaunch => constants::ERROR_BROKER_INVALID_LAUNCH,
            Self::Transport => constants::ERROR_BROKER_TRANSPORT,
            Self::PeerAuthentication => constants::ERROR_BROKER_PEER_AUTHENTICATION,
            Self::Protocol(_) => constants::ERROR_BROKER_PROTOCOL,
            Self::Request => constants::ERROR_BROKER_REQUEST,
            Self::DeploymentRequired => constants::ERROR_BROKER_UNSUPPORTED,
            Self::UnsupportedPlatform => constants::ERROR_BROKER_UNSUPPORTED,
        })
    }
}
