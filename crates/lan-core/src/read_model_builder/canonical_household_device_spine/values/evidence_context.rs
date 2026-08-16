use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceSource, LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceSource,
};

use super::EvidenceContext;

pub(super) fn evidence_context_for(
    source: &LanCanonicalHouseholdDeviceSource,
    evidence_sources: &[LanDiscoveryEvidenceSource],
) -> EvidenceContext {
    EvidenceContext {
        source: evidence_source_for(source, evidence_sources),
        confidence: evidence_confidence_for(source),
    }
}

fn evidence_source_for(
    source: &LanCanonicalHouseholdDeviceSource,
    evidence_sources: &[LanDiscoveryEvidenceSource],
) -> LanDiscoveryEvidenceSource {
    match source {
        LanCanonicalHouseholdDeviceSource::LocalService => LanDiscoveryEvidenceSource::LocalService,
        LanCanonicalHouseholdDeviceSource::NetworkNeighbor => {
            primary_network_evidence_source(evidence_sources)
        }
        LanCanonicalHouseholdDeviceSource::TrustedRegistry => {
            LanDiscoveryEvidenceSource::TrustedRegistry
        }
    }
}

fn primary_network_evidence_source(
    evidence_sources: &[LanDiscoveryEvidenceSource],
) -> LanDiscoveryEvidenceSource {
    evidence_sources
        .iter()
        .find(|source| {
            matches!(
                source,
                LanDiscoveryEvidenceSource::WindowsNeighborTable
                    | LanDiscoveryEvidenceSource::LinuxProcNetArp
                    | LanDiscoveryEvidenceSource::LinuxIpNeigh
                    | LanDiscoveryEvidenceSource::MacosArp
                    | LanDiscoveryEvidenceSource::MdnsDnsSdQuery
                    | LanDiscoveryEvidenceSource::SsdpUpnpQuery
                    | LanDiscoveryEvidenceSource::ServiceIdentityProbe
            )
        })
        .cloned()
        .unwrap_or(LanDiscoveryEvidenceSource::WindowsNeighborTable)
}

fn evidence_confidence_for(
    source: &LanCanonicalHouseholdDeviceSource,
) -> LanDiscoveryEvidenceConfidence {
    match source {
        LanCanonicalHouseholdDeviceSource::LocalService => {
            LanDiscoveryEvidenceConfidence::Confirmed
        }
        LanCanonicalHouseholdDeviceSource::NetworkNeighbor => {
            LanDiscoveryEvidenceConfidence::Strong
        }
        LanCanonicalHouseholdDeviceSource::TrustedRegistry => {
            LanDiscoveryEvidenceConfidence::ManualRequired
        }
    }
}
