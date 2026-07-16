use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::encrypted_dns::*;
use ocentra_network_evidence::fixtures::visibility::*;
use ocentra_network_evidence::http::*;
use ocentra_network_evidence::tls::*;

#[test]
fn tls_parser_extracts_visible_sni_without_content_claims() {
    let visibility = parse_tls_client_hello_sni(&tls_client_hello_sni_fixture())
        .expect_value("client hello should parse");

    assert_eq!(visibility.sni, Some("video.example.test".to_owned()));
    assert!(!visibility.exact_url_available);
    assert!(!visibility.decrypted_payload_available);
}

#[test]
fn tls_parser_keeps_hidden_sni_as_no_claim() {
    let visibility = parse_tls_client_hello_sni(&tls_client_hello_no_sni_fixture())
        .expect_value("client hello without sni should parse");

    assert_eq!(visibility.sni, None);
    assert!(!visibility.exact_url_available);
    assert!(!visibility.decrypted_payload_available);
}

#[test]
fn http_host_parser_extracts_plain_host_without_exact_url_claim() {
    let observation = parse_http_host(&http_host_request_fixture())
        .expect_value("http payload should parse")
        .expect_value("host should be visible");

    assert_eq!(observation.host, "video.example.test");
    assert!(!observation.exact_url_available);
    assert!(!observation.decrypted_payload_available);
}

#[test]
fn http_host_parser_does_not_claim_https_payload() {
    let observation = parse_http_host(&tls_client_hello_sni_fixture())
        .expect_value("non-http payload is no claim");

    assert_eq!(observation, None);
}

#[test]
fn quic_limitation_detector_marks_domain_visibility_unavailable() {
    let limitation = detect_quic_http3_limitation(&quic_initial_payload_fixture());

    assert!(limitation.likely_quic);
    assert!(!limitation.exact_domain_available);
    assert!(!limitation.decrypted_payload_available);
}

#[test]
fn encrypted_dns_detector_flags_dot_without_visited_domain_claim() {
    let candidate = detect_encrypted_dns_candidate(853, Some("dns.quad9.net"))
        .expect_value("dot port should be a candidate");

    assert_eq!(candidate.protocol, EncryptedDnsProtocol::DnsOverTls);
    assert_eq!(candidate.resolver_host, Some("dns.quad9.net".to_owned()));
    assert!(!candidate.visited_domain_available);
    assert!(!candidate.decrypted_payload_available);
}

#[test]
fn encrypted_dns_detector_flags_doh_resolver_without_visited_domain_claim() {
    let candidate = detect_encrypted_dns_candidate(443, Some("DNS.Google"))
        .expect_value("known doh resolver should be a candidate");

    assert_eq!(candidate.protocol, EncryptedDnsProtocol::DnsOverHttps);
    assert_eq!(candidate.resolver_host, Some("dns.google".to_owned()));
    assert!(!candidate.visited_domain_available);
    assert!(!candidate.decrypted_payload_available);
}
