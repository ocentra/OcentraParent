use super::identifiers::{
    account_id, conflict_ref, contract_version, cursor_ref, deleted_source_ref, evidence_id,
    family_id, parent_action_id, parent_actor_id, parent_authority_id, parent_device_id,
    parent_device_label, policy_version, query_cursor, request_id, sort_key, source_ref, timestamp,
};
use super::*;

pub(super) fn sample_report_query_custody_rows(
    request: &ReportQueryCustodyRequest,
) -> Vec<ReportQueryCustodyRow> {
    let mut rows = sample_report_query_custody_rows_primary(request);
    rows.extend(sample_report_query_custody_rows_secondary(request));
    rows
}

fn sample_report_query_custody_rows_primary(
    request: &ReportQueryCustodyRequest,
) -> Vec<ReportQueryCustodyRow> {
    vec![
        sample_row(ReportQueryCustodySampleRowInput {
            request,
            state: ReportQueryCustodyState::DerivedFresh,
            source_freshness: ReportQueryCustodySourceFreshness::Fresh,
            source_data_class: ReportQueryCustodySourceDataClass::SqliteQueryRow,
            page_index: 1,
            next_cursor_ref: Some(cursor_ref(
                REPORT_QUERY_CUSTODY_SAMPLE_DERIVED_FRESH_NEXT_CURSOR,
            )),
            deleted_source_ref: None,
            deleted_source_at: None,
            conflict_ref: None,
            state_timestamp: None,
            payload_redaction_state: ReportQueryCustodyPayloadRedaction::FullyRedacted,
            tombstone_state: ReportQueryCustodyTombstoneState::NotRequired,
        }),
        sample_row(ReportQueryCustodySampleRowInput {
            request,
            state: ReportQueryCustodyState::DerivedStale,
            source_freshness: ReportQueryCustodySourceFreshness::Stale,
            source_data_class: ReportQueryCustodySourceDataClass::GeneratedSummary,
            page_index: 2,
            next_cursor_ref: Some(cursor_ref(
                REPORT_QUERY_CUSTODY_SAMPLE_DERIVED_STALE_NEXT_CURSOR,
            )),
            deleted_source_ref: None,
            deleted_source_at: None,
            conflict_ref: None,
            state_timestamp: None,
            payload_redaction_state: ReportQueryCustodyPayloadRedaction::FullyRedacted,
            tombstone_state: ReportQueryCustodyTombstoneState::NotRequired,
        }),
        sample_row(ReportQueryCustodySampleRowInput {
            request,
            state: ReportQueryCustodyState::PartiallyRedacted,
            source_freshness: ReportQueryCustodySourceFreshness::Stale,
            source_data_class: ReportQueryCustodySourceDataClass::NotificationHistory,
            page_index: 3,
            next_cursor_ref: Some(cursor_ref(
                REPORT_QUERY_CUSTODY_SAMPLE_PARTIALLY_REDACTED_NEXT_CURSOR,
            )),
            deleted_source_ref: None,
            deleted_source_at: None,
            conflict_ref: None,
            state_timestamp: None,
            payload_redaction_state: ReportQueryCustodyPayloadRedaction::PartiallyRedacted,
            tombstone_state: ReportQueryCustodyTombstoneState::NotRequired,
        }),
        sample_row(ReportQueryCustodySampleRowInput {
            request,
            state: ReportQueryCustodyState::DeletedSource,
            source_freshness: ReportQueryCustodySourceFreshness::Deleted,
            source_data_class: ReportQueryCustodySourceDataClass::AuditEvent,
            page_index: 4,
            next_cursor_ref: None,
            deleted_source_ref: Some(deleted_source_ref(
                REPORT_QUERY_CUSTODY_SAMPLE_DELETED_SOURCE_REF,
            )),
            deleted_source_at: Some(timestamp(REPORT_QUERY_CUSTODY_SAMPLE_DELETED_SOURCE_AT)),
            conflict_ref: None,
            state_timestamp: None,
            payload_redaction_state: ReportQueryCustodyPayloadRedaction::FullyRedacted,
            tombstone_state: ReportQueryCustodyTombstoneState::Written,
        }),
    ]
}

