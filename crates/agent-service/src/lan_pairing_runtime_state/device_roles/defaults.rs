use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    DeviceRoleRuntimeReadModel, DeviceRuntimeLocalAiClaim, DeviceRuntimeRole,
    DeviceRuntimeRoleEntry, DeviceRuntimeRoleState, DeviceRuntimeSurface,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;

use crate::time::timestamp_now;

use super::parsing::role_entry;
use super::surface_state::{ai_provider_state, platform_for_surface, route_state_for_surface};

pub(super) fn default_device_role_read_model(
    override_state: Option<(DeviceRuntimeSurface, Vec<DeviceRuntimeRoleEntry>)>,
) -> DeviceRoleRuntimeReadModel {
    let (surface, roles) = override_state.unwrap_or_else(|| {
        (
            DeviceRuntimeSurface::ChildDesktop,
            default_child_agent_roles(),
        )
    });
    let primary_role = roles
        .first()
        .map(|entry| entry.role.clone())
        .unwrap_or(DeviceRuntimeRole::ChildAgent);
    let has_controller = roles
        .iter()
        .any(|entry| entry.role == DeviceRuntimeRole::ParentController);
    let has_ai_provider = roles
        .iter()
        .any(|entry| entry.role == DeviceRuntimeRole::AiProvider);
    DeviceRoleRuntimeReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT
            .to_string()
            .into(),
        physical_device_id: constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string(),
        surface: surface.clone(),
        platform: platform_for_surface(&surface).to_string(),
        roles,
        primary_role,
        controller_lease_id: if has_controller {
            Some(constants::lan_pairing::CONTROLLER_LEASE_ID.to_string())
        } else {
            None
        },
        parent_authority: if has_controller {
            Some(LanPairingParentAuthority::ActiveController)
        } else if matches!(
            surface,
            DeviceRuntimeSurface::ParentMobile | DeviceRuntimeSurface::ParentDesktop
        ) {
            Some(LanPairingParentAuthority::Observer)
        } else {
            None
        },
        selected_route_id: None,
        route_state: route_state_for_surface(&surface),
        lan_ai_provider_state: ai_provider_state(has_ai_provider),
        local_ai_runtime_claim: if has_ai_provider {
            DeviceRuntimeLocalAiClaim::SharedPhysicalDeviceSingleton
        } else {
            DeviceRuntimeLocalAiClaim::None
        },
        updated_at: timestamp_now(),
    }
}

pub(super) fn default_roles_for_surface(
    surface: &DeviceRuntimeSurface,
) -> Vec<DeviceRuntimeRoleEntry> {
    match surface {
        DeviceRuntimeSurface::ParentDesktop => vec![role_entry(
            DeviceRuntimeRole::ParentController,
            DeviceRuntimeRoleState::Implemented,
        )],
        DeviceRuntimeSurface::ParentMobile => vec![role_entry(
            DeviceRuntimeRole::ParentObserver,
            DeviceRuntimeRoleState::Scaffold,
        )],
        DeviceRuntimeSurface::ChildAndroid | DeviceRuntimeSurface::ChildIos => vec![role_entry(
            DeviceRuntimeRole::ChildAgent,
            DeviceRuntimeRoleState::Scaffold,
        )],
        DeviceRuntimeSurface::ChildDesktop => default_child_agent_roles(),
    }
}

fn default_child_agent_roles() -> Vec<DeviceRuntimeRoleEntry> {
    vec![role_entry(
        DeviceRuntimeRole::ChildAgent,
        DeviceRuntimeRoleState::Implemented,
    )]
}
