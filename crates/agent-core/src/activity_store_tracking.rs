use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceRef, LogFieldValue, LogFields, TrackingReadModel,
    TrackingReadModelCoverageRow, TrackingReadModelProductClaimState, TrackingReadModelRow,
    ACTIVITY_QUERY_SCHEMA_VERSION, TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    TRACKING_READ_MODEL_MISSING_PROOF_CHILD_RUNTIME,
    TRACKING_READ_MODEL_MISSING_PROOF_PLATFORM_REPLAY,
    TRACKING_READ_MODEL_MISSING_PROOF_PRODUCT_UI, TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE,
    TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE, TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS,
    TRACKING_READ_MODEL_SURFACE_CHILD_CHECK_IN, TRACKING_READ_MODEL_SURFACE_EXPECTED_PLACE,
    TRACKING_READ_MODEL_SURFACE_GEOFENCE, TRACKING_READ_MODEL_SURFACE_LOCATION,
    TRACKING_READ_MODEL_SURFACE_RETENTION,
};
use rusqlite::Connection;

use crate::{
    activity_store_tracking_rows::{tracking_rows, TrackingStoreRow},
    ActivityStore, ActivityStoreError,
};

pub fn tracking_read_model_for_store(
    store: &ActivityStore,
    limit: u64,
    generated_at: &str,
) -> Result<TrackingReadModel, ActivityStoreError> {
    tracking_read_model(&store.connection, limit, generated_at)
}

fn tracking_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> Result<TrackingReadModel, ActivityStoreError> {
    let rows = tracking_rows(connection, limit)?;
    let read_rows = rows.into_iter().map(row_from_store).collect::<Vec<_>>();
    let latest = read_rows.first();
    let latest_tombstone = read_rows
        .iter()
        .find(|row| row.query_visibility == TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE);
    let tombstone_rows = read_rows
        .iter()
        .filter(|row| row.query_visibility == TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE)
        .count() as u64;
    let active_rows = read_rows.len() as u64 - tombstone_rows;
    let deleted_evidence_reference_ids = deleted_evidence_reference_ids(&read_rows);
    let capability_status = latest
        .and_then(|row| row.capability_status.clone())
        .unwrap_or_else(|| TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS.to_string());

    Ok(TrackingReadModel {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        custody_label: TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        limit,
        returned: read_rows.len() as u64,
        active_rows,
        tombstone_rows,
        capability_status,
        latest_event_id: latest.map(|row| row.event_id.clone()),
        latest_observed_at: latest.map(|row| row.observed_at.clone()),
        latest_tombstone_event_id: latest_tombstone.map(|row| row.event_id.clone()),
        latest_tombstone_observed_at: latest_tombstone.map(|row| row.observed_at.clone()),
        deleted_evidence_reference_ids,
        coverage_rows: coverage_rows(&read_rows),
        product_claim_state: product_claim_state(),
        rows: read_rows,
    })
}

fn row_from_store(row: TrackingStoreRow) -> TrackingReadModelRow {
    let evidence_reference_ids = evidence_reference_ids(&row.fields, &row.evidence);
    let is_tombstone = row.kind == constants::activity_event_kind::TRACKING_RETENTION_DELETED;
    let query_visibility = query_visibility(is_tombstone);
    let deleted_at = deleted_at(&row, is_tombstone);
    let deleted_evidence_reference_ids = if is_tombstone {
        evidence_reference_ids.clone()
    } else {
        Vec::new()
    };
    let capability_status = string_field(&row.fields, constants::field::CAPABILITY_STATUS);

    TrackingReadModelRow {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        event_id: row.event_id,
        observed_at: row.observed_at,
        device_id: row.device_id,
        platform: row.platform,
        observer: row.observer,
        kind: row.kind,
        subject_kind: row.subject_kind,
        subject_id: row.subject_id,
        subject_display_name: row.subject_display_name,
        capability_status,
        query_visibility,
        deleted_at,
        evidence_reference_ids,
        deleted_evidence_reference_ids,
        evidence: row.evidence,
    }
}

fn query_visibility(is_tombstone: bool) -> String {
    if is_tombstone {
        TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE.to_string()
    } else {
        TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE.to_string()
    }
}

fn deleted_at(row: &TrackingStoreRow, is_tombstone: bool) -> Option<String> {
    if !is_tombstone {
        return None;
    }
    string_field(&row.fields, constants::field::DELETED_AT)
        .or_else(|| Some(row.observed_at.clone()))
}

