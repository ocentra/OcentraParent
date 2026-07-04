use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdDeviceSource, LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind,
    LanDiscoveryEvidenceRecord,
};

use super::assessment::MergeAssessmentContext;
use super::values::option_overlaps;

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
    let mut reasons = Vec::new();
    push_direct_identity_reasons(&mut reasons, existing, source_ref, device);
    push_service_hint_reasons(&mut reasons, existing, device);
    if context.local_service_identity_overlap && context.authoritative_identity_overlap {
        push_merge_reason(
            &mut reasons,
            MergeDecisionReason::SharedLocalServiceIdentityAnchor,
        );
    }
    push_weak_identity_reasons(&mut reasons, existing, source_ref, device);
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

pub(super) fn merge_score(reasons: &[MergeDecisionReason]) -> u16 {
    reasons
        .iter()
        .map(|reason| merge_reason_score(*reason))
        .sum()
}

pub(super) fn merge_reason_is_manual_required(reason: MergeDecisionReason) -> bool {
    matches!(
        reason,
        MergeDecisionReason::SharedIpAddress
            | MergeDecisionReason::SharedHostname
            | MergeDecisionReason::SharedVendor
            | MergeDecisionReason::SharedDeviceType
    )
}

pub(super) fn evidence_kind_overlaps(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
    kinds: &[LanDiscoveryEvidenceKind],
) -> bool {
    super::values::evidence_kind_overlaps(existing, incoming, kinds)
}

pub(super) fn strong_service_hint_overlap(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
    prefixes: &[&str],
) -> bool {
    existing
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| strong_service_hint_record(record, prefixes))
        .any(|existing_record| {
            incoming
                .network_identity
                .evidence_records
                .iter()
                .filter(|record| strong_service_hint_record(record, prefixes))
                .any(|incoming_record| {
                    existing_record
                        .normalized_value
                        .eq_ignore_ascii_case(&incoming_record.normalized_value)
                })
        })
}

pub(super) fn has_agent_or_registry_evidence(device: &LanCanonicalHouseholdDevice) -> bool {
    device.classification == LanCanonicalHouseholdDeviceClassification::ChildAgent
        || device.child_agent_inventory.is_some()
        || device.source_labels.iter().any(|source| {
            matches!(
                source,
                LanCanonicalHouseholdDeviceSource::LocalService
                    | LanCanonicalHouseholdDeviceSource::TrustedRegistry
            )
        })
}

pub(super) fn has_local_service_identity_anchor(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    existing
        .source_labels
        .iter()
        .chain(incoming.source_labels.iter())
        .any(|source| matches!(source, LanCanonicalHouseholdDeviceSource::LocalService))
}

fn push_direct_identity_reasons(
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
    if evidence_kind_overlaps(existing, device, &[LanDiscoveryEvidenceKind::InstallId]) {
        push_merge_reason(reasons, MergeDecisionReason::SharedInstallId);
    }
    if evidence_kind_overlaps(existing, device, &[LanDiscoveryEvidenceKind::PairingId]) {
        push_merge_reason(reasons, MergeDecisionReason::SharedPairingId);
    }
}

fn push_service_hint_reasons(
    reasons: &mut Vec<MergeDecisionReason>,
    existing: &LanCanonicalHouseholdDevice,
    device: &LanCanonicalHouseholdDevice,
) {
    if strong_service_hint_overlap(existing, device, &["mdns-instance-name:"]) {
        push_merge_reason(reasons, MergeDecisionReason::SharedMdnsInstanceName);
    }
    if strong_service_hint_overlap(existing, device, &["ssdp-udn:"]) {
        push_merge_reason(reasons, MergeDecisionReason::SharedSsdpUdn);
    }
}

fn push_weak_identity_reasons(
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
    if evidence_overlap_by_kind(existing, device, &LanDiscoveryEvidenceKind::Vendor) {
        push_merge_reason(reasons, MergeDecisionReason::SharedVendor);
    }
    if strong_service_hint_overlap(
        existing,
        device,
        &["mdns-service-type:", "ssdp-device-type:"],
    ) {
        push_merge_reason(reasons, MergeDecisionReason::SharedDeviceType);
    }
}

fn merge_reason_score(reason: MergeDecisionReason) -> u16 {
    match reason {
        MergeDecisionReason::SameCanonicalDeviceId => 120,
        MergeDecisionReason::SharedInstallId => 110,
        MergeDecisionReason::SharedPairingId => 110,
        MergeDecisionReason::SharedStableMac => 100,
        MergeDecisionReason::SharedMdnsInstanceName => 95,
        MergeDecisionReason::SharedSsdpUdn => 95,
        MergeDecisionReason::SharedLocalServiceIdentityAnchor => 90,
        MergeDecisionReason::SharedIpAddress => 25,
        MergeDecisionReason::SharedHostname => 20,
        MergeDecisionReason::SharedVendor => 10,
        MergeDecisionReason::SharedDeviceType => 12,
        MergeDecisionReason::ConflictingOcentraDeviceId
        | MergeDecisionReason::ConflictingChildProfileId => 0,
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

fn evidence_overlap_by_kind(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
    evidence_kind: &LanDiscoveryEvidenceKind,
) -> bool {
    existing
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| record.evidence_kind == *evidence_kind)
        .any(|existing_record| {
            incoming
                .network_identity
                .evidence_records
                .iter()
                .filter(|record| record.evidence_kind == *evidence_kind)
                .any(|incoming_record| {
                    existing_record
                        .normalized_value
                        .eq_ignore_ascii_case(&incoming_record.normalized_value)
                })
        })
}

fn strong_service_hint_record(record: &LanDiscoveryEvidenceRecord, prefixes: &[&str]) -> bool {
    record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint
        && record.confidence == LanDiscoveryEvidenceConfidence::Strong
        && prefixes.iter().any(|prefix| {
            record
                .value
                .get(..prefix.len())
                .is_some_and(|value| value.eq_ignore_ascii_case(prefix))
        })
}