fn sample_report_query_custody_rows_secondary(
    request: &ReportQueryCustodyRequest,
) -> Vec<ReportQueryCustodyRow> {
    vec![
        sample_row(ReportQueryCustodySampleRowInput {
            request,
            state: ReportQueryCustodyState::SyncConflict,
            source_freshness: ReportQueryCustodySourceFreshness::Conflicted,
            source_data_class: ReportQueryCustodySourceDataClass::SqliteQueryRow,
            page_index: 5,
            next_cursor_ref: Some(cursor_ref(
                REPORT_QUERY_CUSTODY_SAMPLE_SYNC_CONFLICT_NEXT_CURSOR,
            )),
            deleted_source_ref: None,
            deleted_source_at: None,
            conflict_ref: Some(conflict_ref(REPORT_QUERY_CUSTODY_SAMPLE_CONFLICT_REF)),
            state_timestamp: None,
            payload_redaction_state: ReportQueryCustodyPayloadRedaction::FullyRedacted,
            tombstone_state: ReportQueryCustodyTombstoneState::NotRequired,
        }),
        sample_row(ReportQueryCustodySampleRowInput {
            request,
            state: ReportQueryCustodyState::CursorExpired,
            source_freshness: ReportQueryCustodySourceFreshness::Expired,
            source_data_class: ReportQueryCustodySourceDataClass::GeneratedSummary,
            page_index: 6,
            next_cursor_ref: None,
            deleted_source_ref: None,
            deleted_source_at: None,
            conflict_ref: None,
            state_timestamp: Some(timestamp(REPORT_QUERY_CUSTODY_SAMPLE_CURSOR_EXPIRED_AT)),
            payload_redaction_state: ReportQueryCustodyPayloadRedaction::FullyRedacted,
            tombstone_state: ReportQueryCustodyTombstoneState::NotRequired,
        }),
        sample_row(ReportQueryCustodySampleRowInput {
            request,
            state: ReportQueryCustodyState::RateLimited,
            source_freshness: ReportQueryCustodySourceFreshness::RateLimited,
            source_data_class: ReportQueryCustodySourceDataClass::NotificationHistory,
            page_index: 7,
            next_cursor_ref: None,
            deleted_source_ref: None,
            deleted_source_at: None,
            conflict_ref: None,
            state_timestamp: Some(timestamp(REPORT_QUERY_CUSTODY_SAMPLE_RATE_LIMITED_UNTIL_AT)),
            payload_redaction_state: ReportQueryCustodyPayloadRedaction::FullyRedacted,
            tombstone_state: ReportQueryCustodyTombstoneState::NotRequired,
        }),
    ]
}

struct ReportQueryCustodySampleRowInput<'a> {
    request: &'a ReportQueryCustodyRequest,
    state: ReportQueryCustodyState,
    source_freshness: ReportQueryCustodySourceFreshness,
    source_data_class: ReportQueryCustodySourceDataClass,
    page_index: u32,
    next_cursor_ref: Option<ReportQueryCustodyCursorRef>,
    deleted_source_ref: Option<ReportQueryCustodyDeletedSourceRef>,
    deleted_source_at: Option<ParentTimestamp>,
    conflict_ref: Option<ReportQueryCustodyConflictRef>,
    state_timestamp: Option<ParentTimestamp>,
    payload_redaction_state: ReportQueryCustodyPayloadRedaction,
    tombstone_state: ReportQueryCustodyTombstoneState,
}

