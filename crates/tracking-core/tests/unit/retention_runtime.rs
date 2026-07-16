use ocentra_tracking_core::retention_runtime::{
    apply_tracking_retention_delete, apply_tracking_retention_export, TrackingCapabilityStatus,
    TrackingCustodyLabel, TrackingEvidenceId, TrackingRemoteSyncDefault,
    TrackingRetentionDeleteInput, TrackingRetentionDeviceStatusRow,
    TrackingRetentionExpectedPlaceDecisionRow, TrackingRetentionExportInput,
    TrackingRetentionGeofenceTransitionRow, TrackingRetentionLocationRow, TrackingRetentionMode,
    TrackingRetentionNearbyPlaceRow, TrackingRetentionPolicy, TrackingRetentionReadModel,
    TrackingRetentionTimelineRow, TrackingTimestamp,
};

#[test]
fn retention_delete_removes_related_rows_and_marks_stale_when_history_is_empty() {
    let deleted_evidence_id = TrackingEvidenceId::new("location-evidence-1");
    let proof = apply_tracking_retention_delete(TrackingRetentionDeleteInput {
        read_model: sample_read_model(),
        generated_at: TrackingTimestamp::new("2026-06-03T03:00:00.000Z"),
        deleted_evidence_ids: vec![deleted_evidence_id.clone()],
    });

    assert_eq!(proof.before_location_rows, 1);
    assert_eq!(proof.after_location_rows, 0);
    assert!(proof.read_model.location_rows.is_empty());
    assert!(proof.read_model.geofence_transitions.is_empty());
    assert!(proof.read_model.expected_place_decisions.is_empty());
    assert!(proof.read_model.nearby_place_rows.is_empty());
    assert!(proof.read_model.timeline.is_empty());
    assert_eq!(
        proof.read_model.capability_status,
        TrackingCapabilityStatus::stale()
    );
    assert_eq!(proof.deleted_evidence_ids, vec![deleted_evidence_id]);
}

#[test]
fn retention_export_rewrites_custody_and_mode_without_enabling_remote_sync() {
    let proof = apply_tracking_retention_export(TrackingRetentionExportInput {
        read_model: sample_read_model(),
        generated_at: TrackingTimestamp::new("2026-06-03T03:05:00.000Z"),
        policy: TrackingRetentionPolicy {
            export_allowed: true,
            custody_label: TrackingCustodyLabel::new("parent-owned-export"),
            mode: TrackingRetentionMode::new("export-only"),
            remote_sync_default: TrackingRemoteSyncDefault::new("disabled"),
        },
    });

    assert!(proof.export_allowed);
    assert_eq!(proof.source_location_rows, 1);
    assert_eq!(proof.exported_location_rows, 1);
    assert_eq!(proof.custody_label.as_str(), "parent-owned-export");
    assert_eq!(proof.retention_mode.as_str(), "export-only");
    assert_eq!(proof.remote_sync_default.as_str(), "disabled");
    assert_eq!(
        proof.read_model.location_rows[0].custody_label.as_str(),
        "parent-owned-export"
    );
    assert_eq!(
        proof.read_model.device_status_rows[0]
            .retention_mode
            .as_str(),
        "export-only"
    );
}

fn sample_read_model() -> TrackingRetentionReadModel {
    TrackingRetentionReadModel {
        generated_at: TrackingTimestamp::new("2026-06-03T02:02:00.000Z"),
        capability_status: TrackingCapabilityStatus::new("recent"),
        location_rows: vec![TrackingRetentionLocationRow {
            evidence_id: TrackingEvidenceId::new("location-evidence-1"),
            custody_label: TrackingCustodyLabel::new("child-device-local"),
            retention_mode: TrackingRetentionMode::new("24h"),
        }],
        device_status_rows: vec![TrackingRetentionDeviceStatusRow {
            last_location_evidence_id: Some(TrackingEvidenceId::new("location-evidence-1")),
            custody_label: TrackingCustodyLabel::new("child-device-local"),
            retention_mode: TrackingRetentionMode::new("24h"),
        }],
        geofence_transitions: vec![TrackingRetentionGeofenceTransitionRow {
            location_evidence_id: TrackingEvidenceId::new("location-evidence-1"),
        }],
        expected_place_decisions: vec![TrackingRetentionExpectedPlaceDecisionRow {
            location_evidence_id: TrackingEvidenceId::new("location-evidence-1"),
        }],
        nearby_place_rows: vec![TrackingRetentionNearbyPlaceRow {
            location_evidence_id: TrackingEvidenceId::new("location-evidence-1"),
        }],
        timeline: vec![TrackingRetentionTimelineRow {
            row_id: TrackingEvidenceId::new("location-evidence-1"),
        }],
    }
}
