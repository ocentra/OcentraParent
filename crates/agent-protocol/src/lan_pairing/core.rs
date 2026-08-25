use serde::{Deserialize, Serialize};

use super::{
    deserialize_lan_schema_version, LanPairingDeviceHardwareProfile, LanPairingOptionalText,
    LanPairingProductionDiscoveryState, LanPairingText,
};
use crate::constants;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingNetworkMode {
    Loopback,
    LocalNetwork,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingTrustState {
    Unpaired,
    Pairing,
    Paired,
    Revoked,
    Expired,
}

const LAN_PAIRING_TRUST_STATE_STRINGS: [&str; 5] = [
    constants::value::LAN_PAIRING_UNPAIRED,
    constants::value::LAN_PAIRING_PAIRING,
    constants::value::LAN_PAIRING_PAIRED,
    constants::value::LAN_PAIRING_REVOKED,
    constants::value::LAN_PAIRING_EXPIRED,
];

impl LanPairingTrustState {
    pub fn as_str(&self) -> LanPairingText {
        LanPairingText(LAN_PAIRING_TRUST_STATE_STRINGS[*self as usize].to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingAuthenticationState {
    Unauthenticated,
    Unpaired,
    Paired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingDeviceReachability {
    Online,
    Offline,
    Stale,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingDiscoveryRuntimeStatus {
    PlannedUnsupported,
    WebsocketDirect,
    NetworkNeighbor,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingIntentKind {
    HealthQuery,
    RuleQuery,
    RuleUpdate,
    ApprovalDecision,
    ConfigurationUpdate,
    ControllerLeaseRenew,
    ControllerLeaseRelease,
    ControllerLeaseTakeover,
    LanAiProviderStatus,
    LanAiJobSubmit,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingResponseState {
    Accepted,
    Rejected,
    Queued,
    Completed,
    Degraded,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanSignedChildAgentMessageKind {
    Hello,
    Heartbeat,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingRejectionReason {
    Anonymous,
    ControllerLeaseMissing,
    ControllerLeaseExpired,
    WrongOrigin,
    WrongDevice,
    WrongController,
    Expired,
    Replayed,
    Malformed,
    Stale,
    Offline,
    Revoked,
    SignedChildAgentContextUnavailable,
    LocalNetworkDisabled,
    UnsupportedRoute,
    UnselectedDevice,
    ObserverReadOnly,
    TakeoverDenied,
    LanAiProviderUnavailable,
    LanAiJobUnauthorized,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingAuditEventType {
    DiscoveryAdvertised,
    PairingChallengeIssued,
    PairingProofAccepted,
    PairingProofRejected,
    ControlAccepted,
    ControlRejected,
    RouteSelected,
    PairingRevoked,
    SelectedDeviceChanged,
    ControllerLeaseRenewed,
    ControllerLeaseReleased,
    ControllerLeaseTakeoverAccepted,
    ControllerLeaseTakeoverRejected,
    LanAiProviderAdvertised,
    LanAiJobAccepted,
    LanAiJobRejected,
    LanAiJobCompleted,
    LanAiJobDegraded,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingDeviceRef {
    pub device_id: String,
    pub child_profile_id: Option<String>,
    pub label: String,
    pub platform: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub install_id: Option<String>,
    #[serde(default)]
    pub ip_address: Option<String>,
    #[serde(default)]
    pub mac_address: Option<String>,
    #[serde(default)]
    pub hostname: Option<String>,
    #[serde(default)]
    pub network_interface: Option<String>,
    #[serde(default)]
    pub agent_status: Option<String>,
    #[serde(default)]
    pub hardware_profile: Option<LanPairingDeviceHardwareProfile>,
}

impl LanPairingDeviceRef {
    pub fn new<D, C, L, P>(device_id: D, child_profile_id: C, label: L, platform: P) -> Self
    where
        D: Into<LanPairingText>,
        C: Into<LanPairingOptionalText>,
        L: Into<LanPairingText>,
        P: Into<LanPairingText>,
    {
        let device_id = device_id.into().0;
        let child_profile_id = child_profile_id.into().0;
        let label = label.into().0;
        let platform = platform.into().0;
        Self {
            device_id,
            child_profile_id,
            label,
            platform,
            install_id: None,
            ip_address: None,
            mac_address: None,
            hostname: None,
            network_interface: None,
            agent_status: None,
            hardware_profile: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingEnablement {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub enabled: bool,
    pub network_mode: LanPairingNetworkMode,
    pub allowed_origins: Vec<String>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingDiscoveryDevice {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub discovered_at: String,
    pub child_device: LanPairingDeviceRef,
    pub agent_peer_id: String,
    pub route_id: String,
    pub network_mode: LanPairingNetworkMode,
    pub reachability: LanPairingDeviceReachability,
    pub address_ref: String,
    pub discovery_status: LanPairingDiscoveryRuntimeStatus,
    pub discovery_state: LanPairingProductionDiscoveryState,
}
