use ocentra_parent_agent_protocol::constants;

use crate::browser_bridge_event::BrowserBridgeTargetObservation;

pub(crate) fn browser_event_id(
    observation: &BrowserBridgeTargetObservation,
    observed_at: &str,
    sequence_index: usize,
) -> String {
    bridge_id(
        constants::browser::EVENT_ID_PREFIX,
        observation,
        observed_at,
        sequence_index,
    )
}

pub(crate) fn browser_evidence_id(
    observation: &BrowserBridgeTargetObservation,
    observed_at: &str,
    sequence_index: usize,
) -> String {
    bridge_id(
        constants::browser::EVIDENCE_ID_PREFIX,
        observation,
        observed_at,
        sequence_index,
    )
}

pub(crate) fn browser_subject_id(domain: &str) -> String {
    let mut subject_id = String::from(constants::browser::SUBJECT_ID_PREFIX);
    subject_id.push_str(domain);
    subject_id
}

pub(crate) fn browser_tab_id(target_id: &str) -> String {
    let mut tab_id = String::from(constants::browser::TAB_ID_PREFIX);
    tab_id.push_str(target_id);
    tab_id
}

fn bridge_id(
    prefix: &str,
    observation: &BrowserBridgeTargetObservation,
    observed_at: &str,
    sequence_index: usize,
) -> String {
    let mut value = String::from(prefix);
    value.push_str(&sequence_index.to_string());
    value.push(constants::delimiter::HYPHEN);
    value.push_str(&observation.target_id);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(observed_at);
    value
}
