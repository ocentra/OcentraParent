use std::fmt;

use super::{identity::DatabaseIdentity, SealContext, SealedState};

pub(crate) struct BrokerRecord {
    pub(crate) record_namespace: Vec<u8>,
    pub(crate) schema_version: u32,
    pub(crate) binding_version: u16,
    pub(crate) database_identity: DatabaseIdentity,
    pub(crate) record_id: [u8; 32],
    pub(crate) lookup_digest: [u8; 32],
    pub(crate) binding_digest: [u8; 32],
    pub(crate) canonical_binding: Vec<u8>,
    pub(crate) state: SealedState,
    pub(crate) sequence: u64,
    pub(crate) key_epoch: u64,
    pub(crate) writer_epoch: u64,
    pub(crate) anti_rollback_watermark: u64,
    pub(crate) sealed: Vec<u8>,
}

impl BrokerRecord {
    pub(crate) fn seal_context(&self) -> SealContext<'_> {
        SealContext {
            record_namespace: &self.record_namespace,
            schema_version: self.schema_version,
            binding_version: self.binding_version,
            database_identity: self.database_identity,
            record_id: &self.record_id,
            lookup_digest: &self.lookup_digest,
            binding_digest: &self.binding_digest,
            canonical_binding: &self.canonical_binding,
            state: self.state,
            sequence: self.sequence,
            key_epoch: self.key_epoch,
            writer_epoch: self.writer_epoch,
            anti_rollback_watermark: self.anti_rollback_watermark,
        }
    }
}

impl fmt::Debug for BrokerRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BrokerRecord")
            .field("state", &self.state)
            .field("sequence", &self.sequence)
            .field("opaque", &"<redacted>")
            .finish()
    }
}
