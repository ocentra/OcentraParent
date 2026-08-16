use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord,
};

use super::super::super::assessment_reasons::MergeDecisionReason;

pub(super) fn matches(reason: MergeDecisionReason, record: &LanDiscoveryEvidenceRecord) -> bool {
    match reason {
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
        _ => false,
    }
}
