use ocentra_parent_agent_protocol::constants;

use super::MAX_NAME_EVIDENCE_BYTES;

pub(super) fn normalize_name_evidence_value(value: &str) -> Option<String> {
    let candidate = value.trim();
    let candidate = candidate.strip_suffix('.').unwrap_or(candidate);
    let valid = !candidate.is_empty()
        && candidate != constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME
        && candidate.len() <= MAX_NAME_EVIDENCE_BYTES
        && candidate.is_ascii()
        && !candidate.split('.').any(invalid_hostname_label);
    valid.then(|| candidate.to_string())
}

pub(super) fn invalid_hostname_label(label: &str) -> bool {
    label.is_empty()
        || label.len() > 63
        || label.starts_with('-')
        || label.ends_with('-')
        || !label
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-')
}
