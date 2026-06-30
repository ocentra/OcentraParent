use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize,
};

use crate::{constants, LanPairingParentAuthority, ParentEvidenceReference};

#[path = "lan_pairing/device_roles.rs"]
mod device_roles;
pub type DeviceRuntimeRole = device_roles::DeviceRuntimeRole;
pub type DeviceRuntimeRoleState = device_roles::DeviceRuntimeRoleState;
pub type DeviceRuntimeSurface = device_roles::DeviceRuntimeSurface;
pub type DeviceRuntimeRouteState = device_roles::DeviceRuntimeRouteState;
pub type DeviceRuntimeAiProviderState = device_roles::DeviceRuntimeAiProviderState;
pub type DeviceRuntimeLocalAiClaim = device_roles::DeviceRuntimeLocalAiClaim;
pub type DeviceRuntimeRoleEntry = device_roles::DeviceRuntimeRoleEntry;
pub type DeviceRoleRuntimeReadModel = device_roles::DeviceRoleRuntimeReadModel;
#[path = "lan_pairing/discovery_states.rs"]
mod discovery_states;
pub type LanPairingProductionDiscoveryState = discovery_states::LanPairingProductionDiscoveryState;
pub type LanAiProviderRoutingState = discovery_states::LanAiProviderRoutingState;
#[path = "lan_pairing/device_hardware.rs"]
mod device_hardware;
pub type LanPairingDeviceHardwareProfile = device_hardware::LanPairingDeviceHardwareProfile;
#[path = "lan_pairing/household_proof.rs"]
mod household_proof;
pub type V09ProductionDiscoveryHouseholdProofBoundary =
    household_proof::V09ProductionDiscoveryHouseholdProofBoundary;
pub type V09ProductionDiscoveryHouseholdRuntimeOwner =
    household_proof::V09ProductionDiscoveryHouseholdRuntimeOwner;
pub type V09ProductionDiscoveryHouseholdCheck =
    household_proof::V09ProductionDiscoveryHouseholdCheck;
pub type V09ProductionDiscoveryHouseholdSourceState =
    household_proof::V09ProductionDiscoveryHouseholdSourceState;
pub type V09ProductionDiscoveryHouseholdRouteRecoveryState =
    household_proof::V09ProductionDiscoveryHouseholdRouteRecoveryState;
pub type V09ProductionDiscoveryHouseholdProofState =
    household_proof::V09ProductionDiscoveryHouseholdProofState;
pub type V09ProductionDiscoveryHouseholdReadinessDecision =
    household_proof::V09ProductionDiscoveryHouseholdReadinessDecision;
pub type V09ProductionDiscoveryHouseholdManualProofGate =
    household_proof::V09ProductionDiscoveryHouseholdManualProofGate;
pub type V09ProductionDiscoveryHouseholdStateEvidence =
    household_proof::V09ProductionDiscoveryHouseholdStateEvidence;
pub type V09ProductionDiscoveryHouseholdManualChecklistItem =
    household_proof::V09ProductionDiscoveryHouseholdManualChecklistItem;
pub type V09ProductionDiscoveryHouseholdProofReadModel =
    household_proof::V09ProductionDiscoveryHouseholdProofReadModel;

fn deserialize_lan_schema_version<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let version = u16::deserialize(deserializer)?;
    if version == constants::lan_pairing::SCHEMA_VERSION {
        Ok(version)
    } else {
        Err(de::Error::custom(format!(
            "unsupported LAN schema version {version}; expected {}",
            constants::lan_pairing::SCHEMA_VERSION
        )))
    }
}

fn deserialize_lan_schema_version_text<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let version = String::deserialize(deserializer)?;
    if version == constants::lan_pairing::SCHEMA_VERSION_TEXT {
        Ok(version)
    } else {
        Err(de::Error::custom(format!(
            "unsupported LAN schema version {version}; expected {}",
            constants::lan_pairing::SCHEMA_VERSION_TEXT
        )))
    }
}
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

impl LanPairingTrustState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unpaired => constants::value::LAN_PAIRING_UNPAIRED,
            Self::Pairing => constants::value::LAN_PAIRING_PAIRING,
            Self::Paired => constants::value::LAN_PAIRING_PAIRED,
            Self::Revoked => constants::value::LAN_PAIRING_REVOKED,
            Self::Expired => constants::value::LAN_PAIRING_EXPIRED,
        }
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanMdnsAdvertisementLifecycleState {
    Start,
    Update,
    Stop,
    Degraded,
}

impl LanMdnsAdvertisementLifecycleState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start => constants::lan_pairing::MDNS_TXT_VALUE_START,
            Self::Update => constants::lan_pairing::MDNS_TXT_VALUE_UPDATE,
            Self::Stop => constants::lan_pairing::MDNS_TXT_VALUE_STOP,
            Self::Degraded => constants::lan_pairing::MDNS_TXT_VALUE_DEGRADED,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanMdnsAdvertisementSupportState {
    Supported,
    Degraded,
    UnsupportedPlatform,
}