fn sample_row(input: ReportQueryCustodySampleRowInput<'_>) -> ReportQueryCustodyRow {
    let ReportQueryCustodySampleRowInput {
        request,
        state,
        source_freshness,
        source_data_class,
        page_index,
        next_cursor_ref,
        deleted_source_ref,
        deleted_source_at,
        conflict_ref,
        state_timestamp,
        payload_redaction_state,
        tombstone_state,
    } = input;

    let is_rate_limited = state == ReportQueryCustodyState::RateLimited;
    let is_cursor_expired = state == ReportQueryCustodyState::CursorExpired;
    let row_cursor_ref = match page_index {
        1 => cursor_ref(request.requested_cursor.to_string()),
        2 => cursor_ref(REPORT_QUERY_CUSTODY_SAMPLE_DERIVED_FRESH_NEXT_CURSOR),
        3 => cursor_ref(REPORT_QUERY_CUSTODY_SAMPLE_DERIVED_STALE_NEXT_CURSOR),
        4 => cursor_ref(REPORT_QUERY_CUSTODY_SAMPLE_PARTIALLY_REDACTED_NEXT_CURSOR),
        6 => cursor_ref(REPORT_QUERY_CUSTODY_SAMPLE_SYNC_CONFLICT_NEXT_CURSOR),
        _ => cursor_ref(format!("report-query-custody-cursor-{}", state.as_str())),
    };
    let stable_sort_key = sort_key(format!(
        "{}-{:02}",
        REPORT_QUERY_CUSTODY_SAMPLE_STABLE_SORT_KEY, page_index
    ));

    ReportQueryCustodyRow {
        row_id: source_ref(format!("report-query-custody-row-{}", state.as_str())),
        request_id: request.request_id.clone(),
        state,
        source_freshness,
        source_data_class,
        cursor_ref: row_cursor_ref,
        source_cursor_ref: cursor_ref(REPORT_QUERY_CUSTODY_SAMPLE_SOURCE_CURSOR_REF),
        next_cursor_ref,
        page_index,
        page_size: request.page_size,
        stable_sort_key,
        requested_data_classes: request.requested_data_classes.clone(),
        allowed_source_data_classes: request.allowed_source_data_classes.clone(),
        source_citation_refs: request.source_citation_refs.clone(),
        assistant_citation_refs: request.assistant_citation_refs.clone(),
        notification_payload_boundary: request.notification_payload_boundary,
        payload_redaction_state,
        tombstone_state,
        deleted_source_ref,
        deleted_source_at,
        conflict_ref,
        cursor_expired_at: if is_cursor_expired {
            state_timestamp.clone()
        } else {
            None
        },
        rate_limited_until_at: if is_rate_limited {
            state_timestamp
        } else {
            None
        },
        parent_authority: request.parent_authority.clone(),
        raw_child_evidence_included: false,
        report_cache_mutated: false,
        second_truth_store_claimed: false,
        claim_safe: true,
    }
}

pub(super) fn allowed_data_classes() -> Vec<ReportQueryCustodySourceDataClass> {
    vec![
        ReportQueryCustodySourceDataClass::SqliteQueryRow,
        ReportQueryCustodySourceDataClass::NotificationHistory,
        ReportQueryCustodySourceDataClass::AuditEvent,
        ReportQueryCustodySourceDataClass::GeneratedSummary,
    ]
}

pub(super) fn allowed_citation_refs(timestamp: &ParentTimestamp) -> Vec<ParentEvidenceReference> {
    vec![
        ParentEvidenceReference {
            evidence_reference_id: evidence_id(REPORT_QUERY_CUSTODY_SAMPLE_EVIDENCE_ID_ONE),
            kind: ParentEvidenceReferenceKind::QueryStoreSummary,
            observed_at: timestamp.clone(),
            family_id: family_id(REPORT_QUERY_CUSTODY_SAMPLE_FAMILY_ID),
            child_profile_id: None,
            source_data_class: ReportQueryCustodySourceDataClass::SqliteQueryRow,
            source_reference: source_ref(REPORT_QUERY_CUSTODY_SAMPLE_SOURCE_REF_ONE),
        },
        ParentEvidenceReference {
            evidence_reference_id: evidence_id(REPORT_QUERY_CUSTODY_SAMPLE_EVIDENCE_ID_TWO),
            kind: ParentEvidenceReferenceKind::QueryStoreSummary,
            observed_at: timestamp.clone(),
            family_id: family_id(REPORT_QUERY_CUSTODY_SAMPLE_FAMILY_ID),
            child_profile_id: None,
            source_data_class: ReportQueryCustodySourceDataClass::NotificationHistory,
            source_reference: source_ref(REPORT_QUERY_CUSTODY_SAMPLE_SOURCE_REF_TWO),
        },
    ]
}

pub(super) fn report_query_custody_known_gaps() -> [&'static str; 5] {
    [
        REPORT_QUERY_CUSTODY_KNOWN_GAP_NO_SECOND_TRUTH_STORE,
        REPORT_QUERY_CUSTODY_KNOWN_GAP_NO_PORTAL_OR_RAW_CHILD,
        REPORT_QUERY_CUSTODY_KNOWN_GAP_PAGINATION_DERIVED,
        REPORT_QUERY_CUSTODY_KNOWN_GAP_EXPLICIT_OUTCOMES,
        REPORT_QUERY_CUSTODY_KNOWN_GAP_QUERY_STORE_SUMMARY_ONLY,
    ]
}

