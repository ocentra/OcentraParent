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
    }
}
