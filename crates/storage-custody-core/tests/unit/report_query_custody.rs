use crate::support::StorageCustodyTestValueExt;

use ocentra_schema::report_query_custody as contracts;
use ocentra_storage_custody_core::report_query_custody::{
    build_report_query_custody_proof, derive_report_query_custody_row,
    ReportQueryCustodyDerivationError, ReportQueryCustodyDerivationInput, ReportQueryCustodySignal,
};

macro_rules! derivation_input {
    ($suffix:expr, $source_data_class:expr, $page_index:expr, $signal:expr $(,)?) => {
        ReportQueryCustodyDerivationInput {
            row_id: contracts::ReportQueryCustodySourceRef::parse(format!("row-{}", $suffix))
                .assume_ok(),
            source_data_class: $source_data_class,
            signal: $signal,
            cursor_ref: contracts::ReportQueryCustodyCursorRef::parse(format!(
                "cursor-{}",
                $suffix
            ))
            .assume_ok(),
            source_cursor_ref: contracts::ReportQueryCustodyCursorRef::parse(
                "source-cursor-proof-1",
            )
            .assume_ok(),
            next_cursor_ref: Some(
                contracts::ReportQueryCustodyCursorRef::parse(format!("next-cursor-{}", $suffix))
                    .assume_ok(),
            ),
            page_index: $page_index,
            stable_sort_key: contracts::ReportQueryCustodySortKey::parse(format!(
                "sort-key-{}",
                $suffix
            ))
            .assume_ok(),
            deleted_source_ref: None,
            deleted_source_at: None,
            conflict_ref: None,
            cursor_expired_at: None,
            rate_limited_until_at: None,
            raw_child_evidence_included: false,
            tombstone_confirmed: false,
        }
    };
}

#[test]
fn report_query_custody_derives_fresh_stale_and_partially_redacted_rows() {
    let request = sample_request();

    let fresh = derive_report_query_custody_row(
        &request,
        derivation_input!(
            "derived-fresh",
            contracts::ReportQueryCustodySourceDataClass::SqliteQueryRow,
            1,
            ReportQueryCustodySignal::Fresh,
        ),
    )
    .assume_ok();
    let stale = derive_report_query_custody_row(
        &request,
        derivation_input!(
            "derived-stale",
            contracts::ReportQueryCustodySourceDataClass::GeneratedSummary,
            2,
            ReportQueryCustodySignal::Stale,
        ),
    )
    .assume_ok();
    let partially_redacted = derive_report_query_custody_row(
        &request,
        derivation_input!(
            "partially-redacted",
            contracts::ReportQueryCustodySourceDataClass::NotificationHistory,
            3,
            ReportQueryCustodySignal::PartiallyRedacted,
        ),
    )
    .assume_ok();

    assert_eq!(
        fresh.state,
        contracts::ReportQueryCustodyState::DerivedFresh
    );
    assert_eq!(
        stale.state,
        contracts::ReportQueryCustodyState::DerivedStale
    );
    assert_eq!(
        partially_redacted.state,
        contracts::ReportQueryCustodyState::PartiallyRedacted
    );
    assert_eq!(
        partially_redacted.payload_redaction_state,
        contracts::ReportQueryCustodyPayloadRedaction::PartiallyRedacted
    );
    assert_eq!(
        fresh.notification_payload_boundary,
        contracts::ReportQueryCustodyBoundary::ParentOwnedCitationsOnly
    );
}

#[test]
fn report_query_custody_deleted_source_requires_tombstone_and_metadata() {
    let request = sample_request();
    let mut input = derivation_input!(
        "deleted-source",
        contracts::ReportQueryCustodySourceDataClass::AuditEvent,
        1,
        ReportQueryCustodySignal::Deleted,
    );
    input.next_cursor_ref = None;
    input.deleted_source_ref = Some(
        contracts::ReportQueryCustodyDeletedSourceRef::parse("deleted-source-ref-1").assume_ok(),
    );
    input.deleted_source_at =
        Some(contracts::ParentTimestamp::parse("2026-06-28T16:00:00.000Z").assume_ok());

    let missing_tombstone = derive_report_query_custody_row(&request, input.clone());
    assert_eq!(
        missing_tombstone,
        Err(ReportQueryCustodyDerivationError::TombstoneRequiredForDeletedSource)
    );

    input.tombstone_confirmed = true;
    let deleted = derive_report_query_custody_row(&request, input).assume_ok();
    assert_eq!(
        deleted.state,
        contracts::ReportQueryCustodyState::DeletedSource
    );
    assert_eq!(
        deleted.tombstone_state,
        contracts::ReportQueryCustodyTombstoneState::Written
    );
    assert!(deleted.next_cursor_ref.is_none());
}

