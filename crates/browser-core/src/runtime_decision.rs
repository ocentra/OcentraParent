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

macro_rules! browser_text_id {
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

browser_text_id!(BrowserRuntimeDecisionId, "browser.runtime_decision_id");
browser_text_id!(BrowserAggregateId, "browser.aggregate_id");

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
    if input.capability_state == BrowserCapabilityState::Missing {
        return BrowserRuntimeDecision {
            observation_intent: BrowserObservationIntent::InventoryObservationOnly,
            runtime_action_state: BrowserRuntimeActionState::ManualRequired,
            ai_handoff_state: BrowserAiHandoffState::NotRequired,
            policy_handoff_state: BrowserPolicyHandoffState::DoNotPublish,
        };
    }

    if input.foreground_state == BrowserForegroundState::Foreground {
        return match input.classification_state {
            BrowserClassificationState::KnownPolicyNavigation => BrowserRuntimeDecision {
                observation_intent: BrowserObservationIntent::KnownPolicyNavigationRequiresPolicy,
                runtime_action_state: BrowserRuntimeActionState::RecordForegroundNavigation,
                ai_handoff_state: BrowserAiHandoffState::NotRequired,
                policy_handoff_state: BrowserPolicyHandoffState::Publish,
            },
            BrowserClassificationState::AmbiguousNavigation => BrowserRuntimeDecision {
                observation_intent: BrowserObservationIntent::AmbiguousNavigationRequiresAi,
                runtime_action_state: BrowserRuntimeActionState::RecordForegroundNavigation,
                ai_handoff_state: BrowserAiHandoffState::Required,
                policy_handoff_state: BrowserPolicyHandoffState::DoNotPublish,
            },
            BrowserClassificationState::InventoryOnly => BrowserRuntimeDecision {
                observation_intent: BrowserObservationIntent::InventoryObservationOnly,
                runtime_action_state: BrowserRuntimeActionState::RecordForegroundNavigation,
                ai_handoff_state: BrowserAiHandoffState::NotRequired,
                policy_handoff_state: BrowserPolicyHandoffState::DoNotPublish,
            },
        };
    }

    BrowserRuntimeDecision {
        observation_intent: BrowserObservationIntent::InventoryObservationOnly,
        runtime_action_state: BrowserRuntimeActionState::RecordInventory,
        ai_handoff_state: BrowserAiHandoffState::NotRequired,
        policy_handoff_state: BrowserPolicyHandoffState::DoNotPublish,
    }
}

pub fn browser_runtime_observed_event(input: BrowserRuntimeInput) -> ChildDomainObservedEvent {
    browser_observed_event(evaluate_browser_runtime(input).observation_intent)
}

