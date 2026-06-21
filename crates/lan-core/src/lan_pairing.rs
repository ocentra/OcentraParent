#![forbid(unsafe_code)]

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
use ocentra_parent_agent_protocol::{constants, LAN_PAIRING_SCHEMA_VERSION};
use serde::{Deserialize, Serialize};

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
            SchemaVersion::new(LAN_PAIRING_SCHEMA_VERSION)?,
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
