use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainObservedEvent;
use serde::{Deserialize, Serialize};

use crate::{browser_observed_event, BrowserObservationIntent};

const BROWSER_SCHEMA_VERSION: u16 = 1;
const BROWSER_RUNTIME_DECISION_RECORDED_EVENT_TYPE: &str = "browser.runtime.decision-recorded";
const BROWSER_IDEMPOTENCY_SEPARATOR: &str = ":";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserCapabilityState {
    #[serde(rename = "supported")]
    Supported,
    #[serde(rename = "missing")]
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserForegroundState {
    #[serde(rename = "foreground")]
    Foreground,
    #[serde(rename = "background")]
    Background,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserClassificationState {
    #[serde(rename = "known-policy-navigation")]
    KnownPolicyNavigation,
    #[serde(rename = "ambiguous-navigation")]
    AmbiguousNavigation,
    #[serde(rename = "inventory-only")]
    InventoryOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserRuntimeActionState {
    #[serde(rename = "record-foreground-navigation")]
    RecordForegroundNavigation,
    #[serde(rename = "record-inventory")]
    RecordInventory,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserAiHandoffState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyHandoffState {
    #[serde(rename = "publish")]
    Publish,
    #[serde(rename = "do-not-publish")]
    DoNotPublish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserRuntimeInput {
    pub capability_state: BrowserCapabilityState,
    pub foreground_state: BrowserForegroundState,
    pub classification_state: BrowserClassificationState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserRuntimeDecision {
    pub observation_intent: BrowserObservationIntent,
    pub runtime_action_state: BrowserRuntimeActionState,
    pub ai_handoff_state: BrowserAiHandoffState,
    pub policy_handoff_state: BrowserPolicyHandoffState,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct BrowserRuntimeDecisionId(String);

impl BrowserRuntimeDecisionId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(EventingError::EmptyValue {
                field: "browser.runtime_decision_id",
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for BrowserRuntimeDecisionId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<BrowserRuntimeDecisionId> for String {
    fn from(value: BrowserRuntimeDecisionId) -> Self {
        value.0
    }
}

impl std::fmt::Display for BrowserRuntimeDecisionId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct BrowserAggregateId(String);

impl BrowserAggregateId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(EventingError::EmptyValue {
                field: "browser.aggregate_id",
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for BrowserAggregateId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<BrowserAggregateId> for String {
    fn from(value: BrowserAggregateId) -> Self {
        value.0
    }
}

impl std::fmt::Display for BrowserAggregateId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserRuntimeDecisionRecordedEvent {
    pub aggregate_id: BrowserAggregateId,
    pub decision_id: BrowserRuntimeDecisionId,
    pub input: BrowserRuntimeInput,
    pub decision: BrowserRuntimeDecision,
}

impl DomainEvent for BrowserRuntimeDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(BROWSER_RUNTIME_DECISION_RECORDED_EVENT_TYPE)?,
            SchemaVersion::new(BROWSER_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "{}{}{}",
            BROWSER_RUNTIME_DECISION_RECORDED_EVENT_TYPE,
            BROWSER_IDEMPOTENCY_SEPARATOR,
            self.decision_id
        ))
    }
}

pub fn evaluate_browser_runtime(input: BrowserRuntimeInput) -> BrowserRuntimeDecision {
    runtime_decision_impl::evaluate_browser_runtime(input)
}

pub fn browser_runtime_observed_event(input: BrowserRuntimeInput) -> ChildDomainObservedEvent {
    runtime_decision_impl::browser_runtime_observed_event(input)
}

pub fn browser_runtime_decision_recorded_event(
    aggregate_id: BrowserAggregateId,
    decision_id: BrowserRuntimeDecisionId,
    input: BrowserRuntimeInput,
) -> BrowserRuntimeDecisionRecordedEvent {
    runtime_decision_impl::browser_runtime_decision_recorded_event(aggregate_id, decision_id, input)
}

#[path = "../../browser-core-generated/runtime_decision_impl.rs"]
mod runtime_decision_impl;
