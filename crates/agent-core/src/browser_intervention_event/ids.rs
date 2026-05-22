use ocentra_parent_agent_protocol::constants;

pub(super) fn browser_intervention_id(sequence_index: usize) -> String {
    let mut value = String::from(constants::browser::INTERVENTION_ID_PREFIX);
    value.push_str(&sequence_index.to_string());
    value
}

pub(super) fn browser_intervention_event_id(sequence_index: usize) -> String {
    let mut value = String::from(constants::browser::INTERVENTION_EVENT_ID_PREFIX);
    value.push_str(&sequence_index.to_string());
    value
}

pub(super) fn browser_intervention_subject_id(sequence_index: usize) -> String {
    let mut value = String::from(constants::browser::INTERVENTION_SUBJECT_ID_PREFIX);
    value.push_str(&sequence_index.to_string());
    value
}
