use super::{
    read_len, read_u24, TlsClientHelloError, CLIENT_HELLO_FIXED_LEN, CLIENT_HELLO_HANDSHAKE,
    TLS_HANDSHAKE_HEADER_LEN, TLS_HANDSHAKE_RECORD, TLS_RECORD_HEADER_LEN,
};

pub(super) fn tls_handshake_payload(record: &[u8]) -> Result<&[u8], TlsClientHelloError> {
    if record.len() < TLS_RECORD_HEADER_LEN {
        return Err(TlsClientHelloError::RecordTooShort);
    }
    if record[0] != TLS_HANDSHAKE_RECORD {
        return Err(TlsClientHelloError::NotHandshakeRecord);
    }

    let record_len = usize::from(u16::from_be_bytes([record[3], record[4]]));
    let record_end = TLS_RECORD_HEADER_LEN + record_len;
    if record.len() < record_end {
        return Err(TlsClientHelloError::RecordTruncated);
    }

    let handshake = &record[TLS_RECORD_HEADER_LEN..record_end];
    if handshake.len() < TLS_HANDSHAKE_HEADER_LEN {
        return Err(TlsClientHelloError::HandshakeTooShort);
    }
    if handshake[0] != CLIENT_HELLO_HANDSHAKE {
        return Err(TlsClientHelloError::NotClientHello);
    }

    let handshake_len = read_u24(&handshake[1..4]);
    let handshake_end = TLS_HANDSHAKE_HEADER_LEN + handshake_len;
    if handshake.len() < handshake_end {
        return Err(TlsClientHelloError::HandshakeTooShort);
    }

    Ok(&handshake[TLS_HANDSHAKE_HEADER_LEN..handshake_end])
}

pub(super) fn client_hello_extensions(handshake: &[u8]) -> Result<&[u8], TlsClientHelloError> {
    if handshake.len() < CLIENT_HELLO_FIXED_LEN {
        return Err(TlsClientHelloError::ClientHelloTruncated);
    }

    let session_id_len = usize::from(handshake[CLIENT_HELLO_FIXED_LEN]);
    let cipher_len_offset = CLIENT_HELLO_FIXED_LEN + 1 + session_id_len;
    let cipher_len = read_len(handshake, cipher_len_offset)?;
    let compression_len_offset = cipher_len_offset + 2 + cipher_len;
    let compression_len = usize::from(
        *handshake
            .get(compression_len_offset)
            .ok_or(TlsClientHelloError::ClientHelloTruncated)?,
    );
    let extensions_len_offset = compression_len_offset + 1 + compression_len;
    let extensions_len = read_len(handshake, extensions_len_offset)?;
    let extensions_start = extensions_len_offset + 2;
    let extensions_end = extensions_start + extensions_len;
    handshake
        .get(extensions_start..extensions_end)
        .ok_or(TlsClientHelloError::ClientHelloTruncated)
}
