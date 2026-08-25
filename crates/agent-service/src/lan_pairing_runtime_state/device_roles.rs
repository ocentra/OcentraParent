use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    DeviceRoleRuntimeReadModel, DeviceRuntimeAiProviderState, DeviceRuntimeRoleEntry,
    DeviceRuntimeSurface, LanPairingText,
};

use crate::lan_pairing::LanPairingRuntime;

#[path = "device_roles/defaults.rs"]
mod defaults;
#[path = "device_roles/environment.rs"]
mod environment;
#[path = "device_roles/parsing.rs"]
mod parsing;
#[path = "device_roles/surface_state.rs"]
mod surface_state;

impl LanPairingRuntime {
    pub(crate) fn device_role_read_model(&self) -> DeviceRoleRuntimeReadModel {
        let mut read_model = self.device_roles.clone();
        if let Some(selected) = self.selected_target() {
            read_model.selected_route_id = Some(selected.route_id);
            read_model.route_state =
                ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRouteState::LocalNetwork;
        }
        if let Ok(registry) = self.registry.lock() {
            if let Some(lease) = registry.active_controller_lease() {
                read_model.controller_lease_id = Some(lease.controller_lease_id.clone());
                read_model.parent_authority = Some(
                    ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority::ActiveController,
                );
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

    pub(crate) fn lan_ai_provider_capability_flags(&self) -> LanPairingText {
        if self.lan_ai_provider_capabilities.is_empty() {
            LanPairingText(constants::local_ai_runtime::CAPABILITY_FLAGS_NONE.to_string())
        } else {
            LanPairingText(
                self.lan_ai_provider_capabilities
                    .join(&constants::delimiter::LIST.to_string()),
            )
        }
    }

    pub(crate) fn lan_ai_provider_supports_capability(&self, capability: &LanPairingText) -> bool {
        self.lan_ai_provider_capabilities
            .iter()
            .any(|candidate| candidate == &capability.0)
    }
}

pub(super) fn device_role_read_model_from_env() -> DeviceRoleRuntimeReadModel {
    environment::device_role_read_model_from_env()
}

pub(super) fn default_device_role_read_model(
    override_state: Option<(DeviceRuntimeSurface, Vec<DeviceRuntimeRoleEntry>)>,
) -> DeviceRoleRuntimeReadModel {
    defaults::default_device_role_read_model(override_state)
}

pub(super) fn lan_ai_provider_capabilities_from_env() -> Vec<LanPairingText> {
    environment::lan_ai_provider_capabilities_from_env()
}

pub(super) fn non_empty_env(env_var_name: LanPairingText) -> Option<LanPairingText> {
    std::env::var(env_var_name.0)
        .ok()
        .filter(|value| !value.is_empty())
        .map(LanPairingText)
}
