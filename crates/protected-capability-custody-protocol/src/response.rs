mod accessors;
mod debug;
mod generations;
mod status;

use crate::constants::{AUTHENTICATION_TAG_BYTES, REQUEST_DIGEST_BYTES};
use crate::request::{authenticated::AuthenticatedRequest, RequestKind, RequestSessionEnvelope};
use crate::types::{
    AuthenticationDomain, AuthenticationTag, BootstrapAuthenticator, OpaquePreparedToken,
    ProtocolError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ResponseStatus {
    Prepared = 1,
    Committed = 2,
    Aborted = 3,
    CommitAmbiguous = 4,
    AbortAmbiguous = 5,
    Rejected = 6,
    Unavailable = 7,
    UnsupportedPlatform = 8,
    PrepareAmbiguous = 9,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservedGenerations {
    authority: u64,
    target: u64,
    key: u64,
    writer: u64,
}

impl ObservedGenerations {
    pub fn try_new(
        authority: u64,
        target: u64,
        key: u64,
        writer: u64,
    ) -> Result<Self, ProtocolError> {
        if authority == 0 || target == 0 || key == 0 || writer == 0 {
            return Err(ProtocolError::InvalidEpoch);
        }
        Ok(Self {
            authority,
            target,
            key,
            writer,
        })
    }
}

pub(crate) struct DecodedResponseValues {
    pub(crate) session: RequestSessionEnvelope,
    pub(crate) request_kind: RequestKind,
    pub(crate) request_digest: [u8; REQUEST_DIGEST_BYTES],
    pub(crate) status: ResponseStatus,
    pub(crate) observed_generations: Option<ObservedGenerations>,
    pub(crate) opaque_token: Option<OpaquePreparedToken>,
}

pub struct UntrustedResponse {
    pub(crate) session: RequestSessionEnvelope,
    pub(crate) request_kind: RequestKind,
    pub(crate) request_digest: [u8; REQUEST_DIGEST_BYTES],
    pub(crate) status: ResponseStatus,
    pub(crate) observed_generations: Option<ObservedGenerations>,
    pub(crate) opaque_token: Option<OpaquePreparedToken>,
    pub(crate) authentication_tag: AuthenticationTag,
}

impl UntrustedResponse {
    pub fn authenticate_wire(
        request: &AuthenticatedRequest,
        status: ResponseStatus,
        observed_generations: Option<ObservedGenerations>,
        opaque_token: Option<OpaquePreparedToken>,
        authenticator: &BootstrapAuthenticator,
    ) -> Result<Self, ProtocolError> {
        let request = request.as_untrusted();
        let values = DecodedResponseValues {
            session: request.session,
            request_kind: request.kind(),
            request_digest: request.request_digest(),
            status,
            observed_generations,
            opaque_token,
        };
        let mut response = Self::from_values(
            values,
            AuthenticationTag::from_tag([1_u8; AUTHENTICATION_TAG_BYTES]),
        )?;
        response.authentication_tag = authenticator
            .authenticate(AuthenticationDomain::Response, &response.response_digest())?;
        Ok(response)
    }

    pub(crate) fn from_decoded(
        values: DecodedResponseValues,
        authentication_tag: AuthenticationTag,
    ) -> Result<Self, ProtocolError> {
        Self::from_values(values, authentication_tag)
    }

    fn from_values(
        values: DecodedResponseValues,
        authentication_tag: AuthenticationTag,
    ) -> Result<Self, ProtocolError> {
        status::validation::validate_result(
            values.request_kind,
            values.status,
            values.observed_generations,
            values.opaque_token.as_ref(),
        )?;
        Ok(Self {
            session: values.session,
            request_kind: values.request_kind,
            request_digest: values.request_digest,
            status: values.status,
            observed_generations: values.observed_generations,
            opaque_token: values.opaque_token,
            authentication_tag,
        })
    }
}
