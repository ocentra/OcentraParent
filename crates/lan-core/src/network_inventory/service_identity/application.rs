use ocentra_parent_agent_protocol::constants;

use super::{
    merge_service_identity_probe_evidence, LanNetworkInventoryDevice,
    LanServiceIdentityProbeObservation,
};

pub(super) fn apply_service_identity_probe(
    device: &mut LanNetworkInventoryDevice,
    probe_match: LanServiceIdentityProbeObservation,
) {
    let observed_allowed_snmp_response = probe_match.observed_allowed_snmp_response();
    let selected_interface = device.network_interface.clone();
    device.agent_status =
        Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS.to_string());
    let incoming = probe_match.into_evidence_with_selected_interface(selected_interface);
    device.service_identity_probe_evidence = merge_service_identity_probe_evidence(
        std::mem::take(&mut device.service_identity_probe_evidence),
        incoming,
    );
    append_allowed_snmp_scan_source(device, observed_allowed_snmp_response);
}

fn append_allowed_snmp_scan_source(
    device: &mut LanNetworkInventoryDevice,
    observed_allowed_snmp_response: bool,
) {
    if !observed_allowed_snmp_response || has_allowed_snmp_scan_source(device) {
        return;
    }
    device
        .scan_sources
        .push(constants::lan_pairing::LAN_SCAN_SOURCE_ALLOWED_SNMP_RESPONSE.to_string());
}

fn has_allowed_snmp_scan_source(device: &LanNetworkInventoryDevice) -> bool {
    device
        .scan_sources
        .iter()
        .any(|source| source == constants::lan_pairing::LAN_SCAN_SOURCE_ALLOWED_SNMP_RESPONSE)
}
