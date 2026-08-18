use ocentra_schema::report_query_custody as contracts;

use super::{
    ReportQueryCustodySourceOwnerEvidence, ReportQueryCustodySourceStateEvidence,
    ReportQueryCustodyTombstoneEvidence,
};
use crate::report_query_custody::{
    ReportQueryCustodyDerivationError, ReportQueryCustodyDerivationInput, ReportQueryCustodySignal,
};

pub(super) struct ReportQueryCustodySourceStateProjection {
    signal: ReportQueryCustodySignal,
    next_cursor_ref: Option<contracts::ReportQueryCustodyCursorRef>,
    deleted_source_ref: Option<contracts::ReportQueryCustodyDeletedSourceRef>,
    deleted_source_at: Option<contracts::ParentTimestamp>,
    conflict_ref: Option<contracts::ReportQueryCustodyConflictRef>,
    cursor_expired_at: Option<contracts::ParentTimestamp>,
    rate_limited_until_at: Option<contracts::ParentTimestamp>,
    tombstone_confirmed: bool,
}

impl ReportQueryCustodySourceStateProjection {
    fn with_signal(signal: ReportQueryCustodySignal) -> Self {
        Self {
            signal,
            next_cursor_ref: None,
            deleted_source_ref: None,
            deleted_source_at: None,
            conflict_ref: None,
            cursor_expired_at: None,
            rate_limited_until_at: None,
            tombstone_confirmed: false,
        }
    }

    fn continuation(
        signal: ReportQueryCustodySignal,
        next_cursor_ref: contracts::ReportQueryCustodyCursorRef,
    ) -> Self {
        Self {
            next_cursor_ref: Some(next_cursor_ref),
            ..Self::with_signal(signal)
        }
    }

    fn deleted(
        owner: &ReportQueryCustodySourceOwnerEvidence,
        tombstone: ReportQueryCustodyTombstoneEvidence,
    ) -> Result<Self, ReportQueryCustodyDerivationError> {
        if tombstone.authority != owner.authority {
            return Err(ReportQueryCustodyDerivationError::TrustedSourceResolutionUnavailable);
        }
        Ok(Self {
            deleted_source_ref: Some(tombstone.deleted_source_ref),
            deleted_source_at: Some(tombstone.deleted_source_at),
            tombstone_confirmed: true,
            ..Self::with_signal(ReportQueryCustodySignal::Deleted)
        })
    }

    fn cursor_expired(expired_at: contracts::ParentTimestamp) -> Self {
        Self {
            cursor_expired_at: Some(expired_at),
            ..Self::with_signal(ReportQueryCustodySignal::CursorExpired)
        }
    }

    fn rate_limited(until_at: contracts::ParentTimestamp) -> Self {
        Self {
            rate_limited_until_at: Some(until_at),
            ..Self::with_signal(ReportQueryCustodySignal::RateLimited)
        }
    }

    pub(super) fn from_state(
        owner: &ReportQueryCustodySourceOwnerEvidence,
        state: ReportQueryCustodySourceStateEvidence,
    ) -> Result<Self, ReportQueryCustodyDerivationError> {
        match state {
            ReportQueryCustodySourceStateEvidence::Fresh { next_cursor_ref } => Ok(
                Self::continuation(ReportQueryCustodySignal::Fresh, next_cursor_ref),
            ),
            ReportQueryCustodySourceStateEvidence::Stale { next_cursor_ref } => Ok(
                Self::continuation(ReportQueryCustodySignal::Stale, next_cursor_ref),
            ),
            ReportQueryCustodySourceStateEvidence::PartiallyRedacted { next_cursor_ref } => Ok(
                Self::continuation(ReportQueryCustodySignal::PartiallyRedacted, next_cursor_ref),
            ),
            ReportQueryCustodySourceStateEvidence::Deleted { tombstone } => {
                Self::deleted(owner, tombstone)
            }
            ReportQueryCustodySourceStateEvidence::Conflict {
                next_cursor_ref,
                conflict_ref,
            } => Ok(Self {
                conflict_ref: Some(conflict_ref),
                ..Self::continuation(ReportQueryCustodySignal::Conflict, next_cursor_ref)
            }),
            ReportQueryCustodySourceStateEvidence::CursorExpired { expired_at } => {
                Ok(Self::cursor_expired(expired_at))
            }
            ReportQueryCustodySourceStateEvidence::RateLimited { until_at } => {
                Ok(Self::rate_limited(until_at))
            }
        }
    }

    pub(super) fn into_input(
        self,
        row_id: contracts::ReportQueryCustodySourceRef,
        source_data_class: contracts::ReportQueryCustodySourceDataClass,
        cursor_ref: contracts::ReportQueryCustodyCursorRef,
        source_cursor_ref: contracts::ReportQueryCustodyCursorRef,
        page_index: u32,
        stable_sort_key: contracts::ReportQueryCustodySortKey,
    ) -> ReportQueryCustodyDerivationInput {
        ReportQueryCustodyDerivationInput {
            row_id,
            source_data_class,
            signal: self.signal,
            cursor_ref,
            source_cursor_ref,
            next_cursor_ref: self.next_cursor_ref,
            page_index,
            stable_sort_key,
            deleted_source_ref: self.deleted_source_ref,
            deleted_source_at: self.deleted_source_at,
            conflict_ref: self.conflict_ref,
            cursor_expired_at: self.cursor_expired_at,
            rate_limited_until_at: self.rate_limited_until_at,
            raw_child_evidence_included: false,
            tombstone_confirmed: self.tombstone_confirmed,
        }
    }
}
