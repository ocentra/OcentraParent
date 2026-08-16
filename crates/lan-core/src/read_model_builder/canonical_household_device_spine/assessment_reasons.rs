#[path = "assessment_reason_evaluation.rs"]
mod evaluation;
#[path = "assessment_reason_evidence.rs"]
mod evidence;
#[path = "assessment_reason_scoring.rs"]
mod scoring;
#[path = "assessment_reason_sources.rs"]
mod sources;

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanDiscoveryEvidenceKind,
};

use super::assessment::MergeAssessmentContext;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MergeDecisionReason {
    SameCanonicalDeviceId,
    SharedInstallId,
    SharedPairingId,
    SharedStableMac,
    SharedMdnsInstanceName,
    SharedSsdpUdn,
    SharedLocalServiceIdentityAnchor,
    SharedIpAddress,
    SharedHostname,
    SharedVendor,
    SharedDeviceType,
    ConflictingOcentraDeviceId,
    ConflictingChildProfileId,
}

pub(super) fn merge_reasons(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
    context: &MergeAssessmentContext,
) -> Vec<MergeDecisionReason> {
    evaluation::merge_reasons(existing, source_ref, device, context)
}

pub(super) fn push_merge_reason(
    reasons: &mut Vec<MergeDecisionReason>,
    reason: MergeDecisionReason,
) {
    evaluation::push_merge_reason(reasons, reason)
}

pub(super) fn merge_score(reasons: &[MergeDecisionReason]) -> u16 {
    scoring::merge_score(reasons)
}

pub(super) fn merge_reason_is_manual_required(reason: MergeDecisionReason) -> bool {
    scoring::is_manual_required(reason)
}

pub(super) fn evidence_kind_overlaps(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
    kinds: &[LanDiscoveryEvidenceKind],
) -> bool {
    evidence::evidence_kind_overlaps(existing, incoming, kinds)
}

pub(super) fn strong_service_hint_overlap(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
    prefixes: &[&str],
) -> bool {
    evidence::strong_service_hint_overlap(existing, incoming, prefixes)
}

pub(super) fn has_agent_or_registry_evidence(device: &LanCanonicalHouseholdDevice) -> bool {
    sources::has_agent_or_registry_evidence(device)
}

pub(super) fn has_local_service_identity_anchor(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    sources::has_local_service_identity_anchor(existing, incoming)
}