#[test]
fn report_query_custody_conflict_requires_conflict_ref_and_stays_claim_safe() {
    let request = sample_request();
    let mut input = derivation_input!(
        "sync-conflict",
        contracts::ReportQueryCustodySourceDataClass::SqliteQueryRow,
        1,
        ReportQueryCustodySignal::Conflict,
    );
    input.conflict_ref = None;

    let missing_conflict = derive_report_query_custody_row(&request, input.clone());
    assert_eq!(
        missing_conflict,
        Err(ReportQueryCustodyDerivationError::MissingConflictRef)
    );

    input.conflict_ref =
        Some(contracts::ReportQueryCustodyConflictRef::parse("conflict-ref-1").assume_ok());
    let conflict = derive_report_query_custody_row(&request, input).assume_ok();
    assert_eq!(
        conflict.state,
        contracts::ReportQueryCustodyState::SyncConflict
    );
    assert!(conflict.claim_safe);
}

#[test]
fn report_query_custody_cursor_expired_and_rate_limited_block_cursor_advancement() {
    let request = sample_request();

    let mut expired_input = derivation_input!(
        "cursor-expired",
        contracts::ReportQueryCustodySourceDataClass::GeneratedSummary,
        1,
        ReportQueryCustodySignal::CursorExpired,
    );
    expired_input.next_cursor_ref = None;
    expired_input.cursor_expired_at =
        Some(contracts::ParentTimestamp::parse("2026-06-28T16:01:00.000Z").assume_ok());
    let expired = derive_report_query_custody_row(&request, expired_input).assume_ok();
    assert_eq!(
        expired.state,
        contracts::ReportQueryCustodyState::CursorExpired
    );
    assert!(expired.next_cursor_ref.is_none());

    let mut rate_limited_input = derivation_input!(
        "rate-limited",
        contracts::ReportQueryCustodySourceDataClass::NotificationHistory,
        2,
        ReportQueryCustodySignal::RateLimited,
    );
    rate_limited_input.next_cursor_ref = None;
    rate_limited_input.rate_limited_until_at =
        Some(contracts::ParentTimestamp::parse("2026-06-28T16:05:00.000Z").assume_ok());
    let rate_limited = derive_report_query_custody_row(&request, rate_limited_input).assume_ok();
    assert_eq!(
        rate_limited.state,
        contracts::ReportQueryCustodyState::RateLimited
    );
    assert!(rate_limited.next_cursor_ref.is_none());
}

#[test]
fn report_query_custody_proof_builder_keeps_cursor_pagination_stable_across_required_states() {
    let request = sample_request();
    let proof = build_report_query_custody_proof(
        &request,
        vec![
            derivation_input!(
                "derived-fresh",
                contracts::ReportQueryCustodySourceDataClass::SqliteQueryRow,
                1,
                ReportQueryCustodySignal::Fresh,
            ),
            derivation_input!(
                "derived-stale",
                contracts::ReportQueryCustodySourceDataClass::GeneratedSummary,
                2,
                ReportQueryCustodySignal::Stale,
            ),
            derivation_input!(
                "partially-redacted",
                contracts::ReportQueryCustodySourceDataClass::NotificationHistory,
                3,
                ReportQueryCustodySignal::PartiallyRedacted,
            ),
            deleted_input(4),
            conflict_input(5),
            cursor_expired_input(6),
            rate_limited_input(7),
        ],
        contracts::ParentTimestamp::parse("2026-06-28T16:06:00.000Z").assume_ok(),
    )
    .assume_ok();

    assert_eq!(proof.rows.len(), 7);
    assert_eq!(
        proof
            .rows
            .iter()
            .map(|row| row.page_index)
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5, 6, 7]
    );
    assert_eq!(
        proof
            .rows
            .iter()
            .map(|row| row.state.as_str())
            .collect::<Vec<_>>(),
        vec![
            "derivedFresh",
            "derivedStale",
            "partiallyRedacted",
            "deletedSource",
            "syncConflict",
            "cursorExpired",
            "rateLimited",
        ]
    );
}

