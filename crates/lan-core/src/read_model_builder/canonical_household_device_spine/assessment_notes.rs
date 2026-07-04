use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceConfidence, LanDiscoveryEvidenceKind,
    LanDiscoveryEvidenceRecord,
};

use super::assessment::{MergeAssessment, MergeDecisionState};
use super::assessment_reasons::MergeDecisionReason;

const DEDUPE_DECISION_NOTE_PREFIX: &str = "dedupe-decision=";

pub(super) fn annotate_merge_assessment(
    device: &mut LanCanonicalHouseholdDevice,
    assessment: &MergeAssessment,
) {
    if assessment.state == MergeDecisionState::NoMatch || assessment.reasons.is_empty() {
        return;
    }
    if assessment.state != MergeDecisionState::Automatic {
        device.network_identity.confidence = LanCanonicalHouseholdDeviceConfidence::ManualRequired;
    }
    let note = merge_assessment_note(assessment);
    if !annotate_matching_records(device, assessment, &note) {
        if let Some(record) = device.network_identity.evidence_records.first_mut() {
            append_merge_note(&mut record.note, &note);
        }
    }
}

fn annotate_matching_records(
    device: &mut LanCanonicalHouseholdDevice,
    assessment: &MergeAssessment,
    note: &str,
) -> bool {
    let mut annotated = false;
    for reason in &assessment.reasons {
        for record in device
            .network_identity
            .evidence_records
            .iter_mut()
            .filter(|record| merge_reason_matches_record(*reason, record))
        {
            append_merge_note(&mut record.note, note);
            annotated = true;
        }
    }
    annotated
}

fn merge_assessment_note(assessment: &MergeAssessment) -> String {
    let reasons = assessment
        .reasons
        .iter()
        .map(|reason| merge_reason_label(*reason))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{DEDUPE_DECISION_NOTE_PREFIX}{} score={} reasons={reasons}",
        merge_state_label(assessment.state),
        assessment.score
    )
}

fn merge_state_label(state: MergeDecisionState) -> &'static str {
    match state {
        MergeDecisionState::Automatic => "automatic",
        MergeDecisionState::ManualRequired => "manual-required",
        MergeDecisionState::Forbidden => "forbidden",
        MergeDecisionState::NoMatch => "no-match",
    }
}

fn merge_reason_label(reason: MergeDecisionReason) -> &'static str {
    match reason {
        MergeDecisionReason::SameCanonicalDeviceId => "same-canonical-device-id",
        MergeDecisionReason::SharedInstallId => "shared-install-id",
        MergeDecisionReason::SharedPairingId => "shared-pairing-id",
        MergeDecisionReason::SharedStableMac => "shared-stable-mac",
        MergeDecisionReason::SharedMdnsInstanceName => "shared-mdns-instance-name",
        MergeDecisionReason::SharedSsdpUdn => "shared-ssdp-udn",
        MergeDecisionReason::SharedLocalServiceIdentityAnchor => {
            "shared-local-service-identity-anchor"
        }
        MergeDecisionReason::SharedIpAddress => "shared-ip-address",
        MergeDecisionReason::SharedHostname => "shared-hostname",
        MergeDecisionReason::SharedVendor => "shared-vendor",
        MergeDecisionReason::SharedDeviceType => "shared-device-type",
        MergeDecisionReason::ConflictingOcentraDeviceId => "conflicting-ocentra-device-id",
        MergeDecisionReason::ConflictingChildProfileId => "conflicting-child-profile-id",
    }
}

fn merge_reason_matches_record(
    reason: MergeDecisionReason,
    record: &LanDiscoveryEvidenceRecord,
) -> bool {
    match reason {
        MergeDecisionReason::SameCanonicalDeviceId => matches!(
            record.evidence_kind,
            LanDiscoveryEvidenceKind::ChildAgentPresence
                | LanDiscoveryEvidenceKind::TrustedRegistry
        ),
        MergeDecisionReason::SharedInstallId => {
            record.evidence_kind == LanDiscoveryEvidenceKind::InstallId
        }
        MergeDecisionReason::SharedPairingId => {
            record.evidence_kind == LanDiscoveryEvidenceKind::PairingId
        }
        MergeDecisionReason::SharedStableMac => {
            record.evidence_kind == LanDiscoveryEvidenceKind::MacAddress
        }
        MergeDecisionReason::SharedMdnsInstanceName => {
            service_probe_prefix_match(record, "mdns-instance-name:")
        }
        MergeDecisionReason::SharedSsdpUdn => service_probe_prefix_match(record, "ssdp-udn:"),
        MergeDecisionReason::SharedLocalServiceIdentityAnchor
        | MergeDecisionReason::SharedIpAddress => {
            record.evidence_kind == LanDiscoveryEvidenceKind::IpAddress
        }
        MergeDecisionReason::SharedHostname => {
            record.evidence_kind == LanDiscoveryEvidenceKind::Hostname
        }
        MergeDecisionReason::SharedVendor => {
            record.evidence_kind == LanDiscoveryEvidenceKind::Vendor
        }
        MergeDecisionReason::SharedDeviceType => {
            service_probe_prefix_match(record, "mdns-service-type:")
                || service_probe_prefix_match(record, "ssdp-device-type:")
        }
        MergeDecisionReason::ConflictingOcentraDeviceId
        | MergeDecisionReason::ConflictingChildProfileId => false,
    }
}

fn service_probe_prefix_match(record: &LanDiscoveryEvidenceRecord, prefix: &str) -> bool {
    record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint
        && record
            .value
            .get(..prefix.len())
            .is_some_and(|value| value.eq_ignore_ascii_case(prefix))
}

fn append_merge_note(note: &mut Option<String>, merge_note: &str) {
    match note {
        Some(existing) => {
            if !existing.contains(merge_note) {
                existing.push_str(" | ");
                existing.push_str(merge_note);
            }
        }
        None => *note = Some(merge_note.to_string()),
    }
}
