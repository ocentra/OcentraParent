use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpStream};
use std::sync::Arc;
use std::time::Duration;

use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{ClientConfig, ClientConnection, DigitallySignedStruct, SignatureScheme, StreamOwned};

use super::http::{
    extract_html_title, first_header_value, header_values, parse_certificate_subject,
    parse_http_response, sanitize_probe_reference, sanitize_probe_text,
};
use super::snmp::probe_snmp_identity_query;
use super::wsd::probe_wsd_identity_query;
use super::{
    AllowedSnmpResponseObserver, LanServiceIdentityProbeObservation, ProbeTarget, ProbeTransport,
    SERVICE_IDENTITY_PROBE_CONNECT_TIMEOUT_MS, SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES,
    SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES, SERVICE_IDENTITY_PROBE_READ_TIMEOUT_MS,
};

#[derive(Debug)]
pub struct AcceptAnyServerCertVerifier;

impl ServerCertVerifier for AcceptAnyServerCertVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::RSA_PKCS1_SHA1,
            SignatureScheme::ECDSA_SHA1_Legacy,
            SignatureScheme::ED25519,
            SignatureScheme::ED448,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::ECDSA_NISTP521_SHA512,
            SignatureScheme::RSA_PSS_SHA256,
            SignatureScheme::RSA_PSS_SHA384,
            SignatureScheme::RSA_PSS_SHA512,
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::ML_DSA_44,
            SignatureScheme::ML_DSA_65,
            SignatureScheme::ML_DSA_87,
        ]
    }
}

pub fn probe_service_identity(
    ip_address: &str,
    device_id: Option<&str>,
    targets: &[ProbeTarget],
    settings: super::ServiceIdentityProbeSettings,
    deadline: std::time::Instant,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
) -> Option<LanServiceIdentityProbeObservation> {
    for target in targets {
        if std::time::Instant::now() >= deadline {
            return None;
        }
        if let Some(probe_match) = probe_service_identity_on_target(ip_address, *target) {
            return Some(probe_match);
        }
    }
    if settings.allow_wsd_identity_query && std::time::Instant::now() < deadline {
        if let Some(probe_match) = probe_wsd_identity_query(ip_address, device_id) {
            return Some(probe_match);
        }
    }
    if settings.allow_snmp_identity_query && std::time::Instant::now() < deadline {
        return probe_snmp_identity_query(ip_address, allowed_snmp_response_observer);
    }
    None
}

pub fn probe_service_identity_on_target(
    ip_address: &str,
    target: ProbeTarget,
) -> Option<LanServiceIdentityProbeObservation> {
    let ip_address = ip_address.parse::<Ipv4Addr>().ok()?;
    let endpoint = SocketAddr::new(ip_address.into(), target.port);

    for path in target.request_paths {
        let timeout = Duration::from_millis(SERVICE_IDENTITY_PROBE_CONNECT_TIMEOUT_MS);
        let stream = TcpStream::connect_timeout(&endpoint, timeout).ok()?;
        let read_timeout = Some(Duration::from_millis(
            SERVICE_IDENTITY_PROBE_READ_TIMEOUT_MS,
        ));
        let _ = stream.set_read_timeout(read_timeout);
        let _ = stream.set_write_timeout(read_timeout);

        match target.transport {
            ProbeTransport::Http => {
                if let Some(probe_match) = probe_service_identity_over_http(stream, &endpoint, path)
                {
                    return Some(probe_match);
                }
            }
            ProbeTransport::Https => {
                if let Some(probe_match) =
                    probe_service_identity_over_https(stream, &endpoint, ip_address, path)
                {
                    return Some(probe_match);
                }
            }
        }
    }

    None
}

pub fn probe_service_identity_over_http(
    mut stream: TcpStream,
    endpoint: &SocketAddr,
    path: &str,
) -> Option<LanServiceIdentityProbeObservation> {
    write_probe_request(&mut stream, endpoint, path).ok()?;
    let _ = stream.shutdown(Shutdown::Write);
    let response = read_probe_response(&mut stream)?;
    parse_probe_observation(&response, None)
}

pub fn probe_service_identity_over_https(
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

pub fn tls_client_config() -> Option<Arc<ClientConfig>> {
    static TLS_CLIENT_CONFIG: std::sync::OnceLock<Option<Arc<ClientConfig>>> =
        std::sync::OnceLock::new();

    TLS_CLIENT_CONFIG
        .get_or_init(|| {
            let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
            let config = ClientConfig::builder()
                .dangerous()
                .with_custom_certificate_verifier(Arc::new(AcceptAnyServerCertVerifier))
                .with_no_client_auth();
            Some(Arc::new(config))
        })
        .clone()
}

pub fn write_probe_request<W: Write>(
    stream: &mut W,
    endpoint: &SocketAddr,
    path: &str,
) -> std::io::Result<()> {
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nAccept: text/html, application/json;q=0.9, */*;q=0.1\r\nConnection: close\r\n\r\n",
        path,
        endpoint
    );
    stream.write_all(request.as_bytes())
}

pub fn read_probe_response<R: Read>(stream: &mut R) -> Option<Vec<u8>> {
    let mut response = Vec::new();
    let mut chunk = [0_u8; 1024];

    loop {
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(read) => {
                if response.len().saturating_add(read) > SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES {
                    return None;
                }
                response.extend_from_slice(&chunk[..read]);
            }
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut =>
            {
                break;
            }
            Err(_) => return None,
        }
    }

    (!response.is_empty()).then_some(response)
}

pub fn parse_probe_observation(
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
