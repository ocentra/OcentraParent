use ocentra_parent_agent_protocol::{
    constants, DeviceRoleRuntimeReadModel, DeviceRuntimeAiProviderState, DeviceRuntimeLocalAiClaim,
    DeviceRuntimeRole, DeviceRuntimeRoleEntry, DeviceRuntimeRoleState, DeviceRuntimeRouteState,
    DeviceRuntimeSurface, LanPairingParentAuthority,
};

use crate::lan_pairing::LanPairingRuntime;

#[test]
fn device_role_read_model_reports_dual_role_without_duplicate_ai_runtime_claims() {
    let runtime =
        LanPairingRuntime::empty_with_device_role_read_model(DeviceRoleRuntimeReadModel {
            schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string(),
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

fn role_entry(role: DeviceRuntimeRole) -> DeviceRuntimeRoleEntry {
    DeviceRuntimeRoleEntry {
        role,
        state: DeviceRuntimeRoleState::Implemented,
    }
}
