use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphEdge;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphEdgeKind;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphNode;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphNodeKind;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphReadModel;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphTrace;
use ocentra_parent_agent_protocol::activity_memory_graph::ACTIVITY_MEMORY_GRAPH_INDEX_VERSION;
use ocentra_parent_agent_protocol::constants;
use rusqlite::{params, Connection};

use crate::ActivityStoreError;

pub fn persist_read_model(
    connection: &Connection,
    read_model: &ActivityMemoryGraphReadModel,
    source_event_count: u64,
) -> Result<(), ActivityStoreError> {
    for node in &read_model.nodes {
        persist_node(connection, node, &read_model.generated_at)?;
    }
    for edge in &read_model.edges {
        persist_edge(connection, edge, &read_model.generated_at)?;
    }
    connection.execute(
        constants::sqlite::UPSERT_ACTIVITY_MEMORY_GRAPH_DERIVATION_RUN,
        params![
            constants::activity_store::MEMORY_GRAPH_DERIVATION_RUN_ID,
            read_model.generated_at,
            ACTIVITY_MEMORY_GRAPH_INDEX_VERSION,
            source_event_count as i64,
            read_model.nodes.len() as i64,
            read_model.edges.len() as i64
        ],
    )?;
    Ok(())
}

fn persist_node(
    connection: &Connection,
    node: &ActivityMemoryGraphNode,
    updated_at: &str,
) -> Result<(), ActivityStoreError> {
    let node_json = serde_json::to_string(node)?;
    let trace_json = serde_json::to_string(&node.trace)?;
    connection.execute(
        constants::sqlite::UPSERT_ACTIVITY_MEMORY_GRAPH_NODE,
        params![
            node.node_id,
            node.graph_id,
            node_kind_label(&node.node_kind)?,
            node.label,
            node_json,
            trace_json,
            updated_at
        ],
    )?;
    persist_citations(
        connection,
        &node.node_id,
        constants::activity_store::MEMORY_GRAPH_NODE_ENTRY_KIND,
        &node.trace,
    )
}

fn persist_edge(
    connection: &Connection,
    edge: &ActivityMemoryGraphEdge,
    updated_at: &str,
) -> Result<(), ActivityStoreError> {
    let edge_json = serde_json::to_string(edge)?;
    let trace_json = serde_json::to_string(&edge.trace)?;
    connection.execute(
        constants::sqlite::UPSERT_ACTIVITY_MEMORY_GRAPH_EDGE,
        params![
            edge.edge_id,
            edge.graph_id,
            edge_kind_label(&edge.edge_kind)?,
            edge.from_node_id,
            edge.to_node_id,
            edge.observed_from,
            edge.observed_until,
            edge.duration_ms.map(|duration| duration as i64),
            edge_json,
            trace_json,
            updated_at
        ],
    )?;
    persist_citations(
        connection,
        &edge.edge_id,
        constants::activity_store::MEMORY_GRAPH_EDGE_ENTRY_KIND,
        &edge.trace,
    )
}

fn persist_citations(
    connection: &Connection,
    entry_id: &str,
    entry_kind: &str,
    trace: &ActivityMemoryGraphTrace,
) -> Result<(), ActivityStoreError> {
    connection.execute(
        constants::sqlite::DELETE_ACTIVITY_MEMORY_GRAPH_CITATIONS_FOR_ENTRY,
        params![entry_id, entry_kind],
    )?;
    for reference in &trace.source_evidence_references {
        connection.execute(
            constants::sqlite::INSERT_ACTIVITY_MEMORY_GRAPH_CITATION,
            params![
                entry_id,
                entry_kind,
                reference.evidence_reference_id,
                evidence_kind_label(&reference.kind)?,
                reference.observed_at
            ],
        )?;
    }
    Ok(())
}

fn node_kind_label(kind: &ActivityMemoryGraphNodeKind) -> Result<String, ActivityStoreError> {
    json_label(&serde_json::to_string(kind)?)
}

fn edge_kind_label(kind: &ActivityMemoryGraphEdgeKind) -> Result<String, ActivityStoreError> {
    json_label(&serde_json::to_string(kind)?)
}

fn evidence_kind_label(kind: &ParentEvidenceReferenceKind) -> Result<String, ActivityStoreError> {
    json_label(&serde_json::to_string(kind)?)
}

fn json_label(json: &str) -> Result<String, ActivityStoreError> {
    match serde_json::from_str::<serde_json::Value>(json)? {
        serde_json::Value::String(label) => Ok(label),
        value => Ok(value.to_string()),
    }
}
