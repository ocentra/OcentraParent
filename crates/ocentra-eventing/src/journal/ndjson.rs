use std::{
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

use crate::{JournalDispatchPhase, StoredEventEnvelope};

use super::{JournalAppend, SharedEventJournal};

#[path = "ndjson_io.rs"]
mod ndjson_io;
#[path = "ndjson_state.rs"]
mod ndjson_state;
use self::ndjson_state::NdjsonJournalState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JournalHashChain {
    Disabled,
    Enabled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JournalFlushPolicy {
    Always,
    Buffered,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NdjsonJournalOptions {
    pub hash_chain: JournalHashChain,
    pub flush: JournalFlushPolicy,
}

impl NdjsonJournalOptions {
    pub fn hash_chain() -> Self {
        Self {
            hash_chain: JournalHashChain::Enabled,
            flush: JournalFlushPolicy::Always,
        }
    }
}

impl Default for NdjsonJournalOptions {
    fn default() -> Self {
        Self {
            hash_chain: JournalHashChain::Disabled,
            flush: JournalFlushPolicy::Always,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NdjsonEventJournal {
    path: PathBuf,
    options: NdjsonJournalOptions,
    state: Arc<Mutex<NdjsonJournalState>>,
    append_gate: Arc<Semaphore>,
    sync_failure_for_debug: Arc<AtomicBool>,
}

impl NdjsonEventJournal {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self::with_options(path, NdjsonJournalOptions::default())
    }

    pub fn with_options(path: impl Into<PathBuf>, options: NdjsonJournalOptions) -> Self {
        Self {
            path: path.into(),
            options,
            state: Arc::new(Mutex::new(NdjsonJournalState::default())),
            append_gate: Arc::new(Semaphore::new(1)),
            sync_failure_for_debug: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn shared(self) -> SharedEventJournal {
        Arc::new(self)
    }

    #[cfg(debug_assertions)]
    pub fn inject_next_sync_failure_for_debug(&self) {
        self.sync_failure_for_debug
            .store(true, std::sync::atomic::Ordering::Release);
    }

    pub(crate) fn path_string(&self) -> String {
        self.path.display().to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NdjsonJournalEntry {
    pub append: JournalAppend,
    #[serde(default = "default_journal_phase")]
    pub phase: JournalDispatchPhase,
    pub envelope: StoredEventEnvelope,
}

fn default_journal_phase() -> JournalDispatchPhase {
    JournalDispatchPhase::AfterDispatch
}
