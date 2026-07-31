use std::{future::Future, pin::Pin, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{EventingError, JournalHash, StoredEventEnvelope};

pub(crate) mod hash_chain;
pub mod ndjson;
pub mod policy;

use policy::JournalDispatchPhase;

pub type JournalAppendFuture<'a> =
    Pin<Box<dyn Future<Output = Result<JournalAppend, EventingError>> + Send + 'a>>;

pub trait EventJournal: Send + Sync {
    /// Explicit capability boundary: proof/replay journals must not authorize
    /// a production control grant merely because an append was synchronized.
    fn is_production_durable(&self) -> bool {
        false
    }

    fn append<'a>(&'a self, envelope: &'a StoredEventEnvelope) -> JournalAppendFuture<'a>;

    fn append_phase<'a>(
        &'a self,
        envelope: &'a StoredEventEnvelope,
        _phase: JournalDispatchPhase,
    ) -> JournalAppendFuture<'a> {
        self.append(envelope)
    }
}

pub type SharedEventJournal = Arc<dyn EventJournal>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalAppend {
    pub sequence: u64,
    pub previous_hash: Option<JournalHash>,
    pub current_hash: Option<JournalHash>,
    /// Hash-input format used for this persisted append. Missing values are
    /// legacy v1 entries, whose authenticated input predates durability.
    #[serde(default)]
    pub hash_version: JournalHashVersion,
    /// Whether the append is known to have been synchronized before it was
    /// persisted. Missing values from older journal entries fail closed.
    #[serde(default)]
    pub durability: JournalAppendDurability,
    /// Durability the caller requested. V3 authenticates this separately from
    /// the persisted achieved result so a line written before fsync cannot
    /// attest a synchronization that later failed.
    #[serde(default)]
    pub requested_durability: JournalAppendDurability,
    /// A V3 acknowledgement may claim synchronization only when this
    /// completion hash authenticates the achieved durability after the sync.
    /// It is deliberately absent from the immutable pre-sync journal line.
    #[serde(default)]
    pub synchronization_hash: Option<JournalHash>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum JournalAppendDurability {
    #[default]
    Buffered,
    Synchronized,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum JournalHashVersion {
    #[default]
    LegacyV1,
    V2,
    V3,
}

impl JournalAppend {
    pub fn is_synchronized(&self) -> bool {
        self.durability == JournalAppendDurability::Synchronized
            && (self.hash_version != JournalHashVersion::V3
                || self
                    .synchronization_hash
                    .as_ref()
                    .is_some_and(|hash| hash_chain::verify_synchronization_receipt(self, hash)))
    }

    /// Returns whether this append carries the V3 completion proof required
    /// before an authorization boundary can rely on its synchronization claim.
    pub fn has_verified_synchronization_proof(&self) -> bool {
        self.hash_version == JournalHashVersion::V3 && self.is_synchronized()
    }

    /// Attests a V3 append after its journal implementation has completed its
    /// required synchronization work. Implementations retain responsibility
    /// for performing that work before issuing this acknowledgement.
    pub fn with_synchronization_proof(mut self) -> Result<Self, EventingError> {
        if self.hash_version != JournalHashVersion::V3 {
            return Err(EventingError::InvalidValue {
                field: "journal_append.hash_version",
                value: "synchronization proof requires V3".to_owned(),
            });
        }
        self.durability = JournalAppendDurability::Synchronized;
        self.synchronization_hash = Some(hash_chain::synchronization_receipt_hash(&self)?);
        Ok(self)
    }
}
