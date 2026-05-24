use serde::{Deserialize, Serialize};

use crate::{LanPairingDiscoveryRuntimeStatus, LanPairingTrustState};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingTransport {
    Websocket,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingHttpEndpointSupport {
    PlannedUnsupported,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingPersistenceMode {
    InMemoryFailClosed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingRestartBehavior {
    FailClosedUnpaired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingProofMode {
    DirectProofSubmit,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingRouteRequirement {
    PairedDevice,
    AllowedOrigin,
    TargetDeviceMatch,
    RouteIdMatch,
    UnexpiredIntent,
    NonReplayedIntent,
    UnrevokedPairing,
    SelectedDeviceReachable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingManualProofGap {
    ManualLanBindProof,
    ManualFirewallProof,
    ManualPhysicalDeviceProof,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingUnsupportedHttpEndpoint {
    pub endpoint_id: String,
    pub path: String,
    pub support: LanPairingHttpEndpointSupport,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingRuntimeSupportSurface {
    pub schema_version: u16,
    pub transport: LanPairingTransport,
    #[serde(rename = "supportedWebSocketCommands")]
    pub supported_websocket_commands: Vec<String>,
    pub unsupported_http_endpoints: Vec<LanPairingUnsupportedHttpEndpoint>,
    pub pairing_state: LanPairingTrustState,
    pub trusted_device_count: u32,
    pub discovery_status: LanPairingDiscoveryRuntimeStatus,
    pub challenge_status: LanPairingDiscoveryRuntimeStatus,
    pub proof_preview_status: LanPairingDiscoveryRuntimeStatus,
    pub persistence_mode: LanPairingPersistenceMode,
    pub restart_behavior: LanPairingRestartBehavior,
    pub proof_mode: LanPairingProofMode,
    pub route_requirements: Vec<LanPairingRouteRequirement>,
    pub manual_proof_gaps: Vec<LanPairingManualProofGap>,
}
