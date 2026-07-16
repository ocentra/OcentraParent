use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanDiscoveryEvidenceKind,
};

use super::super::super::assessment_reasons::MergeDecisionReason;
use super::super::super::values::option_overlaps;
use super::push_merge_reason;

pub(super) fn push(
    reasons: &mut Vec<MergeDecisionReason>,
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
) {
    if existing.canonical_device_id == device.canonical_device_id {
        push_merge_reason(reasons, MergeDecisionReason::SameCanonicalDeviceId);
    }
    if option_overlaps(
        existing.network_identity.mac_address.as_ref(),
        device.network_identity.mac_address.as_ref(),
    ) || option_overlaps(
        existing.network_identity.mac_address.as_ref(),
        source_ref.mac_address.as_ref(),
    ) {
        push_merge_reason(reasons, MergeDecisionReason::SharedStableMac);
    }
    if super::super::evidence::evidence_kind_overlaps(
        existing,
        device,
        &[LanDiscoveryEvidenceKind::InstallId],
    ) {
        push_merge_reason(reasons, MergeDecisionReason::SharedInstallId);
    }
    if super::super::evidence::evidence_kind_overlaps(
        existing,
        device,
        &[LanDiscoveryEvidenceKind::PairingId],
    ) {
        push_merge_reason(reasons, MergeDecisionReason::SharedPairingId);
    }
}
