use std::io::Write;
use std::net::{Ipv4Addr, Shutdown, SocketAddr, TcpStream};
use std::time::Duration;

use super::http::{first_xml_text_by_local_name, sanitize_probe_text};
use super::probe::read_probe_response;
use super::{
    LanServiceIdentityProbeObservation, SERVICE_IDENTITY_PROBE_CONNECT_TIMEOUT_MS,
    SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES, SERVICE_IDENTITY_PROBE_READ_TIMEOUT_MS,
};

pub fn probe_wsd_identity_query(
    ip_address: &str,
    device_id: Option<&str>,
) -> Option<LanServiceIdentityProbeObservation> {
    let ip_address = ip_address.parse::<Ipv4Addr>().ok()?;
    let endpoint = SocketAddr::new(ip_address.into(), 5357);
    probe_wsd_identity_query_at_endpoint(endpoint, device_id)
}

pub fn probe_wsd_identity_query_at_endpoint(
    endpoint: SocketAddr,
    device_id: Option<&str>,
) -> Option<LanServiceIdentityProbeObservation> {
    let device_id = sanitize_wsd_device_id(device_id)?;
    let timeout = Duration::from_millis(SERVICE_IDENTITY_PROBE_CONNECT_TIMEOUT_MS);
    let mut stream = TcpStream::connect_timeout(&endpoint, timeout).ok()?;
    let read_timeout = Some(Duration::from_millis(
        SERVICE_IDENTITY_PROBE_READ_TIMEOUT_MS,
    ));
    let _ = stream.set_read_timeout(read_timeout);
    let _ = stream.set_write_timeout(read_timeout);
    write_wsd_metadata_request(&mut stream, &endpoint, &device_id).ok()?;
    let _ = stream.shutdown(Shutdown::Write);
    let response = read_probe_response(&mut stream)?;
    parse_wsd_probe_observation(&response)
}

pub fn sanitize_wsd_device_id(device_id: Option<&str>) -> Option<String> {
    let device_id = device_id?.trim();
    if device_id.is_empty()
        || device_id.len() > SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES
        || !device_id
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-')
    {
        return None;
    }
    Some(device_id.to_string())
}

pub fn write_wsd_metadata_request<W: Write>(
    stream: &mut W,
    endpoint: &SocketAddr,
    device_id: &str,
) -> std::io::Result<()> {
    let path = format!("/{device_id}");
    let body = format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\
<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\" \
xmlns:a=\"http://schemas.xmlsoap.org/ws/2004/08/addressing\" \
xmlns:w=\"http://schemas.xmlsoap.org/ws/2004/09/transfer\">\
<s:Header>\
<a:Action s:mustUnderstand=\"1\">http://schemas.xmlsoap.org/ws/2004/09/transfer/Get</a:Action>\
<a:MessageID>urn:uuid:00000000-0000-0000-0000-000000000000</a:MessageID>\
<a:ReplyTo><a:Address>http://schemas.xmlsoap.org/ws/2004/08/addressing/role/anonymous</a:Address></a:ReplyTo>\
<a:To s:mustUnderstand=\"1\">http://{}{}</a:To>\
</s:Header>\
<s:Body/>\
</s:Envelope>",
        endpoint, path,
    );
    let request = format!(
        "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/soap+xml; charset=utf-8; action=\"http://schemas.xmlsoap.org/ws/2004/09/transfer/Get\"\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        path,
        endpoint,
        body.len(),
        body,
    );
    stream.write_all(request.as_bytes())
}

pub fn parse_wsd_probe_observation(response: &[u8]) -> Option<LanServiceIdentityProbeObservation> {
    let (status_code, _, body) = super::http::parse_http_response(response)?;
    let text = std::str::from_utf8(body).ok()?;
    let wsd_endpoint_address = first_xml_text_by_local_name(text, "Address")
        .and_then(|value| sanitize_probe_text(&value, SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES));
    let wsd_types = first_xml_text_by_local_name(text, "Types")
        .and_then(|value| sanitize_probe_text(&value, SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES));
    let observation = LanServiceIdentityProbeObservation {
        status_code: Some(status_code),
        title: None,
        server_header: None,
        banner: None,
        redirect_location: None,
        certificate_subject: None,
        descriptor_links: Vec::new(),
        wsd_endpoint_address,
        wsd_types,
        snmp_sys_descr: None,
        snmp_sys_name: None,
    };
    observation.is_meaningful().then_some(observation)
}
