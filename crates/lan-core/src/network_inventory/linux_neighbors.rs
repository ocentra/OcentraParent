use std::time::{Duration, Instant};
use std::{collections::HashMap, sync::atomic::AtomicBool};

use chrono::Utc;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use super::neighbor_support::filter_neighbor_observations_for_selected_interface;
use super::service_identity::{
    enrich_service_identity_probes_with_cancellation, AllowedSnmpResponseObserver,
};
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
    linux_lan_neighbors_with_cancellation(
        identity_hint_devices,
        previous_devices,
        probe_suppression_devices,
        selected_interface,
        allowed_snmp_response_observer,
        None,
        None,
    )
}

pub fn linux_lan_neighbors_with_cancellation(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface: Option<&str>,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
    cancellation: Option<&AtomicBool>,
    deadline: Option<Instant>,
) -> Vec<LanNetworkInventoryDevice> {
    let trusted_inventory = LanIdentityHintInventory::from_devices(identity_hint_devices);
    let previous_inventory = LanPreviousNetworkInventory::from_devices(previous_devices);
    let observed_at = Utc::now().to_rfc3339();
    let ip_neighbors = match cancellation {
        Some(cancellation) => {
            observations::linux_ip_neigh_observations_with_cancellation(&observed_at, cancellation)
        }
        None => observations::linux_ip_neigh_observations_with_observed_at(&observed_at),
    };
    let observations =
        merge::merge_neighbor_observations(filter_neighbor_observations_for_selected_interface(
            ip_neighbors
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
    enrich_service_identity_probes_with_cancellation(
        &mut devices,
        probe_suppression_devices,
        selected_interface,
        allowed_snmp_response_observer,
        cancellation,
        deadline,
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
