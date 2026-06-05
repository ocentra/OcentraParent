use std::{future::Future, pin::Pin, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{EventingError, JournalHash, StoredEventEnvelope};

mod ndjson;
mod policy;

pub use ndjson::{
    JournalFlushPolicy, JournalHashChain, NdjsonEventJournal, NdjsonJournalEntry,
    NdjsonJournalOptions,
};
pub use policy::{JournalDispatchPhase, JournalMode, JournalPolicy, JournalSelector};

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
}
