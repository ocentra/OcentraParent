use std::io::Write;
use std::net::{Ipv4Addr, Shutdown, SocketAddr, TcpStream};
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use std::time::Instant;

use super::http::{first_xml_text_by_local_name, sanitize_probe_text};
use super::probe::transport::{
    poll_timeout, read_probe_response_until, write_all_until, SERVICE_IDENTITY_IO_POLL_SLICE,
};
use super::{
    LanServiceIdentityProbeObservation, SERVICE_IDENTITY_PROBE_CONNECT_TIMEOUT_MS,
    SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES,
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
    let deadline =
        Instant::now() + Duration::from_millis(SERVICE_IDENTITY_PROBE_CONNECT_TIMEOUT_MS);
    probe_wsd_identity_query_at_endpoint_until(endpoint, device_id, deadline, None)
}

pub(super) fn probe_wsd_identity_query_until(
    ip_address: &str,
    device_id: Option<&str>,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<LanServiceIdentityProbeObservation> {
    let ip_address = ip_address.parse::<Ipv4Addr>().ok()?;
    let endpoint = SocketAddr::new(ip_address.into(), 5357);
    probe_wsd_identity_query_at_endpoint_until(endpoint, device_id, deadline, cancellation)
}

fn probe_wsd_identity_query_at_endpoint_until(
    endpoint: SocketAddr,
    device_id: Option<&str>,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<LanServiceIdentityProbeObservation> {
    let device_id = sanitize_wsd_device_id(device_id)?;
    let mut stream = connect_until(endpoint, deadline, cancellation)?;
    stream
        .set_read_timeout(Some(SERVICE_IDENTITY_IO_POLL_SLICE))
        .ok()?;
    stream
        .set_write_timeout(Some(SERVICE_IDENTITY_IO_POLL_SLICE))
        .ok()?;
    let request = wsd_metadata_request(&endpoint, &device_id);
    write_all_until(&mut stream, request.as_bytes(), deadline, cancellation)?;
    let _ = stream.shutdown(Shutdown::Write);
    let response = read_probe_response_until(&mut stream, deadline, cancellation)?;
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
    stream.write_all(wsd_metadata_request(endpoint, device_id).as_bytes())
}

fn wsd_metadata_request(endpoint: &SocketAddr, device_id: &str) -> String {
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
    request
}

fn connect_until(
    endpoint: SocketAddr,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<TcpStream> {
    loop {
        match TcpStream::connect_timeout(&endpoint, poll_timeout(deadline, cancellation)?) {
            Ok(stream) => return Some(stream),
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(_) => return None,
        }
    }
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
