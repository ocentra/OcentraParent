use super::super::labels::compact_identifier;
use super::super::text::compact_summary;
use super::{parse_passive_dhcp_observation, PassiveDhcpObservation};

pub(super) fn passive_dhcp_summary(payload: &[u8]) -> Option<String> {
    let observation = parse_passive_dhcp_observation(payload)?;
    let parts = summary_parts(observation);
    (!parts.is_empty()).then(|| compact_summary(format!("DHCP packet: {}", parts.join("; "))))
}

pub(super) fn passive_dhcp_device_id(payload: &[u8]) -> Option<String> {
    let observation = parse_passive_dhcp_observation(payload)?;
    observation.client_mac.or_else(|| {
        observation
            .client_id
            .map(|value| compact_identifier(&value))
            .filter(|value| !value.is_empty())
    })
}

fn summary_parts(observation: PassiveDhcpObservation) -> Vec<String> {
    let mut parts = Vec::new();
    append_part(&mut parts, "type", observation.message_type);
    append_part(&mut parts, "client-mac", observation.client_mac);
    append_part(&mut parts, "client-id", observation.client_id);
    append_part(&mut parts, "hostname", observation.hostname);
    append_part(&mut parts, "vendor-class", observation.vendor_class);
    append_part(
        &mut parts,
        "params",
        observation.parameter_request_fingerprint,
    );
    parts
}

fn append_part(parts: &mut Vec<String>, name: &str, value: Option<String>) {
    if let Some(value) = value {
        parts.push(format!("{name}={value}"));
    }
}
