use crate::app_game_platform_proof_status::APP_GAME_WINDOWS_LOCAL_POLICY_EVIDENCE_REF_PREFIX;

const UTC_TIMESTAMP_LEN: usize = 20;
const UTC_SEPARATORS: [(usize, u8); 6] = [
    (4, b'-'),
    (7, b'-'),
    (10, b'T'),
    (13, b':'),
    (16, b':'),
    (19, b'Z'),
];
const SHA_256_HEX_LEN: usize = 64;

pub(super) fn is_canonical_utc_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == UTC_TIMESTAMP_LEN
        && UTC_SEPARATORS
            .iter()
            .all(|(index, expected)| bytes[*index] == *expected)
        && bytes
            .iter()
            .enumerate()
            .filter(|(index, _)| !is_separator_index(*index))
            .all(|(_, byte)| byte.is_ascii_digit())
}

pub(super) fn is_opaque_local_policy_reference(value: &str) -> bool {
    value
        .strip_prefix(APP_GAME_WINDOWS_LOCAL_POLICY_EVIDENCE_REF_PREFIX)
        .is_some_and(is_lower_hex_digest)
}

fn is_separator_index(index: usize) -> bool {
    UTC_SEPARATORS
        .iter()
        .any(|(separator_index, _)| *separator_index == index)
}

fn is_lower_hex_digest(value: &str) -> bool {
    value.len() == SHA_256_HEX_LEN
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
