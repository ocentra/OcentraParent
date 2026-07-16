use std::collections::BTreeSet;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowIndicator;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowObservation;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowRollup;

const NETWORK_FLOW_INDICATOR_LIMIT: usize = 5;

#[derive(Clone, Copy)]
struct IndicatorTextRef<'a>(&'a str);

#[derive(Clone, Copy)]
struct TcpStateTextRef<'a>(&'a str);

struct EvidenceIdList(Vec<String>);

pub(crate) fn network_indicators(
    observations: &[ActivityNetworkFlowObservation],
    top_destinations: &[ActivityNetworkFlowRollup],
) -> Vec<ActivityNetworkFlowIndicator> {
    let mut indicators = Vec::new();
    for observation in observations {
        push_adapter_indicator(observation, &mut indicators);
        push_unknown_process_indicator(observation, &mut indicators);
        push_encrypted_content_indicator(observation, &mut indicators);
        push_repeated_failure_indicator(observation, &mut indicators);
        push_vpn_proxy_indicator(observation, &mut indicators);
        if indicators.len() >= NETWORK_FLOW_INDICATOR_LIMIT {
            break;
        }
    }
    push_high_volume_indicator(observations, top_destinations, &mut indicators);
    indicators.truncate(NETWORK_FLOW_INDICATOR_LIMIT);
    indicators
}

fn push_adapter_indicator(
    observation: &ActivityNetworkFlowObservation,
    indicators: &mut Vec<ActivityNetworkFlowIndicator>,
) {
    push_once(
        indicators,
        observation.capability_status != constants::activity_capture::CAPABILITY_STATUS_AVAILABLE,
        IndicatorTextRef(constants::network_flow::INDICATOR_ADAPTER_UNAVAILABLE),
        IndicatorTextRef(constants::network_flow::INDICATOR_LABEL_ADAPTER_UNAVAILABLE),
        observation,
    );
}

fn push_unknown_process_indicator(
    observation: &ActivityNetworkFlowObservation,
    indicators: &mut Vec<ActivityNetworkFlowIndicator>,
) {
    push_once(
        indicators,
        observation.process_attribution_status
            == constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_UNKNOWN,
        IndicatorTextRef(constants::network_flow::INDICATOR_UNUSUAL_UNKNOWN_PROCESS),
        IndicatorTextRef(constants::network_flow::INDICATOR_LABEL_UNUSUAL_UNKNOWN_PROCESS),
        observation,
    );
}

fn push_encrypted_content_indicator(
    observation: &ActivityNetworkFlowObservation,
    indicators: &mut Vec<ActivityNetworkFlowIndicator>,
) {
    push_once(
        indicators,
        observation.protocol.as_deref() == Some(constants::activity_capture::NETWORK_PROTOCOL_TCP)
            && observation.destination_domain.is_none()
            && observation.destination_endpoint.ip.is_some(),
        IndicatorTextRef(constants::network_flow::INDICATOR_ENCRYPTED_CONTENT_UNAVAILABLE),
        IndicatorTextRef(constants::network_flow::INDICATOR_LABEL_ENCRYPTED_CONTENT_UNAVAILABLE),
        observation,
    );
}

fn push_repeated_failure_indicator(
    observation: &ActivityNetworkFlowObservation,
    indicators: &mut Vec<ActivityNetworkFlowIndicator>,
) {
    push_once(
        indicators,
        observation.protocol.as_deref() == Some(constants::activity_capture::NETWORK_PROTOCOL_TCP)
            && is_failure_tcp_state(observation.tcp_state.as_deref().map(TcpStateTextRef)),
        IndicatorTextRef(constants::network_flow::INDICATOR_REPEATED_FAILURE),
        IndicatorTextRef(constants::network_flow::INDICATOR_LABEL_REPEATED_FAILURE),
        observation,
    );
}

fn push_vpn_proxy_indicator(
    observation: &ActivityNetworkFlowObservation,
    indicators: &mut Vec<ActivityNetworkFlowIndicator>,
) {
    push_once(
        indicators,
        matches!(
            observation.destination_endpoint.port,
            Some(1080 | 3128 | 8080 | 9050)
        ),
        IndicatorTextRef(constants::network_flow::INDICATOR_VPN_PROXY_TUNNEL),
        IndicatorTextRef(constants::network_flow::INDICATOR_LABEL_VPN_PROXY_TUNNEL),
        observation,
    );
}

fn push_high_volume_indicator(
    observations: &[ActivityNetworkFlowObservation],
    top_destinations: &[ActivityNetworkFlowRollup],
    indicators: &mut Vec<ActivityNetworkFlowIndicator>,
) {
    if observations.len() < 10
        || indicators
            .iter()
            .any(|indicator| indicator.kind == constants::network_flow::INDICATOR_HIGH_VOLUME)
    {
        return;
    }
    let observed_at = observations
        .first()
        .map(|observation| observation.observed_at.clone())
        .unwrap_or_default();
    let evidence_ids = top_destinations
        .iter()
        .flat_map(|rollup| rollup.evidence_ids.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    indicators.push(ActivityNetworkFlowIndicator {
        kind: constants::network_flow::INDICATOR_HIGH_VOLUME.to_string(),
        label: constants::network_flow::INDICATOR_LABEL_HIGH_VOLUME.to_string(),
        observed_at,
        evidence_ids,
    });
}

fn push_once(
    indicators: &mut Vec<ActivityNetworkFlowIndicator>,
    condition: bool,
    kind: IndicatorTextRef<'_>,
    indicator_text: IndicatorTextRef<'_>,
    observation: &ActivityNetworkFlowObservation,
) {
    if !condition || indicators.iter().any(|indicator| indicator.kind == kind.0) {
        return;
    }
    indicators.push(ActivityNetworkFlowIndicator {
        kind: kind.0.to_string(),
        label: indicator_text.0.to_string(),
        observed_at: observation.observed_at.clone(),
        evidence_ids: evidence_ids(observation).0,
    });
}

fn is_failure_tcp_state(value: Option<TcpStateTextRef<'_>>) -> bool {
    matches!(
        value.map(|value| value.0),
        Some(constants::activity_capture::TCP_STATE_CLOSE_WAIT)
            | Some(constants::activity_capture::TCP_STATE_CLOSED)
            | Some(constants::activity_capture::TCP_STATE_CLOSING)
            | Some(constants::activity_capture::TCP_STATE_LAST_ACK)
            | Some(constants::activity_capture::TCP_STATE_SYN_RECEIVED)
            | Some(constants::activity_capture::TCP_STATE_SYN_SENT)
            | Some(constants::activity_capture::TCP_STATE_TIME_WAIT)
    )
}

fn evidence_ids(observation: &ActivityNetworkFlowObservation) -> EvidenceIdList {
    EvidenceIdList(
        observation
            .evidence
            .iter()
            .map(|evidence| evidence.evidence_id.clone())
            .collect(),
    )
}
