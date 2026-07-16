use std::collections::HashMap;

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanDiscoveryEvidenceKind,
};

use super::assessment_reasons::{
    evidence_kind_overlaps, merge_reason_is_manual_required, merge_reasons, merge_score,
    push_merge_reason, strong_service_hint_overlap, MergeDecisionReason,
};
use super::merge::conflicting_source_identity;
use super::values::child_profile::{
    child_profile_device_id, child_profile_identity_from_canonical,
};
use super::values::option_overlaps;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MergeDecisionState {
    Automatic,
    ManualRequired,
    Forbidden,
    NoMatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct MergeAssessment {
    pub(super) state: MergeDecisionState,
    pub(super) score: u16,
    pub(super) reasons: Vec<MergeDecisionReason>,
}

pub(super) fn assess_merge_candidate(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
    assigned_child_profiles: &HashMap<String, String>,
) -> MergeAssessment {
    let context = MergeAssessmentContext::new(existing, source_ref, device);
    let mut reasons = merge_reasons(existing, source_ref, device, &context);
    let score = merge_score(&reasons);
    if conflicting_child_profile_identity(existing, source_ref, device, assigned_child_profiles) {
        push_merge_reason(&mut reasons, MergeDecisionReason::ConflictingChildProfileId);
        return merge_assessment(MergeDecisionState::Forbidden, score, reasons);
    }
    if forbidden_source_conflict(existing, device, &context, &reasons) {
        push_merge_reason(
            &mut reasons,
            MergeDecisionReason::ConflictingOcentraDeviceId,
        );
        return merge_assessment(MergeDecisionState::Forbidden, score, reasons);
    }
    if automatic_match(existing, source_ref, device, &context) {
        return merge_assessment(MergeDecisionState::Automatic, score, reasons);
    }
    if reasons
        .iter()
        .any(|reason| merge_reason_is_manual_required(*reason))
    {
        return merge_assessment(MergeDecisionState::ManualRequired, score, reasons);
    }
    merge_assessment(MergeDecisionState::NoMatch, 0, reasons)
}

pub(super) struct MergeAssessmentContext {
    pub(super) authoritative_identity_overlap: bool,
    pub(super) local_service_identity_overlap: bool,
    pub(super) strong_hint_overlap: bool,
}

impl MergeAssessmentContext {
    fn new(
        existing: &LanCanonicalHouseholdDevice,
        source_ref: &LanPairingDeviceRef,
        device: &LanCanonicalHouseholdDevice,
    ) -> Self {
        let authoritative_identity_overlap =
            authoritative_mac_overlap(existing, source_ref, device)
                || authoritative_ip_overlap(existing, source_ref, device);
        Self {
            authoritative_identity_overlap,
            local_service_identity_overlap: authoritative_identity_overlap
                && super::assessment_reasons::has_local_service_identity_anchor(existing, device),
            strong_hint_overlap: strong_service_hint_overlap(
                existing,
                device,
                &["mdns-instance-name:", "ssdp-udn:"],
            ),
        }
    }
}

fn forbidden_source_conflict(
    existing: &LanCanonicalHouseholdDevice,
    device: &LanCanonicalHouseholdDevice,
    context: &MergeAssessmentContext,
    reasons: &[MergeDecisionReason],
) -> bool {
    let has_non_manual_overlap = reasons
        .iter()
        .any(|reason| !merge_reason_is_manual_required(*reason));
    conflicting_source_identity(existing, device)
        && (context.authoritative_identity_overlap || has_non_manual_overlap)
        && !context.strong_hint_overlap
        && !context.local_service_identity_overlap
}

fn automatic_match(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
    context: &MergeAssessmentContext,
) -> bool {
    existing.canonical_device_id == device.canonical_device_id
        || option_overlaps(
            existing.network_identity.mac_address.as_ref(),
            device.network_identity.mac_address.as_ref(),
        )
        || option_overlaps(
            existing.network_identity.mac_address.as_ref(),
            source_ref.mac_address.as_ref(),
        )
        || strong_evidence_overlap(existing, device)
        || context.strong_hint_overlap
        || context.local_service_identity_overlap
        || context.authoritative_identity_overlap
}

fn strong_evidence_overlap(
    existing: &LanCanonicalHouseholdDevice,
    device: &LanCanonicalHouseholdDevice,
) -> bool {
    evidence_kind_overlaps(
        existing,
        device,
        &[
            LanDiscoveryEvidenceKind::MacAddress,
            LanDiscoveryEvidenceKind::InstallId,
            LanDiscoveryEvidenceKind::PairingId,
            LanDiscoveryEvidenceKind::ChildAgentPresence,
            LanDiscoveryEvidenceKind::TrustedRegistry,
        ],
    )
}

fn merge_assessment(
    state: MergeDecisionState,
    score: u16,
    reasons: Vec<MergeDecisionReason>,
) -> MergeAssessment {
    MergeAssessment {
        state,
        score,
        reasons,
    }
}

fn conflicting_child_profile_identity(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    incoming: &LanCanonicalHouseholdDevice,
    assigned_child_profiles: &HashMap<String, String>,
) -> bool {
    let Some(existing_child_profile_id) =
        assigned_child_profile_identity(existing, assigned_child_profiles).or_else(|| {
            child_profile_identity_from_canonical(&existing.canonical_device_id)
                .map(ToOwned::to_owned)
        })
    else {
        return false;
    };
    let Some(incoming_child_profile_id) =
        assigned_child_profile_identity(incoming, assigned_child_profiles)
            .or_else(|| child_profile_device_id(source_ref))
    else {
        return false;
    };
    existing_child_profile_id != incoming_child_profile_id
}

fn assigned_child_profile_identity(
    device: &LanCanonicalHouseholdDevice,
    assigned_child_profiles: &HashMap<String, String>,
) -> Option<String> {
    assigned_child_profiles
        .get(&device.canonical_device_id)
        .cloned()
}

fn authoritative_mac_overlap(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
) -> bool {
    let mac_matches = option_overlaps(
        existing.network_identity.mac_address.as_ref(),
        device.network_identity.mac_address.as_ref(),
    ) || option_overlaps(
        existing.network_identity.mac_address.as_ref(),
        source_ref.mac_address.as_ref(),
    );
    mac_matches
        && (super::assessment_reasons::has_agent_or_registry_evidence(existing)
            || super::assessment_reasons::has_agent_or_registry_evidence(device))
}

fn authoritative_ip_overlap(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
) -> bool {
    let ip_matches = source_ref
        .ip_address
        .as_ref()
        .is_some_and(|ip| existing.network_identity.ip_addresses.contains(ip));
    ip_matches
        && (super::assessment_reasons::has_agent_or_registry_evidence(existing)
            || super::assessment_reasons::has_agent_or_registry_evidence(device))
}
