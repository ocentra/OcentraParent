use ocentra_parent_agent_protocol::constants;

use super::TargetedArpProbeCommand;

pub(super) fn targeted_arp_probe_commands(
    ip_address: &str,
    selected_interface: Option<&str>,
) -> Vec<TargetedArpProbeCommand> {
    if cfg!(target_os = "windows") {
        return vec![windows_targeted_arp_probe_command(ip_address)];
    }
    if cfg!(target_os = "linux") {
        return linux_targeted_arp_probe_commands(ip_address, selected_interface);
    }
    Vec::new()
}

fn windows_targeted_arp_probe_command(ip_address: &str) -> TargetedArpProbeCommand {
    TargetedArpProbeCommand {
        program: constants::lan_pairing::PING_EXE,
        args: vec![
            constants::lan_pairing::PING_WINDOWS_COUNT_ARG.to_string(),
            "1".to_string(),
            constants::lan_pairing::PING_WINDOWS_TIMEOUT_ARG.to_string(),
            "200".to_string(),
            ip_address.to_string(),
        ],
    }
}

fn linux_targeted_arp_probe_commands(
    ip_address: &str,
    selected_interface: Option<&str>,
) -> Vec<TargetedArpProbeCommand> {
    let mut commands = optional_linux_arping_command(ip_address, selected_interface)
        .into_iter()
        .collect::<Vec<_>>();
    commands.push(linux_ping_command(ip_address));
    commands
}

fn optional_linux_arping_command(
    ip_address: &str,
    selected_interface: Option<&str>,
) -> Option<TargetedArpProbeCommand> {
    let selected_interface = selected_interface
        .map(str::trim)
        .filter(|value| !value.is_empty())?;
    Some(TargetedArpProbeCommand {
        program: "arping",
        args: vec![
            "-I".to_string(),
            selected_interface.to_string(),
            "-c".to_string(),
            "1".to_string(),
            "-w".to_string(),
            "1".to_string(),
            ip_address.to_string(),
        ],
    })
}

fn linux_ping_command(ip_address: &str) -> TargetedArpProbeCommand {
    TargetedArpProbeCommand {
        program: constants::lan_pairing::PING_EXE,
        args: vec![
            constants::lan_pairing::PING_LINUX_COUNT_ARG.to_string(),
            "1".to_string(),
            constants::lan_pairing::PING_LINUX_TIMEOUT_ARG.to_string(),
            "1".to_string(),
            ip_address.to_string(),
        ],
    }
}
