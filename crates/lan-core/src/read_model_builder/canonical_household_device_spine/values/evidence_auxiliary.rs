use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceSource, LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind,
    LanDiscoveryEvidenceRecord, LanDiscoveryEvidenceSource,
};

use super::evidence_record::{push_evidence_record, EvidenceRecordInput};
use super::EvidenceContext;

pub(super) fn push_hint_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    hint_sources: &[LanDiscoveryEvidenceSource],
    observed_at: &str,
) {
    if !hint_sources.contains(&LanDiscoveryEvidenceSource::PreviousScanSnapshot) {
        return;
    }
    push_evidence_record(
        records,
        EvidenceRecordInput {
            device,
            source: LanDiscoveryEvidenceSource::PreviousScanSnapshot,
            evidence_kind: LanDiscoveryEvidenceKind::HistoricalIdentityHint,
            value: constants::lan_pairing::LAN_PREVIOUS_SCAN_CONTINUITY_VALUE,
            merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_PREVIOUS_SCAN_PREFIX,
            confidence: LanDiscoveryEvidenceConfidence::Weak,
            observed_at,
            note: Some(constants::lan_pairing::LAN_PREVIOUS_SCAN_CONTINUITY_NOTE.to_string()),
        },
    );
}

pub(super) fn push_trusted_registry_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    source: &LanCanonicalHouseholdDeviceSource,
    observed_at: &str,
) {
    if *source != LanCanonicalHouseholdDeviceSource::TrustedRegistry {
        return;
    }
    push_evidence_record(
        records,
        EvidenceRecordInput {
            device,
            source: LanDiscoveryEvidenceSource::TrustedRegistry,
            evidence_kind: LanDiscoveryEvidenceKind::TrustedRegistry,
            value: &device.device_id,
            merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_TRUSTED_PREFIX,
            confidence: LanDiscoveryEvidenceConfidence::ManualRequired,
            observed_at,
            note: None,
        },
    );
}

pub(super) fn push_router_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    context: &EvidenceContext,
    observed_at: &str,
) {
    if device.platform != constants::lan_pairing::PLATFORM_ROUTER {
        return;
    }
    let router_value = device.ip_address.as_ref().unwrap_or(&device.device_id);
    push_evidence_record(
        records,
        EvidenceRecordInput {
            device,
            source: context.source.clone(),
            evidence_kind: LanDiscoveryEvidenceKind::RouterClassification,
            value: router_value,
            merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_ROUTER_PREFIX,
            confidence: context.confidence.clone(),
            observed_at,
            note: None,
        },
    );
}

pub(super) fn push_fallback_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    observed_at: &str,
) {
    if !records.is_empty() {
        return;
    }
    push_evidence_record(
        records,
        EvidenceRecordInput {
            device,
            source: LanDiscoveryEvidenceSource::ParentAssignment,
            evidence_kind: LanDiscoveryEvidenceKind::ParentDecision,
            value: &device.device_id,
            merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_TRUSTED_PREFIX,
            confidence: LanDiscoveryEvidenceConfidence::ManualRequired,
            observed_at,
            note: None,
        },
    );
}
