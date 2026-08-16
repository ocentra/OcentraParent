use ocentra_parent_agent_protocol::activity::policy_context::ParentDeviceReference;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphEdgeKind;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphEntryStatus;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphNode;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphNodeKind;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphTrace;
use ocentra_parent_agent_protocol::activity_memory_graph::ACTIVITY_MEMORY_GRAPH_INDEX_VERSION;
use ocentra_parent_agent_protocol::constants;

use crate::activity_store_memory_graph_rows::MemoryGraphStoreRow;

#[path = "activity_store_memory_graph_nodes/helpers.rs"]
mod helpers;

use self::helpers::{
    confidence_for_row, evidence_references, looks_like_game, node_kind, node_label,
};

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
