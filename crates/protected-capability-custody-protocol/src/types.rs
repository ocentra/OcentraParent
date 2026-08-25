use zeroize::Zeroizing;

use crate::constants::{
    ATTESTATION_DIGEST_BYTES, AUTHENTICATION_TAG_BYTES, BOOTSTRAP_AUTHENTICATOR_BYTES,
    CORRELATION_BYTES, NONCE_BYTES, OPAQUE_TOKEN_BYTES, SESSION_HANDLE_BYTES,
    TRANSCRIPT_DIGEST_BYTES,
};

mod authentication;
mod display;
mod identity;
mod protocol;
mod token;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolVersion(pub(crate) u16);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolGeneration(pub(crate) u64);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Nonce(pub(crate) [u8; NONCE_BYTES]);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct CorrelationId(pub(crate) [u8; CORRELATION_BYTES]);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct SessionHandle(pub(crate) [u8; SESSION_HANDLE_BYTES]);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct AttestationDigest(pub(crate) [u8; ATTESTATION_DIGEST_BYTES]);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct SessionTranscriptDigest(pub(crate) [u8; TRANSCRIPT_DIGEST_BYTES]);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct AuthenticationTag(pub(crate) [u8; AUTHENTICATION_TAG_BYTES]);

#[derive(Clone, Eq, PartialEq)]
pub struct BootstrapAuthenticator(Zeroizing<[u8; BOOTSTRAP_AUTHENTICATOR_BYTES]>);

#[derive(Clone, Copy)]
pub(crate) enum AuthenticationDomain {
    BrokerAttestation,
    Request,
    Response,
}

pub struct OpaquePreparedToken(Zeroizing<[u8; OPAQUE_TOKEN_BYTES]>);

pub(crate) struct BindingEpochs {
    pub(crate) client_process_epoch: u64,
    pub(crate) broker_epoch: u64,
    pub(crate) broker_key_epoch: u64,
    pub(crate) writer_lease_epoch: u64,
    pub(crate) authority_generation: u64,
    pub(crate) target_generation: u64,
    pub(crate) key_generation: u64,
    pub(crate) writer_generation: u64,
}

#[derive(Debug)]
pub enum ProtocolError {
    EmptyFrame,
    FrameTooLarge,
    InvalidFrameLength,
    TrailingBytes,
    UnsupportedVersion(u16),
    InvalidDomain,
    InvalidMessageKind(u8),
    InvalidNonce,
    InvalidCorrelationId,
    EmptyField,
    FieldTooLarge,
    UnsupportedRequest(u8),
    UnsupportedAction(u8),
    UnsupportedTarget(u8),
    UnsupportedStatus(u8),
    InvalidOpaqueToken,
    UnexpectedOpaqueToken,
    InvalidEpoch,
    InvalidProcessId,
    InvalidSessionHandle,
    InvalidAttestationDigest,
    InvalidTranscriptDigest,
    InvalidAuthenticationTag,
    AuthenticationFailed,
    InvalidSequence,
    InvalidExpiry,
    InvalidBootstrap,
    Truncated,
    InvalidStatusForRequest,
    InvalidDiscriminant(u8),
    Transport,
    Randomness,
}

impl ProtocolError {
    fn from_randomness(_error: getrandom::Error) -> Self {
        Self::Randomness
    }

    fn from_nonce_length(_error: std::array::TryFromSliceError) -> Self {
        Self::InvalidNonce
    }

    fn from_correlation_length(_error: std::array::TryFromSliceError) -> Self {
        Self::InvalidCorrelationId
    }

    fn from_session_handle_length(_error: std::array::TryFromSliceError) -> Self {
        Self::InvalidSessionHandle
    }

    fn from_attestation_length(_error: std::array::TryFromSliceError) -> Self {
        Self::InvalidAttestationDigest
    }

    fn from_transcript_length(_error: std::array::TryFromSliceError) -> Self {
        Self::InvalidTranscriptDigest
    }

    fn from_authentication_tag_length(_error: std::array::TryFromSliceError) -> Self {
        Self::InvalidAuthenticationTag
    }

    fn from_bootstrap_length(_error: std::array::TryFromSliceError) -> Self {
        Self::InvalidBootstrap
    }

    fn from_authentication_failure(_error: ring::error::Unspecified) -> Self {
        Self::AuthenticationFailed
    }
}