pub fn browser_runtime_decision_recorded_event(
    aggregate_id: BrowserAggregateId,
    decision_id: BrowserRuntimeDecisionId,
    input: BrowserRuntimeInput,
) -> BrowserRuntimeDecisionRecordedEvent {
    BrowserRuntimeDecisionRecordedEvent {
        aggregate_id,
        decision_id,
        input,
        decision: evaluate_browser_runtime(input),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        browser_runtime_decision_recorded_event, browser_runtime_observed_event,
        evaluate_browser_runtime, BrowserAggregateId, BrowserAiHandoffState,
        BrowserCapabilityState, BrowserClassificationState, BrowserForegroundState,
        BrowserPolicyHandoffState, BrowserRuntimeActionState, BrowserRuntimeDecisionId,
        BrowserRuntimeInput,
    };
    use crate::{
        browser_ai_analysis_requested_event, browser_evidence_recorded_event,
        browser_policy_evaluation_requested_event, BrowserObservationIntent,
    };
    use ocentra_eventing::expect_value::ExpectValue;
    use ocentra_parent_agent_protocol::child_domain_runtime::{
        child_domain_evidence_ref_from_observation_id,
        child_domain_observation_id_from_subject_ref, ChildDomainAiAnalysisRequestedEvent,
        ChildDomainEvidenceRecordedEvent, ChildRuntimeDomain,
    };

    #[test]
    fn foreground_known_policy_navigation_publishes_policy_without_ai() {
        let input = BrowserRuntimeInput {
            capability_state: BrowserCapabilityState::Supported,
            foreground_state: BrowserForegroundState::Foreground,
            classification_state: BrowserClassificationState::KnownPolicyNavigation,
        };

        let decision = evaluate_browser_runtime(input);
        let observed = browser_runtime_observed_event(input);
        let evidence = browser_evidence_recorded_event(&observed);

        assert_eq!(
            decision.observation_intent,
            BrowserObservationIntent::KnownPolicyNavigationRequiresPolicy
        );
        assert_eq!(
            decision.runtime_action_state,
            BrowserRuntimeActionState::RecordForegroundNavigation
        );
        assert_eq!(screened_ai(&evidence), None);
        assert_eq!(
            browser_policy_evaluation_requested_event(&evidence)
                .expect_value("known navigation should publish policy evidence")
                .evidence_refs,
            vec![evidence.evidence_ref]
        );
        assert_eq!(
            observed.observation_id,
            child_domain_observation_id_from_subject_ref(
                ChildRuntimeDomain::Browser,
                &observed.subject_ref,
                &observed.observed_state
            )
        );
    }

    #[test]
    fn foreground_ambiguous_navigation_routes_to_ai_boundary() {
        let input = BrowserRuntimeInput {
            capability_state: BrowserCapabilityState::Supported,
            foreground_state: BrowserForegroundState::Foreground,
            classification_state: BrowserClassificationState::AmbiguousNavigation,
        };

        let decision = evaluate_browser_runtime(input);
        let observed = browser_runtime_observed_event(input);
        let evidence = browser_evidence_recorded_event(&observed);

        assert_eq!(
            decision.observation_intent,
            BrowserObservationIntent::AmbiguousNavigationRequiresAi
        );
        assert_eq!(
            decision.runtime_action_state,
            BrowserRuntimeActionState::RecordForegroundNavigation
        );
        assert_eq!(
            screened_ai(&evidence)
                .expect_value("ambiguous navigation should request AI")
                .evidence_refs,
            vec![evidence.evidence_ref.clone()]
        );
        assert_eq!(browser_policy_evaluation_requested_event(&evidence), None);
    }

    #[test]
    fn missing_browser_capability_forces_manual_review_without_handoffs() {
        let input = BrowserRuntimeInput {
            capability_state: BrowserCapabilityState::Missing,
            foreground_state: BrowserForegroundState::Foreground,
            classification_state: BrowserClassificationState::KnownPolicyNavigation,
        };

        let decision = evaluate_browser_runtime(input);
        let observed = browser_runtime_observed_event(input);
        let evidence = browser_evidence_recorded_event(&observed);

        assert_eq!(
            decision.observation_intent,
            BrowserObservationIntent::InventoryObservationOnly
        );
        assert_eq!(
            decision.runtime_action_state,
            BrowserRuntimeActionState::ManualRequired
        );
        assert_eq!(screened_ai(&evidence), None);
        assert_eq!(browser_policy_evaluation_requested_event(&evidence), None);
    }

    #[test]
    fn background_inventory_navigation_records_typed_runtime_decision() {
        let input = BrowserRuntimeInput {
            capability_state: BrowserCapabilityState::Supported,
            foreground_state: BrowserForegroundState::Background,
            classification_state: BrowserClassificationState::InventoryOnly,
        };
        let event = browser_runtime_decision_recorded_event(
            BrowserAggregateId::parse("browser.aggregate.1").expect_value("aggregate id"),
            BrowserRuntimeDecisionId::parse("browser.runtime-decision.1")
                .expect_value("decision id"),
            input,
        );

        assert_eq!(
            event.decision.runtime_action_state,
            BrowserRuntimeActionState::RecordInventory
        );
        assert_eq!(
            event.decision.ai_handoff_state,
            BrowserAiHandoffState::NotRequired
        );
        assert_eq!(
            event.decision.policy_handoff_state,
            BrowserPolicyHandoffState::DoNotPublish
        );
    }

    #[test]
    fn browser_runtime_observed_event_drives_derived_evidence_chain() {
        let observed = browser_runtime_observed_event(BrowserRuntimeInput {
            capability_state: BrowserCapabilityState::Supported,
            foreground_state: BrowserForegroundState::Foreground,
            classification_state: BrowserClassificationState::KnownPolicyNavigation,
        });
        let evidence = browser_evidence_recorded_event(&observed);

        assert_eq!(
            evidence.evidence_ref,
            child_domain_evidence_ref_from_observation_id(
                ChildRuntimeDomain::Browser,
                &observed.observation_id
            )
        );
    }

    fn screened_ai(
        evidence: &ChildDomainEvidenceRecordedEvent,
    ) -> Option<ChildDomainAiAnalysisRequestedEvent> {
        browser_ai_analysis_requested_event(evidence)
    }
}
