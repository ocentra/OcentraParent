const BASE64URL: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

pub(crate) fn encode_base64url(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len().div_ceil(3) * 4);
    let mut index = 0;
    while index < bytes.len() {
        let remaining = bytes.len() - index;
        let first = bytes[index];
        let second = if remaining > 1 { bytes[index + 1] } else { 0 };
        let third = if remaining > 2 { bytes[index + 2] } else { 0 };
        output.push(BASE64URL[(first >> 2) as usize] as char);
        output.push(BASE64URL[((first & 0x03) << 4 | second >> 4) as usize] as char);
        if remaining > 1 {
            output.push(BASE64URL[((second & 0x0f) << 2 | third >> 6) as usize] as char);
        }
        if remaining > 2 {
            output.push(BASE64URL[(third & 0x3f) as usize] as char);
        }
        index += 3;
    }
    output
}

pub(crate) fn is_canonical_base64url(value: &str) -> bool {
    decode_base64url(value).is_some()
}

fn decode_base64url(value: &str) -> Option<Vec<u8>> {
    if value.is_empty() || value.contains('=') || value.len() % 4 == 1 {
        return None;
    }
    let mut output = Vec::with_capacity(value.len() * 3 / 4);
    let mut accumulator = 0_u32;
    let mut bits = 0_u8;
    for byte in value.bytes() {
        let digit = BASE64URL.iter().position(|candidate| *candidate == byte)? as u32;
        accumulator = (accumulator << 6) | digit;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            output.push((accumulator >> bits) as u8);
            accumulator &= (1_u32 << bits).saturating_sub(1);
        }
    }
    (bits == 0 || accumulator == 0).then_some(output)
}
