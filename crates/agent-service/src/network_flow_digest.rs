use std::collections::BTreeSet;

use ocentra_parent_agent_protocol::{
    ActivityEvidenceRef, ActivityNetworkFlowDigest, ActivityNetworkFlowObservation,
    ActivityNetworkFlowReadModel, ActivityNetworkRuntimeDelivery, NETWORK_FLOW_SCHEMA_VERSION,
};

use crate::{
    network_flow_digest_indicators::network_indicators,
    network_flow_digest_rollups::{rollups_for_destinations, rollups_for_processes},
    network_runtime_delivery::NetworkRuntimeServiceDeliveryReport,
};

pub(crate) fn network_flow_digest(
    read_model: &ActivityNetworkFlowReadModel,
    delivery: Option<&NetworkRuntimeServiceDeliveryReport>,
) -> ActivityNetworkFlowDigest {
    let top_destinations = rollups_for_destinations(&read_model.rows);
    ActivityNetworkFlowDigest {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        generated_at: read_model.generated_at.clone(),
        custody: read_model.custody.clone(),
        evidence: unique_evidence(&read_model.rows),
        top_processes: rollups_for_processes(&read_model.rows),
        top_destinations: top_destinations.clone(),
        unusual_indicators: network_indicators(&read_model.rows, &top_destinations),
        runtime_delivery: delivery.map(runtime_delivery),
    }
}

fn runtime_delivery(
    delivery: &NetworkRuntimeServiceDeliveryReport,
) -> ActivityNetworkRuntimeDelivery {
    ActivityNetworkRuntimeDelivery {
        observed_rows: delivery.observed_rows as u64,
        delivered_rows: delivery.delivered_rows as u64,
        failed_rows: delivery.failed_rows as u64,
        publish_reports: delivery.publish_reports as u64,
        stored_events: delivery.stored_events as u64,
        dead_letters: delivery.dead_letters as u64,
        manual_required_rows: delivery.manual_required_rows as u64,
        enforcement_command_events: delivery.enforcement_command_events as u64,
    }
}

fn unique_evidence(observations: &[ActivityNetworkFlowObservation]) -> Vec<ActivityEvidenceRef> {
    let mut seen = BTreeSet::<String>::new();
    let mut evidence_refs = Vec::new();
    for evidence in observations
        .iter()
        .flat_map(|observation| observation.evidence.iter())
    {
        if seen.insert(evidence.evidence_id.clone()) {
            evidence_refs.push(evidence.clone());
        }
    }
    evidence_refs
}