fn deleted_evidence_reference_ids(rows: &[TrackingReadModelRow]) -> Vec<String> {
    let mut ids = Vec::new();
    for row in rows {
        for id in &row.deleted_evidence_reference_ids {
            if !ids.iter().any(|existing| existing == id) {
                ids.push(id.clone());
            }
        }
    }
    ids
}

fn coverage_rows(rows: &[TrackingReadModelRow]) -> Vec<TrackingReadModelCoverageRow> {
    vec![
        coverage_row(
            rows,
            TRACKING_READ_MODEL_SURFACE_LOCATION,
            &[constants::activity_event_kind::LOCATION_OBSERVED],
            TRACKING_READ_MODEL_MISSING_PROOF_PLATFORM_REPLAY,
        ),
        coverage_row(
            rows,
            TRACKING_READ_MODEL_SURFACE_GEOFENCE,
            &[constants::activity_event_kind::TRACKING_GEOFENCE_TRANSITION_EVALUATED],
            TRACKING_READ_MODEL_MISSING_PROOF_PLATFORM_REPLAY,
        ),
        coverage_row(
            rows,
            TRACKING_READ_MODEL_SURFACE_EXPECTED_PLACE,
            &[constants::activity_event_kind::TRACKING_EXPECTED_PLACE_EVALUATED],
            TRACKING_READ_MODEL_MISSING_PROOF_PLATFORM_REPLAY,
        ),
        coverage_row(
            rows,
            TRACKING_READ_MODEL_SURFACE_CHILD_CHECK_IN,
            &[constants::activity_event_kind::TRACKING_CHILD_CHECK_IN_RESPONDED],
            TRACKING_READ_MODEL_MISSING_PROOF_CHILD_RUNTIME,
        ),
        coverage_row(
            rows,
            TRACKING_READ_MODEL_SURFACE_RETENTION,
            &[constants::activity_event_kind::TRACKING_RETENTION_DELETED],
            TRACKING_READ_MODEL_MISSING_PROOF_PRODUCT_UI,
        ),
    ]
}

fn coverage_row(
    rows: &[TrackingReadModelRow],
    surface: &str,
    event_kinds: &[&str],
    missing_proof: &str,
) -> TrackingReadModelCoverageRow {
    let matching = rows
        .iter()
        .filter(|row| event_kinds.iter().any(|kind| kind == &row.kind))
        .collect::<Vec<_>>();
    let active_rows = matching
        .iter()
        .filter(|row| row.query_visibility == TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE)
        .count() as u64;
    let tombstone_rows = matching
        .iter()
        .filter(|row| row.query_visibility == TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE)
        .count() as u64;
    let latest = matching.first();

    TrackingReadModelCoverageRow {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        surface: surface.to_string(),
        active_rows,
        tombstone_rows,
        citation_count: coverage_citation_count(&matching),
        latest_event_id: latest.map(|row| row.event_id.clone()),
        latest_observed_at: latest.map(|row| row.observed_at.clone()),
        ready_for_product_claim: false,
        missing_proof: missing_proof.to_string(),
    }
}

fn coverage_citation_count(rows: &[&TrackingReadModelRow]) -> u64 {
    let mut ids = Vec::new();
    for row in rows {
        for id in &row.evidence_reference_ids {
            push_unique(&mut ids, id);
        }
        for id in &row.deleted_evidence_reference_ids {
            push_unique(&mut ids, id);
        }
    }
    ids.len() as u64
}

fn push_unique(ids: &mut Vec<String>, value: &str) {
    if !ids.iter().any(|existing| existing == value) {
        ids.push(value.to_string());
    }
}

fn product_claim_state() -> TrackingReadModelProductClaimState {
    TrackingReadModelProductClaimState {
        physical_device_claimed: false,
        provider_delivery_claimed: false,
        notification_delivery_claimed: false,
        child_device_runtime_claimed: false,
        ocentra_hosted_storage_claimed: false,
        product_complete_claimed: false,
    }
}

fn evidence_reference_ids(fields: &LogFields, evidence: &[ActivityEvidenceRef]) -> Vec<String> {
    let mut ids = string_field(fields, constants::field::EVIDENCE_REFERENCE_IDS)
        .map(|value| split_evidence_reference_ids(&value))
        .unwrap_or_default();

    for reference in evidence {
        if !ids.iter().any(|id| id == &reference.evidence_id) {
            ids.push(reference.evidence_id.clone());
        }
    }
    ids
}

fn split_evidence_reference_ids(value: &str) -> Vec<String> {
    value
        .split(constants::delimiter::LIST)
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}
