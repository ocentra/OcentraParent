use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

use super::super::linux_neighbors::current_linux_neighbor_ipv4_observations_with_timeout;
use super::super::macos_neighbors::current_macos_neighbor_ipv4_observations_with_timeout;
use super::super::windows_neighbors::current_windows_neighbor_ipv4_observations_with_timeout;
use super::remaining_budget_until;
use super::targets::normalized_household_ipv4_ip;

pub fn current_active_refresh_ipv4_observations() -> HashMap<String, String> {
    let timeout = Duration::from_millis(250);
    if cfg!(target_os = "windows") {
        current_windows_neighbor_ipv4_observations_with_timeout(timeout)
    } else if cfg!(any(target_os = "linux", target_os = "android")) {
        current_linux_neighbor_ipv4_observations_with_timeout(timeout)
    } else if cfg!(target_os = "macos") {
        current_macos_neighbor_ipv4_observations_with_timeout(timeout)
    } else {
        HashMap::new()
    }
}

pub fn current_active_refresh_ipv4_observations_until(
    deadline: Instant,
) -> HashMap<String, String> {
    let Some(timeout) = remaining_budget_until(deadline) else {
        return HashMap::new();
    };
    if cfg!(target_os = "windows") {
        current_windows_neighbor_ipv4_observations_with_timeout(timeout)
    } else if cfg!(any(target_os = "linux", target_os = "android")) {
        current_linux_neighbor_ipv4_observations_with_timeout(timeout)
    } else if cfg!(target_os = "macos") {
        current_macos_neighbor_ipv4_observations_with_timeout(timeout)
    } else {
        HashMap::new()
    }
}

pub fn current_active_refresh_ipv4_observations_by_ip() -> HashMap<Ipv4Addr, String> {
    current_active_refresh_ipv4_observations()
        .into_iter()
        .filter_map(|(ip_address, mac_address)| {
            let ip_address = normalized_household_ipv4_ip(Some(&ip_address))?;
            Some((ip_address, mac_address))
        })
        .collect()
}

pub fn current_active_refresh_ipv4_observations_by_ip_until(
    deadline: Instant,
) -> HashMap<Ipv4Addr, String> {
    current_active_refresh_ipv4_observations_until(deadline)
        .into_iter()
        .filter_map(|(ip_address, mac_address)| {
            let ip_address = normalized_household_ipv4_ip(Some(&ip_address))?;
            Some((ip_address, mac_address))
        })
        .collect()
}
