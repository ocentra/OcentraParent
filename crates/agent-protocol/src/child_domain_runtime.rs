use ocentra_eventing::{
    AggregateKey, DomainEvent, EventContract, EventType, EventingError, IdempotencyKey,
    SchemaVersion,
};
use serde::{Deserialize, Serialize};

use crate::{constants, AGENT_PROTOCOL_SCHEMA_VERSION};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildDomainObservedEvent {
    pub event_type: String,
    pub domain: String,
    pub child_device_id: String,
    pub child_profile_id: String,
    pub observation_id: String,
    pub subject_ref: String,
    pub observed_state: String,
    pub observed_at: String,
    pub requires_ai_analysis: bool,
    pub requires_policy_evaluation: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildDomainEvidenceRecordedEvent {
    pub event_type: String,
    pub domain: String,
    pub child_device_id: String,
    pub child_profile_id: String,
    pub evidence_ref: String,
    pub source_observation_id: String,
    pub signal: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildDomainAiAnalysisRequestedEvent {
    pub event_type: String,
    pub domain: String,
    pub child_device_id: String,
    pub child_profile_id: String,
    pub ai_request_id: String,
    pub evidence_refs: Vec<String>,
    pub allowed_analysis_purpose: String,
    pub raw_private_payload_included: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildDomainPolicyEvaluationRequestedEvent {
    pub event_type: String,
    pub domain: String,
    pub child_device_id: String,
    pub child_profile_id: String,
    pub policy_request_id: String,
    pub evidence_refs: Vec<String>,
    pub source_fact_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildDomainPolicyViolationDetectedEvent {
    pub event_type: String,
    pub domain: String,
    pub child_device_id: String,
    pub child_profile_id: String,
    pub violation_id: String,
    pub policy_rule_ref: String,
    pub severity: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildDomainNotificationRequestedEvent {
    pub event_type: String,
    pub domain: String,
    pub child_device_id: String,
    pub child_profile_id: String,
    pub notification_id: String,
    pub source_policy_violation_id: String,
    pub channel: String,
    pub evidence_refs: Vec<String>,
}

impl DomainEvent for ChildDomainObservedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(&self.domain, &self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, &self.observation_id)
    }
}

impl DomainEvent for ChildDomainEvidenceRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(&self.domain, &self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, &self.evidence_ref)
    }
}

impl DomainEvent for ChildDomainAiAnalysisRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(&self.domain, &self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, &self.ai_request_id)
    }
}

impl DomainEvent for ChildDomainPolicyEvaluationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(&self.domain, &self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, &self.policy_request_id)
    }
}

impl DomainEvent for ChildDomainPolicyViolationDetectedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(&self.domain, &self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, &self.violation_id)
    }
}

impl DomainEvent for ChildDomainNotificationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(&self.domain, &self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, &self.notification_id)
    }
}

pub fn child_domain_ref(domain: &str, suffix: &str) -> String {
    format!(
        "{}{}{}",
        domain,
        constants::child_domain_runtime::IDEMPOTENCY_SEPARATOR,
        suffix
    )
}

fn child_domain_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
    ))
}

fn child_domain_aggregate_key(
    domain: &str,
    child_device_id: &str,
    child_profile_id: &str,
) -> Result<AggregateKey, EventingError> {
    AggregateKey::parse(format!(
        "{}{}{}{}{}",
        domain,
        constants::child_domain_runtime::IDEMPOTENCY_SEPARATOR,
        child_device_id,
        constants::child_domain_runtime::IDEMPOTENCY_SEPARATOR,
        child_profile_id
    ))
}

fn child_domain_idempotency_key(
    event_type: &str,
    unique_ref: &str,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type,
        constants::child_domain_runtime::IDEMPOTENCY_SEPARATOR,
        unique_ref
    ))
}
