#[path = "assessment_reason_direct.rs"]
mod direct;
#[path = "assessment_reason_service.rs"]
mod service;
#[path = "assessment_reason_weak.rs"]
mod weak;

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;

use super::super::assessment::MergeAssessmentContext;
use super::super::assessment_reasons::MergeDecisionReason;

pub(super) fn merge_reasons(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
    context: &MergeAssessmentContext,
) -> Vec<MergeDecisionReason> {
    let mut reasons = Vec::new();
    direct::push(&mut reasons, existing, source_ref, device);
    service::push(&mut reasons, existing, device);
    if context.local_service_identity_overlap && context.authoritative_identity_overlap {
        push_merge_reason(
            &mut reasons,
            MergeDecisionReason::SharedLocalServiceIdentityAnchor,
        );
    }
    weak::push(&mut reasons, existing, source_ref, device);
    reasons
}

pub(super) fn push_merge_reason(
    reasons: &mut Vec<MergeDecisionReason>,
    reason: MergeDecisionReason,
) {
    if !reasons.contains(&reason) {
        reasons.push(reason);
    }
}
