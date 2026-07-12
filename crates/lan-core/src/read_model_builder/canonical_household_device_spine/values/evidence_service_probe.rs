#[path = "evidence_service_probe_kind.rs"]
mod kind;
#[path = "evidence_service_probe_kind_metadata.rs"]
mod kind_metadata;
#[path = "evidence_service_probe_kind_network.rs"]
mod kind_network;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord,
    LanDiscoveryEvidenceSource, LanServiceIdentityProbeEvidence,
    LanServiceIdentityProbeEvidenceKind,
};

use super::evidence_record::{push_evidence_record, EvidenceRecordInput};

pub(super) fn push_service_probe_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    service_identity_probe_evidence: &[LanServiceIdentityProbeEvidence],
    observed_at: &str,
) {
    for evidence in service_identity_probe_evidence {
        let value = service_probe_evidence_value(evidence);
        push_evidence_record(
            records,
            EvidenceRecordInput {
                device,
                source: LanDiscoveryEvidenceSource::ServiceIdentityProbe,
                evidence_kind: LanDiscoveryEvidenceKind::ServiceProbeHint,
                value: &value,
                merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_SERVICE_PROBE_PREFIX,
                confidence: service_probe_evidence_confidence(&evidence.evidence_kind),
                observed_at,
                note: service_probe_evidence_note(&evidence.evidence_kind),
            },
        );
    }
}

fn service_probe_evidence_value(evidence: &LanServiceIdentityProbeEvidence) -> String {
    let mut value = String::from(kind::service_probe_kind_value(&evidence.evidence_kind));
    value.push(':');
    value.push_str(&evidence.value);
    value
}

fn service_probe_evidence_confidence(
    kind: &LanServiceIdentityProbeEvidenceKind,
) -> LanDiscoveryEvidenceConfidence {
    match kind {
        LanServiceIdentityProbeEvidenceKind::MdnsServiceType
        | LanServiceIdentityProbeEvidenceKind::MdnsInstanceName
        | LanServiceIdentityProbeEvidenceKind::SsdpUdn
        | LanServiceIdentityProbeEvidenceKind::SsdpDeviceType => {
            LanDiscoveryEvidenceConfidence::Strong
        }
        LanServiceIdentityProbeEvidenceKind::HttpStatus
        | LanServiceIdentityProbeEvidenceKind::HtmlTitle
        | LanServiceIdentityProbeEvidenceKind::ServerHeader
        | LanServiceIdentityProbeEvidenceKind::Banner
        | LanServiceIdentityProbeEvidenceKind::RedirectLocation
        | LanServiceIdentityProbeEvidenceKind::CertificateSubject
        | LanServiceIdentityProbeEvidenceKind::DescriptorLink
        | LanServiceIdentityProbeEvidenceKind::WsdEndpointAddress
        | LanServiceIdentityProbeEvidenceKind::WsdTypes
        | LanServiceIdentityProbeEvidenceKind::SnmpSysDescr
        | LanServiceIdentityProbeEvidenceKind::SnmpSysName => LanDiscoveryEvidenceConfidence::Weak,
    }
}

fn service_probe_evidence_note(kind: &LanServiceIdentityProbeEvidenceKind) -> Option<String> {
    match kind {
        LanServiceIdentityProbeEvidenceKind::MdnsServiceType
        | LanServiceIdentityProbeEvidenceKind::MdnsInstanceName
        | LanServiceIdentityProbeEvidenceKind::SsdpUdn
        | LanServiceIdentityProbeEvidenceKind::SsdpDeviceType => {
            Some("network discovery identity hint".to_string())
        }
        LanServiceIdentityProbeEvidenceKind::HttpStatus
        | LanServiceIdentityProbeEvidenceKind::HtmlTitle
        | LanServiceIdentityProbeEvidenceKind::ServerHeader
        | LanServiceIdentityProbeEvidenceKind::Banner
        | LanServiceIdentityProbeEvidenceKind::RedirectLocation
        | LanServiceIdentityProbeEvidenceKind::CertificateSubject
        | LanServiceIdentityProbeEvidenceKind::DescriptorLink => {
            Some(constants::lan_pairing::LAN_SERVICE_PROBE_HINT_NOTE.to_string())
        }
        LanServiceIdentityProbeEvidenceKind::WsdEndpointAddress
        | LanServiceIdentityProbeEvidenceKind::WsdTypes => {
            Some("ws-discovery metadata hint".to_string())
        }
        LanServiceIdentityProbeEvidenceKind::SnmpSysDescr
        | LanServiceIdentityProbeEvidenceKind::SnmpSysName => {
            Some("snmp metadata hint".to_string())
        }
    }
}
