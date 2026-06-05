use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    sync::Semaphore,
};

use crate::{EventingError, JournalDispatchPhase, JournalHash, StoredEventEnvelope};

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
    append_gate: Arc<Semaphore>,
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
        phase: JournalDispatchPhase,
    ) -> Result<JournalAppend, EventingError> {
        let _append_permit = Arc::clone(&self.append_gate)
            .acquire_owned()
            .await
            .expect("journal append gate remains open");
        self.recover_state().await?;
        let append = {
            let state = self.state.lock().expect("journal state lock");
            let next_sequence = state.next_sequence.saturating_add(1);
            let previous_hash = previous_hash(&self.options, &state);
            let current_hash = current_hash(
                &self.options,
                next_sequence,
                &previous_hash,
                envelope,
                phase,
            )?;
            JournalAppend {
                sequence: next_sequence,
                previous_hash,
                current_hash,
            }
        };
        self.write_entry(&append, envelope, phase).await?;
        {
            let mut state = self.state.lock().expect("journal state lock");
            state.next_sequence = append.sequence;
            state.previous_hash = append.current_hash.clone();
            state.recovered = true;
        }
        Ok(append)
    }

    async fn recover_state(&self) -> Result<(), EventingError> {
        if self.state.lock().expect("journal state lock").recovered {
            return Ok(());
        }
        let recovered = self.read_recovered_state().await?;
        let mut state = self.state.lock().expect("journal state lock");
        if !state.recovered {
            *state = recovered;
        }
        Ok(())
    }

    async fn read_recovered_state(&self) -> Result<NdjsonJournalState, EventingError> {
        let file = match File::open(&self.path).await {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(NdjsonJournalState::recovered_empty());
            }
            Err(error) => return Err(EventingError::journal_io(self.path_string(), error)),
        };
        let mut lines = BufReader::new(file).lines();
        let mut line_number = 0_usize;
        let mut state = NdjsonJournalState::recovered_empty();
        while let Some(line) = lines
            .next_line()
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), error))?
        {
            line_number += 1;
            if line.trim().is_empty() {
                continue;
            }
            let entry: NdjsonJournalEntry =
                serde_json::from_str(&line).map_err(|error| EventingError::JournalCorruptLine {
                    line: line_number,
                    reason: error.to_string(),
                })?;
            state.next_sequence = entry.append.sequence;
            state.previous_hash = entry.append.current_hash;
        }
        Ok(state)
    }

    async fn write_entry(
        &self,
        append: &JournalAppend,
        envelope: &StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> Result<(), EventingError> {
        let entry = NdjsonJournalEntry {
            append: append.clone(),
            phase,
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
        Box::pin(async move {
            self.append_entry(envelope, JournalDispatchPhase::AfterDispatch)
                .await
        })
    }

    fn append_phase<'a>(
        &'a self,
        envelope: &'a StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> JournalAppendFuture<'a> {
        Box::pin(async move { self.append_entry(envelope, phase).await })
    }
}

#[derive(Default, Debug)]
struct NdjsonJournalState {
    next_sequence: u64,
    previous_hash: Option<JournalHash>,
    recovered: bool,
}

impl NdjsonJournalState {
    fn recovered_empty() -> Self {
        Self {
            next_sequence: 0,
            previous_hash: None,
            recovered: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NdjsonJournalEntry {
    pub append: JournalAppend,
    #[serde(default = "default_journal_phase")]
    pub phase: JournalDispatchPhase,
    pub envelope: StoredEventEnvelope,
}

#[derive(Serialize)]
struct JournalHashInput<'a> {
    sequence: u64,
    previous_hash: Option<&'a JournalHash>,
    phase: JournalDispatchPhase,
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
    phase: JournalDispatchPhase,
) -> Result<Option<JournalHash>, EventingError> {
    match options.hash_chain {
        JournalHashChain::Disabled => Ok(None),
        JournalHashChain::Enabled => {
            hash_entry(sequence, previous_hash.as_ref(), envelope, phase).map(Some)
        }
    }
}

fn hash_entry(
    sequence: u64,
    previous_hash: Option<&JournalHash>,
    envelope: &StoredEventEnvelope,
    phase: JournalDispatchPhase,
) -> Result<JournalHash, EventingError> {
    let input = JournalHashInput {
        sequence,
        previous_hash,
        phase,
        envelope,
    };
    let bytes = serde_json::to_vec(&input).map_err(EventingError::journal_encode)?;
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    JournalHash::parse(format!("{JOURNAL_HASH_PREFIX}{:016x}", hasher.finish()))
}

fn default_journal_phase() -> JournalDispatchPhase {
    JournalDispatchPhase::AfterDispatch
}
