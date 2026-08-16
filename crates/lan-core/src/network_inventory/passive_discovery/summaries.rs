use super::LanPassiveDiscoverySource;

mod device_id;
mod summary;

pub fn passive_native_datagram_summary(
    source: LanPassiveDiscoverySource,
    payload: &[u8],
) -> Option<String> {
    summary::passive_native_datagram_summary(source, payload)
}

pub fn passive_native_datagram_device_id(
    source: LanPassiveDiscoverySource,
    payload: &[u8],
) -> Option<String> {
    device_id::passive_native_datagram_device_id(source, payload)
}
