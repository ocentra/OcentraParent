use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use chrono::Utc;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use crate::network_inventory_command::{
    command_json_records, command_json_records_with_timeout,
    command_json_records_with_timeout_and_cancellation, normalize_mac_address, record_text,
    value_text,
};

use super::neighbor_support::{
    filter_neighbor_observations_for_selected_interface, interface_matches_selected_scope,
    is_household_unicast, is_supported_neighbor_ip, likely_router_address_text,
    normalize_neighbor_hostname, normalized_optional_interface_name,
};
use super::service_identity::{
    enrich_service_identity_probes_with_cancellation, AllowedSnmpResponseObserver,
};
use super::{
    merge_neighbor_observations_by_mac, LanIdentityHintInventory, LanNeighborObservation,
    LanNetworkInventoryDevice, LanPreviousNetworkInventory,
};

pub mod identity;
pub mod netbios;
mod reachability;

pub fn windows_lan_neighbors(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface: Option<&str>,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
) -> Vec<LanNetworkInventoryDevice> {
    windows_lan_neighbors_with_cancellation(
        identity_hint_devices,
        previous_devices,
        probe_suppression_devices,
        selected_interface,
        allowed_snmp_response_observer,
        None,
        None,
    )
}

pub fn windows_lan_neighbors_with_cancellation(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface: Option<&str>,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
    cancellation: Option<&AtomicBool>,
    deadline: Option<Instant>,
) -> Vec<LanNetworkInventoryDevice> {
    if cancellation.is_some_and(|value| value.load(Ordering::Acquire)) {
        return Vec::new();
    }
    let netbios_names = netbios::windows_netbios_cache_names_with_cancellation(cancellation);
    let identity_hint_inventory = LanIdentityHintInventory::from_devices(identity_hint_devices);
    let previous_inventory = LanPreviousNetworkInventory::from_devices(previous_devices);
    let observed_at = Utc::now().to_rfc3339();
    let command_args = [
        constants::lan_pairing::POWERSHELL_NO_PROFILE_ARG,
        constants::lan_pairing::POWERSHELL_EXECUTION_POLICY_ARG,
        constants::lan_pairing::POWERSHELL_BYPASS_ARG,
        constants::lan_pairing::POWERSHELL_COMMAND_ARG,
        constants::lan_pairing::POWERSHELL_LAN_NEIGHBOR_COMMAND,
    ];
    let records = match cancellation {
        Some(cancellation) => command_json_records_with_timeout_and_cancellation(
            constants::lan_pairing::POWERSHELL_EXE,
            &command_args,
            Duration::from_millis(constants::lan_pairing::LAN_NETWORK_INVENTORY_COMMAND_TIMEOUT_MS),
            cancellation,
        ),
        None => command_json_records(constants::lan_pairing::POWERSHELL_EXE, &command_args),
    };
    let observations = records
        .into_iter()
        .filter_map(|record| {
            windows_neighbor_observation_from_record_with_observed_at(&record, &observed_at)
        })
        .collect::<Vec<_>>();
    let mut devices = merge_neighbor_observations_by_mac(
        filter_neighbor_observations_for_selected_interface(observations, selected_interface),
    )
    .into_iter()
    .filter_map(|observation| {
        network_device_from_windows_observation(
            observation,
            &netbios_names,
            &identity_hint_inventory,
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

pub fn current_windows_neighbor_ipv4_observations_with_timeout(
    timeout: Duration,
) -> HashMap<String, String> {
    let observed_at = Utc::now().to_rfc3339();
    windows_neighbor_ipv4_observations_from_records(
        command_json_records_with_timeout(
            constants::lan_pairing::POWERSHELL_EXE,
            &[
                constants::lan_pairing::POWERSHELL_NO_PROFILE_ARG,
                constants::lan_pairing::POWERSHELL_EXECUTION_POLICY_ARG,
                constants::lan_pairing::POWERSHELL_BYPASS_ARG,
                constants::lan_pairing::POWERSHELL_COMMAND_ARG,
                constants::lan_pairing::POWERSHELL_LAN_NEIGHBOR_COMMAND,
            ],
            timeout,
        ),
        &observed_at,
    )
}

pub fn windows_neighbor_ipv4_observations_from_records(
    records: Vec<serde_json::Value>,
    observed_at: &str,
) -> HashMap<String, String> {
    let mut by_ip = HashMap::new();
    let observations = records
        .into_iter()
        .filter_map(|record| {
            windows_neighbor_observation_from_record_with_observed_at(&record, observed_at)
        })
        .collect::<Vec<_>>();
    for observation in merge_neighbor_observations_by_mac(observations) {
        let Some(ip_address) = observation.ip_address.parse::<Ipv4Addr>().ok() else {
            continue;
        };
        if !is_household_unicast(ip_address) {
            continue;
        }
        by_ip
            .entry(ip_address.to_string().to_ascii_lowercase())
            .or_insert(observation.mac_address.to_ascii_lowercase());
    }
    by_ip
}

pub fn windows_neighbor_observation_from_record(
    record: &serde_json::Value,
) -> Option<LanNeighborObservation> {
    windows_neighbor_observation_from_record_with_observed_at(record, &Utc::now().to_rfc3339())
}

pub fn windows_neighbor_observation_from_record_with_observed_at(
    record: &serde_json::Value,
    observed_at: &str,
) -> Option<LanNeighborObservation> {
    let ip_address = record_text(record, constants::lan_pairing::JSON_KEY_IP_ADDRESS)?;
    if !is_supported_neighbor_ip(&ip_address) {
        return None;
    }
    let mac_address = normalize_mac_address(&record_text(
        record,
        constants::lan_pairing::JSON_KEY_LINK_LAYER_ADDRESS,
    )?)?;
    let network_interface = normalized_optional_interface_name(record_text(
        record,
        constants::lan_pairing::JSON_KEY_INTERFACE_ALIAS,
    ));
    Some(LanNeighborObservation {
        ip_address,
        mac_address,
        network_interface,
        hostname: record_text(record, constants::lan_pairing::JSON_KEY_HOSTNAME)
            .and_then(|value| normalize_neighbor_hostname(&value)),
        observed_at: observed_at.to_string(),
        reachability: reachability_from_windows_state(
            record.get(constants::lan_pairing::JSON_KEY_STATE),
        ),
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
    })
}

pub fn network_device_from_windows_neighbor(
    record: &serde_json::Value,
    netbios_names: &HashMap<String, String>,
    identity_hint_inventory: &LanIdentityHintInventory,
    previous_inventory: &LanPreviousNetworkInventory,
    selected_interface: Option<&str>,
) -> Option<LanNetworkInventoryDevice> {
    network_device_from_windows_neighbor_with_observed_at(
        record,
        netbios_names,
        identity_hint_inventory,
        previous_inventory,
        selected_interface,
        &Utc::now().to_rfc3339(),
    )
}

pub fn network_device_from_windows_neighbor_with_observed_at(
    record: &serde_json::Value,
    netbios_names: &HashMap<String, String>,
    identity_hint_inventory: &LanIdentityHintInventory,
    previous_inventory: &LanPreviousNetworkInventory,
    selected_interface: Option<&str>,
    observed_at: &str,
) -> Option<LanNetworkInventoryDevice> {
    let observation =
        windows_neighbor_observation_from_record_with_observed_at(record, observed_at)?;
    if !interface_matches_selected_scope(
        observation.network_interface.as_deref(),
        selected_interface,
    ) {
        return None;
    }
    network_device_from_windows_observation(
        observation,
        netbios_names,
        identity_hint_inventory,
        previous_inventory,
    )
}

fn network_device_from_windows_observation(
    observation: LanNeighborObservation,
    netbios_names: &HashMap<String, String>,
    identity_hint_inventory: &LanIdentityHintInventory,
    previous_inventory: &LanPreviousNetworkInventory,
) -> Option<LanNetworkInventoryDevice> {
    let LanNeighborObservation {
        ip_address,
        mac_address,
        network_interface,
        hostname,
        observed_at,
        reachability,
        mut scan_sources,
    } = observation;
    let supports_netbios = netbios::windows_neighbor_supports_netbios(&ip_address);
    let platform = if likely_router_address_text(&ip_address) {
        constants::lan_pairing::PLATFORM_ROUTER
    } else {
        constants::lan_pairing::PLATFORM_UNKNOWN
    }
    .to_string();
    let trusted_device = identity_hint_inventory.find(&mac_address, &ip_address);
    let previous_device = previous_inventory.find(&mac_address, &ip_address);
    let resolved_identity =
        identity::resolved_windows_neighbor_identity(identity::WindowsNeighborIdentityInput {
            ip_address: &ip_address,
            mac_address: &mac_address,
            platform,
            supports_netbios,
            reachability: &reachability,
            dns_hostname: hostname,
            netbios_cache_hostname: netbios_names.get(&ip_address).cloned(),
            trusted_device,
            previous_device,
        });
    let mut device_id = String::from(constants::lan_pairing::NETWORK_NEIGHBOR_DEVICE_PREFIX);
    device_id.push_str(
        &mac_address
            .chars()
            .filter(|character| *character != '-')
            .collect::<String>(),
    );
    for source in &resolved_identity.name_scan_sources {
        identity::push_unique_scan_source(&mut scan_sources, source);
    }

    Some(LanNetworkInventoryDevice {
        device_id,
        label: resolved_identity.label,
        platform: resolved_identity.platform,
        ip_address,
        mac_address,
        hostname: resolved_identity.hostname,
        network_interface,
        observed_at,
        reachability,
        agent_status: None,
        scan_sources,
        used_previous_scan_hint: resolved_identity.used_previous_scan_hint,
        service_identity_probe_evidence: Vec::new(),
    })
}

pub fn reachability_from_windows_state(
    state: Option<&serde_json::Value>,
) -> LanPairingDeviceReachability {
    reachability::from_windows_state(state.and_then(value_text))
}
