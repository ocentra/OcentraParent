use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanDiscoveryEvidenceKind,
};

use super::super::super::assessment_reasons::MergeDecisionReason;
use super::push_merge_reason;

pub(super) fn push(
    reasons: &mut Vec<MergeDecisionReason>,
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
) {
    if shared_ip_overlap(existing, source_ref, device) {
        push_merge_reason(reasons, MergeDecisionReason::SharedIpAddress);
    }
    if shared_hostname_overlap(existing, device) {
        push_merge_reason(reasons, MergeDecisionReason::SharedHostname);
    }
    if super::super::evidence::evidence_kind_overlaps(
        existing,
        device,
        &[LanDiscoveryEvidenceKind::Vendor],
    ) {
        push_merge_reason(reasons, MergeDecisionReason::SharedVendor);
    }
    if super::super::evidence::strong_service_hint_overlap(
        existing,
        device,
        &["mdns-service-type:", "ssdp-device-type:"],
    ) {
        push_merge_reason(reasons, MergeDecisionReason::SharedDeviceType);
    }
}

fn shared_ip_overlap(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    incoming
        .network_identity
        .ip_addresses
        .iter()
        .any(|ip| existing.network_identity.ip_addresses.contains(ip))
        || source_ref
            .ip_address
            .as_ref()
            .is_some_and(|ip| existing.network_identity.ip_addresses.contains(ip))
}

fn shared_hostname_overlap(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    existing
        .network_identity
        .hostname
        .as_ref()
        .zip(incoming.network_identity.hostname.as_ref())
        .is_some_and(|(left, right)| left.eq_ignore_ascii_case(right))
}
