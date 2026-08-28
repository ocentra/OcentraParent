use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdRoleState, LanCanonicalHouseholdRouteState, LanCanonicalHouseholdSurface,
    LanChildAgentInventoryPacket,
};

use super::device_identity::display_name_for;
use super::value_support::child_agent_capabilities;
use crate::network_inventory::api::is_confirmed_agent_status;

pub(super) fn child_agent_inventory_for(
    is_child_agent: bool,
    device: &LanPairingDeviceRef,
    trust_state: LanPairingTrustState,
    route_state: LanCanonicalHouseholdRouteState,
) -> Option<LanChildAgentInventoryPacket> {
    let has_child_agent_truth = device.child_profile_id.is_some()
        || is_confirmed_agent_status(device.agent_status.as_deref())
        || trust_state == LanPairingTrustState::Paired;
    if !is_child_agent || !has_child_agent_truth {
        return None;
    }
    let hardware = device.hardware_profile.clone().unwrap_or_default();
    Some(LanChildAgentInventoryPacket {
        device_name: display_name_for(device),
        platform: device.platform.clone(),
        os: device.platform.clone(),
        cpu_model: hardware.cpu_model,
        cpu_cores: hardware.cpu_cores,
        memory_total: hardware.memory_total,
        gpu_model: hardware.gpu_model,
        gpu_driver: hardware.gpu_driver,
        gpu_memory: hardware.gpu_memory,
        nvidia_smi: hardware.nvidia_smi,
        network_interfaces: device.network_interface.clone().into_iter().collect(),
        capabilities: child_agent_capabilities(),
        role_state: LanCanonicalHouseholdRoleState::Implemented,
        route_state,
        pairing_trust_state: trust_state,
    })
}

pub(super) fn surfaces_for(is_child_agent: bool) -> Vec<LanCanonicalHouseholdSurface> {
    if !is_child_agent {
        return vec![
            LanCanonicalHouseholdSurface::Devices,
            LanCanonicalHouseholdSurface::Network,
        ];
    }
    vec![
        LanCanonicalHouseholdSurface::Devices,
        LanCanonicalHouseholdSurface::Policy,
        LanCanonicalHouseholdSurface::Browser,
        LanCanonicalHouseholdSurface::App,
        LanCanonicalHouseholdSurface::Screen,
        LanCanonicalHouseholdSurface::Network,
        LanCanonicalHouseholdSurface::Activity,
        LanCanonicalHouseholdSurface::Tracking,
        LanCanonicalHouseholdSurface::Ai,
    ]
}
