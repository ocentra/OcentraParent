use ocentra_protected_capability_custody_core::broker_admission::{
    BrokerAuthorizedClientTranscript, BrokerCustodyRuntime, BrokerPeerAdmissionObservation,
    BrokerProcessIdentity, BrokerRuntimeError,
};
use ocentra_protected_capability_custody_protocol::bootstrap::BootstrapPacket;
use ocentra_protected_capability_custody_protocol::handshake::UntrustedClientHello;
use widestring::U16CString;

use crate::custody::BrokerPipeSecurityDescriptor;

use super::RuntimeState;

pub(crate) fn preflight_service_start() -> Result<(), crate::BrokerError> {
    BrokerCustodyRuntime::preflight_service_start().map_err(map_peer_error)
}

impl RuntimeState {
    pub(crate) fn observe_impersonated_named_pipe_client(
        &self,
        pipe_process_id: u32,
        pipe_session_id: u32,
    ) -> Result<BrokerPeerAdmissionObservation, crate::BrokerError> {
        self.ready_runtime()?
            .observe_impersonated_named_pipe_client(pipe_process_id, pipe_session_id)
            .map_err(map_peer_error)
    }

    pub(crate) fn authorize_client_peer(
        &self,
        observation: &BrokerPeerAdmissionObservation,
    ) -> Result<(), crate::BrokerError> {
        self.ready_runtime()?
            .authorize_client_peer(observation)
            .map_err(map_peer_error)
    }

    pub(crate) fn authorize_client_transcript(
        &self,
        observation: BrokerPeerAdmissionObservation,
        bootstrap: &BootstrapPacket,
        hello: &UntrustedClientHello,
        pipe_process_id: u32,
        pipe_session_id: u32,
    ) -> Result<BrokerAuthorizedClientTranscript, crate::BrokerError> {
        self.ready_runtime()?
            .authorize_client_transcript(
                observation,
                bootstrap,
                hello,
                pipe_process_id,
                pipe_session_id,
            )
            .map_err(map_peer_error)
    }

    pub(crate) fn broker_pipe_sddl(
        &self,
    ) -> Result<BrokerPipeSecurityDescriptor, crate::BrokerError> {
        let text = self
            .ready_runtime()?
            .broker_pipe_sddl()
            .map_err(map_peer_error)?;
        let descriptor =
            U16CString::from_str(text).map_err(|_| crate::BrokerError::InvalidLaunch)?;
        Ok(BrokerPipeSecurityDescriptor(descriptor))
    }

    pub(crate) fn peer_admission_available(&self) -> Result<(), crate::BrokerError> {
        self.ready_runtime()?
            .peer_admission_available()
            .map_err(map_peer_error)
    }

    pub(crate) fn current_process_identity(
        &self,
    ) -> Result<BrokerProcessIdentity, crate::BrokerError> {
        self.ready_runtime()?
            .broker_process_identity()
            .map_err(map_peer_error)
    }

    fn ready_runtime(&self) -> Result<&BrokerCustodyRuntime, crate::BrokerError> {
        match self {
            Self::Ready { runtime, .. } => Ok(runtime.as_ref()),
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
