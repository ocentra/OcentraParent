use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    DeviceRuntimeRole, DeviceRuntimeRoleEntry, DeviceRuntimeRoleState, DeviceRuntimeSurface,
    LanPairingText,
};

pub(super) fn device_role_entries(value: &LanPairingText) -> Vec<DeviceRuntimeRoleEntry> {
    value
        .0
        .split(constants::delimiter::LIST)
        .filter_map(|role| device_runtime_role(&LanPairingText(role.trim().to_string())))
        .map(|role| role_entry(role, DeviceRuntimeRoleState::Implemented))
        .collect()
}

pub(super) fn role_entry(
    role: DeviceRuntimeRole,
    state: DeviceRuntimeRoleState,
) -> DeviceRuntimeRoleEntry {
    DeviceRuntimeRoleEntry { role, state }
}

pub(super) fn device_runtime_role(value: &LanPairingText) -> Option<DeviceRuntimeRole> {
    match value.0.as_str() {
        constants::value::DEVICE_ROLE_PARENT_CONTROLLER => {
            Some(DeviceRuntimeRole::ParentController)
        }
        constants::value::DEVICE_ROLE_PARENT_OBSERVER => Some(DeviceRuntimeRole::ParentObserver),
        constants::value::DEVICE_ROLE_CHILD_AGENT => Some(DeviceRuntimeRole::ChildAgent),
        constants::value::DEVICE_ROLE_AI_PROVIDER => Some(DeviceRuntimeRole::AiProvider),
        _ => None,
    }
}

pub(super) fn device_runtime_surface(value: &LanPairingText) -> DeviceRuntimeSurface {
    match value.0.as_str() {
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
