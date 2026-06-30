use super::text::compact_summary;
use super::super::ssdp_upnp::parse_ssdp_response;

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
    let text = std::str::from_utf8(payload).ok()?;
    let mut lines = text.lines();
    let first_line = lines.next()?.trim_end_matches('\r').trim();
    if !first_line.eq_ignore_ascii_case("NOTIFY * HTTP/1.1") {
        return None;
    }
    let mut headers = PassiveSsdpNotifyHeaders::default();
    for line in lines {
        let line = line.trim_end_matches('\r');
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        headers.apply_header(name, value.trim());
    }
    Some(headers)
}

#[derive(Clone, Debug, Default)]
pub struct PassiveSsdpNotifyHeaders {
    notification_type: Option<String>,
    notification_subtype: Option<String>,
    usn: Option<String>,
    location: Option<String>,
}

impl PassiveSsdpNotifyHeaders {
    fn apply_header(&mut self, name: &str, value: &str) {
        match name.trim().to_ascii_lowercase().as_str() {
            "nt" => self.notification_type = Some(value.to_string()),
            "nts" => self.notification_subtype = Some(value.to_string()),
            "usn" => self.usn = Some(value.to_string()),
            "location" => self.location = Some(value.to_string()),
            _ => {}
        }
    }

    fn summary(self) -> String {
        let mut summary = String::from("SSDP notify");
        append_ssdp_summary_part(&mut summary, "nt", self.notification_type);
        append_ssdp_summary_part(&mut summary, "nts", self.notification_subtype);
        append_ssdp_summary_part(&mut summary, "usn", self.usn);
        append_ssdp_summary_part(&mut summary, "location", self.location);
        summary
    }
}

fn append_ssdp_summary_part(summary: &mut String, name: &str, value: Option<String>) {
    if let Some(value) = value {
        if summary == "SSDP notify" {
            summary.push(':');
        } else {
            summary.push(';');
        }
        summary.push(' ');
        summary.push_str(name);
        summary.push('=');
        summary.push_str(&value);
    }
}
