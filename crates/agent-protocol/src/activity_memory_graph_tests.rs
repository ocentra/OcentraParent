use super::{
    constants, ActivityMemoryGraphEdge, ActivityMemoryGraphEdgeKind,
    ActivityMemoryGraphEntryStatus, ActivityMemoryGraphNode, ActivityMemoryGraphNodeKind,
    ActivityMemoryGraphQuery, ActivityMemoryGraphQueryKind, ActivityMemoryGraphReadModel,
    ActivityMemoryGraphTimeRange, ActivityMemoryGraphTrace, ParentDeviceReference,
    ParentEvidenceReference, ParentEvidenceReferenceKind, ACTIVITY_MEMORY_GRAPH_CAPABILITY_READY,
    ACTIVITY_MEMORY_GRAPH_CUSTODY_ACTIVITY_STORE, ACTIVITY_MEMORY_GRAPH_INDEX_VERSION,
    ACTIVITY_MEMORY_GRAPH_SCHEMA_VERSION,
};

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
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES);

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
        graph_id: graph_id(),
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
        graph_id: graph_id(),
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
        graph_id: graph_id(),
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

fn graph_id() -> String {
    String::from("activity-memory-graph-test")
}
