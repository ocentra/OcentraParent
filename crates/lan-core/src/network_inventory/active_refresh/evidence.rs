use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use ocentra_parent_agent_protocol::constants;

use crate::network_inventory_command::{
    command_succeeded_with_timeout, normalize_mac_address, targeted_arp_probe_commands,
};

use super::super::{LanTargetedArpRefreshEvidence, LanTargetedArpRefreshOutcome};
use super::throttle::targeted_arp_refresh_throttled;
use super::{
    remaining_budget_until, CommandTargetedArpRefreshPacketIo, TargetedArpRefreshObservation,
    TargetedArpRefreshPacketIo, TargetedArpRefreshTarget, TARGETED_ARP_REFRESH_SCAN_BUDGET_MS,
};

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
    enum TargetedArpAttempt<'a> {
        Probed(&'a TargetedArpRefreshTarget),
        Throttled(&'a TargetedArpRefreshTarget),
    }

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
    let observations = if probed && packet_io.has_observation_budget(deadline) {
        Some(observations_by_ip(packet_io.observations(deadline)))
    } else if probed {
        None
    } else {
        Some(HashMap::new())
    };
    let observed_at_unix_ms = unix_timestamp_ms();
    let mut evidence = Vec::new();
    for attempt in attempts {
        match attempt {
            TargetedArpAttempt::Probed(target) => {
                let Some(observations) = observations.as_ref() else {
                    continue;
                };
                let observed_mac_address = observations.get(&target.ip_address).cloned();
                evidence.push(targeted_arp_refresh_evidence_from_observation(
                    target,
                    observed_mac_address,
                    observed_at_unix_ms,
                    false,
                ));
            }
            TargetedArpAttempt::Throttled(target) => {
                evidence.push(targeted_arp_refresh_evidence_from_observation(
                    target,
                    None,
                    observed_at_unix_ms,
                    true,
                ));
            }
        }
    }
    evidence
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
    if cfg!(target_os = "windows") {
        constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR
    } else if cfg!(target_os = "macos") {
        constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP
    } else {
        constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH
    }
}

pub fn unix_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}
