use super::text::compact_summary;
use super::xml::first_xml_text_by_local_name;

pub fn passive_ws_discovery_summary(payload: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(payload).ok()?;
    let has_discovery_namespace = text.contains("schemas.xmlsoap.org/ws/2005/04/discovery")
        || text.contains("docs.oasis-open.org/ws-dd/ns/discovery");
    if !has_discovery_namespace {
        return None;
    }

    let mut parts = Vec::new();
    push_xml_summary_part(&mut parts, text, "Action", "action");
    push_xml_summary_part(&mut parts, text, "Address", "endpoint");
    push_xml_summary_part(&mut parts, text, "Types", "types");
    push_xml_summary_part(&mut parts, text, "XAddrs", "xaddrs");

    if parts.is_empty() {
        return Some(String::from("WS-Discovery packet"));
    }
    Some(compact_summary(format!(
        "WS-Discovery packet: {}",
        parts.join("; ")
    )))
}

pub fn passive_ws_discovery_device_id(payload: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(payload).ok()?;
    let endpoint = first_xml_text_by_local_name(text, "Address")?;
    let endpoint = endpoint
        .trim()
        .strip_prefix("urn:uuid:")
        .or_else(|| endpoint.trim().strip_prefix("uuid:"))
        .unwrap_or(endpoint.trim());
    let endpoint = endpoint.trim();
    (!endpoint.is_empty()).then(|| endpoint.to_string())
}

fn push_xml_summary_part(parts: &mut Vec<String>, text: &str, local_name: &str, label: &str) {
    if let Some(value) = first_xml_text_by_local_name(text, local_name) {
        parts.push(format!("{label}={value}"));
    }
}
