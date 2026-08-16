#[path = "handshake.rs"]
mod handshake;
#[path = "sni.rs"]
mod sni;

use super::{
    TlsClientHelloError, CLIENT_HELLO_FIXED_LEN, CLIENT_HELLO_HANDSHAKE, TLS_HANDSHAKE_HEADER_LEN,
    TLS_HANDSHAKE_RECORD, TLS_RECORD_HEADER_LEN,
};

pub(super) fn tls_handshake_payload(record: &[u8]) -> Result<&[u8], TlsClientHelloError> {
    handshake::tls_handshake_payload(record)
}

pub(super) fn client_hello_extensions(handshake: &[u8]) -> Result<&[u8], TlsClientHelloError> {
    handshake::client_hello_extensions(handshake)
}

pub(super) fn parse_sni_extension(
    extensions: &[u8],
) -> Result<Option<String>, TlsClientHelloError> {
    sni::parse_sni_extension(extensions)
}

pub(super) fn read_len(bytes: &[u8], offset: usize) -> Result<usize, TlsClientHelloError> {
    if bytes.len() < offset + 2 {
        return Err(TlsClientHelloError::ClientHelloTruncated);
    }

    Ok(usize::from(u16::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
    ])))
}

fn read_u24(bytes: &[u8]) -> usize {
    (usize::from(bytes[0]) << 16) | (usize::from(bytes[1]) << 8) | usize::from(bytes[2])
}
