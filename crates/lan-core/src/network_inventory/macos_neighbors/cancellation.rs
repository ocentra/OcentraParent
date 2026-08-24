use std::{sync::atomic::AtomicBool, time::Duration};

use ocentra_parent_agent_protocol::constants;

use crate::network_inventory::LanNeighborObservation;
use crate::network_inventory_command::command_stdout_with_timeout_and_cancellation;

use super::{macos_arp_observation_with_observed_at, macos_arp_observations_with_observed_at};

pub(super) fn arp_observations(
    observed_at: &str,
    cancellation: Option<&AtomicBool>,
) -> Vec<LanNeighborObservation> {
    let Some(cancellation) = cancellation else {
        return macos_arp_observations_with_observed_at(observed_at);
    };
    command_stdout_with_timeout_and_cancellation(
        "arp",
        &["-a"],
        Duration::from_millis(constants::lan_pairing::LAN_NETWORK_INVENTORY_COMMAND_TIMEOUT_MS),
        cancellation,
    )
    .map(|output| {
        output
            .lines()
            .filter_map(|line| macos_arp_observation_with_observed_at(line, observed_at))
            .collect()
    })
    .unwrap_or_default()
}
