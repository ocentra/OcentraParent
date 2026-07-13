#[macro_use]
#[path = "../support/unit_root_basic_harness.rs"]
mod unit_root_basic_harness;
declare_agent_service_unit_root_basic_harness!();

use crate::test_text::TestText;
use ocentra_parent_agent_protocol::activity::{
    policy::ParentEvidenceReference, policy::ParentEvidenceReferenceKind,
    policy_context::ParentDeviceReference,
};
use ocentra_parent_agent_protocol::activity_memory_graph::{
    ActivityMemoryGraphEdge, ActivityMemoryGraphEdgeKind, ActivityMemoryGraphEntryStatus,
    ActivityMemoryGraphNode, ActivityMemoryGraphNodeKind, ActivityMemoryGraphQuery,
    ActivityMemoryGraphQueryKind, ActivityMemoryGraphReadModel, ActivityMemoryGraphTimeRange,
    ActivityMemoryGraphTrace,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::ACTIVITY_MEMORY_GRAPH_CAPABILITY_READY;
use ocentra_parent_agent_protocol::ACTIVITY_MEMORY_GRAPH_CUSTODY_ACTIVITY_STORE;
use ocentra_parent_agent_protocol::ACTIVITY_MEMORY_GRAPH_INDEX_VERSION;
use ocentra_parent_agent_protocol::ACTIVITY_MEMORY_GRAPH_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::activity_memory_graph_payload_for_test;

#[test]
fn activity_memory_graph_payload_contains_contract_digest_json() -> Result<(), TestText> {
    let read_model = read_model();

    let payload = activity_memory_graph_payload_for_test(&read_model);
    let digest_json = match payload.get(constants::field::ACTIVITY_DIGEST) {
        Some(LogFieldValue::String(value)) => value.as_str(),
        _ => {
            return Err(TestText::from_display(
                constants::error::AGENT_EVENT_SERIALIZES,
            ));
        }
    };
    let digest =
        serde_json::from_str::<ActivityMemoryGraphReadModel>(digest_json).map_err(|error| {
            TestText::from_display(format!(
                "{}: {error}",
                constants::error::AGENT_EVENT_SERIALIZES
            ))
        })?;

    assert_eq!(
        payload.get(constants::field::RETURNED),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(digest.returned_edge_count, 1);
    assert_eq!(
        digest.edges[0].edge_kind,
        ActivityMemoryGraphEdgeKind::Visited
    );
    assert_eq!(
        digest.edges[0].trace.source_evidence_references[0].kind,
        ParentEvidenceReferenceKind::ActivityEvent
    );
    Ok(())
}

fn read_model() -> ActivityMemoryGraphReadModel {
    ActivityMemoryGraphReadModel {
        schema_version: ACTIVITY_MEMORY_GRAPH_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody: ACTIVITY_MEMORY_GRAPH_CUSTODY_ACTIVITY_STORE.to_string(),
        capability_status: ACTIVITY_MEMORY_GRAPH_CAPABILITY_READY.to_string(),
        query: ActivityMemoryGraphQuery {
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
        },
        read_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        nodes: vec![
            node(ActivityMemoryGraphNodeKind::Device),
            node(ActivityMemoryGraphNodeKind::BrowserUrl),
        ],
        edges: vec![edge()],
        returned_node_count: 2,
        returned_edge_count: 1,
        omitted_edge_count: 0,
        degraded_reasons: Vec::new(),
    }
}

fn node(node_kind: ActivityMemoryGraphNodeKind) -> ActivityMemoryGraphNode {
    ActivityMemoryGraphNode {
        graph_id: ACTIVITY_MEMORY_GRAPH_INDEX_VERSION.to_string(),
        node_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
        node_kind,
        label: constants::activity_store::TEST_BROWSER_URL.to_string(),
        child_profile: None,
        device: Some(device()),
        trace: trace(),
    }
}

fn edge() -> ActivityMemoryGraphEdge {
    ActivityMemoryGraphEdge {
        graph_id: ACTIVITY_MEMORY_GRAPH_INDEX_VERSION.to_string(),
        edge_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
        edge_kind: ActivityMemoryGraphEdgeKind::Visited,
        from_node_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
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
        source_evidence_references: vec![ParentEvidenceReference {
            evidence_reference_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
            kind: ParentEvidenceReferenceKind::ActivityEvent,
            observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        }],
        source_policy_version: None,
        source_parent_action_references: Vec::new(),
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        expires_at: None,
        confidence: 1.0,
        derived_index_version: ACTIVITY_MEMORY_GRAPH_INDEX_VERSION.to_string(),
        degraded_reasons: Vec::new(),
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
