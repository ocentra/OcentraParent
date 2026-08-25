use super::{identity::DatabaseIdentity, SealedState};

#[derive(Clone, Copy)]
pub(crate) struct BrokerLookup<'a> {
    pub(crate) record_namespace: &'a [u8],
    pub(crate) schema_version: u32,
    pub(crate) binding_version: u16,
    pub(crate) database_identity: DatabaseIdentity,
    pub(crate) lookup_digest: &'a [u8; 32],
}

#[derive(Clone, Copy)]
pub(crate) struct TransitionRequest<'a> {
    pub(crate) record_namespace: &'a [u8],
    pub(crate) schema_version: u32,
    pub(crate) binding_version: u16,
    pub(crate) database_identity: DatabaseIdentity,
    pub(crate) record_id: &'a [u8; 32],
    pub(crate) lookup_digest: &'a [u8; 32],
    pub(crate) binding_digest: &'a [u8; 32],
    pub(crate) canonical_binding: &'a [u8],
    pub(crate) state: SealedState,
    pub(crate) sequence: u64,
    pub(crate) key_epoch: u64,
    pub(crate) writer_epoch: u64,
    pub(crate) minimum_watermark: u64,
}
