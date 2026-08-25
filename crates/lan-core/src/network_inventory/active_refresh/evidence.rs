use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use ocentra_parent_agent_protocol::constants;

use crate::network_inventory_command::{
    command_succeeded_with_timeout, normalize_mac_address, protected_command_adapter_state,
    targeted_arp_probe_commands, ProtectedCommandAdapterState,
};

use super::super::{LanTargetedArpRefreshEvidence, LanTargetedArpRefreshOutcome};
use super::throttle::targeted_arp_refresh_throttled;
use super::{
    remaining_budget_until, CommandTargetedArpRefreshPacketIo, TargetedArpRefreshObservation,
    TargetedArpRefreshPacketIo, TargetedArpRefreshTarget, TARGETED_ARP_REFRESH_SCAN_BUDGET_MS,
};

enum TargetedArpAttempt<'a> {
    Probed(&'a TargetedArpRefreshTarget),
    Throttled(&'a TargetedArpRefreshTarget),
}

pub fn targeted_arp_refresh_targets_with_evidence(
    targets: &[TargetedArpRefreshTarget],
) -> Vec<LanTargetedArpRefreshEvidence> {
    targeted_arp_refresh_targets_with_evidence_until(
        targets,
        Instant::now() + Duration::from_millis(TARGETED_ARP_REFRESH_SCAN_BUDGET_MS),
    )
}

pub fn targeted_arp_refresh_targets_with_evidence_until(
    targets: &[TargetedArpRefreshTarget],
    deadline: Instant,
) -> Vec<LanTargetedArpRefreshEvidence> {
    if protected_command_adapter_state() == ProtectedCommandAdapterState::Unavailable {
        return Vec::new();
    }
    targeted_arp_refresh_targets_with_packet_io_until(
        targets,
        deadline,
        &mut CommandTargetedArpRefreshPacketIo,
    )
}

pub fn targeted_arp_refresh_targets_with_packet_io_until(
    targets: &[TargetedArpRefreshTarget],
    deadline: Instant,
    packet_io: &mut dyn TargetedArpRefreshPacketIo,
) -> Vec<LanTargetedArpRefreshEvidence> {
    let mut attempts = Vec::new();
    for target in targets {
        if Instant::now() >= deadline {
            break;
        }
        if targeted_arp_refresh_throttled(target).is_some() {
            attempts.push(TargetedArpAttempt::Throttled(target));
            continue;
        }
        if !packet_io.probe_target(target, deadline) {
            break;
        }
        attempts.push(TargetedArpAttempt::Probed(target));
    }

    let probed = attempts
        .iter()
        .any(|attempt| matches!(attempt, TargetedArpAttempt::Probed(_)));
    let observations = probed
        .then(|| {
            packet_io
                .has_observation_budget(deadline)
                .then(|| observations_by_ip(packet_io.observations(deadline)))
        })
        .flatten()
        .or_else(|| (!probed).then(HashMap::new));
    let observed_at_unix_ms = unix_timestamp_ms();
    attempts
        .iter()
        .filter_map(|attempt| {
            targeted_arp_attempt_evidence(attempt, observations.as_ref(), observed_at_unix_ms)
        })
        .collect()
}

fn targeted_arp_attempt_evidence(
    attempt: &TargetedArpAttempt<'_>,
    observations: Option<&HashMap<Ipv4Addr, String>>,
    observed_at_unix_ms: u128,
) -> Option<LanTargetedArpRefreshEvidence> {
    match attempt {
        TargetedArpAttempt::Probed(target) => observations.map(|observations| {
            targeted_arp_refresh_evidence_from_observation(
                target,
                observations.get(&target.ip_address).cloned(),
                observed_at_unix_ms,
                false,
            )
        }),
        TargetedArpAttempt::Throttled(target) => Some(
            targeted_arp_refresh_evidence_from_observation(target, None, observed_at_unix_ms, true),
        ),
    }
}

pub fn observations_by_ip(
    observations: Vec<TargetedArpRefreshObservation>,
) -> HashMap<Ipv4Addr, String> {
    let mut by_ip = HashMap::new();
    for observation in observations {
        let Some(mac_address) = normalize_mac_address(&observation.mac_address) else {
            continue;
        };
        by_ip
            .entry(observation.ip_address)
            .or_insert(mac_address.to_ascii_lowercase());
    }
    by_ip
}

pub fn probe_targeted_arp_refresh_target_until(
    target: &TargetedArpRefreshTarget,
    deadline: Instant,
) -> bool {
    let ip_address = target.ip_address.to_string();
    let probe_commands =
        targeted_arp_probe_commands(&ip_address, target.network_interface.as_deref());
    let mut attempted = false;
    for command in &probe_commands {
        let Some(timeout) = remaining_budget_until(deadline) else {
            break;
        };
        let arg_refs = command.args.iter().map(String::as_str).collect::<Vec<_>>();
        attempted = true;
        let _ = command_succeeded_with_timeout(command.program, &arg_refs, timeout);
    }
    attempted
}

pub fn targeted_arp_refresh_evidence_from_observation(
    target: &TargetedArpRefreshTarget,
    observed_mac_address: Option<String>,
    observed_at_unix_ms: u128,
    throttled: bool,
) -> LanTargetedArpRefreshEvidence {
    let observed_mac_address = observed_mac_address.and_then(|value| normalize_mac_address(&value));
    let outcome = if throttled {
        None
    } else if observed_mac_address.is_some() {
        Some(LanTargetedArpRefreshOutcome::Response)
    } else {
        Some(LanTargetedArpRefreshOutcome::NoResponse)
    };
    let strong_identity_match = observed_mac_address
        .as_ref()
        .zip(target.expected_mac_address.as_ref())
        .map(|(observed_mac_address, expected_mac_address)| {
            observed_mac_address.eq_ignore_ascii_case(expected_mac_address)
        })
        .unwrap_or(false);

    LanTargetedArpRefreshEvidence {
        target_ip_address: target.ip_address.to_string(),
        selected_interface: target.network_interface.clone(),
        expected_mac_address: target.expected_mac_address.clone(),
        observed_mac_address,
        observed_at_unix_ms,
        source: targeted_arp_refresh_source().to_string(),
        outcome,
        strong_identity_match,
        throttled,
    }
}

pub fn targeted_arp_refresh_source() -> &'static str {
    targeted_arp_refresh_source_for_platform()
}

#[cfg(target_os = "windows")]
fn targeted_arp_refresh_source_for_platform() -> &'static str {
    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR
}

#[cfg(target_os = "macos")]
fn targeted_arp_refresh_source_for_platform() -> &'static str {
    constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn targeted_arp_refresh_source_for_platform() -> &'static str {
    constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH
}

pub fn unix_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}
