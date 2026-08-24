use ocentra_protected_capability_custody_core::broker_admission::BrokerPlatformSessionState;
use ocentra_protected_capability_custody_protocol::request::authenticated::AuthenticatedRequest;
use ocentra_protected_capability_custody_protocol::response::UntrustedResponse;
use ocentra_protected_capability_custody_protocol::types::BootstrapAuthenticator;

use crate::BrokerError;

mod response;
mod runtime;

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

    pub(crate) fn execute(
        &self,
        request: &AuthenticatedRequest,
        authenticator: &BootstrapAuthenticator,
    ) -> Result<UntrustedResponse, BrokerError> {
        response::execute(&self.state, request, authenticator)
    }
}
