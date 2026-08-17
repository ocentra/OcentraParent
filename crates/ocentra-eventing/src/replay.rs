use serde::{Deserialize, Serialize};

use crate::{
    CorrelationId, EventType, NdjsonEventJournal, NdjsonJournalEntry, StoredEventEnvelope,
};

mod read;

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

/// Journal-owned action replay capability.
///
/// Projection records remain inspectable data, but action dispatch must only
/// accept this value, which can be created by the journal reader after it has
/// applied the configured hash-chain, synchronization, and phase checks.
/// Keeping the records private prevents a caller from minting action authority
/// from arbitrary stored envelopes.
#[derive(Clone, Debug, PartialEq)]
pub struct ReplayActionReport {
    records: Vec<ReplayRecord>,
    cursor: ReplayCursor,
    skipped_count: usize,
}

impl ReplayActionReport {
    pub(crate) fn from_read_report(report: ReplayReadReport) -> Result<Self, crate::EventingError> {
        if report.mode != ReplayMode::ActionHandlersAllowed {
            let event_type = report
                .records
                .first()
                .map(|record| record.envelope.contract.event_type.clone())
                .unwrap_or(crate::EventType::parse("projection-only-replay")?);
            return Err(crate::EventingError::ReplayActionNotAllowed { event_type });
        }
        Ok(Self {
            records: report.records,
            cursor: report.cursor,
            skipped_count: report.skipped_count,
        })
    }

    pub fn cursor(&self) -> &ReplayCursor {
        &self.cursor
    }

    pub fn records(&self) -> &[ReplayRecord] {
        &self.records
    }

    pub fn skipped_count(&self) -> usize {
        self.skipped_count
    }
}

impl NdjsonEventJournal {}
