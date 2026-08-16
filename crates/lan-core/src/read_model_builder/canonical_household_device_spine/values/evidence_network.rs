use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord, LanDiscoveryEvidenceSource,
};

use super::super::value_support::known_hostname;
use super::evidence_record::push_optional_evidence;
use super::EvidenceContext;
use crate::mac_identity::LanMacIdentityAssessment;

pub(super) fn push_network_identity_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    context: &EvidenceContext,
    evidence_sources: &[LanDiscoveryEvidenceSource],
    observed_at: &str,
    mac_assessment: Option<&LanMacIdentityAssessment>,
) {
    push_optional_evidence(
        records,
        device,
        context,
        LanDiscoveryEvidenceKind::IpAddress,
        device.ip_address.as_deref(),
        constants::lan_pairing::LAN_EVIDENCE_KEY_IP_PREFIX,
        observed_at,
    );
    push_optional_evidence(
        records,
        device,
        context,
        LanDiscoveryEvidenceKind::MacAddress,
        mac_assessment.and_then(LanMacIdentityAssessment::normalized),
        constants::lan_pairing::LAN_EVIDENCE_KEY_MAC_PREFIX,
        observed_at,
    );
    if !has_weak_name_source(evidence_sources) {
        push_optional_evidence(
            records,
            device,
            context,
            LanDiscoveryEvidenceKind::Hostname,
            known_hostname(device).as_deref(),
            constants::lan_pairing::LAN_EVIDENCE_KEY_HOSTNAME_PREFIX,
            observed_at,
        );
    }
    push_optional_evidence(
        records,
        device,
        context,
        LanDiscoveryEvidenceKind::Interface,
        device.network_interface.as_deref(),
        constants::lan_pairing::LAN_EVIDENCE_KEY_INTERFACE_PREFIX,
        observed_at,
    );
}

fn has_weak_name_source(evidence_sources: &[LanDiscoveryEvidenceSource]) -> bool {
    evidence_sources.contains(&LanDiscoveryEvidenceSource::DnsCache)
        || evidence_sources.contains(&LanDiscoveryEvidenceSource::Netbios)
        || evidence_sources.contains(&LanDiscoveryEvidenceSource::Llmnr)
}
