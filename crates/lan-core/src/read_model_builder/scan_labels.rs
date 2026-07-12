use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanDiscoveryEvidenceSource,
};

pub(super) fn scan_source_labels(devices: &[LanBrowserAddDeviceDiscoveryDevice]) -> Vec<String> {
    let mut labels = vec![constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string()];
    for label in devices
        .iter()
        .flat_map(|device| device.evidence_sources.iter())
        .filter_map(super::label_mapping::scan_source_label)
    {
        if !labels.iter().any(|existing| existing == label) {
            labels.push(label.to_string());
        }
    }
    if devices.iter().any(|device| {
        crate::network_inventory::api::is_service_identity_probe_status(
            device.child_device.agent_status.as_deref(),
        )
    }) && !labels
        .iter()
        .any(|label| label == crate::network_inventory::api::service_identity_probe_scan_source())
    {
        labels
            .push(crate::network_inventory::api::service_identity_probe_scan_source().to_string());
    }
    if devices.iter().any(|device| {
        device.service_identity_probe_evidence.iter().any(|evidence| {
            matches!(
                evidence.evidence_kind,
                ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind::SnmpSysDescr
                    | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind::SnmpSysName
            )
        })
    }) && !labels
        .iter()
        .any(|label| label == constants::lan_pairing::LAN_SCAN_SOURCE_ALLOWED_SNMP_RESPONSE)
    {
        labels.push(constants::lan_pairing::LAN_SCAN_SOURCE_ALLOWED_SNMP_RESPONSE.to_string());
    }
    if devices.iter().any(|device| {
        device
            .hint_sources
            .contains(&LanDiscoveryEvidenceSource::PreviousScanSnapshot)
    }) {
        labels.push(constants::lan_pairing::LAN_SCAN_SOURCE_PREVIOUS_SCAN_SNAPSHOT.to_string());
    }
    labels
}
