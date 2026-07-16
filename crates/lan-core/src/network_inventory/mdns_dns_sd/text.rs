use ocentra_parent_agent_protocol::constants;

use super::MDNS_MAX_TEXT_BYTES;

pub fn sanitize_mdns_text(value: &str) -> Option<String> {
    let sanitized = value
        .chars()
        .map(|character| {
            if character.is_ascii()
                && !character.is_control()
                && !matches!(character, '<' | '>' | '&' | '"' | '\'' | '`')
            {
                character
            } else {
                ' '
            }
        })
        .collect::<String>();
    let sanitized = sanitized.split_whitespace().collect::<Vec<_>>().join(" ");
    let sanitized = sanitized
        .trim()
        .chars()
        .take(MDNS_MAX_TEXT_BYTES)
        .collect::<String>();

    (!sanitized.is_empty() && sanitized != constants::value::UNKNOWN_HOST).then_some(sanitized)
}

pub fn display_name_from_instance_name(instance_name: &str, service_type: &str) -> Option<String> {
    let suffix = format!(".{}", service_type);
    let candidate = strip_case_insensitive_suffix(instance_name, &suffix).unwrap_or(instance_name);
    sanitize_mdns_text(candidate)
}

pub fn strip_case_insensitive_suffix<'a>(value: &'a str, suffix: &str) -> Option<&'a str> {
    if value.len() < suffix.len() {
        return None;
    }
    let start = value.len() - suffix.len();
    if value[start..].eq_ignore_ascii_case(suffix) {
        Some(&value[..start])
    } else {
        None
    }
}
