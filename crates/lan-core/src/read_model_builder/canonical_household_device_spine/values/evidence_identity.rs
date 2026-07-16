use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord,
};

use super::evidence_record::{push_optional_evidence_with_confidence, OptionalEvidenceInput};
use super::EvidenceContext;

const LAN_EVIDENCE_KEY_INSTALL_ID_PREFIX: &str = "install:";
const LAN_EVIDENCE_KEY_PAIRING_ID_PREFIX: &str = "pairing:";

pub(super) fn push_strong_identity_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    pairing_id: Option<&str>,
    context: &EvidenceContext,
    observed_at: &str,
) {
    push_optional_evidence_with_confidence(
        records,
        OptionalEvidenceInput {
            device,
            source: context.source.clone(),
            evidence_kind: LanDiscoveryEvidenceKind::InstallId,
            value: device.install_id.as_deref(),
            merge_key_prefix: LAN_EVIDENCE_KEY_INSTALL_ID_PREFIX,
            confidence: strong_identity_confidence(&context.source),
            observed_at,
        },
    );
    push_optional_evidence_with_confidence(
        records,
        OptionalEvidenceInput {
            device,
            source: context.source.clone(),
            evidence_kind: LanDiscoveryEvidenceKind::PairingId,
            value: pairing_id,
            merge_key_prefix: LAN_EVIDENCE_KEY_PAIRING_ID_PREFIX,
            confidence: strong_identity_confidence(&context.source),
            observed_at,
        },
    );
}

fn strong_identity_confidence(
    source: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource,
) -> LanDiscoveryEvidenceConfidence {
    match source {
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::LocalService => {
            LanDiscoveryEvidenceConfidence::Confirmed
        }
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::TrustedRegistry => {
            LanDiscoveryEvidenceConfidence::Strong
        }
        _ => LanDiscoveryEvidenceConfidence::Strong,
    }
}
