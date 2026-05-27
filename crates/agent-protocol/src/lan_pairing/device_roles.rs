use serde::{Deserialize, Serialize};

use crate::LanPairingParentAuthority;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceRuntimeRole {
    ParentController,
    ParentObserver,
    ChildAgent,
    AiProvider,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceRuntimeRoleState {
    Implemented,
    Scaffold,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceRuntimeSurface {
    ParentDesktop,
    ParentMobile,
    ChildDesktop,
    ChildAndroid,
    ChildIos,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceRuntimeRouteState {
    Localhost,
    LocalNetwork,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceRuntimeAiProviderState {
    Available,
    Degraded,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceRuntimeLocalAiClaim {
    None,
    SharedPhysicalDeviceSingleton,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRuntimeRoleEntry {
    pub role: DeviceRuntimeRole,
    pub state: DeviceRuntimeRoleState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRoleRuntimeReadModel {
    pub schema_version: String,
    pub physical_device_id: String,
    pub surface: DeviceRuntimeSurface,
    pub platform: String,
    pub roles: Vec<DeviceRuntimeRoleEntry>,
    pub primary_role: DeviceRuntimeRole,
    pub controller_lease_id: Option<String>,
    pub parent_authority: Option<LanPairingParentAuthority>,
    pub selected_route_id: Option<String>,
    pub route_state: DeviceRuntimeRouteState,
    pub lan_ai_provider_state: DeviceRuntimeAiProviderState,
    pub local_ai_runtime_claim: DeviceRuntimeLocalAiClaim,
    pub updated_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants;

    #[test]
    fn device_role_runtime_read_model_serializes_dual_parent_child_ai_provider_state() {
        let read_model = DeviceRoleRuntimeReadModel {
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
            lan_ai_provider_state: DeviceRuntimeAiProviderState::Available,
            local_ai_runtime_claim: DeviceRuntimeLocalAiClaim::SharedPhysicalDeviceSingleton,
            updated_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        };

        let serialized =
            serde_json::to_string(&read_model).expect(constants::error::AGENT_EVENT_SERIALIZES);

        assert!(serialized.contains(constants::value::DEVICE_ROLE_PARENT_CONTROLLER));
        assert!(serialized.contains(constants::value::DEVICE_ROLE_CHILD_AGENT));
        assert!(serialized.contains(constants::value::DEVICE_ROLE_AI_PROVIDER));
        assert!(
            serialized.contains(constants::value::DEVICE_RUNTIME_LOCAL_AI_CLAIM_SHARED_SINGLETON)
        );
    }

    fn role_entry(role: DeviceRuntimeRole) -> DeviceRuntimeRoleEntry {
        DeviceRuntimeRoleEntry {
            role,
            state: DeviceRuntimeRoleState::Implemented,
        }
    }
}
