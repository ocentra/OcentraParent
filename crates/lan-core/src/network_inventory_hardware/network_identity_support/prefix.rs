use super::address;

pub(super) fn normalized_ipv6_prefix(value: &str) -> Option<String> {
    let (address, prefix_length) = value.trim().split_once('/')?;
    let prefix_length = prefix_length.parse::<u8>().ok()?;
    let valid = prefix_length <= 128 && address::supported_local_ipv6_text(address);
    valid.then(|| format!("{address}/{prefix_length}"))
}
