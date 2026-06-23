use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainObservedEvent;
use serde::{Deserialize, Serialize};

use crate::{app_observed_event, AppObservationIntent};

const APP_SCHEMA_VERSION: u16 = 1;
const APP_RUNTIME_DECISION_RECORDED_EVENT_TYPE: &str = "app.runtime.decision-recorded";
const APP_IDEMPOTENCY_SEPARATOR: &str = ":";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppCapabilityState {
    #[serde(rename = "supported")]
    Supported,
    #[serde(rename = "missing")]
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppForegroundState {
    #[serde(rename = "foreground")]
    Foreground,
    #[serde(rename = "background")]
    Background,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppClassificationState {
    #[serde(rename = "known-policy-app")]
    KnownPolicyApp,
    #[serde(rename = "unknown-app")]
    UnknownApp,
    #[serde(rename = "inventory-only")]
    InventoryOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppRuntimeActionState {
    #[serde(rename = "record-foreground")]
    RecordForeground,
    #[serde(rename = "record-inventory")]
    RecordInventory,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppAiHandoffState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppPolicyHandoffState {
    #[serde(rename = "publish")]
    Publish,
    #[serde(rename = "do-not-publish")]
    DoNotPublish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppRuntimeInput {
    pub capability_state: AppCapabilityState,
    pub foreground_state: AppForegroundState,
    pub classification_state: AppClassificationState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppRuntimeDecision {
    pub observation_intent: AppObservationIntent,
    pub runtime_action_state: AppRuntimeActionState,
    pub ai_handoff_state: AppAiHandoffState,
    pub policy_handoff_state: AppPolicyHandoffState,
}

macro_rules! app_text_id {
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

app_text_id!(AppRuntimeDecisionId, "app.runtime_decision_id");
app_text_id!(AppAggregateId, "app.aggregate_id");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppRuntimeDecisionRecordedEvent {
    pub aggregate_id: AppAggregateId,
    pub decision_id: AppRuntimeDecisionId,
    pub input: AppRuntimeInput,
    pub decision: AppRuntimeDecision,
}

impl DomainEvent for AppRuntimeDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(APP_RUNTIME_DECISION_RECORDED_EVENT_TYPE)?,
            SchemaVersion::new(APP_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "{}{}{}",
            APP_RUNTIME_DECISION_RECORDED_EVENT_TYPE, APP_IDEMPOTENCY_SEPARATOR, self.decision_id,
        ))
    }
}

pub fn evaluate_app_runtime(input: AppRuntimeInput) -> AppRuntimeDecision {
    if input.capability_state == AppCapabilityState::Missing {
        return AppRuntimeDecision {
            observation_intent: AppObservationIntent::InventoryObservationOnly,
            runtime_action_state: AppRuntimeActionState::ManualRequired,
            ai_handoff_state: AppAiHandoffState::NotRequired,
            policy_handoff_state: AppPolicyHandoffState::DoNotPublish,
        };
    }

    if input.foreground_state == AppForegroundState::Foreground {
        return match input.classification_state {
            AppClassificationState::KnownPolicyApp => AppRuntimeDecision {
                observation_intent: AppObservationIntent::ForegroundAppRequiresPolicy,
                runtime_action_state: AppRuntimeActionState::RecordForeground,
                ai_handoff_state: AppAiHandoffState::NotRequired,
                policy_handoff_state: AppPolicyHandoffState::Publish,
            },
            AppClassificationState::UnknownApp => AppRuntimeDecision {
                observation_intent: AppObservationIntent::UnknownAppRequiresAi,
                runtime_action_state: AppRuntimeActionState::RecordForeground,
                ai_handoff_state: AppAiHandoffState::Required,
                policy_handoff_state: AppPolicyHandoffState::DoNotPublish,
            },
            AppClassificationState::InventoryOnly => AppRuntimeDecision {
                observation_intent: AppObservationIntent::InventoryObservationOnly,
                runtime_action_state: AppRuntimeActionState::RecordForeground,
                ai_handoff_state: AppAiHandoffState::NotRequired,
                policy_handoff_state: AppPolicyHandoffState::DoNotPublish,
            },
        };
    }

    AppRuntimeDecision {
        observation_intent: AppObservationIntent::InventoryObservationOnly,
        runtime_action_state: AppRuntimeActionState::RecordInventory,
        ai_handoff_state: AppAiHandoffState::NotRequired,
        policy_handoff_state: AppPolicyHandoffState::DoNotPublish,
    }
}

pub fn app_runtime_observed_event(input: AppRuntimeInput) -> ChildDomainObservedEvent {
    app_observed_event(evaluate_app_runtime(input).observation_intent)
}

pub fn app_runtime_decision_recorded_event(
    aggregate_id: AppAggregateId,
    decision_id: AppRuntimeDecisionId,
    input: AppRuntimeInput,
) -> AppRuntimeDecisionRecordedEvent {
    AppRuntimeDecisionRecordedEvent {
        aggregate_id,
        decision_id,
        input,
        decision: evaluate_app_runtime(input),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        app_runtime_decision_recorded_event, app_runtime_observed_event, evaluate_app_runtime,
        AppAggregateId, AppAiHandoffState, AppCapabilityState, AppClassificationState,
        AppForegroundState, AppPolicyHandoffState, AppRuntimeActionState, AppRuntimeDecisionId,
        AppRuntimeInput,
    };
    use crate::AppObservationIntent;
    use ocentra_eventing::envelope::DomainEvent;
    use ocentra_parent_agent_protocol::child_domain_runtime::{
        child_domain_evidence_ref_from_observation_id,
        child_domain_observation_id_from_subject_ref, ChildRuntimeDomain,
    };

    #[test]
    fn foreground_known_policy_app_publishes_policy_without_ai() {
        let decision = evaluate_app_runtime(AppRuntimeInput {
            capability_state: AppCapabilityState::Supported,
            foreground_state: AppForegroundState::Foreground,
            classification_state: AppClassificationState::KnownPolicyApp,
        });

        assert_eq!(
            decision.observation_intent,
            AppObservationIntent::ForegroundAppRequiresPolicy
        );
        assert_eq!(
            decision.runtime_action_state,
            AppRuntimeActionState::RecordForeground
        );
        assert_eq!(decision.ai_handoff_state, AppAiHandoffState::NotRequired);
        assert_eq!(
            decision.policy_handoff_state,
            AppPolicyHandoffState::Publish
        );
    }

    #[test]
    fn foreground_unknown_app_routes_to_ai_boundary() {
        let input = AppRuntimeInput {
            capability_state: AppCapabilityState::Supported,
            foreground_state: AppForegroundState::Foreground,
            classification_state: AppClassificationState::UnknownApp,
        };
        let decision = evaluate_app_runtime(input);
        let observed = app_runtime_observed_event(AppRuntimeInput {
            capability_state: AppCapabilityState::Supported,
            foreground_state: AppForegroundState::Foreground,
            classification_state: AppClassificationState::UnknownApp,
        });

        assert_eq!(
            decision.observation_intent,
            AppObservationIntent::UnknownAppRequiresAi
        );
        assert_eq!(
            decision.runtime_action_state,
            AppRuntimeActionState::RecordForeground
        );
        assert_eq!(decision.ai_handoff_state, AppAiHandoffState::Required);
        assert_eq!(
            observed.event_type,
            ChildRuntimeDomain::App.observed_event_type()
        );
        assert_eq!(
            observed.observation_id,
            child_domain_observation_id_from_subject_ref(
                ChildRuntimeDomain::App,
                &observed.subject_ref,
                &observed.observed_state
            )
        );
    }

    #[test]
    fn missing_capability_forces_manual_review_without_handoffs() {
        let decision = evaluate_app_runtime(AppRuntimeInput {
            capability_state: AppCapabilityState::Missing,
            foreground_state: AppForegroundState::Foreground,
            classification_state: AppClassificationState::KnownPolicyApp,
        });

        assert_eq!(
            decision.observation_intent,
            AppObservationIntent::InventoryObservationOnly
        );
        assert_eq!(
            decision.runtime_action_state,
            AppRuntimeActionState::ManualRequired
        );
        assert_eq!(decision.ai_handoff_state, AppAiHandoffState::NotRequired);
        assert_eq!(
            decision.policy_handoff_state,
            AppPolicyHandoffState::DoNotPublish
        );
    }

    #[test]
    fn background_inventory_state_records_decision_event_with_typed_contract() {
        let input = AppRuntimeInput {
            capability_state: AppCapabilityState::Supported,
            foreground_state: AppForegroundState::Background,
            classification_state: AppClassificationState::InventoryOnly,
        };
        let aggregate_id_result = AppAggregateId::parse("app.aggregate.child-device-1");
        assert!(
            aggregate_id_result.is_ok(),
            "aggregate id parses: {aggregate_id_result:?}"
        );
        let Ok(aggregate_id) = aggregate_id_result else {
            return;
        };
        let decision_id_result = AppRuntimeDecisionId::parse("app.runtime-decision-1");
        assert!(
            decision_id_result.is_ok(),
            "decision id parses: {decision_id_result:?}"
        );
        let Ok(decision_id) = decision_id_result else {
            return;
        };

        let recorded = app_runtime_decision_recorded_event(aggregate_id, decision_id, input);

        assert_eq!(
            recorded.decision.observation_intent,
            AppObservationIntent::InventoryObservationOnly
        );
        assert_eq!(
            recorded.decision.runtime_action_state,
            AppRuntimeActionState::RecordInventory
        );
        let contract_result = recorded.contract();
        assert!(
            contract_result.is_ok(),
            "app runtime contract parses: {contract_result:?}"
        );
        let Ok(contract) = contract_result else {
            return;
        };
        assert_eq!(
            contract.event_type.as_str(),
            "app.runtime.decision-recorded"
        );
    }

    #[test]
    fn app_runtime_observed_event_drives_derived_evidence_chain() {
        let observed = app_runtime_observed_event(AppRuntimeInput {
            capability_state: AppCapabilityState::Supported,
            foreground_state: AppForegroundState::Foreground,
            classification_state: AppClassificationState::KnownPolicyApp,
        });
        let evidence = crate::app_evidence_recorded_event(&observed);

        assert_eq!(
            evidence.evidence_ref,
            child_domain_evidence_ref_from_observation_id(
                ChildRuntimeDomain::App,
                &observed.observation_id
            )
        );
    }
}
