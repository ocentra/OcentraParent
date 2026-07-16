use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord,
};

use super::evidence_record::{push_evidence_record, EvidenceRecordInput};
use super::EvidenceContext;
use crate::mac_identity::{LanMacIdentityAssessment, LanMacIdentityDisposition};

pub(super) fn push_vendor_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    context: &EvidenceContext,
    observed_at: &str,
    mac_assessment: Option<&LanMacIdentityAssessment>,
) {
    let Some(mac_assessment) = mac_assessment else {
        return;
    };
    let note = mac_assessment.vendor_evidence_note().map(str::to_string);
    let confidence = vendor_evidence_confidence(mac_assessment);
    let vendor_value = mac_assessment.vendor_evidence_value();
    push_evidence_record(
        records,
        EvidenceRecordInput {
            device,
            source: context.source.clone(),
            evidence_kind: LanDiscoveryEvidenceKind::Vendor,
            value: &vendor_value,
            merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_VENDOR_PREFIX,
            confidence,
            observed_at,
            note,
        },
    );
}

fn vendor_evidence_confidence(
    mac_assessment: &LanMacIdentityAssessment,
) -> LanDiscoveryEvidenceConfidence {
    match mac_assessment.disposition() {
        LanMacIdentityDisposition::KnownVendor => LanDiscoveryEvidenceConfidence::Strong,
        LanMacIdentityDisposition::UnknownVendor => LanDiscoveryEvidenceConfidence::Weak,
        LanMacIdentityDisposition::LocallyAdministered => {
            LanDiscoveryEvidenceConfidence::ManualRequired
        }
        LanMacIdentityDisposition::RejectedMulticast
        | LanMacIdentityDisposition::RejectedMalformed => LanDiscoveryEvidenceConfidence::Rejected,
    }
}
