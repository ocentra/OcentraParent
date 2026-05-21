use std::collections::{BTreeMap, BTreeSet};

use ocentra_parent_agent_protocol::{
    constants, ActivityNetworkFlowObservation, ActivityNetworkFlowRollup,
};

const NETWORK_FLOW_ROLLUP_LIMIT: usize = 5;

pub(crate) fn rollups_for_processes(
    observations: &[ActivityNetworkFlowObservation],
) -> Vec<ActivityNetworkFlowRollup> {
    let mut map = BTreeMap::<String, RollupAccumulator>::new();
    for observation in observations {
        let key = process_rollup_key(observation);
        let label = observation
            .process_name
            .clone()
            .unwrap_or_else(|| constants::network_flow::LABEL_PROCESS_UNKNOWN.to_string());
        map.entry(key)
            .or_insert_with(|| RollupAccumulator::new(label))
            .push(observation);
    }
    rollups_from_map(map)
}

pub(crate) fn rollups_for_destinations(
    observations: &[ActivityNetworkFlowObservation],
) -> Vec<ActivityNetworkFlowRollup> {
    let mut map = BTreeMap::<String, RollupAccumulator>::new();
    for observation in observations {
        let key = destination_rollup_key(observation);
        let label = observation
            .destination_domain
            .clone()
            .or_else(|| endpoint_text(&observation.destination_endpoint))
            .unwrap_or_else(|| constants::network_flow::LABEL_DESTINATION_UNKNOWN.to_string());
        map.entry(key)
            .or_insert_with(|| RollupAccumulator::new(label))
            .push(observation);
    }
    rollups_from_map(map)
}

fn rollups_from_map(map: BTreeMap<String, RollupAccumulator>) -> Vec<ActivityNetworkFlowRollup> {
    let mut rollups: Vec<ActivityNetworkFlowRollup> = map
        .into_iter()
        .map(|(key, accumulator)| ActivityNetworkFlowRollup {
            key,
            label: accumulator.label,
            connection_count: accumulator.connection_count,
            bytes_sent: accumulator.bytes_sent,
            bytes_received: accumulator.bytes_received,
            evidence_ids: accumulator.evidence_ids.into_iter().collect(),
        })
        .collect();
    rollups.sort_by(|left, right| {
        right
            .connection_count
            .cmp(&left.connection_count)
            .then_with(|| left.label.cmp(&right.label))
    });
    rollups.truncate(NETWORK_FLOW_ROLLUP_LIMIT);
    rollups
}

fn process_rollup_key(observation: &ActivityNetworkFlowObservation) -> String {
    if let Some(process_id) = observation.process_id {
        return prefixed_value(
            constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX,
            &process_id.to_string(),
        );
    }
    if let Some(process_name) = &observation.process_name {
        return prefixed_value(
            constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX,
            process_name,
        );
    }
    constants::network_flow::LABEL_PROCESS_UNKNOWN.to_string()
}

fn destination_rollup_key(observation: &ActivityNetworkFlowObservation) -> String {
    if let Some(domain) = &observation.destination_domain {
        return prefixed_value(
            constants::activity_capture::NETWORK_SUBJECT_ID_PREFIX,
            domain,
        );
    }
    if let Some(endpoint) = endpoint_text(&observation.destination_endpoint) {
        return prefixed_value(
            constants::activity_capture::NETWORK_SUBJECT_ID_PREFIX,
            &endpoint,
        );
    }
    constants::network_flow::LABEL_DESTINATION_UNKNOWN.to_string()
}

fn endpoint_text(
    endpoint: &ocentra_parent_agent_protocol::ActivityNetworkEndpoint,
) -> Option<String> {
    let ip = endpoint.ip.as_ref()?;
    match endpoint.port {
        Some(port) => {
            let mut text = ip.clone();
            text.push(constants::delimiter::COLON);
            text.push_str(&port.to_string());
            Some(text)
        }
        None => Some(ip.clone()),
    }
}

fn prefixed_value(prefix: &str, value: &str) -> String {
    let mut text = String::from(prefix);
    text.push_str(value);
    text
}

#[derive(Clone)]
struct RollupAccumulator {
    label: String,
    connection_count: u64,
    bytes_sent: Option<u64>,
    bytes_received: Option<u64>,
    evidence_ids: BTreeSet<String>,
}

impl RollupAccumulator {
    fn new(label: String) -> Self {
        Self {
            label,
            connection_count: 0,
            bytes_sent: None,
            bytes_received: None,
            evidence_ids: BTreeSet::new(),
        }
    }

    fn push(&mut self, observation: &ActivityNetworkFlowObservation) {
        self.connection_count += observation.counters.connection_count;
        self.bytes_sent = merged_counter(self.bytes_sent, observation.counters.bytes_sent);
        self.bytes_received =
            merged_counter(self.bytes_received, observation.counters.bytes_received);
        for evidence in &observation.evidence {
            let _ = self.evidence_ids.insert(evidence.evidence_id.clone());
        }
    }
}

fn merged_counter(current: Option<u64>, next: Option<u64>) -> Option<u64> {
    match (current, next) {
        (Some(left), Some(right)) => Some(left + right),
        (Some(left), None) => Some(left),
        (None, Some(right)) => Some(right),
        (None, None) => None,
    }
}
