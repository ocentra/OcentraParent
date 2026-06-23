use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::activity_query::ACTIVITY_QUERY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::tracking::identifiers::{
    TrackingEvidenceRef, TrackingReadModelCapabilityStatus, TrackingReadModelCountValue,
    TrackingReadModelCustodyLabel, TrackingReadModelDeletedAt, TrackingReadModelDeviceId,
    TrackingReadModelEventId, TrackingReadModelGeneratedAt, TrackingReadModelKind,
    TrackingReadModelObservedAt, TrackingReadModelObserver, TrackingReadModelPlatform,
    TrackingReadModelQueryVisibility, TrackingReadModelSubjectDisplayName,
    TrackingReadModelSubjectId, TrackingReadModelSubjectKind,
};
use ocentra_parent_agent_protocol::tracking::read_model::{
    TrackingReadModel, TrackingReadModelCount, TrackingReadModelRow,
    TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE, TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE,
    TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS,
};
use rusqlite::Connection;
use std::collections::BTreeMap;

use super::read_model_rows::{tracking_rows, TrackingStoreRow};

macro_rules! parse_read_model {
    ($parse:path, $value:expr, $expectation:expr) => {
        match $parse($value) {
            Ok(parsed_value) => parsed_value,
            Err(_) => unreachable!("tracking read-model parse drift: {}", $expectation),
        }
    };
}

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
            parse_read_model!(
                TrackingReadModelCapabilityStatus::parse,
                TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS,
                TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS
            )
        });

    Ok(TrackingReadModel {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: parse_read_model!(
            TrackingReadModelGeneratedAt::parse,
            generated_at,
            constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT
        ),
        custody_label: parse_read_model!(
            TrackingReadModelCustodyLabel::parse,
            TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
            TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE
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
        event_id: parse_read_model!(
            TrackingReadModelEventId::parse,
            &row.event_id,
            constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID
        ),
        observed_at: parse_read_model!(
            TrackingReadModelObservedAt::parse,
            &row.observed_at,
            constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT
        ),
        device_id: parse_read_model!(
            TrackingReadModelDeviceId::parse,
            &row.device_id,
            constants::activity_store::TEST_REMOTE_DEVICE_ID
        ),
        platform: parse_read_model!(
            TrackingReadModelPlatform::parse,
            &row.platform,
            constants::activity_store::TEST_TRACKING_PLATFORM_ANDROID
        ),
        observer: parse_read_model!(
            TrackingReadModelObserver::parse,
            &row.observer,
            constants::activity_observer::ANDROID_LOCATION
        ),
        kind: parse_read_model!(
            TrackingReadModelKind::parse,
            &row.kind,
            constants::activity_event_kind::LOCATION_OBSERVED
        ),
        subject_kind: parse_read_model!(
            TrackingReadModelSubjectKind::parse,
            &row.subject_kind,
            constants::activity_subject_kind::LOCATION
        ),
        subject_id: parse_read_model!(
            TrackingReadModelSubjectId::parse,
            &row.subject_id,
            constants::activity_store::TEST_TRACKING_SUBJECT_ID
        ),
        subject_display_name: row.subject_display_name.as_deref().map(|value| {
            parse_read_model!(
                TrackingReadModelSubjectDisplayName::parse,
                value,
                constants::activity_store::TEST_TRACKING_SUBJECT_NAME
            )
        }),
        capability_status: capability_status.as_deref().map(|value| {
            parse_read_model!(
                TrackingReadModelCapabilityStatus::parse,
                value,
                TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS
            )
        }),
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
        TrackingReadModelRowLifecycleState::Tombstone => parse_read_model!(
            TrackingReadModelQueryVisibility::parse,
            TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE,
            TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE
        ),
        TrackingReadModelRowLifecycleState::Active => parse_read_model!(
            TrackingReadModelQueryVisibility::parse,
            TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE,
            TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE
        ),
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
                    parse_read_model!(
                        TrackingReadModelDeletedAt::parse,
                        value.as_str(),
                        constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT
                    )
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
            value: parse_read_model!(
                TrackingReadModelCountValue::parse,
                value,
                TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS
            ),
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
        let evidence_ref = parse_read_model!(
            TrackingEvidenceRef::parse,
            reference.evidence_id.clone(),
            constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID
        );
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
            parse_read_model!(
                TrackingEvidenceRef::parse,
                id,
                constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID
            )
        })
        .collect()
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}
