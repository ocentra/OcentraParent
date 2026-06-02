use serde::{Deserialize, Serialize};

use crate::{LanPairingParentAuthority, ParentEvidenceReference};

#[path = "lan_pairing/device_roles.rs"]
mod device_roles;
pub use device_roles::*;
#[path = "lan_pairing/discovery_states.rs"]
mod discovery_states;
pub use discovery_states::*;
#[path = "lan_pairing/device_hardware.rs"]
mod device_hardware;
pub use device_hardware::*;
#[path = "lan_pairing/household_device_spine.rs"]
mod household_device_spine;
pub use household_device_spine::*;
#[path = "lan_pairing/household_proof.rs"]
mod household_proof;
pub use household_proof::*;
#[cfg(test)]
#[path = "lan_pairing/household_proof_tests.rs"]
mod household_proof_tests;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingNetworkMode {
    Loopback,
    LocalNetwork,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingTrustState {
    Unpaired,
    Pairing,
    Paired,
    Revoked,
    Expired,
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
    pub fn new(
        device_id: String,
        child_profile_id: Option<String>,
        label: String,
        platform: String,
    ) -> Self {
        Self {
            device_id,
            child_profile_id,
            label,
            platform,
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
    pub schema_version: u16,
    pub enabled: bool,
    pub network_mode: LanPairingNetworkMode,
    pub allowed_origins: Vec<String>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingDiscoveryDevice {
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingChallenge {
    pub schema_version: u16,
    pub challenge_id: String,
    pub child_device: LanPairingDeviceRef,
    pub parent_device: LanPairingDeviceRef,
    pub route_id: String,
    pub origin: String,
    pub issued_at: String,
    pub expires_at: String,
    pub challenge_status: LanPairingDiscoveryRuntimeStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingChallengeRequest {
    pub schema_version: u16,
    pub child_device_id: String,
    pub parent_device_id: String,
    pub route_id: String,
    pub origin: String,
    pub issued_at: String,
    pub expires_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingProofPreview {
    pub schema_version: u16,
    pub challenge_id: String,
    pub child_device_id: String,
    pub parent_device_id: String,
    pub route_id: String,
    pub origin: String,
    pub proof_digest: String,
    pub issued_at: String,
    pub expires_at: String,
    pub proof_preview_status: LanPairingDiscoveryRuntimeStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingProof {
    pub schema_version: u16,
    pub pairing_id: String,
    pub challenge_id: String,
    pub child_device_id: String,
    pub parent_device_id: String,
    pub route_id: String,
    pub origin: String,
    pub proof_digest: String,
    pub issued_at: String,
    pub expires_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanTrustedDeviceRegistryEntry {
    pub schema_version: u16,
    pub pairing_id: String,
    pub child_device: LanPairingDeviceRef,
    pub parent_device: LanPairingDeviceRef,
    pub route_id: String,
    pub origin: String,
    pub proof_digest: String,
    pub trust_state: LanPairingTrustState,
    pub trusted_at: String,
    pub expires_at: String,
    pub revoked_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanSelectedRouteTarget {
    pub schema_version: u16,
    pub selected_child_device_id: String,
    pub route_id: String,
    pub pairing_id: Option<String>,
    pub trust_state: LanPairingTrustState,
    pub network_mode: LanPairingNetworkMode,
    pub reachability: LanPairingDeviceReachability,
    pub stale_at: Option<String>,
    pub offline_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanTrustedDeviceRegistrySnapshot {
    pub schema_version: u16,
    pub entries: Vec<LanTrustedDeviceRegistryEntry>,
    pub selected_target: Option<LanSelectedRouteTarget>,
    pub authentication_state: LanPairingAuthenticationState,
    pub trusted_device_count: u32,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingRouteSelectionRequest {
    pub schema_version: u16,
    pub pairing_id: String,
    pub target_child_device_id: String,
    pub route_id: String,
    pub origin: String,
    pub issued_at: String,
    pub expires_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingRoutingDecision {
    pub schema_version: u16,
    pub intent_id: Option<String>,
    pub target_child_device_id: String,
    pub route_id: String,
    pub pairing_id: Option<String>,
    pub authentication_state: LanPairingAuthenticationState,
    pub state: LanPairingResponseState,
    pub rejection_reason: Option<LanPairingRejectionReason>,
    pub audit_event_id: String,
    pub decided_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanParentIntentEnvelope {
    pub schema_version: u16,
    pub intent_id: String,
    pub intent_kind: LanPairingIntentKind,
    pub target_child_device_id: String,
    pub route_id: String,
    pub pairing_id: String,
    pub proof_digest: String,
    pub origin: String,
    pub issued_at: String,
    pub expires_at: String,
    pub controller_lease_id: String,
    pub controller_device_id: String,
    pub parent_actor_id: String,
    pub parent_authority: LanPairingParentAuthority,
    pub controller_lease_issued_at: String,
    pub controller_lease_expires_at: String,
    pub evidence_references: Vec<ParentEvidenceReference>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanChildAgentResponse {
    pub schema_version: u16,
    pub intent_id: String,
    pub target_child_device_id: String,
    pub route_id: String,
    pub state: LanPairingResponseState,
    pub rejection_reason: Option<LanPairingRejectionReason>,
    pub audit_event_id: String,
    pub responded_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingAuditEvent {
    pub schema_version: u16,
    pub audit_event_id: String,
    pub event_type: LanPairingAuditEventType,
    pub pairing_id: Option<String>,
    pub intent_id: Option<String>,
    pub child_device_id: Option<String>,
    pub parent_device_id: Option<String>,
    pub controller_lease_id: Option<String>,
    pub controller_device_id: Option<String>,
    pub parent_actor_id: Option<String>,
    pub route_id: String,
    pub origin: Option<String>,
    pub rejection_reason: Option<LanPairingRejectionReason>,
    pub observed_at: String,
    pub evidence_references: Vec<ParentEvidenceReference>,
}
