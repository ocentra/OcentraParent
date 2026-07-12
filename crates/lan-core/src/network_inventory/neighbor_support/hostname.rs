use ocentra_parent_agent_protocol::constants;

use super::MAX_NEIGHBOR_HOSTNAME_BYTES;

pub(super) fn normalize_neighbor_hostname(value: &str) -> Option<String> {
    let candidate = value.trim().trim_end_matches('.');
    let valid = !candidate.is_empty()
        && candidate != constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME
        && candidate.len() <= MAX_NEIGHBOR_HOSTNAME_BYTES
        && candidate.is_ascii()
        && !candidate.split('.').any(invalid_hostname_label);
    valid.then(|| candidate.to_string())
}

fn invalid_hostname_label(label: &str) -> bool {
    label.is_empty()
        || label.len() > 63
        || label.starts_with('-')
        || label.ends_with('-')
        || !label
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-')
}
