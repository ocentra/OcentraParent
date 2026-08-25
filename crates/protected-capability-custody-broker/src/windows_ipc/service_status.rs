use windows_service::service::ServiceExitCode;

use crate::BrokerError;

pub(super) fn exit_code(error: &BrokerError) -> ServiceExitCode {
    let code = match error {
        BrokerError::DeploymentRequired => 2,
        BrokerError::InvalidLaunch => 3,
        BrokerError::PeerAuthentication => 4,
        BrokerError::Protocol(_) | BrokerError::Request => 5,
        BrokerError::Transport => 6,
        BrokerError::UnsupportedPlatform => 7,
    };
    ServiceExitCode::ServiceSpecific(code)
}
