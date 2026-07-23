use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainObservedEvent;
use serde::{Deserialize, Serialize};

use crate::runtime_ids::{AppAggregateId, AppRuntimeDecisionId};
use crate::{app_observed_event, AppObservationIntent};

pub const APP_RUNTIME_DECISION_SCHEMA_VERSION: u16 = 1;
pub const APP_RUNTIME_DECISION_RECORDED_EVENT_TYPE: &str = "app.runtime.decision-recorded";
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
            SchemaVersion::new(APP_RUNTIME_DECISION_SCHEMA_VERSION)?,
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
        return manual_required_decision();
    }

    if input.foreground_state != AppForegroundState::Foreground {
        return inventory_decision(AppRuntimeActionState::RecordInventory);
    }

    foreground_decision(input.classification_state)
}

fn manual_required_decision() -> AppRuntimeDecision {
    AppRuntimeDecision {
        observation_intent: AppObservationIntent::InventoryObservationOnly,
        runtime_action_state: AppRuntimeActionState::ManualRequired,
        ai_handoff_state: AppAiHandoffState::NotRequired,
        policy_handoff_state: AppPolicyHandoffState::DoNotPublish,
    }
}

fn inventory_decision(runtime_action_state: AppRuntimeActionState) -> AppRuntimeDecision {
    AppRuntimeDecision {
        observation_intent: AppObservationIntent::InventoryObservationOnly,
        runtime_action_state,
        ai_handoff_state: AppAiHandoffState::NotRequired,
        policy_handoff_state: AppPolicyHandoffState::DoNotPublish,
    }
}

fn foreground_decision(classification_state: AppClassificationState) -> AppRuntimeDecision {
    match classification_state {
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
        AppClassificationState::InventoryOnly => {
            inventory_decision(AppRuntimeActionState::RecordInventory)
        }
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
