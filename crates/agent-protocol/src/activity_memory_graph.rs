use serde::{Deserialize, Serialize};

use crate::{
    activity::policy::ParentEvidenceReference,
    activity::policy_context::{ChildProfileReference, ParentDeviceReference},
    enforcement::ParentActionReference,
};

pub const ACTIVITY_MEMORY_GRAPH_SCHEMA_VERSION: u16 = crate::ACTIVITY_MEMORY_GRAPH_SCHEMA_VERSION;
pub const ACTIVITY_MEMORY_GRAPH_CUSTODY_ACTIVITY_STORE: &str = "child-device-activity-store";
pub const ACTIVITY_MEMORY_GRAPH_CAPABILITY_READY: &str = "ready";
pub const ACTIVITY_MEMORY_GRAPH_CAPABILITY_NO_EVIDENCE: &str = "no-evidence";
pub const ACTIVITY_MEMORY_GRAPH_INDEX_VERSION: &str = "activity-memory-v1";
pub const ACTIVITY_MEMORY_GRAPH_GAME_LABEL_HINT: &str = "game";
pub const ACTIVITY_MEMORY_GRAPH_REASON_EDGE_LIMIT: &str = "memory-edge-limit";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityMemoryGraphNodeKind {
    #[serde(rename = "child-profile")]
    ChildProfile,
    #[serde(rename = "device")]
    Device,
    #[serde(rename = "browser-url")]
    BrowserUrl,
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "app")]
    App,
    #[serde(rename = "game")]
    Game,
    #[serde(rename = "activity-session")]
    ActivitySession,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityMemoryGraphEdgeKind {
    #[serde(rename = "visited")]
    Visited,
    #[serde(rename = "watched")]
    Watched,
    #[serde(rename = "played")]
    Played,
    #[serde(rename = "active-during")]
    ActiveDuring,
    #[serde(rename = "performed-by-child")]
    PerformedByChild,
    #[serde(rename = "derived-from-evidence")]
    DerivedFromEvidence,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityMemoryGraphEntryStatus {
    #[serde(rename = "usable")]
    Usable,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityMemoryGraphQueryKind {
    #[serde(rename = "visited-urls")]
    VisitedUrls,
    #[serde(rename = "played-games")]
    PlayedGames,
    #[serde(rename = "watched-videos")]
    WatchedVideos,
    #[serde(rename = "activity-by-time-range")]
    ActivityByTimeRange,
    #[serde(rename = "explain-evidence")]
    ExplainEvidence,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMemoryGraphTrace {
    pub entry_status: ActivityMemoryGraphEntryStatus,
    pub source_evidence_references: Vec<ParentEvidenceReference>,
    pub source_policy_version: Option<String>,
    pub source_parent_action_references: Vec<ParentActionReference>,
    pub generated_at: String,
    pub expires_at: Option<String>,
    pub confidence: f64,
    pub derived_index_version: String,
    pub degraded_reasons: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMemoryGraphTimeRange {
    pub observed_from: String,
    pub observed_until: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMemoryGraphNode {
    pub graph_id: String,
    pub node_id: String,
    pub node_kind: ActivityMemoryGraphNodeKind,
    pub label: String,
    pub child_profile: Option<ChildProfileReference>,
    pub device: Option<ParentDeviceReference>,
    pub trace: ActivityMemoryGraphTrace,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMemoryGraphEdge {
    pub graph_id: String,
    pub edge_id: String,
    pub edge_kind: ActivityMemoryGraphEdgeKind,
    pub from_node_id: String,
    pub to_node_id: String,
    pub observed_from: String,
    pub observed_until: Option<String>,
    pub duration_ms: Option<u64>,
    pub trace: ActivityMemoryGraphTrace,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMemoryGraphQuery {
    pub query_id: String,
    pub query_kind: ActivityMemoryGraphQueryKind,
    pub child_profile: Option<ChildProfileReference>,
    pub device: ParentDeviceReference,
    pub time_range: ActivityMemoryGraphTimeRange,
    pub as_of: String,
    pub limit: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMemoryGraphReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody: String,
    pub capability_status: String,
    pub query: ActivityMemoryGraphQuery,
    pub read_at: String,
    pub nodes: Vec<ActivityMemoryGraphNode>,
    pub edges: Vec<ActivityMemoryGraphEdge>,
    pub returned_node_count: u64,
    pub returned_edge_count: u64,
    pub omitted_edge_count: u64,
    pub degraded_reasons: Vec<String>,
}
