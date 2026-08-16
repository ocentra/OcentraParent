use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord,
};

use super::super::super::assessment_reasons::MergeDecisionReason;

pub(super) fn matches(reason: MergeDecisionReason, record: &LanDiscoveryEvidenceRecord) -> bool {
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
        _ => false,
    }
}
