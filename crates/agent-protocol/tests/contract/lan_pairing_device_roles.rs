use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    DeviceRoleRuntimeReadModel, DeviceRuntimeAiProviderState, DeviceRuntimeLocalAiClaim,
    DeviceRuntimeRole, DeviceRuntimeRoleEntry, DeviceRuntimeRoleState, DeviceRuntimeRouteState,
    DeviceRuntimeSurface,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;

#[test]
fn device_role_runtime_read_model_serializes_dual_parent_child_ai_provider_state() {
    let read_model = DeviceRoleRuntimeReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT
            .to_string()
            .into(),
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
        lan_ai_provider_state: DeviceRuntimeAiProviderState::Available,
        local_ai_runtime_claim: DeviceRuntimeLocalAiClaim::SharedPhysicalDeviceSingleton,
        updated_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
    };

    let serialized = serde_json::to_string(&read_model)
        .expect_value("device role runtime read model must serialize");
    let value: serde_json::Value = serde_json::from_str(&serialized)
        .expect_value("device role runtime read model must deserialize");

    assert_eq!(
        value["roles"][0]["role"],
        serde_json::json!(constants::value::DEVICE_ROLE_PARENT_CONTROLLER)
    );
    assert_eq!(
        value["roles"][1]["role"],
        serde_json::json!(constants::value::DEVICE_ROLE_CHILD_AGENT)
    );
    assert_eq!(
        value["roles"][2]["role"],
        serde_json::json!(constants::value::DEVICE_ROLE_AI_PROVIDER)
    );
    assert_eq!(
        value["localAiRuntimeClaim"],
        serde_json::json!(constants::value::DEVICE_RUNTIME_LOCAL_AI_CLAIM_SHARED_SINGLETON)
    );
}

fn role_entry(role: DeviceRuntimeRole) -> DeviceRuntimeRoleEntry {
    DeviceRuntimeRoleEntry {
        role,
        state: DeviceRuntimeRoleState::Implemented,
    }
}
