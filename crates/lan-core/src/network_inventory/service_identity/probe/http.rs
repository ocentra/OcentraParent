use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpStream};

use rustls::pki_types::ServerName;
use rustls::{ClientConnection, StreamOwned};

use super::super::http::{
    extract_html_title, first_header_value, header_values, parse_certificate_subject,
    parse_http_response, sanitize_probe_reference, sanitize_probe_text,
};
use super::super::{LanServiceIdentityProbeObservation, SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES};
use super::{read_probe_response, tls_client_config, write_probe_request};

pub(super) fn probe_service_identity_over_http(
    mut stream: TcpStream,
    endpoint: &SocketAddr,
    path: &str,
) -> Option<LanServiceIdentityProbeObservation> {
    write_probe_request(&mut stream, endpoint, path).ok()?;
    let _ = stream.shutdown(Shutdown::Write);
    let response = read_probe_response(&mut stream)?;
    parse_probe_observation(&response, None)
}

pub(super) fn probe_service_identity_over_https(
    stream: TcpStream,
    endpoint: &SocketAddr,
    ip_address: Ipv4Addr,
    path: &str,
) -> Option<LanServiceIdentityProbeObservation> {
    let config = tls_client_config()?;
    let server_name = ServerName::IpAddress(IpAddr::V4(ip_address).into());
    let connection = ClientConnection::new(config, server_name.to_owned()).ok()?;
    let mut stream = StreamOwned::new(connection, stream);
    write_probe_request(&mut stream, endpoint, path).ok()?;
    let response = read_probe_response(&mut stream)?;
    let certificate_subject = stream
        .conn
        .peer_certificates()
        .and_then(|certificates| certificates.first())
        .and_then(parse_certificate_subject);
    parse_probe_observation(&response, certificate_subject)
}

pub(super) fn parse_probe_observation(
    response: &[u8],
    certificate_subject: Option<String>,
) -> Option<LanServiceIdentityProbeObservation> {
    let (status_code, headers, body) = parse_http_response(response)?;
    let title = extract_html_title(body);
    let server_header = first_header_value(&headers, "server");
    let banner = first_header_value(&headers, "x-powered-by")
        .or_else(|| first_header_value(&headers, "www-authenticate"));
    let redirect_location = match status_code {
        300..=399 => first_header_value(&headers, "location").and_then(sanitize_probe_reference),
        _ => None,
    };
    let descriptor_links = header_values(&headers, "link")
        .into_iter()
        .filter_map(|value| sanitize_probe_text(&value, SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES))
        .filter_map(sanitize_probe_reference)
        .collect::<Vec<_>>();
    let observation = LanServiceIdentityProbeObservation {
        status_code: Some(status_code),
        title,
        server_header,
        banner,
        redirect_location,
        certificate_subject,
        descriptor_links,
        wsd_endpoint_address: None,
        wsd_types: None,
        snmp_sys_descr: None,
        snmp_sys_name: None,
    };
    observation.is_meaningful().then_some(observation)
}
