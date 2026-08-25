use std::fmt;

use ocentra_protected_capability_custody_protocol::constants;
use ocentra_protected_capability_custody_protocol::request::{ExpectedGenerations, RequestKind};
use ocentra_protected_capability_custody_protocol::response::{
    ObservedGenerations, ResponseStatus, UntrustedResponse,
};
use ocentra_protected_capability_custody_protocol::target::{Action, TargetDescriptor};
use ocentra_protected_capability_custody_protocol::types::OpaquePreparedToken;

use crate::ClientError;

pub struct ClientRequest {
    pub(crate) expected_generations: ExpectedGenerations,
    pub(crate) kind: RequestKind,
    pub(crate) operation: Vec<u8>,
    pub(crate) action: Action,
    pub(crate) target: TargetDescriptor,
    pub(crate) opaque_token: Option<OpaquePreparedToken>,
}

impl ClientRequest {
    pub fn new(
        expected_generations: ExpectedGenerations,
        kind: RequestKind,
        operation: Vec<u8>,
        action: Action,
        target: TargetDescriptor,
        opaque_token: Option<OpaquePreparedToken>,
    ) -> Self {
        Self {
            expected_generations,
            kind,
            operation,
            action,
            target,
            opaque_token,
        }
    }
}

impl fmt::Debug for ClientRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(constants::DEBUG_CLIENT_REQUEST)
            .field(constants::DEBUG_FIELD_KIND, &self.kind)
            .field(
                constants::DEBUG_FIELD_OPERATION_LENGTH,
                &self.operation.len(),
            )
            .field(constants::DEBUG_FIELD_ACTION, &self.action)
            .field(constants::DEBUG_FIELD_TARGET, &self.target)
            .field(
                constants::DEBUG_FIELD_OPAQUE_TOKEN,
                &constants::DEBUG_REDACTED,
            )
            .finish()
    }
}

pub struct AuthenticatedBrokerSession {
    _private: SessionPrivate,
}

struct SessionPrivate;

impl AuthenticatedBrokerSession {
    pub fn execute(self, request: ClientRequest) -> Result<AuthenticatedResponse, ClientError> {
        #[cfg(windows)]
        {
            // No safe client-verifiable immutable broker image/SCM anchor is
            // linked into this build. `connect` cannot construct this sealed
            // session, and execution remains unavailable until that adapter
            // authenticates the exact pipe server before any bootstrap bytes
            // are sent.
            let _ = (
                self,
                request.expected_generations,
                request.opaque_token.as_ref(),
            );
            Err(ClientError::DeploymentRequired)
        }
        #[cfg(not(windows))]
        {
            let _request = request;
            Err(ClientError::UnsupportedPlatform)
        }
    }
}

impl fmt::Debug for AuthenticatedBrokerSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(constants::DEBUG_AUTHENTICATED_BROKER_SESSION)
            .field(constants::DEBUG_FIELD_AUTHENTICATED, &true)
            .finish()
    }
}

pub struct AuthenticatedResponse {
    response: UntrustedResponse,
}

impl AuthenticatedResponse {
    pub fn status(&self) -> ResponseStatus {
        self.response.status()
    }

    pub fn observed_generations(&self) -> Option<ObservedGenerations> {
        self.response.observed_generations()
    }

    pub fn into_prepared_token(self) -> Option<OpaquePreparedToken> {
        self.response.into_opaque_token()
    }
}

impl fmt::Debug for AuthenticatedResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(constants::DEBUG_AUTHENTICATED_RESPONSE)
            .field(constants::DEBUG_FIELD_STATUS, &self.status())
            .field(
                constants::DEBUG_FIELD_OPAQUE_TOKEN,
                &constants::DEBUG_REDACTED,
            )
            .finish()
    }
}
