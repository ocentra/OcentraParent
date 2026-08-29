use ocentra_parent_agent_protocol::constants;
use rustls::pki_types::CertificateDer;
use x509_parser::prelude::parse_x509_certificate;

use super::{HttpResponseParts, SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES};

mod xml;

pub fn parse_http_response(response: &[u8]) -> Option<HttpResponseParts<'_>> {
    let header_end = response
        .windows(4)
        .position(|window| window == b"\r\n\r\n")?;
    let header_bytes = &response[..header_end];
    let body = &response[(header_end + 4)..];
    let mut lines = header_bytes.split(|byte| *byte == b'\n');
    let status_line = std::str::from_utf8(lines.next()?)
        .ok()?
        .trim_end_matches('\r');
    let status_code = status_line.split_whitespace().nth(1)?.parse::<u16>().ok()?;
    let mut headers = Vec::new();

    for line in lines {
        let line = std::str::from_utf8(line).ok()?.trim_end_matches('\r');
        let (name, value) = line.split_once(':')?;
        headers.push((name.trim().to_ascii_lowercase(), value.trim().to_string()));
    }

    Some((status_code, headers, body))
}

pub fn header_values(headers: &[(String, String)], name: &str) -> Vec<String> {
    headers
        .iter()
        .filter(|(header_name, _)| header_name.eq_ignore_ascii_case(name))
        .map(|(_, value)| value.clone())
        .collect()
}

pub fn first_header_value(headers: &[(String, String)], name: &str) -> Option<String> {
    header_values(headers, name)
        .into_iter()
        .find_map(|value| sanitize_probe_text(&value, SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES))
}

pub fn extract_html_title(body: &[u8]) -> Option<String> {
    let body = String::from_utf8_lossy(body);
    let lower = body.to_ascii_lowercase();
    let title_start = lower.find("<title")?;
    let tag_end = body[title_start..].find('>')? + title_start + 1;
    let title_end = lower[tag_end..].find("</title>")? + tag_end;
    let title = xml::strip_xml_tags(body[tag_end..title_end].trim());
    sanitize_probe_text(&title, SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES)
}

pub fn parse_certificate_subject(certificate_der: &CertificateDer<'_>) -> Option<String> {
    let (_, certificate) = parse_x509_certificate(certificate_der.as_ref()).ok()?;
    sanitize_probe_text(
        &certificate.subject().to_string(),
        SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES,
    )
}

pub fn first_xml_text_by_local_name(text: &str, local_name: &str) -> Option<String> {
    xml::first_xml_text_by_local_name(text, local_name)
}

pub fn sanitize_probe_text(value: &str, max_length: usize) -> Option<String> {
    let sanitized = value
        .chars()
        .map(|character| {
            if character.is_ascii() && !character.is_control() {
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
        .take(max_length)
        .collect::<String>();

    (!sanitized.is_empty() && sanitized != constants::value::UNKNOWN_HOST).then_some(sanitized)
}

pub fn sanitize_probe_reference(value: impl AsRef<str>) -> Option<String> {
    let normalized = value.as_ref().replace('\\', "/");
    if normalized.split('/').any(|segment| segment == "..") {
        return None;
    }
    Some(normalized)
}
