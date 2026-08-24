use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use crate::network_inventory_hardware::local_network_identity_until;

use super::super::active_refresh::scan_plan_for_identity;
use super::service_identity::AllowedSnmpResponseObserver;
use super::{LanDiscoveryRefreshMode, LanNetworkInventoryDevice};
use crate::network_inventory::LanDiscoveryScanPlan;

pub(in crate::network_inventory) fn plan_lan_discovery_scan_until(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    deadline: Instant,
    cancellation: &AtomicBool,
) -> LanDiscoveryScanPlan {
    let identity = local_network_identity_until(deadline, cancellation);
    scan_plan_for_identity(
        identity.as_ref(),
        identity_hint_devices,
        previous_devices,
        refresh_mode,
        active_refresh_suppression_devices,
    )
}

pub(in crate::network_inventory) fn discover_lan_network_devices_with_cancellation(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface_scope: Option<&str>,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
    cancellation: Option<&AtomicBool>,
    deadline: Option<Instant>,
) -> Vec<LanNetworkInventoryDevice> {
    if unavailable(cancellation, deadline) {
        return Vec::new();
    }
    if refresh_mode == LanDiscoveryRefreshMode::ActiveSubnetRefresh {
        super::super::active_refresh::stimulate_bounded_ipv4_neighbors(
            active_refresh_suppression_devices,
            previous_devices,
            cancellation,
            deadline,
        );
    }
    // This cancellation-aware path receives the interface resolved by the
    // caller's single bounded identity lookup. Falling back here would repeat
    // an ambient identity command outside that lookup's deadline.
    let selected_interface = selected_interface_scope
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let mut devices = if cfg!(target_os = "windows") {
        super::super::windows_neighbors::windows_lan_neighbors_with_cancellation(
            identity_hint_devices,
            previous_devices,
            probe_suppression_devices,
            selected_interface.as_deref(),
            allowed_snmp_response_observer,
            cancellation,
            deadline,
        )
    } else if cfg!(any(target_os = "linux", target_os = "android")) {
        super::super::linux_neighbors::linux_lan_neighbors_with_cancellation(
            identity_hint_devices,
            previous_devices,
            probe_suppression_devices,
            selected_interface.as_deref(),
            allowed_snmp_response_observer,
            cancellation,
            deadline,
        )
    } else if cfg!(target_os = "macos") {
        super::super::macos_neighbors::macos_lan_neighbors_with_cancellation(
            identity_hint_devices,
            previous_devices,
            probe_suppression_devices,
            selected_interface.as_deref(),
            allowed_snmp_response_observer,
            cancellation,
            deadline,
        )
    } else {
        Vec::new()
    };
    super::super::mdns_dns_sd::enrich_mdns_dns_sd_devices_with_cancellation(
        &mut devices,
        selected_interface.as_deref(),
        cancellation,
        deadline,
    );
    if !unavailable(cancellation, deadline) {
        super::super::ssdp_upnp::inventory::enrich_ssdp_upnp_devices_with_cancellation(
            &mut devices,
            selected_interface.as_deref(),
            cancellation,
            deadline,
        );
    }
    devices
}

fn unavailable(cancellation: Option<&AtomicBool>, deadline: Option<Instant>) -> bool {
    cancellation.is_some_and(|value| value.load(Ordering::Acquire))
        || deadline.is_some_and(|deadline| Instant::now() >= deadline)
}
