use serde::{Deserialize, Serialize};

use super::{DeviceRuntimeRoleState, DeviceRuntimeRouteState};
use crate::{
    LanPairingDeviceReachability, LanPairingNetworkMode, LanPairingProductionDiscoveryState,
    LanPairingTrustState,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanCanonicalHouseholdDeviceRole {
    ParentController,
    ParentObserver,
    ChildAgent,
    Portal,
    AiProvider,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanCanonicalHouseholdDeviceClassification {
    ChildAgent,
    NetworkInfrastructure,
    UnsupportedLanDevice,
    UnknownLanDevice,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanCanonicalHouseholdDeviceSource {
    LocalService,
    NetworkNeighbor,
    TrustedRegistry,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanCanonicalHouseholdDeviceConfidence {
    AgentConfirmed,
    MacIpMatch,
    NetworkNeighbor,
    ManualRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanCanonicalHouseholdSurface {
    Devices,
    Policy,
    Browser,
    App,
    Screen,
    Network,
    Activity,
    Tracking,
    Ai,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanCanonicalHouseholdNetworkIdentity {
    #[serde(default)]
    pub hostname: Option<String>,
    #[serde(default)]
    pub ip_addresses: Vec<String>,
    #[serde(default)]
    pub mac_address: Option<String>,
    #[serde(default)]
    pub mac_vendor: Option<String>,
    #[serde(default)]
    pub network_interfaces: Vec<String>,
    pub reachability: LanPairingDeviceReachability,
    pub confidence: LanCanonicalHouseholdDeviceConfidence,
    #[serde(default)]
    pub stale_at: Option<String>,
    #[serde(default)]
    pub offline_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanChildAgentInventoryPacket {
    pub device_name: String,
    pub platform: String,
    pub os: String,
    #[serde(default)]
    pub cpu_model: Option<String>,
    #[serde(default)]
    pub cpu_cores: Option<String>,
    #[serde(default)]
    pub memory_total: Option<String>,
    #[serde(default)]
    pub gpu_model: Option<String>,
    #[serde(default)]
    pub gpu_driver: Option<String>,
    #[serde(default)]
    pub gpu_memory: Option<String>,
    #[serde(default)]
    pub nvidia_smi: Option<String>,
    #[serde(default)]
    pub network_interfaces: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    pub role_state: DeviceRuntimeRoleState,
    pub route_state: DeviceRuntimeRouteState,
    pub pairing_trust_state: LanPairingTrustState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanCanonicalHouseholdDevice {
    pub schema_version: u16,
    pub canonical_device_id: String,
    pub display_name: String,
    pub classification: LanCanonicalHouseholdDeviceClassification,
    pub role_badges: Vec<LanCanonicalHouseholdDeviceRole>,
    pub enrollable: bool,
    pub discovery_state: LanPairingProductionDiscoveryState,
    pub trust_state: LanPairingTrustState,
    #[serde(default)]
    pub route_id: Option<String>,
    pub route_state: DeviceRuntimeRouteState,
    pub network_mode: LanPairingNetworkMode,
    pub source_labels: Vec<LanCanonicalHouseholdDeviceSource>,
    pub network_identity: LanCanonicalHouseholdNetworkIdentity,
    #[serde(default)]
    pub child_agent_inventory: Option<LanChildAgentInventoryPacket>,
    pub policy_target_surfaces: Vec<LanCanonicalHouseholdSurface>,
}
