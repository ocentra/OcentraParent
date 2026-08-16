use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::activity::policy_context::ParentDeviceReference;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphEdge;
use ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphNode;
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

use crate::{
    activity_store_memory_graph_nodes::{
        activity_node, confidence_for_edge, device_from_row, device_node, edge_id, edge_kind,
        graph_id, trace_from_row,
    },
    activity_store_memory_graph_rows::MemoryGraphStoreRow,
};

pub(crate) struct MemoryGraphBuilder {
    limit: u64,
    generated_at: String,
    nodes: BTreeMap<String, ActivityMemoryGraphNode>,
    edges: Vec<ActivityMemoryGraphEdge>,
    omitted_edges: u64,
    first_observed_at: Option<String>,
    last_observed_at: Option<String>,
    query_device: ParentDeviceReference,
}

impl MemoryGraphBuilder {
    pub(crate) fn new(limit: u64, generated_at: &str) -> Self {
        Self {
            limit,
            generated_at: generated_at.to_string(),
            nodes: BTreeMap::new(),
            edges: Vec::new(),
            omitted_edges: 0,
            first_observed_at: None,
            last_observed_at: None,
            query_device: ParentDeviceReference {
                device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                child_profile_id: None,
                label: constants::peer::LOCAL_DEV_AGENT.to_string(),
                platform: std::env::consts::OS.to_string(),
            },
        }
    }

    pub(crate) fn ingest(&mut self, row: &MemoryGraphStoreRow) {
        self.capture_time_range(&row.observed_at);
        self.query_device = device_from_row(row);
        if let Some(edge) = self.edge_from_row(row) {
            if self.edges.len() < self.limit as usize {
                self.edges.push(edge);
            } else {
                self.omitted_edges += 1;
            }
        }
    }

    pub(crate) fn into_read_model(self) -> ActivityMemoryGraphReadModel {
        let returned_edge_count = self.edges.len() as u64;
        ActivityMemoryGraphReadModel {
            schema_version: ACTIVITY_MEMORY_GRAPH_SCHEMA_VERSION,
            generated_at: self.generated_at.clone(),
            custody: ACTIVITY_MEMORY_GRAPH_CUSTODY_ACTIVITY_STORE.to_string(),
            capability_status: capability_status(returned_edge_count),
            query: self.query(),
            read_at: self.generated_at,
            returned_node_count: self.nodes.len() as u64,
            returned_edge_count,
            omitted_edge_count: self.omitted_edges,
            degraded_reasons: degraded_reasons(self.omitted_edges),
            nodes: self.nodes.into_values().collect(),
            edges: self.edges,
        }
    }

    fn query(&self) -> ActivityMemoryGraphQuery {
        ActivityMemoryGraphQuery {
            query_id: self.generated_at.clone(),
            query_kind: ActivityMemoryGraphQueryKind::ActivityByTimeRange,
            child_profile: None,
            device: self.query_device.clone(),
            time_range: ActivityMemoryGraphTimeRange {
                observed_from: self
                    .first_observed_at
                    .clone()
                    .unwrap_or_else(|| self.generated_at.clone()),
                observed_until: self
                    .last_observed_at
                    .clone()
                    .unwrap_or_else(|| self.generated_at.clone()),
            },
            as_of: self.generated_at.clone(),
            limit: self.limit,
        }
    }

    fn edge_from_row(&mut self, row: &MemoryGraphStoreRow) -> Option<ActivityMemoryGraphEdge> {
        let device = device_from_row(row);
        let device_node = device_node(row, &device, &self.generated_at);
        let activity_node = activity_node(row, &device, &self.generated_at)?;
        let edge_kind = edge_kind(row)?;
        let edge = ActivityMemoryGraphEdge {
            graph_id: graph_id(),
            edge_id: edge_id(row),
            edge_kind,
            from_node_id: device_node.node_id.clone(),
            to_node_id: activity_node.node_id.clone(),
            observed_from: row.observed_at.clone(),
            observed_until: None,
            duration_ms: None,
            trace: trace_from_row(row, &self.generated_at, confidence_for_edge(edge_kind)),
        };
        self.nodes.insert(device_node.node_id.clone(), device_node);
        self.nodes
            .insert(activity_node.node_id.clone(), activity_node);
        Some(edge)
    }

    fn capture_time_range(&mut self, observed_at: &str) {
        if self.last_observed_at.is_none() {
            self.last_observed_at = Some(observed_at.to_string());
        }
        self.first_observed_at = Some(observed_at.to_string());
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
