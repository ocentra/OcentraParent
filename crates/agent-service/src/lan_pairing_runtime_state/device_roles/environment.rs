use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    DeviceRoleRuntimeReadModel, DeviceRuntimeSurface, LanPairingText,
};

use super::defaults::{default_device_role_read_model, default_roles_for_surface};
use super::non_empty_env;
use super::parsing::{device_role_entries, device_runtime_surface};

pub(super) fn device_role_read_model_from_env() -> DeviceRoleRuntimeReadModel {
    let surface = non_empty_env(LanPairingText(
        constants::lan_pairing::DEVICE_SURFACE_ENV.to_string(),
    ))
    .map(|value| device_runtime_surface(&value))
    .unwrap_or(DeviceRuntimeSurface::ChildDesktop);
    let roles = non_empty_env(LanPairingText(
        constants::lan_pairing::DEVICE_ROLES_ENV.to_string(),
    ))
    .map(|value| device_role_entries(&value))
    .filter(|entries| !entries.is_empty())
    .unwrap_or_else(|| default_roles_for_surface(&surface));
    default_device_role_read_model(Some((surface, roles)))
}

pub(super) fn lan_ai_provider_capabilities_from_env() -> Vec<LanPairingText> {
    match non_empty_env(LanPairingText(
        constants::lan_pairing::LAN_AI_PROVIDER_CAPABILITIES_ENV.to_string(),
    )) {
        Some(value) => value
            .0
            .split(constants::delimiter::LIST)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| LanPairingText(value.to_owned()))
            .collect(),
        None => Vec::new(),
    }
}
