use std::collections::BTreeSet;

use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityNetworkFlowIndicator,
    ActivityNetworkFlowIndicatorKind, ActivityNetworkFlowObservation, ActivityNetworkFlowRollup,
    ActivityNetworkProtocol, ActivityNetworkTcpState, ActivityProcessAttributionStatus,
};

use crate::{
    network_flow_indicator_helpers::{destination_key, evidence_ids, indicator, is_proxy_like},
    time::timestamp_now,
};

const NETWORK_FLOW_INDICATOR_LIMIT: usize = 5;

pub(crate) fn network_indicators(
    observations: &[ActivityNetworkFlowObservation],
    top_destinations: &[ActivityNetworkFlowRollup],
) -> Vec<ActivityNetworkFlowIndicator> {
    let mut indicators = Vec::new();
    let mut seen_destination_keys = BTreeSet::<String>::new();

    for observation in observations {
        if indicators.len() >= NETWORK_FLOW_INDICATOR_LIMIT {
            break;
        }

        push_adapter_indicator(observation, &mut indicators);
        push_unknown_process_indicator(observation, &mut indicators);
        push_encrypted_content_indicator(observation, &mut indicators);
        push_repeated_failure_indicator(observation, &mut indicators);
        push_vpn_proxy_indicator(observation, &mut indicators);

        let destination_key = destination_key(observation);
        if seen_destination_keys.insert(destination_key) {
            push_new_destination_indicator(observation, &mut indicators);
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
    if observation.capability_status == ActivityCaptureCapabilityStatus::Available
        || indicators
            .iter()
            .any(|indicator| indicator.kind == ActivityNetworkFlowIndicatorKind::AdapterUnavailable)
    {
        return;
    }
    indicators.push(indicator(
        ActivityNetworkFlowIndicatorKind::AdapterUnavailable,
        constants::network_flow::INDICATOR_LABEL_ADAPTER_UNAVAILABLE,
        &observation.observed_at,
        evidence_ids(observation),
    ));
}

fn push_unknown_process_indicator(
    observation: &ActivityNetworkFlowObservation,
    indicators: &mut Vec<ActivityNetworkFlowIndicator>,
) {
    if observation.process_attribution_status != ActivityProcessAttributionStatus::ProcessUnknown
        || indicators.iter().any(|indicator| {
            indicator.kind == ActivityNetworkFlowIndicatorKind::UnusualUnknownProcess
        })
    {
        return;
    }
    indicators.push(indicator(
        ActivityNetworkFlowIndicatorKind::UnusualUnknownProcess,
        constants::network_flow::INDICATOR_LABEL_UNUSUAL_UNKNOWN_PROCESS,
        &observation.observed_at,
        evidence_ids(observation),
    ));
}

fn push_encrypted_content_indicator(
    observation: &ActivityNetworkFlowObservation,
    indicators: &mut Vec<ActivityNetworkFlowIndicator>,
) {
    if observation.protocol != Some(ActivityNetworkProtocol::Tcp)
        || observation.destination_domain.is_some()
        || indicators.iter().any(|indicator| {
            indicator.kind == ActivityNetworkFlowIndicatorKind::EncryptedContentUnavailable
        })
    {
        return;
    }
    indicators.push(indicator(
        ActivityNetworkFlowIndicatorKind::EncryptedContentUnavailable,
        constants::network_flow::INDICATOR_LABEL_ENCRYPTED_CONTENT_UNAVAILABLE,
        &observation.observed_at,
        evidence_ids(observation),
    ));
}

fn push_repeated_failure_indicator(
    observation: &ActivityNetworkFlowObservation,
    indicators: &mut Vec<ActivityNetworkFlowIndicator>,
) {
    if observation.protocol != Some(ActivityNetworkProtocol::Tcp)
        || !matches!(
            observation.tcp_state,
            Some(ActivityNetworkTcpState::CloseWait)
                | Some(ActivityNetworkTcpState::Closed)
                | Some(ActivityNetworkTcpState::TimeWait)
                | Some(ActivityNetworkTcpState::Closing)
                | Some(ActivityNetworkTcpState::LastAck)
                | Some(ActivityNetworkTcpState::SynSent)
                | Some(ActivityNetworkTcpState::SynReceived)
        )
        || indicators
            .iter()
            .any(|indicator| indicator.kind == ActivityNetworkFlowIndicatorKind::RepeatedFailure)
    {
        return;
    }
    indicators.push(indicator(
        ActivityNetworkFlowIndicatorKind::RepeatedFailure,
        constants::network_flow::INDICATOR_LABEL_REPEATED_FAILURE,
        &observation.observed_at,
        evidence_ids(observation),
    ));
}

fn push_vpn_proxy_indicator(
    observation: &ActivityNetworkFlowObservation,
    indicators: &mut Vec<ActivityNetworkFlowIndicator>,
) {
    if !is_proxy_like(observation)
        || indicators
            .iter()
            .any(|indicator| indicator.kind == ActivityNetworkFlowIndicatorKind::VpnProxyTunnel)
    {
        return;
    }
    indicators.push(indicator(
        ActivityNetworkFlowIndicatorKind::VpnProxyTunnel,
        constants::network_flow::INDICATOR_LABEL_VPN_PROXY_TUNNEL,
        &observation.observed_at,
        evidence_ids(observation),
    ));
}

fn push_new_destination_indicator(
    observation: &ActivityNetworkFlowObservation,
    indicators: &mut Vec<ActivityNetworkFlowIndicator>,
) {
    if indicators
        .iter()
        .any(|indicator| indicator.kind == ActivityNetworkFlowIndicatorKind::NewDestination)
    {
        return;
    }
    indicators.push(indicator(
        ActivityNetworkFlowIndicatorKind::NewDestination,
        constants::network_flow::INDICATOR_LABEL_NEW_DESTINATION,
        &observation.observed_at,
        evidence_ids(observation),
    ));
}

fn push_high_volume_indicator(
    observations: &[ActivityNetworkFlowObservation],
    top_destinations: &[ActivityNetworkFlowRollup],
    indicators: &mut Vec<ActivityNetworkFlowIndicator>,
) {
    if observations.len() < 10
        || indicators
            .iter()
            .any(|indicator| indicator.kind == ActivityNetworkFlowIndicatorKind::HighVolume)
    {
        return;
    }
    let observed_at = observations
        .first()
        .map(|observation| observation.observed_at.clone())
        .unwrap_or_else(timestamp_now);
    let evidence_ids = top_destinations
        .iter()
        .flat_map(|rollup| rollup.evidence_ids.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    indicators.push(indicator(
        ActivityNetworkFlowIndicatorKind::HighVolume,
        constants::network_flow::INDICATOR_LABEL_HIGH_VOLUME,
        &observed_at,
        evidence_ids,
    ));
}
