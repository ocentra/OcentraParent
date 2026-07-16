use std::collections::{BTreeMap, BTreeSet};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowObservation;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowRollup;

const NETWORK_FLOW_ROLLUP_LIMIT: usize = 5;

#[derive(Clone)]
struct DisplayText(String);

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct RollupKey(String);

#[derive(Clone, Copy)]
struct PrefixRef<'a>(&'a str);

#[derive(Clone, Copy)]
struct TextValueRef<'a>(&'a str);

pub(crate) fn rollups_for_processes(
    observations: &[ActivityNetworkFlowObservation],
) -> Vec<ActivityNetworkFlowRollup> {
    let mut map = BTreeMap::<RollupKey, RollupAccumulator>::new();
    for observation in observations {
        let key = process_rollup_key(observation);
        let label = observation
            .process_name
            .clone()
            .map(DisplayText)
            .unwrap_or_else(|| {
                DisplayText(constants::network_flow::LABEL_PROCESS_UNKNOWN.to_string())
            });
        map.entry(key)
            .or_insert_with(|| RollupAccumulator::new(label))
            .push(observation);
    }
    rollups_from_map(map)
}

pub(crate) fn rollups_for_destinations(
    observations: &[ActivityNetworkFlowObservation],
) -> Vec<ActivityNetworkFlowRollup> {
    let mut map = BTreeMap::<RollupKey, RollupAccumulator>::new();
    for observation in observations {
        let key = destination_rollup_key(observation);
        let label = observation
            .destination_domain
            .clone()
            .map(DisplayText)
            .or_else(|| endpoint_text(&observation.destination_endpoint))
            .unwrap_or_else(|| {
                DisplayText(constants::network_flow::LABEL_DESTINATION_UNKNOWN.to_string())
            });
        map.entry(key)
            .or_insert_with(|| RollupAccumulator::new(label))
            .push(observation);
    }
    rollups_from_map(map)
}

fn rollups_from_map(map: BTreeMap<RollupKey, RollupAccumulator>) -> Vec<ActivityNetworkFlowRollup> {
    let mut rollups: Vec<ActivityNetworkFlowRollup> = map
        .into_iter()
        .map(|(key, accumulator)| ActivityNetworkFlowRollup {
            key: key.0,
            label: accumulator.display_text,
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

fn process_rollup_key(observation: &ActivityNetworkFlowObservation) -> RollupKey {
    observation
        .process_id
        .map(|process_id| {
            let process_id_text = process_id.to_string();
            prefixed_value(
                PrefixRef(constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX),
                TextValueRef(process_id_text.as_str()),
            )
        })
        .or_else(|| {
            observation.process_name.as_deref().map(|process_name| {
                prefixed_value(
                    PrefixRef(constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX),
                    TextValueRef(process_name),
                )
            })
        })
        .unwrap_or_else(|| RollupKey(constants::network_flow::LABEL_PROCESS_UNKNOWN.to_string()))
}

fn destination_rollup_key(observation: &ActivityNetworkFlowObservation) -> RollupKey {
    observation
        .destination_domain
        .as_deref()
        .map(|domain| {
            prefixed_value(
                PrefixRef(constants::activity_capture::NETWORK_SUBJECT_ID_PREFIX),
                TextValueRef(domain),
            )
        })
        .or_else(|| {
            endpoint_text(&observation.destination_endpoint).map(|endpoint| {
                prefixed_value(
                    PrefixRef(constants::activity_capture::NETWORK_SUBJECT_ID_PREFIX),
                    TextValueRef(endpoint.0.as_str()),
                )
            })
        })
        .unwrap_or_else(|| {
            RollupKey(constants::network_flow::LABEL_DESTINATION_UNKNOWN.to_string())
        })
}

fn endpoint_text(
    endpoint: &ocentra_parent_agent_protocol::network_flow::ActivityNetworkEndpoint,
) -> Option<DisplayText> {
    endpoint.ip.as_ref().map(|ip| {
        let text = endpoint.port.map_or_else(
            || ip.clone(),
            |port| {
                let mut text = ip.clone();
                text.push(constants::delimiter::COLON);
                text.push_str(&port.to_string());
                text
            },
        );
        DisplayText(text)
    })
}

fn prefixed_value(prefix: PrefixRef<'_>, value: TextValueRef<'_>) -> RollupKey {
    let mut text = String::from(prefix.0);
    text.push_str(value.0);
    RollupKey(text)
}

#[derive(Clone)]
struct RollupAccumulator {
    display_text: String,
    connection_count: u64,
    bytes_sent: Option<u64>,
    bytes_received: Option<u64>,
    evidence_ids: BTreeSet<String>,
}

impl RollupAccumulator {
    fn new(display_text: DisplayText) -> Self {
        Self {
            display_text: display_text.0,
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
