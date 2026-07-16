use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord,
    LanDiscoveryEvidenceSource,
};

use super::evidence_record::{push_evidence_record, EvidenceRecordInput};
use crate::network_inventory::api::{is_confirmed_agent_status, is_service_identity_probe_status};

pub(super) fn push_agent_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    observed_at: &str,
) {
    if let Some(agent_status) = device.agent_status.as_ref() {
        let (source, confidence) = if is_confirmed_agent_status(Some(agent_status.as_str())) {
            (
                LanDiscoveryEvidenceSource::LocalService,
                LanDiscoveryEvidenceConfidence::Confirmed,
            )
        } else if is_service_identity_probe_status(Some(agent_status.as_str())) {
            (
                LanDiscoveryEvidenceSource::ServiceIdentityProbe,
                LanDiscoveryEvidenceConfidence::Weak,
            )
        } else {
            return;
        };
        push_evidence_record(
            records,
            EvidenceRecordInput {
                device,
                source,
                evidence_kind: LanDiscoveryEvidenceKind::ChildAgentPresence,
                value: agent_status,
                merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_AGENT_PREFIX,
                confidence,
                observed_at,
                note: None,
            },
        );
    }
}
