use std::path::Path;

use ocentra_protected_capability_custody_core::broker_admission::{
    BrokerCustodyRuntime, BrokerPeerTokenIdentity, BrokerRuntimeError,
};
use widestring::U16CString;

use crate::custody::BrokerPipeSecurityDescriptor;

use super::RuntimeState;

impl RuntimeState {
    pub(crate) fn authorize_client_peer(
        &self,
        process_id: u32,
        executable_path: &Path,
        executable_digest: [u8; 32],
        token: BrokerPeerTokenIdentity,
    ) -> Result<(), crate::BrokerError> {
        match self {
            Self::Ready { runtime, .. } => runtime
                .authorize_client_peer(process_id, executable_path, executable_digest, token)
                .map_err(map_peer_error),
            Self::FailClosed(_) => Err(crate::BrokerError::DeploymentRequired),
        }
    }

    pub(crate) fn broker_pipe_sddl(
        &self,
    ) -> Result<BrokerPipeSecurityDescriptor, crate::BrokerError> {
        match self {
            Self::Ready { runtime, .. } => {
                let text = runtime.broker_pipe_sddl().map_err(map_peer_error)?;
                let descriptor =
                    U16CString::from_str(text).map_err(|_| crate::BrokerError::InvalidLaunch)?;
                Ok(BrokerPipeSecurityDescriptor(descriptor))
            }
            Self::FailClosed(_) => Err(crate::BrokerError::DeploymentRequired),
        }
    }

    pub(crate) fn peer_admission_available(&self) -> Result<(), crate::BrokerError> {
        match self {
            Self::Ready { .. } => {
                BrokerCustodyRuntime::peer_admission_available().map_err(map_peer_error)
            }
            Self::FailClosed(_) => Err(crate::BrokerError::DeploymentRequired),
        }
    }
}

fn map_peer_error(error: BrokerRuntimeError) -> crate::BrokerError {
    match error {
        BrokerRuntimeError::DeploymentRequired => crate::BrokerError::DeploymentRequired,
        _ => crate::BrokerError::PeerAuthentication,
    }
}
