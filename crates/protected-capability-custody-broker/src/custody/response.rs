use ocentra_protected_capability_custody_protocol::request::authenticated::AuthenticatedRequest;
use ocentra_protected_capability_custody_protocol::response::{
    ObservedGenerations, ResponseStatus, UntrustedResponse,
};
use ocentra_protected_capability_custody_protocol::types::{
    BootstrapAuthenticator, OpaquePreparedToken,
};

use super::runtime::{runtime_error_status, RuntimeState};
use crate::BrokerError;

type ResponseParts = (
    ResponseStatus,
    Option<ObservedGenerations>,
    Option<OpaquePreparedToken>,
);

pub(super) fn execute(
    state: &RuntimeState,
    request: &AuthenticatedRequest,
    authenticator: &BootstrapAuthenticator,
) -> Result<UntrustedResponse, BrokerError> {
    let (status, observed, token) = response_parts(state, request);
    UntrustedResponse::authenticate_wire(request, status, observed, token, authenticator)
        .map_err(BrokerError::from)
}

fn response_parts(state: &RuntimeState, request: &AuthenticatedRequest) -> ResponseParts {
    match state {
        RuntimeState::Ready { runtime, .. } => match runtime.execute_authenticated_request(request)
        {
            Ok(outcome) => {
                let status = outcome.status();
                let observed = outcome.observed_generations();
                let token = outcome.into_opaque_token();
                (status, observed, token)
            }
            Err(error) => (runtime_error_status(&error), None, None),
        },
        RuntimeState::FailClosed(status) => (*status, None, None),
    }
}
