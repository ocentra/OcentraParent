use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceKind, ActivityMemoryGraphEdgeKind, ActivityMemoryGraphEntryStatus,
    ActivityMemoryGraphNode, ActivityMemoryGraphNodeKind, ActivityMemoryGraphTrace, LogFieldValue,
    LogFields, ParentDeviceReference, ParentEvidenceReference, ParentEvidenceReferenceKind,
    ACTIVITY_MEMORY_GRAPH_GAME_LABEL_HINT, ACTIVITY_MEMORY_GRAPH_INDEX_VERSION,
};

use crate::activity_store_memory_graph_rows::MemoryGraphStoreRow;

pub(crate) fn graph_id() -> String {
    ACTIVITY_MEMORY_GRAPH_INDEX_VERSION.to_string()
}

pub(crate) fn edge_id(row: &MemoryGraphStoreRow) -> String {
    row.event_id.clone()
}

pub(crate) fn device_from_row(row: &MemoryGraphStoreRow) -> ParentDeviceReference {
    ParentDeviceReference {
        device_id: row.device_id.clone(),
        child_profile_id: None,
        label: row.device_id.clone(),
        platform: row.platform.clone(),
    }
}

pub(crate) fn device_node(
    row: &MemoryGraphStoreRow,
    device: &ParentDeviceReference,
    generated_at: &str,
) -> ActivityMemoryGraphNode {
    ActivityMemoryGraphNode {
        graph_id: graph_id(),
        node_id: device.device_id.clone(),
        node_kind: ActivityMemoryGraphNodeKind::Device,
        label: device.label.clone(),
        child_profile: None,
        device: Some(device.clone()),
        trace: trace_from_row(row, generated_at, 1.0),
    }
}

pub(crate) fn activity_node(
    row: &MemoryGraphStoreRow,
    device: &ParentDeviceReference,
    generated_at: &str,
) -> Option<ActivityMemoryGraphNode> {
    Some(ActivityMemoryGraphNode {
        graph_id: graph_id(),
        node_id: row.subject_id.clone(),
        node_kind: node_kind(row)?,
        label: node_label(row)?,
        child_profile: None,
        device: Some(device.clone()),
        trace: trace_from_row(row, generated_at, confidence_for_row(row)),
    })
}

pub(crate) fn edge_kind(row: &MemoryGraphStoreRow) -> Option<ActivityMemoryGraphEdgeKind> {
    match row.kind.as_str() {
        constants::activity_event_kind::URL_OBSERVED => Some(ActivityMemoryGraphEdgeKind::Visited),
        constants::activity_event_kind::VIDEO_OBSERVED => {
            Some(ActivityMemoryGraphEdgeKind::Watched)
        }
        constants::activity_event_kind::WINDOW_FOCUSED if looks_like_game(row) => {
            Some(ActivityMemoryGraphEdgeKind::Played)
        }
        constants::activity_event_kind::WINDOW_FOCUSED => {
            Some(ActivityMemoryGraphEdgeKind::ActiveDuring)
        }
        _ => None,
    }
}

pub(crate) fn trace_from_row(
    row: &MemoryGraphStoreRow,
    generated_at: &str,
    confidence: f64,
) -> ActivityMemoryGraphTrace {
    ActivityMemoryGraphTrace {
        entry_status: ActivityMemoryGraphEntryStatus::Usable,
        source_evidence_references: evidence_references(row),
        source_policy_version: None,
        source_parent_action_references: Vec::new(),
        generated_at: generated_at.to_string(),
        expires_at: None,
        confidence,
        derived_index_version: ACTIVITY_MEMORY_GRAPH_INDEX_VERSION.to_string(),
        degraded_reasons: Vec::new(),
    }
}

pub(crate) fn confidence_for_edge(edge_kind: ActivityMemoryGraphEdgeKind) -> f64 {
    match edge_kind {
        ActivityMemoryGraphEdgeKind::Played => 0.4,
        _ => 1.0,
    }
}

fn node_kind(row: &MemoryGraphStoreRow) -> Option<ActivityMemoryGraphNodeKind> {
    match row.kind.as_str() {
        constants::activity_event_kind::URL_OBSERVED => {
            Some(ActivityMemoryGraphNodeKind::BrowserUrl)
        }
        constants::activity_event_kind::VIDEO_OBSERVED => Some(ActivityMemoryGraphNodeKind::Video),
        constants::activity_event_kind::WINDOW_FOCUSED if looks_like_game(row) => {
            Some(ActivityMemoryGraphNodeKind::Game)
        }
        constants::activity_event_kind::WINDOW_FOCUSED => Some(ActivityMemoryGraphNodeKind::App),
        _ => None,
    }
}

fn looks_like_game(row: &MemoryGraphStoreRow) -> bool {
    node_label(row)
        .map(|label| {
            label
                .to_ascii_lowercase()
                .contains(ACTIVITY_MEMORY_GRAPH_GAME_LABEL_HINT)
        })
        .unwrap_or(false)
}

fn node_label(row: &MemoryGraphStoreRow) -> Option<String> {
    if row.kind == constants::activity_event_kind::URL_OBSERVED {
        return string_field(&row.fields, constants::field::URL)
            .or_else(|| row.subject_display_name.clone());
    }
    if row.kind == constants::activity_event_kind::VIDEO_OBSERVED {
        return row
            .subject_display_name
            .clone()
            .or_else(|| string_field(&row.fields, constants::field::TITLE))
            .or_else(|| string_field(&row.fields, constants::field::URL));
    }
    row.subject_display_name
        .clone()
        .or_else(|| string_field(&row.fields, constants::field::WINDOW_TITLE))
        .or_else(|| string_field(&row.fields, constants::field::APP_NAME))
        .or_else(|| string_field(&row.fields, constants::field::PROCESS_NAME))
}

fn evidence_references(row: &MemoryGraphStoreRow) -> Vec<ParentEvidenceReference> {
    if row.evidence.is_empty() {
        return vec![ParentEvidenceReference {
            evidence_reference_id: row.event_id.clone(),
            kind: ParentEvidenceReferenceKind::ActivityEvent,
            observed_at: row.observed_at.clone(),
        }];
    }
    row.evidence
        .iter()
        .map(|evidence| ParentEvidenceReference {
            evidence_reference_id: evidence.evidence_id.clone(),
            kind: parent_evidence_kind(&evidence.kind),
            observed_at: row.observed_at.clone(),
        })
        .collect()
}

fn parent_evidence_kind(kind: &ActivityEvidenceKind) -> ParentEvidenceReferenceKind {
    match kind {
        ActivityEvidenceKind::JournalEntry => ParentEvidenceReferenceKind::JournalEvent,
        ActivityEvidenceKind::Screenshot => ParentEvidenceReferenceKind::ActivityEvent,
        ActivityEvidenceKind::StorageObject => ParentEvidenceReferenceKind::ActivityEvent,
        ActivityEvidenceKind::LocalDbRow => ParentEvidenceReferenceKind::ActivityEvent,
    }
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}

fn confidence_for_row(row: &MemoryGraphStoreRow) -> f64 {
    match row.kind.as_str() {
        constants::activity_event_kind::WINDOW_FOCUSED => 0.4,
        _ => 1.0,
    }
}
