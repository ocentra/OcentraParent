pub(super) fn valid_sha256_field(value: &[u8], prefix: &[u8]) -> bool {
    let Some(hex) = value.strip_prefix(prefix) else {
        return false;
    };
    hex.len() == 64
        && hex
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}
