use std::collections::HashMap;
use std::time::Duration;

use chrono::Utc;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use super::neighbor_support::filter_neighbor_observations_for_selected_interface;
use super::service_identity::{enrich_service_identity_probes, AllowedSnmpResponseObserver};
use super::{LanIdentityHintInventory, LanNetworkInventoryDevice, LanPreviousNetworkInventory};

pub mod identity;
pub mod merge;
pub mod observations;

pub fn linux_lan_neighbors(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface: Option<&str>,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
) -> Vec<LanNetworkInventoryDevice> {
    let trusted_inventory = LanIdentityHintInventory::from_devices(identity_hint_devices);
    let previous_inventory = LanPreviousNetworkInventory::from_devices(previous_devices);
    let observed_at = Utc::now().to_rfc3339();
    let observations =
        merge::merge_neighbor_observations(filter_neighbor_observations_for_selected_interface(
            observations::linux_ip_neigh_observations_with_observed_at(&observed_at)
                .into_iter()
                .chain(observations::linux_proc_net_arp_observations_with_observed_at(&observed_at))
                .collect(),
            selected_interface,
        ));
    let mut devices = observations
        .into_iter()
        .filter_map(|observation| {
            identity::network_device_from_neighbor_observation(
                observation,
                &trusted_inventory,
                &previous_inventory,
            )
        })
        .collect::<Vec<_>>();
    enrich_service_identity_probes(
        &mut devices,
        probe_suppression_devices,
        selected_interface,
        allowed_snmp_response_observer,
    );
    devices
}

pub fn current_linux_neighbor_ipv4_observations_with_timeout(
    timeout: Duration,
) -> HashMap<String, String> {
    observations::current_linux_neighbor_ipv4_observations_with_timeout(timeout)
}

pub fn current_linux_proc_net_arp_ipv4_observations_with_timeout(
    timeout: Duration,
) -> HashMap<String, String> {
    observations::current_linux_proc_net_arp_ipv4_observations_with_timeout(timeout)
}

pub fn current_linux_ip_neigh_ipv4_observations_with_timeout(
    timeout: Duration,
) -> HashMap<String, String> {
    observations::current_linux_ip_neigh_ipv4_observations_with_timeout(timeout)
}
