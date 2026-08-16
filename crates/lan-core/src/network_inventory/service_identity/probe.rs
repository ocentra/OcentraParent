use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use std::sync::Arc;

use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{ClientConfig, DigitallySignedStruct, SignatureScheme};

use super::{
    AllowedSnmpResponseObserver, LanServiceIdentityProbeObservation, ProbeTarget,
    ServiceIdentityProbeSettings,
};

mod http;
mod identity;
mod tls;
mod transport;

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
    settings: ServiceIdentityProbeSettings,
    deadline: std::time::Instant,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
) -> Option<LanServiceIdentityProbeObservation> {
    identity::probe_service_identity(
        ip_address,
        device_id,
        targets,
        settings,
        deadline,
        allowed_snmp_response_observer,
    )
}

pub fn probe_service_identity_on_target(
    ip_address: &str,
    target: ProbeTarget,
) -> Option<LanServiceIdentityProbeObservation> {
    identity::probe_service_identity_on_target(ip_address, target)
}

pub fn probe_service_identity_over_http(
    stream: TcpStream,
    endpoint: &SocketAddr,
    path: &str,
) -> Option<LanServiceIdentityProbeObservation> {
    http::probe_service_identity_over_http(stream, endpoint, path)
}

pub fn probe_service_identity_over_https(
    stream: TcpStream,
    endpoint: &SocketAddr,
    ip_address: Ipv4Addr,
    path: &str,
) -> Option<LanServiceIdentityProbeObservation> {
    http::probe_service_identity_over_https(stream, endpoint, ip_address, path)
}

pub fn tls_client_config() -> Option<Arc<ClientConfig>> {
    tls::tls_client_config()
}

pub fn write_probe_request<W: Write>(
    stream: &mut W,
    endpoint: &SocketAddr,
    path: &str,
) -> std::io::Result<()> {
    transport::write_probe_request(stream, endpoint, path)
}

pub fn read_probe_response<R: Read>(stream: &mut R) -> Option<Vec<u8>> {
    transport::read_probe_response(stream)
}

pub fn parse_probe_observation(
    response: &[u8],
    certificate_subject: Option<String>,
) -> Option<LanServiceIdentityProbeObservation> {
    http::parse_probe_observation(response, certificate_subject)
}
