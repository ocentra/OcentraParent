use std::{
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc, Mutex},
};

#[cfg(debug_assertions)]
use std::sync::atomic::AtomicU64;

use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

use crate::{JournalDispatchPhase, JournalHash, StoredEventEnvelope};

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
    synchronization_completion_sync_failure_for_debug: Arc<AtomicBool>,
    partial_write_failure_for_debug: Arc<AtomicBool>,
    directory_sync_failure_for_debug: Arc<AtomicBool>,
    #[cfg(debug_assertions)]
    recovery_count_for_debug: Arc<AtomicU64>,
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
            synchronization_completion_sync_failure_for_debug: Arc::new(AtomicBool::new(false)),
            partial_write_failure_for_debug: Arc::new(AtomicBool::new(false)),
            directory_sync_failure_for_debug: Arc::new(AtomicBool::new(false)),
            #[cfg(debug_assertions)]
            recovery_count_for_debug: Arc::new(AtomicU64::new(0)),
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

    #[cfg(debug_assertions)]
    pub fn inject_next_synchronization_completion_sync_failure_for_debug(&self) {
        self.synchronization_completion_sync_failure_for_debug
            .store(true, std::sync::atomic::Ordering::Release);
    }

    #[cfg(debug_assertions)]
    pub fn inject_next_partial_write_failure_for_debug(&self) {
        self.partial_write_failure_for_debug
            .store(true, std::sync::atomic::Ordering::Release);
    }

    #[cfg(debug_assertions)]
    pub fn inject_next_directory_sync_failure_for_debug(&self) {
        self.directory_sync_failure_for_debug
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

/// A post-fsync V3 acknowledgement.  Event lines are deliberately written as
/// buffered because their own serialization happens before fsync.  This
/// separate line is emitted only after that fsync succeeds, so recovery can
/// distinguish a persisted event from a durably completed publication.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NdjsonJournalSynchronizationCompletion {
    pub sequence: u64,
    pub entry_hash: Option<JournalHash>,
    pub synchronization_hash: JournalHash,
}

/// A separately synchronized activation for a V3 completion marker. Recovery
/// accepts a completion only after this record exists, so a marker that
/// survives a reported marker-fsync error remains fail-closed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NdjsonJournalSynchronizationActivation {
    pub activation: bool,
    pub sequence: u64,
    pub entry_hash: Option<JournalHash>,
    pub synchronization_hash: JournalHash,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NdjsonJournalRecord {
    Entry(Box<NdjsonJournalEntry>),
    SynchronizationCompletion(NdjsonJournalSynchronizationCompletion),
    SynchronizationActivation(NdjsonJournalSynchronizationActivation),
}

impl NdjsonJournalRecord {
    pub fn parse(line: &str, line_number: usize) -> Result<Self, crate::EventingError> {
        serde_json::from_str(line).map_err(|error| crate::EventingError::JournalCorruptLine {
            line: line_number,
            reason: error.to_string(),
        })
    }

    pub fn entry(self) -> Option<NdjsonJournalEntry> {
        match self {
            Self::Entry(entry) => Some(*entry),
            Self::SynchronizationCompletion(_) | Self::SynchronizationActivation(_) => None,
        }
    }
}

fn default_journal_phase() -> JournalDispatchPhase {
    JournalDispatchPhase::AfterDispatch
}
