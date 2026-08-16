pub(super) fn decode_netbios_name(encoded_name: &str) -> Option<String> {
    let encoded = encoded_name.split('.').next().unwrap_or(encoded_name);
    if encoded.len() != 32 {
        return None;
    }

    let mut bytes = Vec::with_capacity(16);
    let mut chars = encoded.bytes();
    while let (Some(high), Some(low)) = (chars.next(), chars.next()) {
        if !(b'A'..=b'P').contains(&high) || !(b'A'..=b'P').contains(&low) {
            return None;
        }
        bytes.push(((high - b'A') << 4) | (low - b'A'));
    }
    let name_bytes = bytes.get(..15)?;
    let decoded = String::from_utf8_lossy(name_bytes)
        .trim_end()
        .trim_matches(char::from(0))
        .to_string();
    (!decoded.is_empty()).then_some(decoded)
}
