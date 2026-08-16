use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind;

pub(super) fn service_probe_kind_value(kind: &LanServiceIdentityProbeEvidenceKind) -> &'static str {
    match kind {
        LanServiceIdentityProbeEvidenceKind::HttpStatus
        | LanServiceIdentityProbeEvidenceKind::HtmlTitle
        | LanServiceIdentityProbeEvidenceKind::ServerHeader
        | LanServiceIdentityProbeEvidenceKind::Banner
        | LanServiceIdentityProbeEvidenceKind::RedirectLocation
        | LanServiceIdentityProbeEvidenceKind::CertificateSubject
        | LanServiceIdentityProbeEvidenceKind::DescriptorLink => {
            super::kind_network::service_probe_kind_value(kind).unwrap_or("")
        }
        LanServiceIdentityProbeEvidenceKind::WsdEndpointAddress
        | LanServiceIdentityProbeEvidenceKind::WsdTypes
        | LanServiceIdentityProbeEvidenceKind::SnmpSysDescr
        | LanServiceIdentityProbeEvidenceKind::SnmpSysName
        | LanServiceIdentityProbeEvidenceKind::MdnsServiceType
        | LanServiceIdentityProbeEvidenceKind::MdnsInstanceName
        | LanServiceIdentityProbeEvidenceKind::SsdpUdn
        | LanServiceIdentityProbeEvidenceKind::SsdpDeviceType => {
            super::kind_metadata::service_probe_kind_value(kind).unwrap_or("")
        }
    }
}
