use std::fmt;

use super::{DatabaseIdentity, SealContext, SealedState};

pub struct BrokerRecord {
    pub record_namespace: Vec<u8>,
    pub schema_version: u32,
    pub binding_version: u16,
    pub database_identity: DatabaseIdentity,
    pub record_id: [u8; 32],
    pub lookup_digest: [u8; 32],
    pub binding_digest: [u8; 32],
    pub canonical_binding: Vec<u8>,
    pub state: SealedState,
    pub sequence: u64,
    pub key_epoch: u64,
    pub writer_epoch: u64,
    pub anti_rollback_watermark: u64,
    pub sealed: Vec<u8>,
}

impl BrokerRecord {
    pub fn seal_context(&self) -> SealContext<'_> {
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
