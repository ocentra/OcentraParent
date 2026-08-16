use std::net::Ipv4Addr;

pub(super) fn is_test_net_remote_address(value: &str) -> bool {
    let Ok(address) = value.parse::<Ipv4Addr>() else {
        return false;
    };
    matches!(
        address.octets(),
        [192, 0, 2, _] | [198, 51, 100, _] | [203, 0, 113, _]
    )
}

pub(super) fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
