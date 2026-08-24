use ocentra_protected_capability_custody_core::broker_admission::{
    BrokerAuthorizedClientTranscript, BrokerPeerAdmissionObservation, BrokerPlatformSessionState,
};
use ocentra_protected_capability_custody_protocol::bootstrap::BootstrapPacket;
use ocentra_protected_capability_custody_protocol::handshake::UntrustedClientHello;
use ocentra_protected_capability_custody_protocol::request::authenticated::AuthenticatedRequest;
use ocentra_protected_capability_custody_protocol::response::UntrustedResponse;
use ocentra_protected_capability_custody_protocol::types::BootstrapAuthenticator;
#[cfg(windows)]
use widestring::U16CString;

use crate::BrokerError;

mod response;
mod runtime;

#[cfg(windows)]
pub(crate) struct BrokerPipeSecurityDescriptor(pub(crate) U16CString);

pub(crate) struct BrokerCustodyService {
    state: runtime::RuntimeState,
}

impl BrokerCustodyService {
    pub(crate) fn open() -> Self {
        Self {
            state: runtime::RuntimeState::open(),
        }
    }

    pub(crate) fn platform_session_state(&self) -> Option<BrokerPlatformSessionState> {
        self.state.platform_session_state()
    }

    #[cfg(windows)]
    pub(crate) fn authorize_client_peer(
        &self,
        observation: &BrokerPeerAdmissionObservation,
    ) -> Result<(), BrokerError> {
        self.state.authorize_client_peer(observation)
    }

    #[cfg(windows)]
    pub(crate) fn authorize_client_transcript(
        &self,
        observation: &BrokerPeerAdmissionObservation,
        bootstrap: &BootstrapPacket,
        hello: &UntrustedClientHello,
    ) -> Result<BrokerAuthorizedClientTranscript, BrokerError> {
        self.state
            .authorize_client_transcript(observation, bootstrap, hello)
    }

    #[cfg(windows)]
    pub(crate) fn broker_pipe_sddl(&self) -> Result<BrokerPipeSecurityDescriptor, BrokerError> {
        self.state.broker_pipe_sddl()
    }

    #[cfg(windows)]
    pub(crate) fn peer_admission_available(&self) -> Result<(), BrokerError> {
        self.state.peer_admission_available()
    }

    pub(crate) fn execute(
        &self,
        request: &AuthenticatedRequest,
        authenticator: &BootstrapAuthenticator,
    ) -> Result<UntrustedResponse, BrokerError> {
        response::execute(&self.state, request, authenticator)
    }
}
