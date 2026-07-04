use serde::{Deserialize, Serialize};

use super::{deserialize_lan_schema_version_text, LanPairingText};
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
    #[serde(deserialize_with = "deserialize_lan_schema_version_text")]
    pub schema_version: LanPairingText,
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
