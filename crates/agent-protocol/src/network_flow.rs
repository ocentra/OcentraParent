use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

pub const NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE: &str = "child-device-query-store";
pub const NETWORK_FLOW_CUSTODY_UNAVAILABLE: &str = "unavailable";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkEndpoint {
    pub ip: Option<String>,
    pub port: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowCounters {
    pub connection_count: u64,
    pub bytes_sent: Option<u64>,
    pub bytes_received: Option<u64>,
    pub first_seen_at: Option<String>,
    pub last_seen_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowObservation {
    pub schema_version: u16,
    pub event_id: String,
    pub observed_at: String,
    pub observer: String,
    pub capability_status: String,
    pub adapter_id: String,
    pub protocol: Option<String>,
    pub tcp_state: Option<String>,
    pub local_endpoint: ActivityNetworkEndpoint,
    pub destination_endpoint: ActivityNetworkEndpoint,
    pub destination_domain: Option<String>,
    pub domain_attribution_status: String,
    pub process_attribution_status: String,
    pub process_id: Option<u64>,
    pub process_name: Option<String>,
    pub counters: ActivityNetworkFlowCounters,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody: String,
    pub limit: u64,
    pub returned: u64,
    pub capability_status: String,
    pub rows: Vec<ActivityNetworkFlowObservation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowRollup {
    pub key: String,
    pub label: String,
    pub connection_count: u64,
    pub bytes_sent: Option<u64>,
    pub bytes_received: Option<u64>,
    pub evidence_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowIndicator {
    pub kind: String,
    pub label: String,
    pub observed_at: String,
    pub evidence_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityNetworkFlowDigest {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody: String,
    pub evidence: Vec<ActivityEvidenceRef>,
    pub top_processes: Vec<ActivityNetworkFlowRollup>,
    pub top_destinations: Vec<ActivityNetworkFlowRollup>,
    pub unusual_indicators: Vec<ActivityNetworkFlowIndicator>,
}

#[path = "network_flow_events.rs"]
mod network_flow_events;

pub use network_flow_events::*;
