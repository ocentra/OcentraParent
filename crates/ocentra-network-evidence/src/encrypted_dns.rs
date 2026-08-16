#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuicVisibilityLimitation {
    pub likely_quic: bool,
    pub exact_domain_available: bool,
    pub decrypted_payload_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedDnsCandidate {
    pub protocol: EncryptedDnsProtocol,
    pub resolver_host: Option<String>,
    pub destination_port: u16,
    pub visited_domain_available: bool,
    pub decrypted_payload_available: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptedDnsProtocol {
    DnsOverTls,
    DnsOverHttps,
}

const DNS_OVER_TLS_PORT: u16 = 853;
const HTTPS_PORT: u16 = 443;
const QUIC_LONG_HEADER_MASK: u8 = 0b1100_0000;
const QUIC_LONG_HEADER_VALUE: u8 = 0b1100_0000;
const DOH_RESOLVER_HOSTS: [&str; 4] = [
    "cloudflare-dns.com",
    "dns.google",
    "dns.quad9.net",
    "mozilla.cloudflare-dns.com",
];

pub fn detect_quic_http3_limitation(udp_payload: &[u8]) -> QuicVisibilityLimitation {
    let likely_quic = udp_payload
        .first()
        .map(|first| first & QUIC_LONG_HEADER_MASK == QUIC_LONG_HEADER_VALUE)
        .unwrap_or(false);

    QuicVisibilityLimitation {
        likely_quic,
        exact_domain_available: false,
        decrypted_payload_available: false,
    }
}

pub fn detect_encrypted_dns_candidate(
    destination_port: u16,
    tls_sni: Option<&str>,
) -> Option<EncryptedDnsCandidate> {
    if destination_port == DNS_OVER_TLS_PORT {
        return Some(candidate(
            EncryptedDnsProtocol::DnsOverTls,
            tls_sni.map(str::to_ascii_lowercase),
            destination_port,
        ));
    }

    let resolver_host = tls_sni?.to_ascii_lowercase();
    if destination_port == HTTPS_PORT && DOH_RESOLVER_HOSTS.contains(&resolver_host.as_str()) {
        return Some(candidate(
            EncryptedDnsProtocol::DnsOverHttps,
            Some(resolver_host),
            destination_port,
        ));
    }

    None
}

fn candidate(
    protocol: EncryptedDnsProtocol,
    resolver_host: Option<String>,
    destination_port: u16,
) -> EncryptedDnsCandidate {
    EncryptedDnsCandidate {
        protocol,
        resolver_host,
        destination_port,
        visited_domain_available: false,
        decrypted_payload_available: false,
    }
}
