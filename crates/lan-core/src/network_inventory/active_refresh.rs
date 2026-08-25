use std::net::Ipv4Addr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;

use crate::network_inventory_hardware::{
    local_network_identity_with_timeout, LocalNetworkIdentity,
};

use self::evidence::{
    probe_targeted_arp_refresh_target_until, targeted_arp_refresh_targets_with_evidence,
    targeted_arp_refresh_targets_with_evidence_until,
};
use self::observations::current_active_refresh_ipv4_observations_by_ip_until;
use self::target_builders::targeted_arp_refresh_targets;
use self::targets::{refresh_metrics, saturating_u32};
use super::service_identity::runtime_service_identity_probe_settings;
use super::{
    LanDiscoveryRefreshMode, LanDiscoveryScanPlan, LanNetworkInventoryDevice,
    LanTargetedArpRefreshEvidence,
};

pub mod evidence;
pub mod observations;
pub mod suppression;
pub mod target_builders;
pub mod targets;
pub mod throttle;

pub const TARGETED_ARP_REFRESH_SCAN_BUDGET_MS: u64 = 2_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetedArpRefreshTarget {
    pub ip_address: Ipv4Addr,
    pub expected_mac_address: Option<String>,
    pub network_interface: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TargetedArpRefreshAttemptKey {
    pub ip_address: Ipv4Addr,
    pub network_interface: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetedArpRefreshObservation {
    pub ip_address: Ipv4Addr,
    pub mac_address: String,
}

pub trait TargetedArpRefreshPacketIo {
    fn probe_target(&mut self, target: &TargetedArpRefreshTarget, deadline: Instant) -> bool;
    fn observations(&mut self, deadline: Instant) -> Vec<TargetedArpRefreshObservation>;
    fn has_observation_budget(&mut self, deadline: Instant) -> bool {
        remaining_budget_until(deadline).is_some()
    }
}

#[derive(Default)]
pub struct CommandTargetedArpRefreshPacketIo;

impl TargetedArpRefreshPacketIo for CommandTargetedArpRefreshPacketIo {
    fn probe_target(&mut self, target: &TargetedArpRefreshTarget, deadline: Instant) -> bool {
        probe_targeted_arp_refresh_target_until(target, deadline)
    }

    fn observations(&mut self, deadline: Instant) -> Vec<TargetedArpRefreshObservation> {
        current_active_refresh_ipv4_observations_by_ip_until(deadline)
            .into_iter()
            .map(|(ip_address, mac_address)| TargetedArpRefreshObservation {
                ip_address,
                mac_address,
            })
            .collect()
    }
}

pub fn stimulate_bounded_ipv4_neighbors(
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    cancellation: Option<&AtomicBool>,
    outer_deadline: Option<Instant>,
) {
    let local_deadline =
        Instant::now() + Duration::from_millis(TARGETED_ARP_REFRESH_SCAN_BUDGET_MS);
    let deadline = outer_deadline.map_or(local_deadline, |outer| outer.min(local_deadline));
    if is_cancelled(cancellation) {
        return;
    }
    let Some(identity_budget) = remaining_budget_until(deadline) else {
        return;
    };
    let Some(identity) = local_network_identity_with_timeout(identity_budget) else {
        return;
    };
    if is_cancelled(cancellation) {
        return;
    }
    let targets = targeted_arp_refresh_targets(
        identity.ip_address.as_deref(),
        identity.ipv4_cidr.as_deref(),
        identity.network_interface.as_deref(),
        active_refresh_suppression_devices,
        previous_devices,
    );
    if targets.is_empty() || is_cancelled(cancellation) {
        return;
    }
    let _refresh_evidence = targeted_arp_refresh_targets_with_evidence_until(&targets, deadline);
}

fn is_cancelled(cancellation: Option<&AtomicBool>) -> bool {
    cancellation.is_some_and(|value| value.load(Ordering::Acquire))
}

pub fn scan_plan_for_identity(
    identity: Option<&LocalNetworkIdentity>,
    _identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
) -> LanDiscoveryScanPlan {
    let selected_interface = identity.and_then(|identity| identity.network_interface.clone());
    let local_ip_address = identity.and_then(|identity| identity.ip_address.clone());
    let ipv4_cidr = identity.and_then(|identity| identity.ipv4_cidr.clone());
    let default_gateway = identity.and_then(|identity| identity.default_gateway.clone());
    let dns_servers = identity
        .map(|identity| identity.dns_servers.clone())
        .unwrap_or_default();
    let dhcp_server = identity.and_then(|identity| identity.dhcp_server.clone());
    let broadcast_address = identity.and_then(|identity| identity.broadcast_address.clone());
    let ipv6_prefixes = identity
        .map(|identity| identity.ipv6_prefixes.clone())
        .unwrap_or_default();

    let (
        active_ipv4_candidate_count,
        active_ipv4_target_count,
        prioritized_previous_target_count,
        active_ipv4_target_timeout_ms,
        suppressed_active_ipv4_targets,
    ) = refresh_metrics(
        refresh_mode,
        local_ip_address.as_deref(),
        ipv4_cidr.as_deref(),
        default_gateway.as_deref(),
        active_refresh_suppression_devices,
        previous_devices,
    );
    let probe_settings = runtime_service_identity_probe_settings();

    LanDiscoveryScanPlan {
        refresh_mode,
        selected_interface,
        local_ip_address,
        ipv4_cidr,
        default_gateway,
        dns_servers,
        dhcp_server,
        broadcast_address,
        ipv6_prefixes,
        trusted_truth_device_count: saturating_u32(active_refresh_suppression_devices.len()),
        previous_device_count: saturating_u32(previous_devices.len()),
        active_ipv4_candidate_count,
        active_ipv4_target_count,
        prioritized_previous_target_count,
        active_ipv4_target_timeout_ms,
        allow_wsd_identity_query: probe_settings.allow_wsd_identity_query,
        allow_snmp_identity_query: probe_settings.allow_snmp_identity_query,
        allow_os_fingerprint: probe_settings.allow_os_fingerprint,
        suppressed_active_ipv4_targets,
        targeted_arp_refresh_evidence: Vec::new(),
    }
}

pub fn targeted_arp_refresh_evidence_for_identity(
    identity: Option<&LocalNetworkIdentity>,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
) -> Vec<LanTargetedArpRefreshEvidence> {
    let Some(identity) = identity else {
        return Vec::new();
    };
    let targets = targeted_arp_refresh_targets(
        identity.ip_address.as_deref(),
        identity.ipv4_cidr.as_deref(),
        identity.network_interface.as_deref(),
        active_refresh_suppression_devices,
        previous_devices,
    );
    if targets.is_empty() {
        return Vec::new();
    }
    targeted_arp_refresh_targets_with_evidence(&targets)
}

pub fn targeted_arp_refresh_evidence_for_scan_plan_until(
    scan_plan: &LanDiscoveryScanPlan,
    previous_devices: &[LanNetworkInventoryDevice],
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    cancellation: Option<&AtomicBool>,
    outer_deadline: Option<Instant>,
) -> Vec<LanTargetedArpRefreshEvidence> {
    if scan_plan.refresh_mode != LanDiscoveryRefreshMode::ActiveSubnetRefresh
        || is_cancelled(cancellation)
    {
        return Vec::new();
    }

    let local_deadline =
        Instant::now() + Duration::from_millis(TARGETED_ARP_REFRESH_SCAN_BUDGET_MS);
    let deadline = outer_deadline.map_or(local_deadline, |outer| outer.min(local_deadline));
    if remaining_budget_until(deadline).is_none() {
        return Vec::new();
    }

    let targets = targeted_arp_refresh_targets(
        scan_plan.local_ip_address.as_deref(),
        scan_plan.ipv4_cidr.as_deref(),
        scan_plan.selected_interface.as_deref(),
        active_refresh_suppression_devices,
        previous_devices,
    );
    if targets.is_empty() || is_cancelled(cancellation) {
        return Vec::new();
    }

    let evidence = targeted_arp_refresh_targets_with_evidence_until(&targets, deadline);
    if is_cancelled(cancellation) {
        Vec::new()
    } else {
        evidence
    }
}

pub fn remaining_budget_until(deadline: Instant) -> Option<Duration> {
    let now = Instant::now();
    if now >= deadline {
        return None;
    }
    let remaining = deadline.saturating_duration_since(now);
    (!remaining.is_zero()).then_some(remaining)
}
