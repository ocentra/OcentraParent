use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord,
    LanDiscoveryEvidenceSource,
};

use super::super::value_support::known_hostname;
use super::evidence_record::{push_evidence_record, EvidenceRecordInput};

pub(super) fn push_weak_name_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    evidence_sources: &[LanDiscoveryEvidenceSource],
    observed_at: &str,
) {
    let Some(hostname) = known_hostname(device) else {
        return;
    };
    for source in weak_name_sources(evidence_sources) {
        let merge_key_prefix = weak_name_merge_key_prefix(&source);
        push_evidence_record(
            records,
            EvidenceRecordInput {
                device,
                source,
                evidence_kind: LanDiscoveryEvidenceKind::Hostname,
                value: &hostname,
                merge_key_prefix,
                confidence: LanDiscoveryEvidenceConfidence::Weak,
                observed_at,
                note: None,
            },
        );
    }
}

fn weak_name_sources(
    evidence_sources: &[LanDiscoveryEvidenceSource],
) -> Vec<LanDiscoveryEvidenceSource> {
    [
        LanDiscoveryEvidenceSource::DnsCache,
        LanDiscoveryEvidenceSource::Netbios,
        LanDiscoveryEvidenceSource::Llmnr,
    ]
    .into_iter()
    .filter(|source| evidence_sources.contains(source))
    .collect()
}

fn weak_name_merge_key_prefix(source: &LanDiscoveryEvidenceSource) -> &'static str {
    match source {
        LanDiscoveryEvidenceSource::DnsCache => {
            constants::lan_pairing::LAN_EVIDENCE_KEY_DNS_CACHE_PREFIX
        }
        LanDiscoveryEvidenceSource::Netbios => {
            constants::lan_pairing::LAN_EVIDENCE_KEY_NETBIOS_PREFIX
        }
        LanDiscoveryEvidenceSource::Llmnr => constants::lan_pairing::LAN_EVIDENCE_KEY_LLMNR_PREFIX,
        _ => constants::lan_pairing::LAN_EVIDENCE_KEY_HOSTNAME_PREFIX,
    }
}
