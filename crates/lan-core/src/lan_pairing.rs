#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use ocentra_parent_agent_protocol::child_domain_runtime::{
    child_domain_ai_analysis_requested_event_if_required,
    child_domain_direct_policy_evaluation_requested_event_if_required,
    child_domain_evidence_recorded_event, child_domain_observed_event,
    ChildDomainAiAnalysisRequestedEvent, ChildDomainAiAnalysisRequirement,
    ChildDomainEvidenceRecordedEvent, ChildDomainObservedEvent, ChildDomainObservedEventProfile,
    ChildDomainObservedSignal, ChildDomainPolicyEvaluationRequestedEvent,
    ChildDomainPolicyEvaluationRequirement, ChildDomainRefSuffix, ChildRuntimeDomain,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanSignedChildAgentClaim, LanSignedChildAgentEnvelope,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const CRATE_NAME: &str = "ocentra-lan-core";
const LAN_DISCOVERY_DECISION_RECORDED_EVENT_TYPE: &str = "lan.discovery.decision-recorded";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LanObservationIntent {
    #[serde(rename = "trusted-presence-requires-policy")]
    TrustedPresenceRequiresPolicy,
    #[serde(rename = "unknown-peer-requires-ai")]
    UnknownPeerRequiresAi,
    #[serde(rename = "discovery-observation-only")]
    DiscoveryObservationOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LanInterfaceState {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LanPeerTrustState {
    #[serde(rename = "trusted")]
    Trusted,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LanRelayState {
    #[serde(rename = "local-direct")]
    LocalDirect,
    #[serde(rename = "relay-required")]
    RelayRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LanDiscoveryActionState {
    #[serde(rename = "advertise-and-listen")]
    AdvertiseAndListen,
    #[serde(rename = "listen-only")]
    ListenOnly,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LanPairingActionState {
    #[serde(rename = "allow-signed-pairing")]
    AllowSignedPairing,
    #[serde(rename = "require-ai-or-manual-review")]
    RequireAiOrManualReview,
    #[serde(rename = "block")]
    Block,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LanMdnsAdvertisementPlatformSupport {
    #[serde(rename = "supported")]
    Supported,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "unsupported-platform")]
    UnsupportedPlatform,
}

impl LanMdnsAdvertisementPlatformSupport {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LanMdnsAdvertisementLifecycleAction {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "degraded")]
    Degraded,
}

impl LanMdnsAdvertisementLifecycleAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start => constants::lan_pairing::MDNS_TXT_VALUE_START,
            Self::Update => constants::lan_pairing::MDNS_TXT_VALUE_UPDATE,
            Self::Stop => constants::lan_pairing::MDNS_TXT_VALUE_STOP,
            Self::Degraded => constants::lan_pairing::MDNS_TXT_VALUE_DEGRADED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanMdnsAdvertisementLifecycleInput {
    pub desired_present: bool,
    pub running: bool,
    pub platform_support: LanMdnsAdvertisementPlatformSupport,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanMdnsAdvertisementLifecycleDecision {
    pub lifecycle_action: LanMdnsAdvertisementLifecycleAction,
    pub hint_only: bool,
    pub platform_support: LanMdnsAdvertisementPlatformSupport,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanSignedChildAgentReplayGuard {
    observed_keys: BTreeSet<String>,
}

impl LanSignedChildAgentReplayGuard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn observed_count(&self) -> usize {
        self.observed_keys.len()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanSignedChildAgentVerificationContext {
    pub expected_parent_device_id: String,
    pub expected_family_hash: String,
    pub expected_route_id: String,
    pub expected_child_device_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanSignedChildAgentVerificationError {
    UnsupportedAlgorithm,
    UnsupportedSchemaVersion,
    EmptyRequiredField,
    InvalidMetadata,
    MalformedTimestamp,
    FutureIssuedAt,
    Expired,
    Replayed,
    WrongFamily,
    WrongParentDevice,
    WrongChildDevice,
    WrongRoute,
    InvalidPublicKey,
    PublicKeyIdMismatch,
    InvalidSignature,
    SignatureRejected,
    SerializationFailed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanDiscoveryInput {
    pub interface_state: LanInterfaceState,
    pub peer_trust_state: LanPeerTrustState,
    pub relay_state: LanRelayState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanDiscoveryDecision {
    pub discovery_action_state: LanDiscoveryActionState,
    pub pairing_action_state: LanPairingActionState,
}

pub fn verify_lan_signed_child_agent_envelope(
    envelope: &LanSignedChildAgentEnvelope,
    observed_at: &str,
    context: &LanSignedChildAgentVerificationContext,
    replay_guard: &mut LanSignedChildAgentReplayGuard,
) -> Result<LanSignedChildAgentClaim, LanSignedChildAgentVerificationError> {
    validate_signed_child_agent_schema(envelope)?;
    validate_signed_child_agent_required_fields(envelope)?;
    validate_signed_child_agent_time_window(&envelope.claim, observed_at)?;

    if envelope.signature_algorithm
        != constants::lan_pairing::SIGNED_CHILD_AGENT_SIGNATURE_ALGORITHM_ED25519
    {
        return Err(LanSignedChildAgentVerificationError::UnsupportedAlgorithm);
    }

    let public_key_bytes = STANDARD
        .decode(&envelope.public_key_base64)
        .map_err(|_error| LanSignedChildAgentVerificationError::InvalidPublicKey)?;
    let key_bytes: [u8; 32] = public_key_bytes
        .as_slice()
        .try_into()
        .map_err(|_error| LanSignedChildAgentVerificationError::InvalidPublicKey)?;
    let verifying_key = VerifyingKey::from_bytes(&key_bytes)
        .map_err(|_error| LanSignedChildAgentVerificationError::InvalidPublicKey)?;
    let expected_key_id = signed_child_agent_public_key_id(&verifying_key);
    if envelope.public_key_id != expected_key_id {
        return Err(LanSignedChildAgentVerificationError::PublicKeyIdMismatch);
    }

    let signature_bytes = STANDARD
        .decode(&envelope.signature_base64)
        .map_err(|_error| LanSignedChildAgentVerificationError::InvalidSignature)?;
    let signature = Signature::from_slice(&signature_bytes)
        .map_err(|_error| LanSignedChildAgentVerificationError::InvalidSignature)?;
    let payload = serde_json::to_vec(&envelope.claim)
        .map_err(|_error| LanSignedChildAgentVerificationError::SerializationFailed)?;
    verifying_key
        .verify(&payload, &signature)
        .map_err(|_error| LanSignedChildAgentVerificationError::SignatureRejected)?;
    validate_signed_child_agent_context(&envelope.claim, context)?;

    let replay_key = signed_child_agent_replay_key(&envelope.claim);
    if !replay_guard.observed_keys.insert(replay_key) {
        return Err(LanSignedChildAgentVerificationError::Replayed);
    }

    Ok(envelope.claim.clone())
}

pub fn signed_child_agent_public_key_id(verifying_key: &VerifyingKey) -> String {
    let digest = Sha256::digest(verifying_key.as_bytes());
    digest[..16]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn validate_signed_child_agent_schema(
    envelope: &LanSignedChildAgentEnvelope,
) -> Result<(), LanSignedChildAgentVerificationError> {
    if envelope.schema_version != constants::lan_pairing::SCHEMA_VERSION
        || envelope.claim.schema_version != constants::lan_pairing::SCHEMA_VERSION
    {
        return Err(LanSignedChildAgentVerificationError::UnsupportedSchemaVersion);
    }
    Ok(())
}

fn validate_signed_child_agent_required_fields(
    envelope: &LanSignedChildAgentEnvelope,
) -> Result<(), LanSignedChildAgentVerificationError> {
    let claim = &envelope.claim;
    let required_fields = [
        envelope.public_key_base64.as_str(),
        envelope.public_key_id.as_str(),
        envelope.signature_base64.as_str(),
        envelope.signature_algorithm.as_str(),
        claim.child_device_id.as_str(),
        claim.parent_device_id.as_str(),
        claim.install_id.as_str(),
        claim.family_hash.as_str(),
        claim.platform.as_str(),
        claim.hostname.as_str(),
        claim.agent_version.as_str(),
        claim.route_id.as_str(),
        claim.nonce.as_str(),
        claim.issued_at.as_str(),
        claim.expires_at.as_str(),
    ];
    if required_fields.iter().any(|value| value.trim().is_empty()) {
        return Err(LanSignedChildAgentVerificationError::EmptyRequiredField);
    }
    if claim.capabilities.is_empty()
        || claim
            .child_profile_hash
            .as_deref()
            .is_some_and(|value| value.trim().is_empty())
        || claim.local_ips.iter().any(|value| value.trim().is_empty())
        || claim
            .mac_addresses
            .iter()
            .any(|value| value.trim().is_empty())
        || claim
            .capabilities
            .iter()
            .any(|value| value.trim().is_empty())
    {
        return Err(LanSignedChildAgentVerificationError::EmptyRequiredField);
    }
    if !signed_child_agent_atom(claim.child_device_id.as_str())
        || !signed_child_agent_atom(claim.parent_device_id.as_str())
        || !signed_child_agent_atom(claim.install_id.as_str())
        || !signed_child_agent_atom(claim.family_hash.as_str())
        || !signed_child_agent_atom(claim.route_id.as_str())
        || !signed_child_agent_atom(claim.nonce.as_str())
        || !signed_child_agent_atom(claim.platform.as_str())
        || !signed_child_agent_atom(claim.hostname.as_str())
        || !signed_child_agent_atom(claim.agent_version.as_str())
        || claim
            .child_profile_hash
            .as_deref()
            .is_some_and(|value| !signed_child_agent_atom(value))
        || claim
            .local_ips
            .iter()
            .any(|value| !signed_child_agent_atom(value))
        || claim
            .mac_addresses
            .iter()
            .any(|value| !signed_child_agent_atom(value))
        || claim
            .capabilities
            .iter()
            .any(|value| !signed_child_agent_atom(value))
    {
        return Err(LanSignedChildAgentVerificationError::InvalidMetadata);
    }
    Ok(())
}

fn validate_signed_child_agent_context(
    claim: &LanSignedChildAgentClaim,
    context: &LanSignedChildAgentVerificationContext,
) -> Result<(), LanSignedChildAgentVerificationError> {
    if claim.family_hash != context.expected_family_hash {
        return Err(LanSignedChildAgentVerificationError::WrongFamily);
    }
    if claim.parent_device_id != context.expected_parent_device_id {
        return Err(LanSignedChildAgentVerificationError::WrongParentDevice);
    }
    if claim.route_id != context.expected_route_id {
        return Err(LanSignedChildAgentVerificationError::WrongRoute);
    }
    if context
        .expected_child_device_id
        .as_ref()
        .is_some_and(|expected_child_device_id| claim.child_device_id != *expected_child_device_id)
    {
        return Err(LanSignedChildAgentVerificationError::WrongChildDevice);
    }
    Ok(())
}

fn signed_child_agent_atom(value: &str) -> bool {
    value.chars().all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.' | ':')
    })
}

fn validate_signed_child_agent_time_window(
    claim: &LanSignedChildAgentClaim,
    observed_at: &str,
) -> Result<(), LanSignedChildAgentVerificationError> {
    let observed_at = parse_signed_child_agent_timestamp(observed_at)?;
    let issued_at = parse_signed_child_agent_timestamp(&claim.issued_at)?;
    let expires_at = parse_signed_child_agent_timestamp(&claim.expires_at)?;
    if issued_at > observed_at {
        return Err(LanSignedChildAgentVerificationError::FutureIssuedAt);
    }
    if expires_at <= observed_at {
        return Err(LanSignedChildAgentVerificationError::Expired);
    }
    Ok(())
}

fn parse_signed_child_agent_timestamp(
    value: &str,
) -> Result<DateTime<Utc>, LanSignedChildAgentVerificationError> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|_error| LanSignedChildAgentVerificationError::MalformedTimestamp)
}

fn signed_child_agent_replay_key(claim: &LanSignedChildAgentClaim) -> String {
    format!(
        "{}|{}|{}|{}",
        claim.child_device_id, claim.route_id, claim.nonce, claim.sequence
    )
}

macro_rules! lan_text_id {
    ($name:ident, $field:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(EventingError::EmptyValue { field: $field });
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

lan_text_id!(LanDiscoveryDecisionId, "lan.discovery_decision_id");
lan_text_id!(LanAggregateId, "lan.aggregate_id");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanDiscoveryDecisionRecordedEvent {
    pub aggregate_id: LanAggregateId,
    pub decision_id: LanDiscoveryDecisionId,
    pub input: LanDiscoveryInput,
    pub decision: LanDiscoveryDecision,
}

impl DomainEvent for LanDiscoveryDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(LAN_DISCOVERY_DECISION_RECORDED_EVENT_TYPE)?,
            SchemaVersion::new(constants::lan_pairing::SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "{}{}{}",
            LAN_DISCOVERY_DECISION_RECORDED_EVENT_TYPE,
            constants::child_domain_runtime::IDEMPOTENCY_SEPARATOR,
            self.decision_id,
        ))
    }
}

pub fn default_lan_observed_event() -> ChildDomainObservedEvent {
    lan_observed_event(LanObservationIntent::TrustedPresenceRequiresPolicy)
}

pub fn lan_observed_event(intent: LanObservationIntent) -> ChildDomainObservedEvent {
    child_domain_observed_event(lan_observed_profile(intent))
}

pub fn lan_observed_profile(intent: LanObservationIntent) -> ChildDomainObservedEventProfile {
    let (observed_state, ai_analysis_requirement, policy_evaluation_requirement) = match intent {
        LanObservationIntent::TrustedPresenceRequiresPolicy => (
            ChildDomainObservedSignal::RequiresPolicy,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        LanObservationIntent::UnknownPeerRequiresAi => (
            ChildDomainObservedSignal::RequiresAi,
            ChildDomainAiAnalysisRequirement::Required,
            ChildDomainPolicyEvaluationRequirement::Required,
        ),
        LanObservationIntent::DiscoveryObservationOnly => (
            ChildDomainObservedSignal::ObserveOnly,
            ChildDomainAiAnalysisRequirement::NotRequired,
            ChildDomainPolicyEvaluationRequirement::NotRequired,
        ),
    };

    ChildDomainObservedEventProfile {
        domain: ChildRuntimeDomain::Lan,
        subject_ref_suffix: ChildDomainRefSuffix::LanSubject,
        observed_state,
        ai_analysis_requirement,
        policy_evaluation_requirement,
    }
}

pub fn lan_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> ChildDomainEvidenceRecordedEvent {
    child_domain_evidence_recorded_event(event)
}

pub fn lan_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainAiAnalysisRequestedEvent> {
    child_domain_ai_analysis_requested_event_if_required(event)
}

pub fn lan_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    child_domain_direct_policy_evaluation_requested_event_if_required(event)
}

pub fn evaluate_lan_discovery(input: LanDiscoveryInput) -> LanDiscoveryDecision {
    if input.interface_state == LanInterfaceState::Unavailable {
        return LanDiscoveryDecision {
            discovery_action_state: LanDiscoveryActionState::ManualRequired,
            pairing_action_state: LanPairingActionState::Block,
        };
    }

    let discovery_action_state = if input.relay_state == LanRelayState::LocalDirect {
        LanDiscoveryActionState::AdvertiseAndListen
    } else {
        LanDiscoveryActionState::ListenOnly
    };
    let pairing_action_state = if input.peer_trust_state == LanPeerTrustState::Trusted {
        LanPairingActionState::AllowSignedPairing
    } else {
        LanPairingActionState::RequireAiOrManualReview
    };

    LanDiscoveryDecision {
        discovery_action_state,
        pairing_action_state,
    }
}

pub fn lan_discovery_decision_recorded_event(
    aggregate_id: LanAggregateId,
    decision_id: LanDiscoveryDecisionId,
    input: LanDiscoveryInput,
) -> LanDiscoveryDecisionRecordedEvent {
    LanDiscoveryDecisionRecordedEvent {
        aggregate_id,
        decision_id,
        input,
        decision: evaluate_lan_discovery(input),
    }
}

pub fn evaluate_lan_mdns_advertisement_lifecycle(
    input: LanMdnsAdvertisementLifecycleInput,
) -> LanMdnsAdvertisementLifecycleDecision {
    let lifecycle_action = if !input.desired_present {
        LanMdnsAdvertisementLifecycleAction::Stop
    } else {
        match input.platform_support {
            LanMdnsAdvertisementPlatformSupport::UnsupportedPlatform => {
                LanMdnsAdvertisementLifecycleAction::Degraded
            }
            LanMdnsAdvertisementPlatformSupport::Degraded => {
                LanMdnsAdvertisementLifecycleAction::Degraded
            }
            LanMdnsAdvertisementPlatformSupport::Supported => {
                if input.running {
                    LanMdnsAdvertisementLifecycleAction::Update
                } else {
                    LanMdnsAdvertisementLifecycleAction::Start
                }
            }
        }
    };

    LanMdnsAdvertisementLifecycleDecision {
        lifecycle_action,
        hint_only: true,
        platform_support: input.platform_support,
    }
}
