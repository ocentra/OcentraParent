use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphNodeKind;

use crate::activity_store_memory_graph_rows::MemoryGraphStoreRow;

#[path = "../activity_store_memory_graph_helpers_evidence.rs"]
mod activity_store_memory_graph_helpers_evidence;
#[path = "../activity_store_memory_graph_helpers_labels.rs"]
mod activity_store_memory_graph_helpers_labels;

pub(crate) fn node_kind(row: &MemoryGraphStoreRow) -> Option<ActivityMemoryGraphNodeKind> {
    activity_store_memory_graph_helpers_labels::node_kind(row)
}

pub(crate) fn looks_like_game(row: &MemoryGraphStoreRow) -> bool {
    activity_store_memory_graph_helpers_labels::looks_like_game(row)
}

pub(crate) fn node_label(row: &MemoryGraphStoreRow) -> Option<String> {
    activity_store_memory_graph_helpers_labels::node_label(row)
}

pub(crate) fn evidence_references(row: &MemoryGraphStoreRow) -> Vec<ParentEvidenceReference> {
    activity_store_memory_graph_helpers_evidence::evidence_references(row)
}

pub(crate) fn confidence_for_row(row: &MemoryGraphStoreRow) -> f64 {
    activity_store_memory_graph_helpers_evidence::confidence_for_row(row)
}
