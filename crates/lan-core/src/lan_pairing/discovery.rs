#![forbid(unsafe_code)]

use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

use super::{
    LanDiscoveryActionState, LanDiscoveryDecision, LanDiscoveryInput, LanInterfaceState,
    LanPairingActionState, LanPeerTrustState, LanRelayState,
};

const LAN_DISCOVERY_DECISION_RECORDED_EVENT_TYPE: &str = "lan.discovery.decision-recorded";

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

pub fn evaluate_lan_discovery(input: LanDiscoveryInput) -> LanDiscoveryDecision {
    if input.interface_state == LanInterfaceState::Unavailable {
        return LanDiscoveryDecision {
            discovery_action_state: LanDiscoveryActionState::ManualRequired,
            pairing_action_state: LanPairingActionState::Block,
        };
    }

    LanDiscoveryDecision {
        discovery_action_state: discovery_action_state(input),
        pairing_action_state: pairing_action_state(input),
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

fn discovery_action_state(input: LanDiscoveryInput) -> LanDiscoveryActionState {
    if input.relay_state == LanRelayState::LocalDirect {
        LanDiscoveryActionState::AdvertiseAndListen
    } else {
        LanDiscoveryActionState::ListenOnly
    }
}

fn pairing_action_state(input: LanDiscoveryInput) -> LanPairingActionState {
    if input.peer_trust_state == LanPeerTrustState::Trusted {
        LanPairingActionState::AllowSignedPairing
    } else {
        LanPairingActionState::RequireAiOrManualReview
    }
}
