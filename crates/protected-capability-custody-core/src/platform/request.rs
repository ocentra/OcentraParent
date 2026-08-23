use super::{DatabaseIdentity, SealedState};

#[derive(Clone, Copy)]
pub struct BrokerLookup<'a> {
    pub record_namespace: &'a [u8],
    pub schema_version: u32,
    pub binding_version: u16,
    pub database_identity: DatabaseIdentity,
    pub lookup_digest: &'a [u8; 32],
}

#[derive(Clone, Copy)]
pub struct TransitionRequest<'a> {
    pub record_namespace: &'a [u8],
    pub schema_version: u32,
    pub binding_version: u16,
    pub database_identity: DatabaseIdentity,
    pub record_id: &'a [u8; 32],
    pub lookup_digest: &'a [u8; 32],
    pub binding_digest: &'a [u8; 32],
    pub canonical_binding: &'a [u8],
    pub state: SealedState,
    pub sequence: u64,
    pub key_epoch: u64,
    pub writer_epoch: u64,
    pub minimum_watermark: u64,
}
