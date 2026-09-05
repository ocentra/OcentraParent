#![forbid(unsafe_code)]

pub mod discovery;
pub mod observed_events;

mod mdns_lifecycle;
mod signed_child_agent;
mod signed_child_agent_metadata;
pub mod signed_household_mesh_ingress;

use std::collections::BTreeSet;

use ed25519_dalek::VerifyingKey;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanSignedChildAgentClaim, LanSignedChildAgentEnvelope,
};
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-lan-core";

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
    signed_child_agent::verify_lan_signed_child_agent_envelope(
        envelope,
        observed_at,
        context,
        replay_guard,
    )
}

pub fn signed_child_agent_public_key_id(verifying_key: &VerifyingKey) -> String {
    signed_child_agent::signed_child_agent_public_key_id(verifying_key)
}

pub fn default_lan_observed_event(
) -> ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainObservedEvent {
    observed_events::default_lan_observed_event()
}

pub fn lan_observed_event(
    intent: LanObservationIntent,
) -> ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainObservedEvent {
    observed_events::lan_observed_event(intent)
}

pub fn lan_observed_profile(
    intent: LanObservationIntent,
) -> ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainObservedEventProfile {
    observed_events::lan_observed_profile(intent)
}

pub fn lan_evidence_recorded_event(
    event: &ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainObservedEvent,
) -> ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainEvidenceRecordedEvent {
    observed_events::lan_evidence_recorded_event(event)
}

pub fn lan_ai_analysis_requested_event(
    event: &ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainEvidenceRecordedEvent,
) -> Option<ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainAiAnalysisRequestedEvent>
{
    observed_events::lan_ai_analysis_requested_event(event)
}

pub fn lan_policy_evaluation_requested_event(
    event: &ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainEvidenceRecordedEvent,
) -> Option<
    ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainPolicyEvaluationRequestedEvent,
> {
    observed_events::lan_policy_evaluation_requested_event(event)
}

pub fn evaluate_lan_discovery(input: LanDiscoveryInput) -> LanDiscoveryDecision {
    discovery::evaluate_lan_discovery(input)
}

pub fn lan_discovery_decision_recorded_event(
    aggregate_id: discovery::LanAggregateId,
    decision_id: discovery::LanDiscoveryDecisionId,
    input: LanDiscoveryInput,
) -> discovery::LanDiscoveryDecisionRecordedEvent {
    discovery::lan_discovery_decision_recorded_event(aggregate_id, decision_id, input)
}

pub fn evaluate_lan_mdns_advertisement_lifecycle(
    input: LanMdnsAdvertisementLifecycleInput,
) -> LanMdnsAdvertisementLifecycleDecision {
    mdns_lifecycle::evaluate_lan_mdns_advertisement_lifecycle(input)
}
