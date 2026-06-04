use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use tokio::{fs::OpenOptions, io::AsyncWriteExt, sync::Mutex};

use crate::{EventingError, JournalHash, StoredEventEnvelope};

use super::{EventJournal, JournalAppend, JournalAppendFuture, SharedEventJournal};

const JOURNAL_HASH_PREFIX: &str = "journal-hash:";

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
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn shared(self) -> SharedEventJournal {
        Arc::new(self)
    }

    async fn append_entry(
        &self,
        envelope: &StoredEventEnvelope,
    ) -> Result<JournalAppend, EventingError> {
        let mut state = self.state.lock().await;
        state.next_sequence += 1;
        let previous_hash = previous_hash(&self.options, &state);
        let current_hash =
            current_hash(&self.options, state.next_sequence, &previous_hash, envelope)?;
        let append = JournalAppend {
            sequence: state.next_sequence,
            previous_hash,
            current_hash: current_hash.clone(),
        };
        self.write_entry(&append, envelope).await?;
        state.previous_hash = current_hash;
        Ok(append)
    }

    async fn write_entry(
        &self,
        append: &JournalAppend,
        envelope: &StoredEventEnvelope,
    ) -> Result<(), EventingError> {
        let entry = NdjsonJournalEntry {
            append: append.clone(),
            envelope: envelope.clone(),
        };
        let mut line = serde_json::to_vec(&entry).map_err(EventingError::journal_encode)?;
        line.push(b'\n');
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), error))?;
        file.write_all(&line)
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), error))?;
        if self.options.flush == JournalFlushPolicy::Always {
            file.flush()
                .await
                .map_err(|error| EventingError::journal_io(self.path_string(), error))?;
        }
        Ok(())
    }

    pub(crate) fn path_string(&self) -> String {
        self.path.display().to_string()
    }
}

impl EventJournal for NdjsonEventJournal {
    fn append<'a>(&'a self, envelope: &'a StoredEventEnvelope) -> JournalAppendFuture<'a> {
        Box::pin(async move { self.append_entry(envelope).await })
    }
}

#[derive(Default, Debug)]
struct NdjsonJournalState {
    next_sequence: u64,
    previous_hash: Option<JournalHash>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NdjsonJournalEntry {
    pub append: JournalAppend,
    pub envelope: StoredEventEnvelope,
}

#[derive(Serialize)]
struct JournalHashInput<'a> {
    sequence: u64,
    previous_hash: Option<&'a JournalHash>,
    envelope: &'a StoredEventEnvelope,
}

fn previous_hash(
    options: &NdjsonJournalOptions,
    state: &NdjsonJournalState,
) -> Option<JournalHash> {
    match options.hash_chain {
        JournalHashChain::Disabled => None,
        JournalHashChain::Enabled => state.previous_hash.clone(),
    }
}

fn current_hash(
    options: &NdjsonJournalOptions,
    sequence: u64,
    previous_hash: &Option<JournalHash>,
    envelope: &StoredEventEnvelope,
) -> Result<Option<JournalHash>, EventingError> {
    match options.hash_chain {
        JournalHashChain::Disabled => Ok(None),
        JournalHashChain::Enabled => {
            hash_entry(sequence, previous_hash.as_ref(), envelope).map(Some)
        }
    }
}

fn hash_entry(
    sequence: u64,
    previous_hash: Option<&JournalHash>,
    envelope: &StoredEventEnvelope,
) -> Result<JournalHash, EventingError> {
    let input = JournalHashInput {
        sequence,
        previous_hash,
        envelope,
    };
    let bytes = serde_json::to_vec(&input).map_err(EventingError::journal_encode)?;
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    JournalHash::parse(format!("{JOURNAL_HASH_PREFIX}{:016x}", hasher.finish()))
}
