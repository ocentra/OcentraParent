use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::http::parse_http_host;

const HTTP_REQUEST_WITH_HOST: &[u8] = b"GET /watch HTTP/1.1\r\nHost: Video.Example.TEST\r\n\r\n";
const HTTP_REQUEST_WITHOUT_HOST: &[u8] = b"GET /watch HTTP/1.1\r\nUser-Agent: Ocentra\r\n\r\n";
const HTTP_RESPONSE_WITH_HOST_LIKE_HEADER: &[u8] =
    b"HTTP/1.1 200 OK\r\nHost: video.example.test\r\n\r\n";

#[test]
fn http_host_parser_extracts_lowercase_host_without_url_or_payload_claims() {
    let observation = parse_http_host(HTTP_REQUEST_WITH_HOST)
        .expect_value("valid utf8 should parse")
        .expect_value("host should be observed");

    assert_eq!(observation.host, "video.example.test");
    assert!(!observation.exact_url_available);
    assert!(!observation.decrypted_payload_available);
}

#[test]
fn http_host_parser_ignores_non_request_or_missing_host_payloads() {
    assert_eq!(
        parse_http_host(HTTP_REQUEST_WITHOUT_HOST).expect_value("valid utf8 should parse"),
        None
    );
    assert_eq!(
        parse_http_host(HTTP_RESPONSE_WITH_HOST_LIKE_HEADER)
            .expect_value("valid utf8 should parse"),
        None
    );
}

#[test]
fn http_host_parser_ignores_invalid_utf8_without_claiming_evidence() {
    assert_eq!(
        parse_http_host(&[0xff, 0xfe]).expect_value("invalid utf8 is ignored"),
        None
    );
}
