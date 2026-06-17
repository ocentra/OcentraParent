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
const SNI_EXTENSION_TYPE: u16 = 0;
const HOST_NAME_TYPE: u8 = 0;

pub fn parse_tls_client_hello_sni(
    record: &[u8],
) -> Result<TlsClientHelloVisibility, TlsClientHelloError> {
    let handshake = tls_handshake_payload(record)?;
    let extensions = client_hello_extensions(handshake)?;
    let sni = parse_sni_extension(extensions)?;
    Ok(TlsClientHelloVisibility {
        sni,
        exact_url_available: false,
        decrypted_payload_available: false,
    })
}

fn tls_handshake_payload(record: &[u8]) -> Result<&[u8], TlsClientHelloError> {
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

fn client_hello_extensions(handshake: &[u8]) -> Result<&[u8], TlsClientHelloError> {
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

fn parse_sni_extension(extensions: &[u8]) -> Result<Option<String>, TlsClientHelloError> {
    let mut offset = 0_usize;
    while offset < extensions.len() {
        if extensions.len() < offset + 4 {
            return Err(TlsClientHelloError::ExtensionTruncated);
        }

        let extension_type = u16::from_be_bytes([extensions[offset], extensions[offset + 1]]);
        let extension_len = usize::from(u16::from_be_bytes([
            extensions[offset + 2],
            extensions[offset + 3],
        ]));
        let data_start = offset + 4;
        let data_end = data_start + extension_len;
        let data = extensions
            .get(data_start..data_end)
            .ok_or(TlsClientHelloError::ExtensionTruncated)?;
        if extension_type == SNI_EXTENSION_TYPE {
            return parse_sni_extension_data(data);
        }
        offset = data_end;
    }

    Ok(None)
}

fn parse_sni_extension_data(data: &[u8]) -> Result<Option<String>, TlsClientHelloError> {
    let list_len = read_len(data, 0)?;
    let mut offset = 2_usize;
    let list_end = offset + list_len;
    if data.len() < list_end {
        return Err(TlsClientHelloError::ExtensionTruncated);
    }

    while offset < list_end {
        if data.len() < offset + 3 {
            return Err(TlsClientHelloError::ExtensionTruncated);
        }
        let name_type = data[offset];
        let name_len = read_len(data, offset + 1)?;
        let name_start = offset + 3;
        let name_end = name_start + name_len;
        let name = data
            .get(name_start..name_end)
            .ok_or(TlsClientHelloError::ExtensionTruncated)?;
        if name_type == HOST_NAME_TYPE {
            return Ok(Some(
                std::str::from_utf8(name)
                    .map_err(|_error| TlsClientHelloError::SniInvalidUtf8)?
                    .to_ascii_lowercase(),
            ));
        }
        offset = name_end;
    }

    Ok(None)
}

fn read_len(bytes: &[u8], offset: usize) -> Result<usize, TlsClientHelloError> {
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
