use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind;

pub(super) fn service_probe_kind_value(
    kind: &LanServiceIdentityProbeEvidenceKind,
) -> Option<&'static str> {
    match kind {
        LanServiceIdentityProbeEvidenceKind::WsdEndpointAddress => Some("wsd-endpoint-address"),
        LanServiceIdentityProbeEvidenceKind::WsdTypes => Some("wsd-types"),
        LanServiceIdentityProbeEvidenceKind::SnmpSysDescr => Some("snmp-sys-descr"),
        LanServiceIdentityProbeEvidenceKind::SnmpSysName => Some("snmp-sys-name"),
        LanServiceIdentityProbeEvidenceKind::MdnsServiceType => Some("mdns-service-type"),
        LanServiceIdentityProbeEvidenceKind::MdnsInstanceName => Some("mdns-instance-name"),
        LanServiceIdentityProbeEvidenceKind::SsdpUdn => Some("ssdp-udn"),
        LanServiceIdentityProbeEvidenceKind::SsdpDeviceType => Some("ssdp-device-type"),
        _ => None,
    }
}
