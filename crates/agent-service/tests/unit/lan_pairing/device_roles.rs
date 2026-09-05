use std::ffi::OsString;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRoleRuntimeReadModel;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeAiProviderState;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeLocalAiClaim;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRole;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRoleEntry;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRoleState;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRouteState;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeSurface;
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;

use crate::{app::lan_pairing::LanPairingRuntime, test_require_ok::require_ok};

#[test]
fn device_role_read_model_reports_dual_role_without_duplicate_ai_runtime_claims() {
    let runtime =
        LanPairingRuntime::empty_with_device_role_read_model(DeviceRoleRuntimeReadModel {
            schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT
                .to_string()
                .into(),
            physical_device_id: constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string(),
            surface: DeviceRuntimeSurface::ParentDesktop,
            platform: constants::local_ai_runtime::PLATFORM_OS_WINDOWS.to_string(),
            roles: vec![
                role_entry(DeviceRuntimeRole::ChildAgent),
                role_entry(DeviceRuntimeRole::ParentController),
                role_entry(DeviceRuntimeRole::AiProvider),
            ],
            primary_role: DeviceRuntimeRole::ParentController,
            controller_lease_id: Some(constants::lan_pairing::CONTROLLER_LEASE_ID.to_string()),
            parent_authority: Some(LanPairingParentAuthority::ActiveController),
            selected_route_id: None,
            route_state: DeviceRuntimeRouteState::Localhost,
            lan_ai_provider_state: DeviceRuntimeAiProviderState::Available,
            local_ai_runtime_claim: DeviceRuntimeLocalAiClaim::SharedPhysicalDeviceSingleton,
            updated_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        });
    let read_model = runtime.device_role_read_model();

    assert_eq!(read_model.roles.len(), 3);
    assert_eq!(read_model.primary_role, DeviceRuntimeRole::ParentController);
    assert_eq!(
        read_model.local_ai_runtime_claim,
        DeviceRuntimeLocalAiClaim::SharedPhysicalDeviceSingleton
    );
    assert_eq!(
        read_model.lan_ai_provider_state,
        DeviceRuntimeAiProviderState::Available
    );
}

#[test]
fn device_role_read_model_defaults_child_mobile_surfaces_to_scaffold_manual_required_routes() {
    assert_child_mobile_surface_defaults(&DeviceRuntimeSurface::ChildAndroid);
    assert_child_mobile_surface_defaults(&DeviceRuntimeSurface::ChildIos);
}

fn role_entry(role: DeviceRuntimeRole) -> DeviceRuntimeRoleEntry {
    DeviceRuntimeRoleEntry {
        role,
        state: DeviceRuntimeRoleState::Implemented,
    }
}

fn assert_child_mobile_surface_defaults(expected_surface: &DeviceRuntimeSurface) {
    let (surface_env_value, expected_platform) =
        if *expected_surface == DeviceRuntimeSurface::ChildAndroid {
            (
                constants::value::DEVICE_RUNTIME_SURFACE_CHILD_ANDROID,
                constants::local_ai_runtime::PLATFORM_OS_ANDROID,
            )
        } else {
            assert_eq!(*expected_surface, DeviceRuntimeSurface::ChildIos);
            (
                constants::value::DEVICE_RUNTIME_SURFACE_CHILD_IOS,
                constants::value::DEVICE_RUNTIME_PLATFORM_IOS,
            )
        };
    let _guard = require_ok(
        crate::LAN_RUNTIME_ENV_LOCK.lock(),
        "lan device-role env lock remains available",
    );
    let previous_registry_path =
        std::env::var_os(constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH);
    let previous_surface = std::env::var_os(constants::lan_pairing::DEVICE_SURFACE_ENV);
    let previous_roles = std::env::var_os(constants::lan_pairing::DEVICE_ROLES_ENV);
    let previous_child_device_id =
        std::env::var_os(constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV);
    let previous_ai_capabilities =
        std::env::var_os(constants::lan_pairing::LAN_AI_PROVIDER_CAPABILITIES_ENV);
    let previous_ai_opt_in = std::env::var_os(constants::lan_pairing::LAN_AI_PROVIDER_OPT_IN_ENV);
    let registry_path =
        std::env::temp_dir().join(format!("lan-device-role-{}.json", surface_env_value));

    std::env::set_var(
        constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH,
        registry_path.as_os_str(),
    );
    std::env::set_var(
        constants::lan_pairing::DEVICE_SURFACE_ENV,
        surface_env_value,
    );
    std::env::remove_var(constants::lan_pairing::DEVICE_ROLES_ENV);
    std::env::set_var(
        constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV,
        format!("{}-child", surface_env_value),
    );
    std::env::remove_var(constants::lan_pairing::LAN_AI_PROVIDER_CAPABILITIES_ENV);
    std::env::remove_var(constants::lan_pairing::LAN_AI_PROVIDER_OPT_IN_ENV);

    let runtime = LanPairingRuntime::from_env();
    let read_model = runtime.device_role_read_model();

    assert_eq!(read_model.surface, *expected_surface);
    assert_eq!(read_model.platform, expected_platform);
    assert_eq!(read_model.primary_role, DeviceRuntimeRole::ChildAgent);
    assert_eq!(read_model.roles.len(), 1);
    assert_eq!(read_model.roles[0].role, DeviceRuntimeRole::ChildAgent);
    assert_eq!(read_model.roles[0].state, DeviceRuntimeRoleState::Scaffold);
    assert_eq!(
        read_model.route_state,
        DeviceRuntimeRouteState::ManualRequired
    );
    assert_eq!(read_model.parent_authority, None);
    assert_eq!(read_model.controller_lease_id, None);
    assert_eq!(
        read_model.lan_ai_provider_state,
        DeviceRuntimeAiProviderState::Unavailable
    );
    assert_eq!(
        read_model.local_ai_runtime_claim,
        DeviceRuntimeLocalAiClaim::None
    );

    restore_env_var(EnvVarSnapshot {
        name: constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH,
        value: previous_registry_path,
    });
    restore_env_var(EnvVarSnapshot {
        name: constants::lan_pairing::DEVICE_SURFACE_ENV,
        value: previous_surface,
    });
    restore_env_var(EnvVarSnapshot {
        name: constants::lan_pairing::DEVICE_ROLES_ENV,
        value: previous_roles,
    });
    restore_env_var(EnvVarSnapshot {
        name: constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV,
        value: previous_child_device_id,
    });
    restore_env_var(EnvVarSnapshot {
        name: constants::lan_pairing::LAN_AI_PROVIDER_CAPABILITIES_ENV,
        value: previous_ai_capabilities,
    });
    restore_env_var(EnvVarSnapshot {
        name: constants::lan_pairing::LAN_AI_PROVIDER_OPT_IN_ENV,
        value: previous_ai_opt_in,
    });
}

struct EnvVarSnapshot {
    name: &'static str,
    value: Option<OsString>,
}

fn restore_env_var(snapshot: EnvVarSnapshot) {
    match snapshot.value {
        Some(value) => std::env::set_var(snapshot.name, value),
        None => std::env::remove_var(snapshot.name),
    }
}
