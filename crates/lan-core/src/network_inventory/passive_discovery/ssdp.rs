use super::super::ssdp_upnp::parse_ssdp_response;
use super::text::compact_summary;

mod notify;

pub fn passive_ssdp_device_id(payload: &[u8]) -> Option<String> {
    if let Ok(response) = parse_ssdp_response(payload) {
        return response
            .udn
            .or_else(|| passive_ssdp_extract_udn(&response.usn))
            .or(Some(response.usn));
    }
    passive_ssdp_notify_headers(payload)?
        .usn
        .as_deref()
        .and_then(passive_ssdp_extract_udn)
        .or_else(|| passive_ssdp_notify_headers(payload).and_then(|headers| headers.usn))
}

pub fn passive_ssdp_summary(payload: &[u8]) -> Option<String> {
    if let Ok(response) = parse_ssdp_response(payload) {
        return Some(compact_summary(format!(
            "SSDP response: st={}, usn={}, location={}",
            response.search_target, response.usn, response.location
        )));
    }
    let headers = passive_ssdp_notify_headers(payload)?;
    if headers.notification_type.is_none() && headers.usn.is_none() {
        return None;
    }
    Some(compact_summary(headers.summary()))
}

pub fn passive_ssdp_extract_udn(value: &str) -> Option<String> {
    let trimmed = value.trim();
    let trimmed = trimmed
        .strip_prefix("uuid:")
        .or_else(|| trimmed.strip_prefix("urn:uuid:"))
        .unwrap_or(trimmed);
    let trimmed = trimmed.split("::").next().unwrap_or(trimmed).trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

pub fn passive_ssdp_notify_headers(payload: &[u8]) -> Option<PassiveSsdpNotifyHeaders> {
    notify::parse_notify_headers(payload)
}

#[derive(Clone, Debug, Default)]
pub struct PassiveSsdpNotifyHeaders {
    notification_type: Option<String>,
    notification_subtype: Option<String>,
    usn: Option<String>,
    location: Option<String>,
}
