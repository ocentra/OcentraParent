use super::{read_len, TlsClientHelloError};

const SNI_EXTENSION_TYPE: u16 = 0;
const HOST_NAME_TYPE: u8 = 0;

pub(super) fn parse_sni_extension(
    extensions: &[u8],
) -> Result<Option<String>, TlsClientHelloError> {
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