impl LanMdnsAdvertisementSupportState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Supported => constants::lan_pairing::MDNS_TXT_VALUE_SUPPORTED,
            Self::Degraded => constants::lan_pairing::MDNS_TXT_VALUE_DEGRADED,
            Self::UnsupportedPlatform => {
                constants::lan_pairing::MDNS_TXT_VALUE_UNSUPPORTED_PLATFORM
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanMdnsAdvertisementConfirmationState {
    HintOnly,
}

impl LanMdnsAdvertisementConfirmationState {
    pub fn as_str(&self) -> &'static str {
        constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanMdnsTxtRecord {
    pub key: String,
    pub value: String,
}

impl LanMdnsTxtRecord {
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ocentra_eventing::error::EventingError> {
        let key = key.into();
        let value = value.into();
        validate_mdns_atom(constants::lan_pairing::MDNS_TXT_KEY_FIELD, &key)?;
        validate_mdns_atom(constants::lan_pairing::MDNS_TXT_VALUE_FIELD, &value)?;
        Ok(Self { key, value })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanParentMdnsAdvertisement {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub service_type: String,
    pub advertisement_id: String,
    pub protocol_version: String,
    pub family_hash: String,
    pub pairing_state: LanPairingTrustState,
    pub lifecycle_state: LanMdnsAdvertisementLifecycleState,
    pub support_state: LanMdnsAdvertisementSupportState,
    pub confirmation_state: LanMdnsAdvertisementConfirmationState,
    pub txt_records: Vec<LanMdnsTxtRecord>,
}

impl LanParentMdnsAdvertisement {
    pub fn new(
        advertisement_id: impl Into<String>,
        protocol_version: impl Into<String>,
        family_hash: impl Into<String>,
        pairing_state: LanPairingTrustState,
        lifecycle_state: LanMdnsAdvertisementLifecycleState,
        support_state: LanMdnsAdvertisementSupportState,
    ) -> Result<Self, ocentra_eventing::error::EventingError> {
        let advertisement_id = advertisement_id.into();
        let protocol_version = protocol_version.into();
        let family_hash = family_hash.into();
        validate_mdns_atom(
            constants::lan_pairing::MDNS_ADVERTISEMENT_ID_FIELD,
            &advertisement_id,
        )?;
        validate_mdns_atom(
            constants::lan_pairing::MDNS_PROTOCOL_VERSION_FIELD,
            &protocol_version,
        )?;
        validate_mdns_atom(constants::lan_pairing::MDNS_FAMILY_HASH_FIELD, &family_hash)?;
        let txt_records = vec![
            LanMdnsTxtRecord::new(
                constants::lan_pairing::MDNS_TXT_KEY_SCHEMA_VERSION,
                constants::lan_pairing::SCHEMA_VERSION.to_string(),
            )?,
            LanMdnsTxtRecord::new(
                constants::lan_pairing::MDNS_TXT_KEY_PROTOCOL_VERSION,
                protocol_version.clone(),
            )?,
            LanMdnsTxtRecord::new(
                constants::lan_pairing::MDNS_TXT_KEY_FAMILY_HASH,
                family_hash.clone(),
            )?,
            LanMdnsTxtRecord::new(
                constants::lan_pairing::MDNS_TXT_KEY_PAIRING_STATE,
                pairing_state.as_str(),
            )?,
            LanMdnsTxtRecord::new(
                constants::lan_pairing::MDNS_TXT_KEY_LIFECYCLE_STATE,
                lifecycle_state.as_str(),
            )?,
            LanMdnsTxtRecord::new(
                constants::lan_pairing::MDNS_TXT_KEY_SUPPORT_STATE,
                support_state.as_str(),
            )?,
            LanMdnsTxtRecord::new(
                constants::lan_pairing::MDNS_TXT_KEY_CONFIRMATION_STATE,
                LanMdnsAdvertisementConfirmationState::HintOnly.as_str(),
            )?,
        ];

        Ok(Self {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            service_type: constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE.to_string(),
            advertisement_id,
            protocol_version,
            family_hash,
            pairing_state,
            lifecycle_state,
            support_state,
            confirmation_state: LanMdnsAdvertisementConfirmationState::HintOnly,
            txt_records,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanChildMdnsAdvertisement {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub service_type: String,
    pub advertisement_id: String,
    pub opaque_device_id: String,
    pub protocol_version: String,
    pub family_hash: String,
    pub platform: String,
    pub agent_version: String,
    pub pairing_state: LanPairingTrustState,
    pub lifecycle_state: LanMdnsAdvertisementLifecycleState,
    pub support_state: LanMdnsAdvertisementSupportState,
    pub confirmation_state: LanMdnsAdvertisementConfirmationState,
    pub txt_records: Vec<LanMdnsTxtRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanChildMdnsAdvertisementInput {
    pub advertisement_id: String,
    pub opaque_device_id: String,
    pub protocol_version: String,
    pub family_hash: String,
    pub platform: String,
    pub agent_version: String,
    pub pairing_state: LanPairingTrustState,
    pub lifecycle_state: LanMdnsAdvertisementLifecycleState,
    pub support_state: LanMdnsAdvertisementSupportState,
}

impl LanChildMdnsAdvertisement {
    pub fn new(
        input: LanChildMdnsAdvertisementInput,
    ) -> Result<Self, ocentra_eventing::error::EventingError> {
        validate_mdns_advertisement_input(&input)?;
        let LanChildMdnsAdvertisementInput {
            advertisement_id,
            opaque_device_id,
            protocol_version,
            family_hash,
            platform,
            agent_version,
            pairing_state,
            lifecycle_state,
            support_state,
        } = input;
        let txt_records = mdns_advertisement_txt_records(
            &opaque_device_id,
            &protocol_version,
            &family_hash,
            &platform,
            &agent_version,
            &pairing_state,
            &lifecycle_state,
            &support_state,
        )?;

        Ok(Self {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            service_type: constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE.to_string(),
            advertisement_id,
            opaque_device_id,
            protocol_version,
            family_hash,
            platform,
            agent_version,
            pairing_state,
            lifecycle_state,
            support_state,
            confirmation_state: LanMdnsAdvertisementConfirmationState::HintOnly,
            txt_records,
        })
    }
}

fn validate_mdns_advertisement_input(
    input: &LanChildMdnsAdvertisementInput,
) -> Result<(), ocentra_eventing::error::EventingError> {
    validate_mdns_atom(
        constants::lan_pairing::MDNS_ADVERTISEMENT_ID_FIELD,
        &input.advertisement_id,
    )?;
    validate_mdns_atom(
        constants::lan_pairing::MDNS_OPAQUE_DEVICE_ID_FIELD,
        &input.opaque_device_id,
    )?;
    validate_mdns_atom(
        constants::lan_pairing::MDNS_PROTOCOL_VERSION_FIELD,
        &input.protocol_version,
    )?;
    validate_mdns_atom(
        constants::lan_pairing::MDNS_FAMILY_HASH_FIELD,
        &input.family_hash,
    )?;
    validate_mdns_atom(constants::lan_pairing::MDNS_PLATFORM_FIELD, &input.platform)?;
    validate_mdns_atom(
        constants::lan_pairing::MDNS_AGENT_VERSION_FIELD,
        &input.agent_version,
    )
}

fn mdns_advertisement_txt_records(
    opaque_device_id: &str,
    protocol_version: &str,
    family_hash: &str,
    platform: &str,
    agent_version: &str,
    pairing_state: &LanPairingTrustState,
    lifecycle_state: &LanMdnsAdvertisementLifecycleState,
    support_state: &LanMdnsAdvertisementSupportState,
) -> Result<Vec<LanMdnsTxtRecord>, ocentra_eventing::error::EventingError> {
    Ok(vec![
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_SCHEMA_VERSION,
            constants::lan_pairing::SCHEMA_VERSION.to_string(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_PROTOCOL_VERSION,
            protocol_version.to_string(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_OPAQUE_DEVICE_ID,
            opaque_device_id.to_string(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_FAMILY_HASH,
            family_hash.to_string(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_PLATFORM,
            platform.to_string(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_AGENT_VERSION,
            agent_version.to_string(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_PAIRING_STATE,
            pairing_state.as_str(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_LIFECYCLE_STATE,
            lifecycle_state.as_str(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_SUPPORT_STATE,
            support_state.as_str(),
        )?,
        LanMdnsTxtRecord::new(
            constants::lan_pairing::MDNS_TXT_KEY_CONFIRMATION_STATE,
            LanMdnsAdvertisementConfirmationState::HintOnly.as_str(),
        )?,
    ])
}

fn validate_mdns_atom(
    field: &'static str,
    value: &str,
) -> Result<(), ocentra_eventing::error::EventingError> {
    if value.trim().is_empty() {
        return Err(ocentra_eventing::error::EventingError::EmptyValue { field });
    }
    if value.chars().all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.' | ':')
    }) {
        return Ok(());
    }
    Err(ocentra_eventing::error::EventingError::InvalidValue {
        field,
        value: value.to_string(),
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingChallenge {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
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
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
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
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
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
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
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
pub struct LanSignedChildAgentClaim {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub message_kind: LanSignedChildAgentMessageKind,
    pub child_device_id: String,
    pub parent_device_id: String,
    pub install_id: String,
    pub family_hash: String,
    pub child_profile_hash: Option<String>,
    pub platform: String,
    pub hostname: String,
    pub agent_version: String,
    pub local_ips: Vec<String>,
    pub mac_addresses: Vec<String>,
    pub capabilities: Vec<String>,
    pub route_id: String,
    pub nonce: String,
    pub sequence: u64,
    pub issued_at: String,
    pub expires_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanSignedChildAgentEnvelope {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub claim: LanSignedChildAgentClaim,
    pub public_key_base64: String,
    pub public_key_id: String,
    pub signature_base64: String,
    pub signature_algorithm: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanTrustedDeviceRegistryEntry {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
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
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
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
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
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
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
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
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
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
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
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
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
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
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
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
