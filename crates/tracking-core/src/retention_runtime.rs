#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TrackingEvidenceId(String);

impl TrackingEvidenceId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingTimestamp(String);

impl TrackingTimestamp {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingCapabilityStatus(String);

impl TrackingCapabilityStatus {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn stale() -> Self {
        Self::new("stale")
    }

    pub fn unavailable() -> Self {
        Self::new("unavailable")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingCustodyLabel(String);

impl TrackingCustodyLabel {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionMode(String);

impl TrackingRetentionMode {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRemoteSyncDefault(String);

impl TrackingRemoteSyncDefault {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionLocationRow {
    pub evidence_id: TrackingEvidenceId,
    pub custody_label: TrackingCustodyLabel,
    pub retention_mode: TrackingRetentionMode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionDeviceStatusRow {
    pub last_location_evidence_id: Option<TrackingEvidenceId>,
    pub custody_label: TrackingCustodyLabel,
    pub retention_mode: TrackingRetentionMode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionGeofenceTransitionRow {
    pub location_evidence_id: TrackingEvidenceId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionExpectedPlaceDecisionRow {
    pub location_evidence_id: TrackingEvidenceId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionNearbyPlaceRow {
    pub location_evidence_id: TrackingEvidenceId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionTimelineRow {
    pub row_id: TrackingEvidenceId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionReadModel {
    pub generated_at: TrackingTimestamp,
    pub capability_status: TrackingCapabilityStatus,
    pub location_rows: Vec<TrackingRetentionLocationRow>,
    pub device_status_rows: Vec<TrackingRetentionDeviceStatusRow>,
    pub geofence_transitions: Vec<TrackingRetentionGeofenceTransitionRow>,
    pub expected_place_decisions: Vec<TrackingRetentionExpectedPlaceDecisionRow>,
    pub nearby_place_rows: Vec<TrackingRetentionNearbyPlaceRow>,
    pub timeline: Vec<TrackingRetentionTimelineRow>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionDeleteInput {
    pub read_model: TrackingRetentionReadModel,
    pub generated_at: TrackingTimestamp,
    pub deleted_evidence_ids: Vec<TrackingEvidenceId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionDeleteProof {
    pub before_location_rows: usize,
    pub after_location_rows: usize,
    pub deleted_evidence_ids: Vec<TrackingEvidenceId>,
    pub read_model: TrackingRetentionReadModel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionPolicy {
    pub export_allowed: bool,
    pub custody_label: TrackingCustodyLabel,
    pub mode: TrackingRetentionMode,
    pub remote_sync_default: TrackingRemoteSyncDefault,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionExportInput {
    pub read_model: TrackingRetentionReadModel,
    pub generated_at: TrackingTimestamp,
    pub policy: TrackingRetentionPolicy,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingRetentionExportProof {
    pub export_allowed: bool,
    pub source_location_rows: usize,
    pub exported_location_rows: usize,
    pub custody_label: TrackingCustodyLabel,
    pub retention_mode: TrackingRetentionMode,
    pub remote_sync_default: TrackingRemoteSyncDefault,
    pub read_model: TrackingRetentionReadModel,
}

pub fn apply_tracking_retention_delete(
    input: TrackingRetentionDeleteInput,
) -> TrackingRetentionDeleteProof {
    let before_location_rows = input.read_model.location_rows.len();
    let deleted = &input.deleted_evidence_ids;
    let location_rows = input
        .read_model
        .location_rows
        .into_iter()
        .filter(|row| !deleted.contains(&row.evidence_id))
        .collect::<Vec<_>>();
    let device_status_rows = input
        .read_model
        .device_status_rows
        .into_iter()
        .filter(|row| {
            row.last_location_evidence_id
                .as_ref()
                .is_none_or(|evidence_id| !deleted.contains(evidence_id))
        })
        .collect::<Vec<_>>();
    let geofence_transitions = input
        .read_model
        .geofence_transitions
        .into_iter()
        .filter(|row| !deleted.contains(&row.location_evidence_id))
        .collect::<Vec<_>>();
    let expected_place_decisions = input
        .read_model
        .expected_place_decisions
        .into_iter()
        .filter(|row| !deleted.contains(&row.location_evidence_id))
        .collect::<Vec<_>>();
    let nearby_place_rows = input
        .read_model
        .nearby_place_rows
        .into_iter()
        .filter(|row| !deleted.contains(&row.location_evidence_id))
        .collect::<Vec<_>>();
    let timeline = input
        .read_model
        .timeline
        .into_iter()
        .filter(|row| !deleted.contains(&row.row_id))
        .collect::<Vec<_>>();

    let capability_status = if location_rows.is_empty() {
        TrackingCapabilityStatus::stale()
    } else {
        input.read_model.capability_status
    };

    let read_model = TrackingRetentionReadModel {
        generated_at: input.generated_at,
        capability_status,
        location_rows,
        device_status_rows,
        geofence_transitions,
        expected_place_decisions,
        nearby_place_rows,
        timeline,
    };

    TrackingRetentionDeleteProof {
        before_location_rows,
        after_location_rows: read_model.location_rows.len(),
        deleted_evidence_ids: input.deleted_evidence_ids,
        read_model,
    }
}

pub fn apply_tracking_retention_export(
    input: TrackingRetentionExportInput,
) -> TrackingRetentionExportProof {
    let source_location_rows = input.read_model.location_rows.len();
    let location_rows = if input.policy.export_allowed {
        input
            .read_model
            .location_rows
            .into_iter()
            .map(|row| TrackingRetentionLocationRow {
                evidence_id: row.evidence_id,
                custody_label: input.policy.custody_label.clone(),
                retention_mode: input.policy.mode.clone(),
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let device_status_rows = if input.policy.export_allowed {
        input
            .read_model
            .device_status_rows
            .into_iter()
            .map(|row| TrackingRetentionDeviceStatusRow {
                last_location_evidence_id: row.last_location_evidence_id,
                custody_label: input.policy.custody_label.clone(),
                retention_mode: input.policy.mode.clone(),
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let timeline = if input.policy.export_allowed {
        input.read_model.timeline
    } else {
        Vec::new()
    };

    let read_model = TrackingRetentionReadModel {
        generated_at: input.generated_at,
        capability_status: if input.policy.export_allowed {
            input.read_model.capability_status
        } else {
            TrackingCapabilityStatus::unavailable()
        },
        location_rows,
        device_status_rows,
        geofence_transitions: input.read_model.geofence_transitions,
        expected_place_decisions: input.read_model.expected_place_decisions,
        nearby_place_rows: input.read_model.nearby_place_rows,
        timeline,
    };

    TrackingRetentionExportProof {
        export_allowed: input.policy.export_allowed,
        source_location_rows,
        exported_location_rows: read_model.location_rows.len(),
        custody_label: input.policy.custody_label,
        retention_mode: input.policy.mode,
        remote_sync_default: input.policy.remote_sync_default,
        read_model,
    }
}
