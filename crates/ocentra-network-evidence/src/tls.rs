mod parsing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlsClientHelloVisibility {
    pub sni: Option<String>,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TlsClientHelloError {
    RecordTooShort,
    NotHandshakeRecord,
    RecordTruncated,
    HandshakeTooShort,
    NotClientHello,
    ClientHelloTruncated,
    ExtensionTruncated,
    SniInvalidUtf8,
}

const TLS_HANDSHAKE_RECORD: u8 = 22;
const CLIENT_HELLO_HANDSHAKE: u8 = 1;
const TLS_RECORD_HEADER_LEN: usize = 5;
const TLS_HANDSHAKE_HEADER_LEN: usize = 4;
const CLIENT_HELLO_FIXED_LEN: usize = 34;

pub fn parse_tls_client_hello_sni(
    record: &[u8],
) -> Result<TlsClientHelloVisibility, TlsClientHelloError> {
    let handshake = parsing::tls_handshake_payload(record)?;
    let extensions = parsing::client_hello_extensions(handshake)?;
    let sni = parsing::parse_sni_extension(extensions)?;
    Ok(TlsClientHelloVisibility {
        sni,
        exact_url_available: false,
        decrypted_payload_available: false,
    })
}
