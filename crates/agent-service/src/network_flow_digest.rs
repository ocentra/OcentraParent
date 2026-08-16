use std::collections::BTreeSet;

use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowDigest;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowObservation;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel;
use ocentra_parent_agent_protocol::NETWORK_FLOW_SCHEMA_VERSION;

use crate::{
    network_flow_digest_indicators::network_indicators,
    network_flow_digest_rollups::{rollups_for_destinations, rollups_for_processes},
};

pub(crate) fn network_flow_digest(
    read_model: &ActivityNetworkFlowReadModel,
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
