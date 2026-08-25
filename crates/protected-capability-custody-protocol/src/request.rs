use crate::handshake::UntrustedBrokerHello;
use crate::target::{Action, TargetDescriptor};
use crate::types::{
    AuthenticationDomain, AuthenticationTag, BindingEpochs, BootstrapAuthenticator, CorrelationId,
    Nonce, OpaquePreparedToken, ProtocolError, ProtocolGeneration, ProtocolVersion, SessionHandle,
    SessionTranscriptDigest,
};

mod accessors;
pub mod authenticated;
mod binding;
mod debug;
mod operations;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum RequestKind {
    Prepare = 1,
    Commit = 2,
    Abort = 3,
    Recover = 4,
    ResolveAmbiguity = 5,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExpectedGenerations {
    authority: u64,
    target: u64,
    key: u64,
    writer: u64,
}

impl ExpectedGenerations {
    /// Explicit non-authoritative sentinel for the first Prepare of a binding.
    /// The broker replaces it with four independently generated current
    /// generations and returns those exact observations.
    pub const fn initial_binding() -> Self {
        Self {
            authority: 1,
            target: 1,
            key: 1,
            writer: 1,
        }
    }

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

    pub fn authority(self) -> u64 {
        self.authority
    }

    pub fn target(self) -> u64 {
        self.target
    }

    pub fn key(self) -> u64 {
        self.key
    }

    pub fn writer(self) -> u64 {
        self.writer
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestSessionEnvelope {
    pub(crate) version: ProtocolVersion,
    pub(crate) protocol_generation: ProtocolGeneration,
    pub(crate) client_nonce: Nonce,
    pub(crate) broker_nonce: Nonce,
    pub(crate) correlation: CorrelationId,
    pub(crate) client_process_id: u32,
    pub(crate) client_process_epoch: u64,
    pub(crate) client_session_id: u32,
    pub(crate) broker_process_id: u32,
    pub(crate) broker_session_id: u32,
    pub(crate) broker_epoch: u64,
    pub(crate) broker_key_epoch: u64,
    pub(crate) writer_lease_epoch: u64,
    pub(crate) watermark: u64,
    pub(crate) session_handle: SessionHandle,
    pub(crate) transcript_digest: SessionTranscriptDigest,
    pub(crate) sequence: u64,
    pub(crate) expires_at_unix_millis: u64,
}

impl RequestSessionEnvelope {
    pub fn from_authenticated_hello(
        hello: &UntrustedBrokerHello,
        transcript_digest: SessionTranscriptDigest,
        sequence: u64,
        expires_at_unix_millis: u64,
    ) -> Result<Self, ProtocolError> {
        if sequence == 0 {
            return Err(ProtocolError::InvalidSequence);
        }
        if expires_at_unix_millis == 0
            || expires_at_unix_millis > hello.session_expires_at_unix_millis()
        {
            return Err(ProtocolError::InvalidExpiry);
        }
        Ok(Self {
            version: hello.version(),
            protocol_generation: hello.protocol_generation(),
            client_nonce: hello.client_nonce(),
            broker_nonce: hello.broker_nonce(),
            correlation: hello.correlation(),
            client_process_id: hello.client_process_id(),
            client_process_epoch: hello.client_process_epoch(),
            client_session_id: hello.client_session_id(),
            broker_process_id: hello.broker_process_id(),
            broker_session_id: hello.broker_session_id(),
            broker_epoch: hello.broker_epoch(),
            broker_key_epoch: hello.broker_key_epoch(),
            writer_lease_epoch: hello.writer_lease_epoch(),
            watermark: hello.watermark(),
            session_handle: hello.session_handle(),
            transcript_digest,
            sequence,
            expires_at_unix_millis,
        })
    }
}

pub struct UntrustedRequestValues {
    pub session: RequestSessionEnvelope,
    pub expected_generations: ExpectedGenerations,
    pub kind: RequestKind,
    pub operation: Vec<u8>,
    pub action: Action,
    pub target: TargetDescriptor,
    pub opaque_token: Option<OpaquePreparedToken>,
}

pub struct UntrustedRequest {
    pub(crate) session: RequestSessionEnvelope,
    pub(crate) expected_generations: ExpectedGenerations,
    pub(crate) kind: RequestKind,
    pub(crate) operation: Vec<u8>,
    pub(crate) action: Action,
    pub(crate) target: TargetDescriptor,
    pub(crate) opaque_token: Option<OpaquePreparedToken>,
    pub(crate) authentication_tag: AuthenticationTag,
}

impl UntrustedRequest {
    pub fn authenticate_wire(
        values: UntrustedRequestValues,
        authenticator: &BootstrapAuthenticator,
    ) -> Result<authenticated::AuthenticatedRequest, ProtocolError> {
        let mut request = Self::from_values(
            values,
            AuthenticationTag::from_tag([1_u8; crate::constants::AUTHENTICATION_TAG_BYTES]),
        )?;
        request.authentication_tag =
            authenticator.authenticate(AuthenticationDomain::Request, &request.request_digest())?;
        Ok(authenticated::AuthenticatedRequest::from_verified(request))
    }

    pub(crate) fn from_decoded(
        values: UntrustedRequestValues,
        authentication_tag: AuthenticationTag,
    ) -> Result<Self, ProtocolError> {
        Self::from_values(values, authentication_tag)
    }

    fn from_values(
        values: UntrustedRequestValues,
        authentication_tag: AuthenticationTag,
    ) -> Result<Self, ProtocolError> {
        BindingEpochs {
            client_process_epoch: values.session.client_process_epoch,
            broker_epoch: values.session.broker_epoch,
            broker_key_epoch: values.session.broker_key_epoch,
            writer_lease_epoch: values.session.writer_lease_epoch,
            authority_generation: values.expected_generations.authority(),
            target_generation: values.expected_generations.target(),
            key_generation: values.expected_generations.key(),
            writer_generation: values.expected_generations.writer(),
        }
        .validate()?;
        if values.session.client_process_id == 0
            || values.session.client_session_id == 0
            || values.session.broker_process_id == 0
        {
            return Err(ProtocolError::InvalidProcessId);
        }
        if values.session.sequence == 0 {
            return Err(ProtocolError::InvalidSequence);
        }
        if values.session.expires_at_unix_millis == 0 {
            return Err(ProtocolError::InvalidExpiry);
        }
        crate::target::validation::validate_field(&values.operation)?;
        if values.kind.requires_token() != values.opaque_token.is_some() {
            return Err(if values.kind.requires_token() {
                ProtocolError::InvalidOpaqueToken
            } else {
                ProtocolError::UnexpectedOpaqueToken
            });
        }
        Ok(Self {
            session: values.session,
            expected_generations: values.expected_generations,
            kind: values.kind,
            operation: values.operation,
            action: values.action,
            target: values.target,
            opaque_token: values.opaque_token,
            authentication_tag,
        })
    }
}
