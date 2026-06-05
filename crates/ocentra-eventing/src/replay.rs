use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

use crate::{
    CorrelationId, EventType, EventingError, JournalDispatchPhase, NdjsonEventJournal,
    NdjsonJournalEntry, StoredEventEnvelope,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReplayMode {
    ProjectionOnly,
    ActionHandlersAllowed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayCursor {
    pub next_sequence: u64,
}

impl ReplayCursor {
    pub fn start() -> Self {
        Self { next_sequence: 1 }
    }

    pub fn after(sequence: u64) -> Self {
        Self {
            next_sequence: sequence.saturating_add(1),
        }
    }
}

impl Default for ReplayCursor {
    fn default() -> Self {
        Self::start()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReplayFilter {
    pub cursor: ReplayCursor,
    pub event_types: Vec<EventType>,
    pub correlation_id: Option<CorrelationId>,
}

impl ReplayFilter {
    pub fn all() -> Self {
        Self::default()
    }

    pub fn for_event_type(event_type: EventType) -> Self {
        Self {
            event_types: vec![event_type],
            ..Self::default()
        }
    }

    pub fn with_correlation_id(mut self, correlation_id: CorrelationId) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_cursor(mut self, cursor: ReplayCursor) -> Self {
        self.cursor = cursor;
        self
    }

    pub(crate) fn matches(&self, entry: &NdjsonJournalEntry) -> bool {
        entry.append.sequence >= self.cursor.next_sequence
            && (self.event_types.is_empty()
                || self
                    .event_types
                    .iter()
                    .any(|event_type| event_type == &entry.envelope.contract.event_type))
            && self
                .correlation_id
                .as_ref()
                .is_none_or(|correlation_id| correlation_id == &entry.envelope.correlation_id)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReplayRecord {
    pub sequence: u64,
    pub envelope: StoredEventEnvelope,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReplayReadReport {
    pub mode: ReplayMode,
    pub cursor: ReplayCursor,
    pub records: Vec<ReplayRecord>,
    pub skipped_count: usize,
}

impl NdjsonEventJournal {
    pub async fn replay_projection(
        &self,
        filter: ReplayFilter,
    ) -> Result<ReplayReadReport, EventingError> {
        self.read(filter, ReplayMode::ProjectionOnly).await
    }

    pub async fn replay_action_records(
        &self,
        filter: ReplayFilter,
    ) -> Result<ReplayReadReport, EventingError> {
        self.read(filter, ReplayMode::ActionHandlersAllowed).await
    }

    async fn read(
        &self,
        filter: ReplayFilter,
        mode: ReplayMode,
    ) -> Result<ReplayReadReport, EventingError> {
        let file = File::open(self.path())
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), error))?;
        let mut lines = BufReader::new(file).lines();
        let mut line_number = 0_usize;
        let mut records = Vec::new();
        let mut skipped_count = 0_usize;
        let mut last_sequence = filter.cursor.next_sequence.saturating_sub(1);

        while let Some(line) = lines
            .next_line()
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), error))?
        {
            line_number += 1;
            if line.trim().is_empty() {
                skipped_count += 1;
                continue;
            }
            let entry: NdjsonJournalEntry =
                serde_json::from_str(&line).map_err(|error| EventingError::JournalCorruptLine {
                    line: line_number,
                    reason: error.to_string(),
                })?;
            last_sequence = last_sequence.max(entry.append.sequence);
            if mode == ReplayMode::ActionHandlersAllowed
                && entry.phase != JournalDispatchPhase::AfterDispatch
            {
                skipped_count += 1;
                continue;
            }
            if filter.matches(&entry) {
                records.push(ReplayRecord {
                    sequence: entry.append.sequence,
                    envelope: entry.envelope,
                });
            } else {
                skipped_count += 1;
            }
        }

        Ok(ReplayReadReport {
            mode,
            cursor: ReplayCursor::after(last_sequence),
            records,
            skipped_count,
        })
    }
}
