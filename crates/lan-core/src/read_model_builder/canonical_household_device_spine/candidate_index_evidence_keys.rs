use std::collections::HashSet;

use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord,
};

use super::push_normalized_key;
use service_keys::push_service_hint_key;

#[path = "candidate_index_service_keys.rs"]
mod service_keys;

pub(super) fn push_evidence_keys(
    keys: &mut HashSet<String>,
    records: &[LanDiscoveryEvidenceRecord],
) {
    for record in records {
        if record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint {
            push_service_hint_key(keys, record);
            continue;
        }
        if let Some(namespace) = evidence_namespace(&record.evidence_kind) {
            push_normalized_key(keys, namespace, &record.normalized_value);
        }
    }
}

fn evidence_namespace(kind: &LanDiscoveryEvidenceKind) -> Option<&'static str> {
    match kind {
        LanDiscoveryEvidenceKind::InstallId => Some("install"),
        LanDiscoveryEvidenceKind::PairingId => Some("pairing"),
        LanDiscoveryEvidenceKind::TrustedRegistry => Some("trusted"),
        LanDiscoveryEvidenceKind::Vendor => Some("vendor"),
        LanDiscoveryEvidenceKind::MacAddress => Some("mac"),
        LanDiscoveryEvidenceKind::IpAddress => Some("ip"),
        LanDiscoveryEvidenceKind::Hostname => Some("hostname"),
        _ => None,
    }
}
