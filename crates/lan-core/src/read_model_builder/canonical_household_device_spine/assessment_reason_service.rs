use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;

use super::super::super::assessment_reasons::MergeDecisionReason;
use super::push_merge_reason;

pub(super) fn push(
    reasons: &mut Vec<MergeDecisionReason>,
    existing: &LanCanonicalHouseholdDevice,
    device: &LanCanonicalHouseholdDevice,
) {
    if super::super::evidence::strong_service_hint_overlap(
        existing,
        device,
        &["mdns-instance-name:"],
    ) {
        push_merge_reason(reasons, MergeDecisionReason::SharedMdnsInstanceName);
    }
    if super::super::evidence::strong_service_hint_overlap(existing, device, &["ssdp-udn:"]) {
        push_merge_reason(reasons, MergeDecisionReason::SharedSsdpUdn);
    }
}
