use ocentra_parent_agent_protocol::{AgentCommandEnvelope, AgentCommandName};

use crate::lan_network_inventory::{discover_lan_network_devices, LanNetworkInventoryDevice};

pub(super) fn network_devices_for_command(
    command: &AgentCommandEnvelope,
) -> Vec<LanNetworkInventoryDevice> {
    if matches!(
        command.command,
        AgentCommandName::AgentLanPairingAddDeviceRequest
    ) {
        return discover_lan_network_devices();
    }
    Vec::new()
}
