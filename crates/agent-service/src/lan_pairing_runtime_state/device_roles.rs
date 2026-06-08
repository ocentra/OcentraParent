use ocentra_parent_agent_protocol::{
    constants, DeviceRoleRuntimeReadModel, DeviceRuntimeAiProviderState, DeviceRuntimeLocalAiClaim,
    DeviceRuntimeRole, DeviceRuntimeRoleEntry, DeviceRuntimeRoleState, DeviceRuntimeRouteState,
    DeviceRuntimeSurface, LanPairingParentAuthority,
};

use crate::{lan_pairing::LanPairingRuntime, time::timestamp_now};

impl LanPairingRuntime {
    #[cfg(test)]
    pub fn empty_with_device_role_read_model(device_roles: DeviceRoleRuntimeReadModel) -> Self {
        let mut runtime = Self::empty();
        runtime.device_roles = device_roles;
        runtime.lan_ai_provider_capabilities = vec![
            constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION.to_string(),
            constants::local_ai_runtime::CAPABILITY_SUMMARIZATION.to_string(),
        ];
        runtime
    }

    pub(crate) fn device_role_read_model(&self) -> DeviceRoleRuntimeReadModel {
        let mut read_model = self.device_roles.clone();
        if let Some(selected) = self.selected_target() {
            read_model.selected_route_id = Some(selected.route_id);
            read_model.route_state = DeviceRuntimeRouteState::LocalNetwork;
        }
        if let Ok(active_lease) = self.controller_lease.lock() {
            if let Some(lease) = active_lease.as_ref() {
                read_model.controller_lease_id = Some(lease.controller_lease_id.clone());
                read_model.parent_authority = Some(LanPairingParentAuthority::ActiveController);
            }
        }
        read_model
    }

    pub(crate) fn lan_ai_provider_available(&self) -> bool {
        self.device_role_read_model().lan_ai_provider_state
            == DeviceRuntimeAiProviderState::Available
            && !self.lan_ai_provider_capabilities.is_empty()
            && self.lan_ai_provider_heartbeat_allows_routing()
            && !self.lan_ai_provider_busy()
    }

    pub(crate) fn lan_ai_provider_capability_flags(&self) -> String {
        if self.lan_ai_provider_capabilities.is_empty() {
            constants::local_ai_runtime::CAPABILITY_FLAGS_NONE.to_string()
        } else {
            self.lan_ai_provider_capabilities
                .join(&constants::delimiter::LIST.to_string())
        }
    }

    pub(crate) fn lan_ai_provider_supports_capability(&self, capability: &str) -> bool {
        self.lan_ai_provider_capabilities
            .iter()
            .any(|candidate| candidate == capability)
    }
}

pub(super) fn device_role_read_model_from_env() -> DeviceRoleRuntimeReadModel {
    let surface = non_empty_env(constants::lan_pairing::DEVICE_SURFACE_ENV)
        .as_deref()
        .map(device_runtime_surface)
        .unwrap_or(DeviceRuntimeSurface::ChildDesktop);
    let roles = non_empty_env(constants::lan_pairing::DEVICE_ROLES_ENV)
        .map(|value| device_role_entries(&value))
        .filter(|entries| !entries.is_empty())
        .unwrap_or_else(|| default_roles_for_surface(&surface));
    default_device_role_read_model(Some((surface, roles)))
}

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
        schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string(),
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

pub(super) fn lan_ai_provider_capabilities_from_env() -> Vec<String> {
    match non_empty_env(constants::lan_pairing::LAN_AI_PROVIDER_CAPABILITIES_ENV) {
        Some(value) => value
            .split(constants::delimiter::LIST)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .collect(),
        None => Vec::new(),
    }
}

pub(super) fn non_empty_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.is_empty())
}

fn default_roles_for_surface(surface: &DeviceRuntimeSurface) -> Vec<DeviceRuntimeRoleEntry> {
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

fn device_role_entries(value: &str) -> Vec<DeviceRuntimeRoleEntry> {
    value
        .split(constants::delimiter::LIST)
        .filter_map(|role| device_runtime_role(role.trim()))
        .map(|role| role_entry(role, DeviceRuntimeRoleState::Implemented))
        .collect()
}

fn role_entry(role: DeviceRuntimeRole, state: DeviceRuntimeRoleState) -> DeviceRuntimeRoleEntry {
    DeviceRuntimeRoleEntry { role, state }
}

fn device_runtime_role(value: &str) -> Option<DeviceRuntimeRole> {
    match value {
        constants::value::DEVICE_ROLE_PARENT_CONTROLLER => {
            Some(DeviceRuntimeRole::ParentController)
        }
        constants::value::DEVICE_ROLE_PARENT_OBSERVER => Some(DeviceRuntimeRole::ParentObserver),
        constants::value::DEVICE_ROLE_CHILD_AGENT => Some(DeviceRuntimeRole::ChildAgent),
        constants::value::DEVICE_ROLE_AI_PROVIDER => Some(DeviceRuntimeRole::AiProvider),
        _ => None,
    }
}

fn device_runtime_surface(value: &str) -> DeviceRuntimeSurface {
    match value {
        constants::value::DEVICE_RUNTIME_SURFACE_PARENT_DESKTOP => {
            DeviceRuntimeSurface::ParentDesktop
        }
        constants::value::DEVICE_RUNTIME_SURFACE_PARENT_MOBILE => {
            DeviceRuntimeSurface::ParentMobile
        }
        constants::value::DEVICE_RUNTIME_SURFACE_CHILD_ANDROID => {
            DeviceRuntimeSurface::ChildAndroid
        }
        constants::value::DEVICE_RUNTIME_SURFACE_CHILD_IOS => DeviceRuntimeSurface::ChildIos,
        _ => DeviceRuntimeSurface::ChildDesktop,
    }
}

fn platform_for_surface(surface: &DeviceRuntimeSurface) -> &'static str {
    match surface {
        DeviceRuntimeSurface::ParentMobile | DeviceRuntimeSurface::ChildAndroid => {
            constants::local_ai_runtime::PLATFORM_OS_ANDROID
        }
        DeviceRuntimeSurface::ChildIos => constants::value::DEVICE_RUNTIME_PLATFORM_IOS,
        DeviceRuntimeSurface::ParentDesktop | DeviceRuntimeSurface::ChildDesktop => {
            constants::local_ai_runtime::PLATFORM_OS_WINDOWS
        }
    }
}

fn route_state_for_surface(surface: &DeviceRuntimeSurface) -> DeviceRuntimeRouteState {
    match surface {
        DeviceRuntimeSurface::ParentMobile
        | DeviceRuntimeSurface::ChildAndroid
        | DeviceRuntimeSurface::ChildIos => DeviceRuntimeRouteState::ManualRequired,
        DeviceRuntimeSurface::ParentDesktop | DeviceRuntimeSurface::ChildDesktop => {
            DeviceRuntimeRouteState::Localhost
        }
    }
}

fn ai_provider_state(has_ai_provider: bool) -> DeviceRuntimeAiProviderState {
    match (
        has_ai_provider,
        non_empty_env(constants::lan_pairing::LAN_AI_PROVIDER_OPT_IN_ENV).as_deref(),
    ) {
        (true, Some(constants::value::TRUE)) => DeviceRuntimeAiProviderState::Available,
        (true, _) => DeviceRuntimeAiProviderState::Degraded,
        (false, _) => DeviceRuntimeAiProviderState::Unavailable,
    }
}
