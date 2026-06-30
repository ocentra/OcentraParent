use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainObservedEvent;
use serde::{Deserialize, Serialize};

use crate::{app_game_observed_event, AppGameObservationIntent};

const APP_GAME_SCHEMA_VERSION: u16 = 1;
const APP_GAME_RUNTIME_DECISION_RECORDED_EVENT_TYPE: &str = "app-game.runtime.decision-recorded";
const APP_GAME_IDEMPOTENCY_SEPARATOR: &str = ":";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppGameCapabilityState {
    #[serde(rename = "supported")]
    Supported,
    #[serde(rename = "missing")]
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppGameForegroundState {
    #[serde(rename = "foreground")]
    Foreground,
    #[serde(rename = "background")]
    Background,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppGameClassificationState {
    #[serde(rename = "known-game")]
    KnownGame,
    #[serde(rename = "unknown-game")]
    UnknownGame,
    #[serde(rename = "inventory-only")]
    InventoryOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppGameRuntimeActionState {
    #[serde(rename = "record-foreground-session")]
    RecordForegroundSession,
    #[serde(rename = "record-inventory")]
    RecordInventory,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppGameAiHandoffState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppGamePolicyHandoffState {
    #[serde(rename = "publish")]
    Publish,
    #[serde(rename = "do-not-publish")]
    DoNotPublish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameRuntimeInput {
    pub capability_state: AppGameCapabilityState,
    pub foreground_state: AppGameForegroundState,
    pub classification_state: AppGameClassificationState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameRuntimeDecision {
    pub observation_intent: AppGameObservationIntent,
    pub runtime_action_state: AppGameRuntimeActionState,
    pub ai_handoff_state: AppGameAiHandoffState,
    pub policy_handoff_state: AppGamePolicyHandoffState,
}

macro_rules! app_game_text_id {
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

app_game_text_id!(AppGameRuntimeDecisionId, "app_game.runtime_decision_id");
app_game_text_id!(AppGameAggregateId, "app_game.aggregate_id");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameRuntimeDecisionRecordedEvent {
    pub aggregate_id: AppGameAggregateId,
    pub decision_id: AppGameRuntimeDecisionId,
    pub input: AppGameRuntimeInput,
    pub decision: AppGameRuntimeDecision,
}

impl DomainEvent for AppGameRuntimeDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(APP_GAME_RUNTIME_DECISION_RECORDED_EVENT_TYPE)?,
            SchemaVersion::new(APP_GAME_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "{}{}{}",
            APP_GAME_RUNTIME_DECISION_RECORDED_EVENT_TYPE,
            APP_GAME_IDEMPOTENCY_SEPARATOR,
            self.decision_id
        ))
    }
}

pub fn evaluate_app_game_runtime(input: AppGameRuntimeInput) -> AppGameRuntimeDecision {
    if input.capability_state == AppGameCapabilityState::Missing {
        return AppGameRuntimeDecision {
            observation_intent: AppGameObservationIntent::InventoryObservationOnly,
            runtime_action_state: AppGameRuntimeActionState::ManualRequired,
            ai_handoff_state: AppGameAiHandoffState::NotRequired,
            policy_handoff_state: AppGamePolicyHandoffState::DoNotPublish,
        };
    }

    if input.foreground_state == AppGameForegroundState::Foreground {
        return match input.classification_state {
            AppGameClassificationState::KnownGame => AppGameRuntimeDecision {
                observation_intent: AppGameObservationIntent::ForegroundUsageRequiresPolicy,
                runtime_action_state: AppGameRuntimeActionState::RecordForegroundSession,
                ai_handoff_state: AppGameAiHandoffState::NotRequired,
                policy_handoff_state: AppGamePolicyHandoffState::Publish,
            },
            AppGameClassificationState::UnknownGame => AppGameRuntimeDecision {
                observation_intent: AppGameObservationIntent::AmbiguousUsageRequiresAi,
                runtime_action_state: AppGameRuntimeActionState::RecordForegroundSession,
                ai_handoff_state: AppGameAiHandoffState::Required,
                policy_handoff_state: AppGamePolicyHandoffState::DoNotPublish,
            },
            AppGameClassificationState::InventoryOnly => AppGameRuntimeDecision {
                observation_intent: AppGameObservationIntent::InventoryObservationOnly,
                runtime_action_state: AppGameRuntimeActionState::RecordForegroundSession,
                ai_handoff_state: AppGameAiHandoffState::NotRequired,
                policy_handoff_state: AppGamePolicyHandoffState::DoNotPublish,
            },
        };
    }

    AppGameRuntimeDecision {
        observation_intent: AppGameObservationIntent::InventoryObservationOnly,
        runtime_action_state: AppGameRuntimeActionState::RecordInventory,
        ai_handoff_state: AppGameAiHandoffState::NotRequired,
        policy_handoff_state: AppGamePolicyHandoffState::DoNotPublish,
    }
}

pub fn app_game_runtime_observed_event(input: AppGameRuntimeInput) -> ChildDomainObservedEvent {
    app_game_observed_event(evaluate_app_game_runtime(input).observation_intent)
}

pub fn app_game_runtime_decision_recorded_event(
    aggregate_id: AppGameAggregateId,
    decision_id: AppGameRuntimeDecisionId,
    input: AppGameRuntimeInput,
) -> AppGameRuntimeDecisionRecordedEvent {
    AppGameRuntimeDecisionRecordedEvent {
        aggregate_id,
        decision_id,
        input,
        decision: evaluate_app_game_runtime(input),
    }
}
