use std::sync::atomic::{AtomicBool, Ordering};

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use super::service_identity::AllowedSnmpResponseObserver;
use super::{
    service_identity_selected_interface_scope, LanDiscoveryRefreshMode, LanNetworkInventoryDevice,
};

pub(in crate::network_inventory) fn discover_lan_network_devices_with_cancellation(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface_scope: Option<&str>,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
    cancellation: Option<&AtomicBool>,
) -> Vec<LanNetworkInventoryDevice> {
    if cancellation.is_some_and(|value| value.load(Ordering::Acquire)) {
        return Vec::new();
    }
    if refresh_mode == LanDiscoveryRefreshMode::ActiveSubnetRefresh {
        super::super::active_refresh::stimulate_bounded_ipv4_neighbors(
            active_refresh_suppression_devices,
            previous_devices,
        );
    }
    let selected_interface = service_identity_selected_interface_scope(selected_interface_scope);
    let mut devices = if cfg!(target_os = "windows") {
        super::super::windows_neighbors::windows_lan_neighbors_with_cancellation(
            identity_hint_devices,
            previous_devices,
            probe_suppression_devices,
            selected_interface.as_deref(),
            allowed_snmp_response_observer,
            cancellation,
        )
    } else if cfg!(any(target_os = "linux", target_os = "android")) {
        super::super::linux_neighbors::linux_lan_neighbors_with_cancellation(
            identity_hint_devices,
            previous_devices,
            probe_suppression_devices,
            selected_interface.as_deref(),
            allowed_snmp_response_observer,
            cancellation,
        )
    } else if cfg!(target_os = "macos") {
        super::super::macos_neighbors::macos_lan_neighbors_with_cancellation(
            identity_hint_devices,
            previous_devices,
            probe_suppression_devices,
            selected_interface.as_deref(),
            allowed_snmp_response_observer,
            cancellation,
        )
    } else {
        Vec::new()
    };
    super::super::mdns_dns_sd::enrich_mdns_dns_sd_devices_with_cancellation(
        &mut devices,
        selected_interface.as_deref(),
        cancellation,
    );
    if !cancellation.is_some_and(|value| value.load(Ordering::Acquire)) {
        super::super::ssdp_upnp::enrich_ssdp_upnp_devices(
            &mut devices,
            selected_interface.as_deref(),
        );
    }
    devices
}
