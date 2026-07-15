use super::{
    constants, ActivityMemoryGraphEdge, ActivityMemoryGraphEdgeKind,
    ActivityMemoryGraphEntryStatus, ActivityMemoryGraphNode, ActivityMemoryGraphNodeKind,
    ActivityMemoryGraphQuery, ActivityMemoryGraphQueryKind, ActivityMemoryGraphReadModel,
    ActivityMemoryGraphTimeRange, ActivityMemoryGraphTrace, ChildProfileReference,
    ParentDeviceReference, ParentEvidenceReference, ParentEvidenceReferenceKind,
    ACTIVITY_MEMORY_GRAPH_CAPABILITY_READY, ACTIVITY_MEMORY_GRAPH_CUSTODY_ACTIVITY_STORE,
    ACTIVITY_MEMORY_GRAPH_INDEX_VERSION, ACTIVITY_MEMORY_GRAPH_SCHEMA_VERSION,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn activity_memory_graph_serializes_evidence_cited_edges() {
    let read_model = ActivityMemoryGraphReadModel {
        schema_version: ACTIVITY_MEMORY_GRAPH_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody: ACTIVITY_MEMORY_GRAPH_CUSTODY_ACTIVITY_STORE.to_string(),
        capability_status: ACTIVITY_MEMORY_GRAPH_CAPABILITY_READY.to_string(),
        query: query(),
        read_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        nodes: vec![device_node(), url_node()],
        edges: vec![visited_edge()],
        returned_node_count: 2,
        returned_edge_count: 1,
        omitted_edge_count: 0,
        degraded_reasons: Vec::new(),
    };

    let serialized =
        serde_json::to_value(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["schemaVersion"],
        ACTIVITY_MEMORY_GRAPH_SCHEMA_VERSION
    );
    assert_eq!(serialized["edges"][0]["edgeKind"], "visited");
    assert_eq!(
        serialized["edges"][0]["trace"]["sourceEvidenceReferences"][0]["evidenceReferenceId"],
        constants::activity_store::TEST_BROWSER_TARGET_ID
    );
    assert_eq!(serialized["query"]["childProfile"], serde_json::Value::Null);
}

#[test]
fn activity_memory_graph_round_trips_child_profile_display_name_in_typescript_shape() {
    let value = child_profile_round_trip_value();

    let parsed: ActivityMemoryGraphReadModel =
        serde_json::from_value(value).expect_value("typescript activity memory graph shape parses");

    assert_eq!(
        parsed.query.child_profile,
        Some(ChildProfileReference {
            child_profile_id: "child-profile-1".to_string(),
            display_name: "Child One".to_string(),
        })
    );
    assert_eq!(
        parsed.nodes[0].child_profile,
        Some(ChildProfileReference {
            child_profile_id: "child-profile-1".to_string(),
            display_name: "Child One".to_string(),
        })
    );

    let serialized =
        serde_json::to_value(parsed).expect_value("activity memory graph round trip serializes");

    assert_eq!(
        serialized["query"]["childProfile"]["displayName"],
        "Child One"
    );
    assert_eq!(
        serialized["nodes"][0]["childProfile"]["displayName"],
        "Child One"
    );
    assert_eq!(serialized["nodes"][0]["nodeKind"], "child-profile");
    assert_eq!(serialized["query"]["queryKind"], "visited-urls");
    assert!(serialized["query"]["childProfile"]
        .get("familyId")
        .is_none());
}

fn query() -> ActivityMemoryGraphQuery {
    ActivityMemoryGraphQuery {
        query_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
        query_kind: ActivityMemoryGraphQueryKind::ActivityByTimeRange,
        child_profile: None,
        device: device(),
        time_range: ActivityMemoryGraphTimeRange {
            observed_from: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
            observed_until: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        },
        as_of: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
    }
}

fn device_node() -> ActivityMemoryGraphNode {
    ActivityMemoryGraphNode {
        graph_id: GRAPH_ID.to_string(),
        node_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        node_kind: ActivityMemoryGraphNodeKind::Device,
        label: constants::peer::LOCAL_DEV_AGENT.to_string(),
        child_profile: None,
        device: Some(device()),
        trace: trace(),
    }
}

fn url_node() -> ActivityMemoryGraphNode {
    ActivityMemoryGraphNode {
        graph_id: GRAPH_ID.to_string(),
        node_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
        node_kind: ActivityMemoryGraphNodeKind::BrowserUrl,
        label: constants::activity_store::TEST_BROWSER_URL.to_string(),
        child_profile: None,
        device: Some(device()),
        trace: trace(),
    }
}

fn visited_edge() -> ActivityMemoryGraphEdge {
    ActivityMemoryGraphEdge {
        graph_id: GRAPH_ID.to_string(),
        edge_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
        edge_kind: ActivityMemoryGraphEdgeKind::Visited,
        from_node_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        to_node_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
        observed_from: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        observed_until: None,
        duration_ms: None,
        trace: trace(),
    }
}

fn trace() -> ActivityMemoryGraphTrace {
    ActivityMemoryGraphTrace {
        entry_status: ActivityMemoryGraphEntryStatus::Usable,
        source_evidence_references: vec![evidence()],
        source_policy_version: None,
        source_parent_action_references: Vec::new(),
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        expires_at: None,
        confidence: 1.0,
        derived_index_version: ACTIVITY_MEMORY_GRAPH_INDEX_VERSION.to_string(),
        degraded_reasons: Vec::new(),
    }
}

fn evidence() -> ParentEvidenceReference {
    ParentEvidenceReference {
        evidence_reference_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
        kind: ParentEvidenceReferenceKind::ActivityEvent,
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
    }
}

fn device() -> ParentDeviceReference {
    ParentDeviceReference {
        device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        child_profile_id: None,
        label: constants::peer::LOCAL_DEV_AGENT.to_string(),
        platform: std::env::consts::OS.to_string(),
    }
}

const GRAPH_ID: &str = "activity-memory-graph-test";
const CHILD_PROFILE_ID: &str = "child-profile-1";
const CHILD_PROFILE_DISPLAY_NAME: &str = "Child One";
const CHILD_PROFILE_QUERY_KIND: &str = "visited-urls";
const CHILD_PROFILE_NODE_KIND: &str = "child-profile";
const CHILD_PROFILE_GRAPH_ID: &str = "activity-memory-graph-ts-shape";

fn child_profile_round_trip_value() -> serde_json::Value {
    child_profile_round_trip_root()
}

fn child_profile_round_trip_root() -> serde_json::Value {
    let mut root = serde_json::Map::new();
    root.insert(
        "schemaVersion".to_string(),
        serde_json::Value::from(ACTIVITY_MEMORY_GRAPH_SCHEMA_VERSION),
    );
    root.insert(
        "generatedAt".to_string(),
        serde_json::Value::String(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
    );
    root.insert(
        "custody".to_string(),
        serde_json::Value::String(ACTIVITY_MEMORY_GRAPH_CUSTODY_ACTIVITY_STORE.to_string()),
    );
    root.insert(
        "capabilityStatus".to_string(),
        serde_json::Value::String(ACTIVITY_MEMORY_GRAPH_CAPABILITY_READY.to_string()),
    );
    root.insert("query".to_string(), child_profile_round_trip_query());
    root.insert(
        "readAt".to_string(),
        serde_json::Value::String(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
    );
    root.insert(
        "nodes".to_string(),
        serde_json::Value::Array(vec![child_profile_round_trip_node()]),
    );
    root.insert("edges".to_string(), serde_json::Value::Array(Vec::new()));
    root.insert("returnedNodeCount".to_string(), serde_json::Value::from(1));
    root.insert("returnedEdgeCount".to_string(), serde_json::Value::from(0));
    root.insert("omittedEdgeCount".to_string(), serde_json::Value::from(0));
    root.insert(
        "degradedReasons".to_string(),
        serde_json::Value::Array(Vec::new()),
    );
    serde_json::Value::Object(root)
}

fn child_profile_round_trip_query() -> serde_json::Value {
    let mut query = serde_json::Map::new();
    query.insert(
        "queryId".to_string(),
        serde_json::Value::String(constants::activity_store::TEST_BROWSER_TARGET_ID.to_string()),
    );
    query.insert(
        "queryKind".to_string(),
        serde_json::Value::String(CHILD_PROFILE_QUERY_KIND.to_string()),
    );
    query.insert(
        "childProfile".to_string(),
        child_profile_round_trip_child_profile(),
    );
    query.insert("device".to_string(), child_profile_round_trip_device());
    query.insert(
        "timeRange".to_string(),
        child_profile_round_trip_time_range(),
    );
    query.insert(
        "asOf".to_string(),
        serde_json::Value::String(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
    );
    query.insert("limit".to_string(), serde_json::Value::from(25));
    serde_json::Value::Object(query)
}

fn child_profile_round_trip_node() -> serde_json::Value {
    let mut node = serde_json::Map::new();
    node.insert(
        "graphId".to_string(),
        serde_json::Value::String(CHILD_PROFILE_GRAPH_ID.to_string()),
    );
    node.insert(
        "nodeId".to_string(),
        serde_json::Value::String(CHILD_PROFILE_ID.to_string()),
    );
    node.insert(
        "nodeKind".to_string(),
        serde_json::Value::String(CHILD_PROFILE_NODE_KIND.to_string()),
    );
    node.insert(
        "label".to_string(),
        serde_json::Value::String(CHILD_PROFILE_DISPLAY_NAME.to_string()),
    );
    node.insert(
        "childProfile".to_string(),
        child_profile_round_trip_child_profile(),
    );
    node.insert("device".to_string(), serde_json::Value::Null);
    node.insert("trace".to_string(), child_profile_round_trip_trace());
    serde_json::Value::Object(node)
}

fn child_profile_round_trip_trace() -> serde_json::Value {
    let mut source_evidence = serde_json::Map::new();
    source_evidence.insert(
        "evidenceReferenceId".to_string(),
        serde_json::Value::String(constants::activity_store::TEST_BROWSER_TARGET_ID.to_string()),
    );
    source_evidence.insert(
        "kind".to_string(),
        serde_json::Value::String("activity-event".to_string()),
    );
    source_evidence.insert(
        "observedAt".to_string(),
        serde_json::Value::String(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
    );

    let mut trace = serde_json::Map::new();
    trace.insert(
        "entryStatus".to_string(),
        serde_json::Value::String("usable".to_string()),
    );
    trace.insert(
        "sourceEvidenceReferences".to_string(),
        serde_json::Value::Array(vec![serde_json::Value::Object(source_evidence)]),
    );
    trace.insert("sourcePolicyVersion".to_string(), serde_json::Value::Null);
    trace.insert(
        "sourceParentActionReferences".to_string(),
        serde_json::Value::Array(Vec::new()),
    );
    trace.insert(
        "generatedAt".to_string(),
        serde_json::Value::String(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
    );
    trace.insert("expiresAt".to_string(), serde_json::Value::Null);
    trace.insert("confidence".to_string(), serde_json::Value::from(1.0));
    trace.insert(
        "derivedIndexVersion".to_string(),
        serde_json::Value::String(ACTIVITY_MEMORY_GRAPH_INDEX_VERSION.to_string()),
    );
    trace.insert(
        "degradedReasons".to_string(),
        serde_json::Value::Array(Vec::new()),
    );
    serde_json::Value::Object(trace)
}

fn child_profile_round_trip_child_profile() -> serde_json::Value {
    let mut child_profile = serde_json::Map::new();
    child_profile.insert(
        "childProfileId".to_string(),
        serde_json::Value::String(CHILD_PROFILE_ID.to_string()),
    );
    child_profile.insert(
        "displayName".to_string(),
        serde_json::Value::String(CHILD_PROFILE_DISPLAY_NAME.to_string()),
    );
    serde_json::Value::Object(child_profile)
}

fn child_profile_round_trip_device() -> serde_json::Value {
    let mut device = serde_json::Map::new();
    device.insert(
        "deviceId".to_string(),
        serde_json::Value::String(constants::peer::LOCAL_DEV_AGENT.to_string()),
    );
    device.insert(
        "childProfileId".to_string(),
        serde_json::Value::String(CHILD_PROFILE_ID.to_string()),
    );
    device.insert(
        "label".to_string(),
        serde_json::Value::String(constants::peer::LOCAL_DEV_AGENT.to_string()),
    );
    device.insert(
        "platform".to_string(),
        serde_json::Value::String(std::env::consts::OS.to_string()),
    );
    serde_json::Value::Object(device)
}

fn child_profile_round_trip_time_range() -> serde_json::Value {
    serde_json::json!({
        "observedFrom": constants::activity_store::TEST_FIRST_OBSERVED_AT,
        "observedUntil": constants::activity_store::TEST_SECOND_OBSERVED_AT
    })
}
