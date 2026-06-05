use std::{future::Future, pin::Pin, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{EventingError, JournalHash};

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
    fn append<'a>(&'a self, envelope: &'a crate::StoredEventEnvelope) -> JournalAppendFuture<'a>;
}

pub type SharedEventJournal = Arc<dyn EventJournal>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalAppend {
    pub sequence: u64,
    pub previous_hash: Option<JournalHash>,
    pub current_hash: Option<JournalHash>,
}
