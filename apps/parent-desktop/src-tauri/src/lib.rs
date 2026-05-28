use std::{
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use ocentra_parent_agent_protocol::{
    constants, DeviceRoleRuntimeReadModel, DeviceRuntimeAiProviderState,
    DeviceRuntimeLocalAiClaim, DeviceRuntimeRole, DeviceRuntimeRoleEntry, DeviceRuntimeRoleState,
    DeviceRuntimeRouteState, DeviceRuntimeSurface, LanPairingParentAuthority,
};
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentDesktopPlatformProofState {
    service_state: String,
    agent_address: String,
    controller_lease_state: String,
    device_role_state: DeviceRoleRuntimeReadModel,
    activity_adapter_state: String,
    parent_assistant_provider_state: DeviceRuntimeAiProviderState,
    route_state: DeviceRuntimeRouteState,
    lan_ai_provider_state: DeviceRuntimeAiProviderState,
    backend_kind: String,
}

#[tauri::command]
fn parent_platform_proof_state() -> ParentDesktopPlatformProofState {
    parent_platform_proof_state_for_address(configured_agent_address())
}

pub fn run() {
    let result = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parent_platform_proof_state])
        .run(tauri::generate_context!());
    if let Err(error) = result {
        panic!("{error}");
    }
}

fn configured_agent_address() -> String {
    std::env::var(constants::env_var::AGENT_ADDR)
        .unwrap_or_else(|_| constants::bind::DEFAULT_AGENT_ADDR.to_string())
}

fn parent_platform_proof_state_for_address(agent_address: String) -> ParentDesktopPlatformProofState {
    let service_state = if agent_service_connects(&agent_address) {
        constants::value::PARENT_DESKTOP_SERVICE_CONNECTED
    } else {
        constants::value::PARENT_DESKTOP_SERVICE_UNAVAILABLE
    };
    let device_role_state = parent_desktop_device_role_state();
    ParentDesktopPlatformProofState {
        service_state: service_state.to_string(),
        agent_address,
        controller_lease_state: constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER
            .to_string(),
        activity_adapter_state: service_state.to_string(),
        parent_assistant_provider_state: device_role_state.lan_ai_provider_state.clone(),
        route_state: device_role_state.route_state.clone(),
        lan_ai_provider_state: device_role_state.lan_ai_provider_state.clone(),
        device_role_state,
        backend_kind: constants::value::PARENT_DESKTOP_BACKEND_RUST_SERVICE.to_string(),
    }
}

fn agent_service_connects(agent_address: &str) -> bool {
    agent_address
        .parse::<SocketAddr>()
        .ok()
        .and_then(|address| TcpStream::connect_timeout(&address, Duration::from_millis(250)).ok())
        .is_some()
}

fn parent_desktop_device_role_state() -> DeviceRoleRuntimeReadModel {
    DeviceRoleRuntimeReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string(),
        physical_device_id: constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string(),
        surface: DeviceRuntimeSurface::ParentDesktop,
        platform: constants::local_ai_runtime::PLATFORM_OS_WINDOWS.to_string(),
        roles: vec![
            role_entry(DeviceRuntimeRole::ParentController),
            role_entry(DeviceRuntimeRole::ChildAgent),
            role_entry(DeviceRuntimeRole::AiProvider),
        ],
        primary_role: DeviceRuntimeRole::ParentController,
        controller_lease_id: Some(constants::lan_pairing::CONTROLLER_LEASE_ID.to_string()),
        parent_authority: Some(LanPairingParentAuthority::ActiveController),
        selected_route_id: Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string()),
        route_state: DeviceRuntimeRouteState::LocalNetwork,
        lan_ai_provider_state: DeviceRuntimeAiProviderState::Degraded,
        local_ai_runtime_claim: DeviceRuntimeLocalAiClaim::SharedPhysicalDeviceSingleton,
        updated_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
    }
}

fn role_entry(role: DeviceRuntimeRole) -> DeviceRuntimeRoleEntry {
    DeviceRuntimeRoleEntry {
        role,
        state: DeviceRuntimeRoleState::Implemented,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parent_platform_proof_state_uses_rust_service_connection_for_package_runtime() {
        let state =
            parent_platform_proof_state_for_address(constants::test_network::LOOPBACK_ANY_PORT.to_string());

        assert_eq!(
            state.service_state,
            constants::value::PARENT_DESKTOP_SERVICE_UNAVAILABLE
        );
        assert_eq!(
            state.backend_kind,
            constants::value::PARENT_DESKTOP_BACKEND_RUST_SERVICE
        );
        assert_eq!(state.route_state, DeviceRuntimeRouteState::LocalNetwork);
        assert_eq!(
            state.lan_ai_provider_state,
            DeviceRuntimeAiProviderState::Degraded
        );
        assert_eq!(
            state.activity_adapter_state,
            constants::value::PARENT_DESKTOP_SERVICE_UNAVAILABLE
        );
        assert_eq!(
            state.parent_assistant_provider_state,
            DeviceRuntimeAiProviderState::Degraded
        );
        assert_eq!(
            state.device_role_state.local_ai_runtime_claim,
            DeviceRuntimeLocalAiClaim::SharedPhysicalDeviceSingleton
        );
    }
}
