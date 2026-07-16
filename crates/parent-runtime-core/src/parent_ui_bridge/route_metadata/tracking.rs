use super::common::{parse_identifier, parse_optional_identifier};
use super::*;

const ROUTE_TITLES: &[(&str, &str)] = &[
    ("overview", "Overview"),
    ("assistant", "Assistant"),
    ("start", "Start"),
    ("activity", "Activity"),
    ("browser", "Browser"),
    ("browser-settings", "Browser settings"),
    ("policy", "Policy"),
    ("policy-apps", "Policy apps"),
    ("policy-games", "Policy games"),
    ("policy-screen", "Policy screen"),
    ("policy-network", "Policy network"),
    ("policy-tracking", "Policy tracking"),
    ("policy-remote-screen", "Policy remote screen"),
    ("rule-management", "Rule management"),
    ("schedules", "Schedules"),
    ("approvals", "Approvals"),
    ("enforcement", "Enforcement"),
    ("privacy-design", "Privacy design"),
    ("memory", "Memory"),
    ("memory-settings", "Memory settings"),
    ("ai-guide", "AI guide"),
    ("ai-runtime", "AI runtime"),
    ("api-providers", "API providers"),
    ("reports-guide", "Reports guide"),
    ("screen-analysis", "Screen analysis"),
    ("app-game-sessions", "App game sessions"),
    ("network-activity", "Network activity"),
    ("devices", "Devices"),
    ("lan-pairing", "LAN pairing"),
    ("capability-status", "Capability status"),
    ("notifications", "Notifications"),
    ("notification-channels", "Notification channels"),
    ("drive-connections", "Drive connections"),
    ("export-retention", "Export retention"),
    ("remote-access", "Remote access"),
    ("report-compiler", "Report compiler"),
    ("audit-history", "Audit history"),
    ("subscription", "Subscription"),
    ("entitlements", "Entitlements"),
    ("platforms-install", "Platforms install"),
    ("install-updates", "Install updates"),
    ("diagnostics", "Diagnostics"),
    ("proof-panels", "Proof panels"),
    ("settings-rules", "Settings rules"),
    ("app-layout", "App layout"),
    ("frame-tuner", "Frame tuner"),
    ("commands", "Commands"),
    ("events", "Events"),
    ("logs", "Logs"),
];

pub(crate) fn tracking_read_model_snapshot(
    read_model: &TrackingReadModel,
) -> ParentActivityTrackingReadModelSnapshot {
    ParentActivityTrackingReadModelSnapshot {
        schema_version: read_model.schema_version,
        generated_at: read_model.generated_at.to_string(),
        custody_label: read_model.custody_label.to_string(),
        limit: read_model.limit,
        returned: read_model.returned,
        active_rows: read_model.active_rows,
        tombstone_rows: read_model.tombstone_rows,
        capability_status: read_model.capability_status.to_string(),
        latest_event_id: parse_optional_identifier(
            read_model.latest_event_id.as_ref().map(ToString::to_string),
            ParentRouteEventId::parse,
        ),
        latest_observed_at: read_model
            .latest_observed_at
            .as_ref()
            .map(ToString::to_string),
        latest_active_event_id: parse_optional_identifier(
            read_model
                .latest_active_event_id
                .as_ref()
                .map(ToString::to_string),
            ParentRouteEventId::parse,
        ),
        latest_active_observed_at: read_model
            .latest_active_observed_at
            .as_ref()
            .map(ToString::to_string),
        latest_tombstone_event_id: parse_optional_identifier(
            read_model
                .latest_tombstone_event_id
                .as_ref()
                .map(ToString::to_string),
            ParentRouteEventId::parse,
        ),
        latest_tombstone_observed_at: read_model
            .latest_tombstone_observed_at
            .as_ref()
            .map(ToString::to_string),
        active_kind_counts: read_model
            .active_kind_counts
            .iter()
            .map(parent_activity_tracking_read_model_count_snapshot)
            .collect(),
        active_device_counts: read_model
            .active_device_counts
            .iter()
            .map(parent_activity_tracking_read_model_count_snapshot)
            .collect(),
        active_capability_status_counts: read_model
            .active_capability_status_counts
            .iter()
            .map(parent_activity_tracking_read_model_count_snapshot)
            .collect(),
        deleted_evidence_reference_ids: read_model
            .deleted_evidence_reference_ids
            .iter()
            .map(|value| {
                parse_identifier(
                    value.to_string(),
                    "tracking deleted_evidence_reference_id",
                    ParentEvidenceReferenceId::parse,
                )
            })
            .collect(),
        rows: read_model
            .rows
            .iter()
            .map(parent_activity_tracking_read_model_row_snapshot)
            .collect(),
    }
}

fn parent_activity_tracking_read_model_count_snapshot(
    count: &TrackingReadModelCount,
) -> ParentActivityTrackingReadModelCountSnapshot {
    ParentActivityTrackingReadModelCountSnapshot {
        value: count.value.to_string(),
        count: count.count,
    }
}

fn parent_activity_tracking_read_model_row_snapshot(
    row: &TrackingReadModelRow,
) -> ParentActivityTrackingReadModelRowSnapshot {
    ParentActivityTrackingReadModelRowSnapshot {
        schema_version: row.schema_version,
        event_id: parse_identifier(row.event_id.to_string(), "tracking event_id", |value| {
            ParentRouteEventId::parse(value)
        }),
        observed_at: row.observed_at.to_string(),
        device_id: parse_identifier(row.device_id.to_string(), "tracking device_id", |value| {
            ParentLanDeviceId::parse(value)
        }),
        platform: row.platform.to_string(),
        observer: row.observer.to_string(),
        kind: row.kind.to_string(),
        subject_kind: row.subject_kind.to_string(),
        subject_id: parse_identifier(row.subject_id.to_string(), "tracking subject_id", |value| {
            ParentSubjectId::parse(value)
        }),
        subject_display_name: row.subject_display_name.as_ref().map(ToString::to_string),
        capability_status: row.capability_status.as_ref().map(ToString::to_string),
        query_visibility: row.query_visibility.to_string(),
        deleted_at: row.deleted_at.as_ref().map(ToString::to_string),
        evidence_reference_ids: row
            .evidence_reference_ids
            .iter()
            .map(|value| {
                parse_identifier(
                    value.to_string(),
                    "tracking evidence_reference_id",
                    ParentEvidenceReferenceId::parse,
                )
            })
            .collect(),
        deleted_evidence_reference_ids: row
            .deleted_evidence_reference_ids
            .iter()
            .map(|value| {
                parse_identifier(
                    value.to_string(),
                    "tracking deleted_evidence_reference_id",
                    ParentEvidenceReferenceId::parse,
                )
            })
            .collect(),
        evidence: row
            .evidence
            .iter()
            .map(|evidence| ParentActivityEvidenceRefSnapshot {
                evidence_id: parse_identifier(
                    evidence.evidence_id.clone(),
                    "tracking evidence_id",
                    ParentEvidenceId::parse,
                ),
                kind: serialized_enum_label(&evidence.kind),
                digest: evidence.digest.clone(),
                uri: evidence.uri.clone(),
            })
            .collect(),
    }
}

pub(super) fn route_title(route: &ParentRouteId) -> &'static str {
    let route = serialized_enum_label(route);
    ROUTE_TITLES
        .iter()
        .find(|(raw, _)| *raw == route)
        .map(|(_, label)| *label)
        .unwrap_or("Overview")
}
