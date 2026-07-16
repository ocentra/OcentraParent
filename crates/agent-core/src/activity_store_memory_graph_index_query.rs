use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::activity::policy_context::ParentDeviceReference;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphEdge;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphNode;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphNodeKind;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphQuery;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphQueryKind;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphReadModel;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphTimeRange;
use ocentra_parent_agent_protocol::activity_memory_graph::ACTIVITY_MEMORY_GRAPH_CAPABILITY_NO_EVIDENCE;
use ocentra_parent_agent_protocol::activity_memory_graph::ACTIVITY_MEMORY_GRAPH_CAPABILITY_READY;
use ocentra_parent_agent_protocol::activity_memory_graph::ACTIVITY_MEMORY_GRAPH_CUSTODY_ACTIVITY_STORE;
use ocentra_parent_agent_protocol::activity_memory_graph::ACTIVITY_MEMORY_GRAPH_REASON_EDGE_LIMIT;
use ocentra_parent_agent_protocol::activity_memory_graph::ACTIVITY_MEMORY_GRAPH_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::constants;
use rusqlite::{params, Connection, OptionalExtension};

use crate::ActivityStoreError;

pub(crate) fn indexed_activity_memory_graph_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> Result<ActivityMemoryGraphReadModel, ActivityStoreError> {
    let total_edge_count = indexed_edge_count(connection)?;
    let edges = indexed_edges(connection, limit)?;
    let nodes = indexed_nodes_for_edges(connection, &edges)?;
    let returned_edge_count = edges.len() as u64;
    let omitted_edge_count = total_edge_count.saturating_sub(returned_edge_count);
    Ok(ActivityMemoryGraphReadModel {
        schema_version: ACTIVITY_MEMORY_GRAPH_SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        custody: ACTIVITY_MEMORY_GRAPH_CUSTODY_ACTIVITY_STORE.to_string(),
        capability_status: capability_status(returned_edge_count),
        query: query_from_index(generated_at, limit, &nodes, &edges),
        read_at: generated_at.to_string(),
        returned_node_count: nodes.len() as u64,
        returned_edge_count,
        omitted_edge_count,
        degraded_reasons: degraded_reasons(omitted_edge_count),
        nodes,
        edges,
    })
}

pub(crate) fn indexed_citation_count(connection: &Connection) -> Result<u64, ActivityStoreError> {
    let count: i64 = connection.query_row(
        constants::sqlite::COUNT_INDEXED_ACTIVITY_MEMORY_GRAPH_CITATIONS,
        [],
        |row| row.get(0),
    )?;
    Ok(count as u64)
}

pub(crate) fn delete_activity_events_for_memory_graph_test(
    connection: &Connection,
) -> Result<(), ActivityStoreError> {
    connection.execute(
        constants::sqlite::DELETE_ACTIVITY_EVENTS_FOR_MEMORY_GRAPH_TEST,
        [],
    )?;
    Ok(())
}

fn indexed_edge_count(connection: &Connection) -> Result<u64, ActivityStoreError> {
    let count: i64 = connection.query_row(
        constants::sqlite::COUNT_INDEXED_ACTIVITY_MEMORY_GRAPH_EDGES,
        [],
        |row| row.get(0),
    )?;
    Ok(count as u64)
}

fn indexed_edges(
    connection: &Connection,
    limit: u64,
) -> Result<Vec<ActivityMemoryGraphEdge>, ActivityStoreError> {
    let mut statement =
        connection.prepare(constants::sqlite::SELECT_INDEXED_ACTIVITY_MEMORY_GRAPH_EDGES)?;
    let rows = statement.query_map(params![limit as i64], |row| {
        let edge_json: String = row.get(0)?;
        serde_json::from_str::<ActivityMemoryGraphEdge>(&edge_json).map_err(json_to_sqlite_error)
    })?;
    let mut edges = Vec::new();
    for row in rows {
        edges.push(row?);
    }
    Ok(edges)
}

fn indexed_nodes_for_edges(
    connection: &Connection,
    edges: &[ActivityMemoryGraphEdge],
) -> Result<Vec<ActivityMemoryGraphNode>, ActivityStoreError> {
    let mut nodes = BTreeMap::new();
    for edge in edges {
        if let Some(node) = indexed_node(connection, &edge.from_node_id)? {
            nodes.insert(node.node_id.clone(), node);
        }
        if let Some(node) = indexed_node(connection, &edge.to_node_id)? {
            nodes.insert(node.node_id.clone(), node);
        }
    }
    Ok(nodes.into_values().collect())
}

fn indexed_node(
    connection: &Connection,
    node_id: &str,
) -> Result<Option<ActivityMemoryGraphNode>, ActivityStoreError> {
    connection
        .query_row(
            constants::sqlite::SELECT_INDEXED_ACTIVITY_MEMORY_GRAPH_NODE,
            params![node_id],
            |row| {
                let node_json: String = row.get(0)?;
                serde_json::from_str::<ActivityMemoryGraphNode>(&node_json)
                    .map_err(json_to_sqlite_error)
            },
        )
        .optional()
        .map_err(ActivityStoreError::from)
}

fn query_from_index(
    generated_at: &str,
    limit: u64,
    nodes: &[ActivityMemoryGraphNode],
    edges: &[ActivityMemoryGraphEdge],
) -> ActivityMemoryGraphQuery {
    ActivityMemoryGraphQuery {
        query_id: generated_at.to_string(),
        query_kind: ActivityMemoryGraphQueryKind::ActivityByTimeRange,
        child_profile: None,
        device: device_from_nodes(nodes),
        time_range: time_range_from_edges(generated_at, edges),
        as_of: generated_at.to_string(),
        limit,
    }
}

fn device_from_nodes(nodes: &[ActivityMemoryGraphNode]) -> ParentDeviceReference {
    nodes
        .iter()
        .find(|node| node.node_kind == ActivityMemoryGraphNodeKind::Device)
        .and_then(|node| node.device.clone())
        .unwrap_or_else(|| ParentDeviceReference {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            child_profile_id: None,
            label: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
        })
}

fn time_range_from_edges(
    generated_at: &str,
    edges: &[ActivityMemoryGraphEdge],
) -> ActivityMemoryGraphTimeRange {
    ActivityMemoryGraphTimeRange {
        observed_from: edges
            .last()
            .map(|edge| edge.observed_from.clone())
            .unwrap_or_else(|| generated_at.to_string()),
        observed_until: edges
            .first()
            .map(|edge| {
                edge.observed_until
                    .clone()
                    .unwrap_or_else(|| edge.observed_from.clone())
            })
            .unwrap_or_else(|| generated_at.to_string()),
    }
}

fn capability_status(returned_edge_count: u64) -> String {
    if returned_edge_count == 0 {
        ACTIVITY_MEMORY_GRAPH_CAPABILITY_NO_EVIDENCE.to_string()
    } else {
        ACTIVITY_MEMORY_GRAPH_CAPABILITY_READY.to_string()
    }
}

fn degraded_reasons(omitted_edges: u64) -> Vec<String> {
    if omitted_edges == 0 {
        Vec::new()
    } else {
        vec![ACTIVITY_MEMORY_GRAPH_REASON_EDGE_LIMIT.to_string()]
    }
}

fn json_to_sqlite_error(error: serde_json::Error) -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure(Box::new(error))
}
