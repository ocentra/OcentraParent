use ocentra_parent_agent_protocol::{
    constants, ActivityNetworkFlowIndicator, ActivityNetworkFlowIndicatorKind,
    ActivityNetworkFlowObservation,
};

pub(crate) fn evidence_ids(observation: &ActivityNetworkFlowObservation) -> Vec<String> {
    observation
        .evidence
        .iter()
        .map(|evidence| evidence.evidence_id.clone())
        .collect()
}

pub(crate) fn indicator(
    kind: ActivityNetworkFlowIndicatorKind,
    label: &str,
    observed_at: &str,
    evidence_ids: Vec<String>,
) -> ActivityNetworkFlowIndicator {
    ActivityNetworkFlowIndicator {
        kind,
        label: label.to_string(),
        observed_at: observed_at.to_string(),
        evidence_ids,
    }
}

pub(crate) fn is_proxy_like(observation: &ActivityNetworkFlowObservation) -> bool {
    let Some(port) = observation.destination_endpoint.port else {
        return false;
    };
    matches!(port, 1080 | 3128 | 8080 | 9050)
}

pub(crate) fn destination_key(observation: &ActivityNetworkFlowObservation) -> String {
    if let Some(domain) = &observation.destination_domain {
        return domain.clone();
    }
    if let Some(endpoint) = observation.destination_endpoint.ip.as_ref() {
        if let Some(port) = observation.destination_endpoint.port {
            let mut text = endpoint.clone();
            text.push(constants::delimiter::COLON);
            text.push_str(&port.to_string());
            return text;
        }
    }
    constants::network_flow::LABEL_DESTINATION_UNKNOWN.to_string()
}