pub(super) fn required_report_query_custody_non_claims() -> Vec<ReportQueryCustodyNonClaim> {
    vec![
        ReportQueryCustodyNonClaim::SecondTruthStore,
        ReportQueryCustodyNonClaim::PortalUi,
        ReportQueryCustodyNonClaim::RawChildEvidence,
        ReportQueryCustodyNonClaim::UnboundedPagination,
        ReportQueryCustodyNonClaim::ProviderRouting,
        ReportQueryCustodyNonClaim::OcentraHostedFamilyDataCustody,
    ]
}

pub(super) fn sample_report_query_custody_contract_proof() -> ReportQueryCustodyContractProof {
    let proof_timestamp = timestamp(REPORT_QUERY_CUSTODY_SAMPLE_PROOF_TIMESTAMP);
    let request = ReportQueryCustodyRequest {
        schema_version: REPORT_QUERY_CUSTODY_SCHEMA_VERSION.to_string(),
        request_id: request_id(REPORT_QUERY_CUSTODY_SAMPLE_REQUEST_ID),
        family: FamilyReference {
            family_id: family_id(REPORT_QUERY_CUSTODY_SAMPLE_FAMILY_ID),
        },
        account: ParentAccountReference {
            parent_account_id: account_id(REPORT_QUERY_CUSTODY_SAMPLE_ACCOUNT_ID),
        },
        device: ParentDeviceReference {
            device_id: parent_device_id(REPORT_QUERY_CUSTODY_SAMPLE_PARENT_DEVICE_ID),
            child_profile_id: None,
            label: parent_device_label(REPORT_QUERY_CUSTODY_SAMPLE_PARENT_DEVICE_LABEL),
            platform: ParentPlatform::Windows,
        },
        parent_action: ParentActionReference {
            action_reference_id: parent_action_id(REPORT_QUERY_CUSTODY_SAMPLE_PARENT_ACTION_ID),
            actor: ParentActorReference {
                actor_id: parent_actor_id(REPORT_QUERY_CUSTODY_SAMPLE_PARENT_ACTOR_ID),
                role: ParentActorRole::Parent,
            },
            policy_version: policy_version(REPORT_QUERY_CUSTODY_SAMPLE_POLICY_VERSION),
            created_at: proof_timestamp.clone(),
        },
        requested_cursor: query_cursor(REPORT_QUERY_CUSTODY_SAMPLE_REQUESTED_CURSOR),
        page_size: REPORT_QUERY_CUSTODY_PAGE_SIZE,
        requested_data_classes: allowed_data_classes(),
        allowed_source_data_classes: allowed_data_classes(),
        source_citation_refs: allowed_citation_refs(&proof_timestamp),
        assistant_citation_refs: allowed_citation_refs(&proof_timestamp),
        notification_payload_boundary: ReportQueryCustodyBoundary::ParentOwnedCitationsOnly,
        parent_authority: ReportQueryCustodyParentAuthorityReference {
            authority_reference_id: parent_authority_id(
                REPORT_QUERY_CUSTODY_SAMPLE_PARENT_AUTHORITY_ID,
            ),
            family_id: family_id(REPORT_QUERY_CUSTODY_SAMPLE_FAMILY_ID),
            parent_account_id: account_id(REPORT_QUERY_CUSTODY_SAMPLE_ACCOUNT_ID),
            device_id: parent_device_id(REPORT_QUERY_CUSTODY_SAMPLE_PARENT_DEVICE_ID),
            child_profile_id: None,
            authority_generation: 1,
        },
        raw_child_evidence_requested: false,
    };

    ReportQueryCustodyContractProof {
        schema_version: REPORT_QUERY_CUSTODY_SCHEMA_VERSION.to_string(),
        contract_version: contract_version(REPORT_QUERY_CUSTODY_SAMPLE_CONTRACT_VERSION),
        request: request.clone(),
        rows: sample_report_query_custody_rows(&request),
        non_claims: required_report_query_custody_non_claims(),
        report_runtime_claimed: false,
        portal_ui_claimed: false,
        provider_routing_claimed: false,
        ocentra_hosted_family_data_custody_claimed: false,
        second_truth_store_claimed: false,
        raw_child_evidence_claimed: false,
        updated_at: proof_timestamp,
    }
}
