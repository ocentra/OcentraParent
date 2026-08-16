use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord,
};

use super::super::super::assessment_reasons::MergeDecisionReason;

pub(super) fn matches(reason: MergeDecisionReason, record: &LanDiscoveryEvidenceRecord) -> bool {
    match reason {
        MergeDecisionReason::SharedMdnsInstanceName => {
            prefix_matches(record, "mdns-instance-name:")
        }
        MergeDecisionReason::SharedSsdpUdn => prefix_matches(record, "ssdp-udn:"),
        MergeDecisionReason::SharedDeviceType => {
            prefix_matches(record, "mdns-service-type:")
                || prefix_matches(record, "ssdp-device-type:")
        }
        _ => false,
    }
}

fn prefix_matches(record: &LanDiscoveryEvidenceRecord, prefix: &str) -> bool {
    record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint
        && record
            .value
            .get(..prefix.len())
            .is_some_and(|value| value.eq_ignore_ascii_case(prefix))
}