#[test]
fn report_query_custody_rejects_duplicate_cursors_and_disallowed_data_classes() {
    let request = sample_request();
    let mut duplicate_cursor = derivation_input!(
        "duplicate-cursor-a",
        contracts::ReportQueryCustodySourceDataClass::SqliteQueryRow,
        1,
        ReportQueryCustodySignal::Fresh,
    );
    duplicate_cursor.cursor_ref =
        contracts::ReportQueryCustodyCursorRef::parse("duplicate-cursor").assume_ok();
    let mut duplicate_cursor_b = derivation_input!(
        "duplicate-cursor-b",
        contracts::ReportQueryCustodySourceDataClass::GeneratedSummary,
        2,
        ReportQueryCustodySignal::Stale,
    );
    duplicate_cursor_b.cursor_ref =
        contracts::ReportQueryCustodyCursorRef::parse("duplicate-cursor").assume_ok();

    let duplicate_result = build_report_query_custody_proof(
        &request,
        vec![duplicate_cursor, duplicate_cursor_b],
        contracts::ParentTimestamp::parse("2026-06-28T16:07:00.000Z").assume_ok(),
    );
    assert_eq!(
        duplicate_result,
        Err(ReportQueryCustodyDerivationError::DuplicateCursorRef)
    );

    let mut disallowed_request = request;
    disallowed_request.allowed_source_data_classes =
        vec![contracts::ReportQueryCustodySourceDataClass::SqliteQueryRow];
    let disallowed_input = derivation_input!(
        "disallowed-data-class",
        contracts::ReportQueryCustodySourceDataClass::GeneratedSummary,
        1,
        ReportQueryCustodySignal::Fresh,
    );
    let disallowed = derive_report_query_custody_row(&disallowed_request, disallowed_input);
    assert_eq!(
        disallowed,
        Err(ReportQueryCustodyDerivationError::DisallowedSourceDataClass)
    );
}

fn sample_request() -> contracts::ReportQueryCustodyRequest {
    contracts::sample_report_query_custody_contract_proof().request
}

fn deleted_input(page_index: u32) -> ReportQueryCustodyDerivationInput {
    let mut input = derivation_input!(
        "deleted-source",
        contracts::ReportQueryCustodySourceDataClass::AuditEvent,
        page_index,
        ReportQueryCustodySignal::Deleted,
    );
    input.next_cursor_ref = None;
    input.deleted_source_ref = Some(
        contracts::ReportQueryCustodyDeletedSourceRef::parse("deleted-source-ref-1").assume_ok(),
    );
    input.deleted_source_at =
        Some(contracts::ParentTimestamp::parse("2026-06-28T15:57:00.000Z").assume_ok());
    input.tombstone_confirmed = true;
    input
}

fn conflict_input(page_index: u32) -> ReportQueryCustodyDerivationInput {
    let mut input = derivation_input!(
        "sync-conflict",
        contracts::ReportQueryCustodySourceDataClass::SqliteQueryRow,
        page_index,
        ReportQueryCustodySignal::Conflict,
    );
    input.conflict_ref =
        Some(contracts::ReportQueryCustodyConflictRef::parse("conflict-ref-1").assume_ok());
    input
}

fn cursor_expired_input(page_index: u32) -> ReportQueryCustodyDerivationInput {
    let mut input = derivation_input!(
        "cursor-expired",
        contracts::ReportQueryCustodySourceDataClass::GeneratedSummary,
        page_index,
        ReportQueryCustodySignal::CursorExpired,
    );
    input.next_cursor_ref = None;
    input.cursor_expired_at =
        Some(contracts::ParentTimestamp::parse("2026-06-28T15:59:00.000Z").assume_ok());
    input
}

fn rate_limited_input(page_index: u32) -> ReportQueryCustodyDerivationInput {
    let mut input = derivation_input!(
        "rate-limited",
        contracts::ReportQueryCustodySourceDataClass::NotificationHistory,
        page_index,
        ReportQueryCustodySignal::RateLimited,
    );
    input.next_cursor_ref = None;
    input.rate_limited_until_at =
        Some(contracts::ParentTimestamp::parse("2026-06-28T16:05:00.000Z").assume_ok());
    input
}
