use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceRef, LogFieldValue, LogFields, TrackingEvidenceRef,
    TrackingReadModel, TrackingReadModelCapabilityStatus, TrackingReadModelCount,
    TrackingReadModelCountValue, TrackingReadModelCustodyLabel, TrackingReadModelDeletedAt,
    TrackingReadModelDeviceId, TrackingReadModelEventId, TrackingReadModelGeneratedAt,
    TrackingReadModelKind, TrackingReadModelObservedAt, TrackingReadModelObserver,
    TrackingReadModelPlatform, TrackingReadModelQueryVisibility, TrackingReadModelRow,
    TrackingReadModelSubjectDisplayName, TrackingReadModelSubjectId, TrackingReadModelSubjectKind,
    ACTIVITY_QUERY_SCHEMA_VERSION, TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE, TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE,
    TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS,
};
use rusqlite::Connection;
use std::collections::BTreeMap;

use super::read_model_rows::{tracking_rows, TrackingStoreRow};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingReadModelRowLifecycleState {
    Active,
    Tombstone,
}

pub fn tracking_read_model_for_connection(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> rusqlite::Result<TrackingReadModel> {
    tracking_read_model(connection, limit, generated_at)
}

fn tracking_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> rusqlite::Result<TrackingReadModel> {
    let rows = tracking_rows(connection, limit)?;
    let read_rows = rows.into_iter().map(row_from_store).collect::<Vec<_>>();
    let latest = read_rows.first();
    let latest_tombstone = read_rows
        .iter()
        .find(|row| row.query_visibility == TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE);
    let latest_active = read_rows
        .iter()
        .find(|row| row.query_visibility == TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE);
    let tombstone_rows = read_rows
        .iter()
        .filter(|row| row.query_visibility == TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE)
        .count() as u64;
    let active_rows = read_rows.len() as u64 - tombstone_rows;
    let deleted_evidence_reference_ids = deleted_evidence_reference_ids(&read_rows);
    let capability_status = latest
        .and_then(|row| row.capability_status.clone())
        .unwrap_or_else(|| {
            read_model_capability_status(TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS)
        });

    Ok(TrackingReadModel {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: read_model_generated_at(generated_at),
        custody_label: read_model_custody_label(
            TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
        ),
        limit,
        returned: read_rows.len() as u64,
        active_rows,
        tombstone_rows,
        capability_status,
        latest_event_id: latest.map(|row| row.event_id.clone()),
        latest_observed_at: latest.map(|row| row.observed_at.clone()),
        latest_active_event_id: latest_active.map(|row| row.event_id.clone()),
        latest_active_observed_at: latest_active.map(|row| row.observed_at.clone()),
        latest_tombstone_event_id: latest_tombstone.map(|row| row.event_id.clone()),
        latest_tombstone_observed_at: latest_tombstone.map(|row| row.observed_at.clone()),
        active_kind_counts: active_counts_by(&read_rows, |row| Some(row.kind.as_str())),
        active_device_counts: active_counts_by(&read_rows, |row| Some(row.device_id.as_str())),
        active_capability_status_counts: active_counts_by(&read_rows, |row| {
            row.capability_status.as_ref().map(|value| value.as_str())
        }),
        deleted_evidence_reference_ids,
        rows: read_rows,
    })
}

fn row_from_store(row: TrackingStoreRow) -> TrackingReadModelRow {
    let evidence_reference_ids = evidence_reference_ids(&row.fields, &row.evidence);
    let lifecycle_state = row_lifecycle_state(&row);
    let query_visibility = query_visibility(lifecycle_state);
    let deleted_at = deleted_at(&row, lifecycle_state);
    let deleted_evidence_reference_ids =
        if lifecycle_state == TrackingReadModelRowLifecycleState::Tombstone {
            evidence_reference_ids.clone()
        } else {
            Vec::new()
        };
    let capability_status = string_field(&row.fields, constants::field::CAPABILITY_STATUS);

    TrackingReadModelRow {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        event_id: read_model_event_id(&row.event_id),
        observed_at: read_model_observed_at(&row.observed_at),
        device_id: read_model_device_id(&row.device_id),
        platform: read_model_platform(&row.platform),
        observer: read_model_observer(&row.observer),
        kind: read_model_kind(&row.kind),
        subject_kind: read_model_subject_kind(&row.subject_kind),
        subject_id: read_model_subject_id(&row.subject_id),
        subject_display_name: row
            .subject_display_name
            .as_deref()
            .map(read_model_subject_display_name),
        capability_status: capability_status
            .as_deref()
            .map(read_model_capability_status),
        query_visibility,
        deleted_at,
        evidence_reference_ids,
        deleted_evidence_reference_ids,
        evidence: row.evidence,
    }
}

fn row_lifecycle_state(row: &TrackingStoreRow) -> TrackingReadModelRowLifecycleState {
    if row.kind == constants::activity_event_kind::TRACKING_RETENTION_DELETED {
        TrackingReadModelRowLifecycleState::Tombstone
    } else {
        TrackingReadModelRowLifecycleState::Active
    }
}

fn query_visibility(
    lifecycle_state: TrackingReadModelRowLifecycleState,
) -> TrackingReadModelQueryVisibility {
    match lifecycle_state {
        TrackingReadModelRowLifecycleState::Tombstone => {
            read_model_query_visibility(TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE)
        }
        TrackingReadModelRowLifecycleState::Active => {
            read_model_query_visibility(TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE)
        }
    }
}

fn deleted_at(
    row: &TrackingStoreRow,
    lifecycle_state: TrackingReadModelRowLifecycleState,
) -> Option<TrackingReadModelDeletedAt> {
    match lifecycle_state {
        TrackingReadModelRowLifecycleState::Active => None,
        TrackingReadModelRowLifecycleState::Tombstone => {
            string_field(&row.fields, constants::field::DELETED_AT)
                .or_else(|| Some(row.observed_at.clone()))
                .map(|value| {
                    TrackingReadModelDeletedAt::parse(value)
                        .expect(constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT)
                })
        }
    }
}

fn deleted_evidence_reference_ids(rows: &[TrackingReadModelRow]) -> Vec<TrackingEvidenceRef> {
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

fn active_counts_by(
    rows: &[TrackingReadModelRow],
    value_for_row: impl Fn(&TrackingReadModelRow) -> Option<&str>,
) -> Vec<TrackingReadModelCount> {
    let mut counts = BTreeMap::<String, u64>::new();
    for row in rows {
        if row.query_visibility != TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE {
            continue;
        }
        if let Some(value) = value_for_row(row) {
            *counts.entry(value.to_string()).or_insert(0) += 1;
        }
    }
    counts
        .into_iter()
        .map(|(value, count)| TrackingReadModelCount {
            value: TrackingReadModelCountValue::parse(value)
                .expect(TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS),
            count,
        })
        .collect()
}

fn evidence_reference_ids(
    fields: &LogFields,
    evidence: &[ActivityEvidenceRef],
) -> Vec<TrackingEvidenceRef> {
    let mut ids = string_field(fields, constants::field::EVIDENCE_REFERENCE_IDS)
        .map(|value| split_evidence_reference_ids(&value))
        .unwrap_or_default();

    for reference in evidence {
        let evidence_ref = TrackingEvidenceRef::parse(reference.evidence_id.clone())
            .expect(constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID);
        if !ids.iter().any(|id| id == &evidence_ref) {
            ids.push(evidence_ref);
        }
    }
    ids
}

fn split_evidence_reference_ids(value: &str) -> Vec<TrackingEvidenceRef> {
    value
        .split(constants::delimiter::LIST)
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(|id| {
            TrackingEvidenceRef::parse(id)
                .expect(constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID)
        })
        .collect()
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}

fn read_model_event_id(value: &str) -> TrackingReadModelEventId {
    TrackingReadModelEventId::parse(value)
        .expect(constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID)
}

fn read_model_observed_at(value: &str) -> TrackingReadModelObservedAt {
    TrackingReadModelObservedAt::parse(value)
        .expect(constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT)
}

fn read_model_device_id(value: &str) -> TrackingReadModelDeviceId {
    TrackingReadModelDeviceId::parse(value).expect(constants::activity_store::TEST_REMOTE_DEVICE_ID)
}

fn read_model_platform(value: &str) -> TrackingReadModelPlatform {
    TrackingReadModelPlatform::parse(value)
        .expect(constants::activity_store::TEST_TRACKING_PLATFORM_ANDROID)
}

fn read_model_observer(value: &str) -> TrackingReadModelObserver {
    TrackingReadModelObserver::parse(value).expect(constants::activity_observer::ANDROID_LOCATION)
}

fn read_model_kind(value: &str) -> TrackingReadModelKind {
    TrackingReadModelKind::parse(value).expect(constants::activity_event_kind::LOCATION_OBSERVED)
}

fn read_model_subject_kind(value: &str) -> TrackingReadModelSubjectKind {
    TrackingReadModelSubjectKind::parse(value).expect(constants::activity_subject_kind::LOCATION)
}

fn read_model_subject_id(value: &str) -> TrackingReadModelSubjectId {
    TrackingReadModelSubjectId::parse(value)
        .expect(constants::activity_store::TEST_TRACKING_SUBJECT_ID)
}

fn read_model_subject_display_name(value: &str) -> TrackingReadModelSubjectDisplayName {
    TrackingReadModelSubjectDisplayName::parse(value)
        .expect(constants::activity_store::TEST_TRACKING_SUBJECT_NAME)
}

fn read_model_capability_status(value: &str) -> TrackingReadModelCapabilityStatus {
    TrackingReadModelCapabilityStatus::parse(value)
        .expect(TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS)
}

fn read_model_query_visibility(value: &'static str) -> TrackingReadModelQueryVisibility {
    TrackingReadModelQueryVisibility::parse(value).expect(TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE)
}

fn read_model_generated_at(value: &str) -> TrackingReadModelGeneratedAt {
    TrackingReadModelGeneratedAt::parse(value)
        .expect(constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT)
}

fn read_model_custody_label(value: &'static str) -> TrackingReadModelCustodyLabel {
    TrackingReadModelCustodyLabel::parse(value)
        .expect(TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE)
}
