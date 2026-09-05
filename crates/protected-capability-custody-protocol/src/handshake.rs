mod broker;
mod client;
mod session;

use crate::types::{
    AttestationDigest, BootstrapAuthenticator, CorrelationId, Nonce, ProtocolGeneration,
    ProtocolVersion, SessionHandle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BrokerSessionWireValues {
    pub broker_nonce: Nonce,
    pub broker_process_id: u32,
    pub broker_session_id: u32,
    pub broker_epoch: u64,
    pub broker_key_epoch: u64,
    pub writer_lease_epoch: u64,
    pub watermark: u64,
    pub session_handle: SessionHandle,
    pub session_expires_at_unix_millis: u64,
}

#[derive(Eq, PartialEq)]
pub struct UntrustedClientHello {
    pub(crate) version: ProtocolVersion,
    pub(crate) protocol_generation: ProtocolGeneration,
    pub(crate) nonce: Nonce,
    pub(crate) correlation: CorrelationId,
    pub(crate) client_process_id: u32,
    pub(crate) client_process_epoch: u64,
    pub(crate) client_session_id: u32,
}

#[derive(Eq, PartialEq)]
pub struct UntrustedBrokerHello {
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
    pub(crate) attestation_digest: AttestationDigest,
    pub(crate) authenticator: BootstrapAuthenticator,
    pub(crate) session_expires_at_unix_millis: u64,
}
