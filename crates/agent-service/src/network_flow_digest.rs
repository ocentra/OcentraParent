use ocentra_parent_agent_protocol::{
    ActivityEvidenceRef, ActivityNetworkCustodyState, ActivityNetworkFlowDigest,
    ActivityNetworkFlowObservation, ACTIVITY_QUERY_SCHEMA_VERSION,
};

use crate::{
    network_flow_indicators::network_indicators, network_flow_rollups::rollups_for_destinations,
    network_flow_rollups::rollups_for_processes, time::timestamp_now,
};

pub(crate) fn network_flow_digest(
    observations: &[ActivityNetworkFlowObservation],
) -> ActivityNetworkFlowDigest {
    let evidence = observations
        .iter()
        .flat_map(|observation| observation.evidence.iter().cloned())
        .fold(Vec::<ActivityEvidenceRef>::new(), |mut acc, evidence| {
            if !acc
                .iter()
                .any(|item: &ActivityEvidenceRef| item.evidence_id == evidence.evidence_id)
            {
                acc.push(evidence);
            }
            acc
        });
    let top_processes = rollups_for_processes(observations);
    let top_destinations = rollups_for_destinations(observations);
    let unusual_indicators = network_indicators(observations, &top_destinations);

    ActivityNetworkFlowDigest {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: timestamp_now(),
        custody: ActivityNetworkCustodyState::ChildDeviceQueryStore,
        evidence,
        top_processes,
        top_destinations,
        unusual_indicators,
    }
}
