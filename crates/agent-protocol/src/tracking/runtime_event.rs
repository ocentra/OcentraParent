use ocentra_eventing::{
    AggregateKey, DomainEvent, EventContract, EventType, EventingError, IdempotencyKey,
    SchemaVersion,
};
use serde::{Deserialize, Serialize};

use crate::{constants, AGENT_PROTOCOL_SCHEMA_VERSION};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingRuntimeConfig {
    pub tracking_enabled: bool,
    pub tracking_mode: String,
    pub ai_boundary_mode: String,
    pub notification_mode: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingLocationObservedEvent {
    pub child_device_id: String,
    pub child_profile_id: String,
    pub observation_id: String,
    pub observed_at: String,
    pub latitude_e7: i32,
    pub longitude_e7: i32,
    pub horizontal_accuracy_meters: u16,
    pub expected_place_ref: String,
    pub config: TrackingRuntimeConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingEvidenceRecordedEvent {
    pub child_device_id: String,
    pub child_profile_id: String,
    pub evidence_ref: String,
    pub source_observation_id: String,
    pub location_relation: String,
    pub requires_ai_analysis: bool,
    pub allowed_ai_purpose: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingAiAnalysisRequestedEvent {
    pub child_device_id: String,
    pub child_profile_id: String,
    pub ai_request_id: String,
    pub evidence_refs: Vec<String>,
    pub uncertainty_code: String,
    pub allowed_analysis_purpose: String,
    pub raw_private_payload_included: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingNearbyPlaceClassifiedEvent {
    pub child_device_id: String,
    pub child_profile_id: String,
    pub source_ai_request_id: String,
    pub evidence_refs: Vec<String>,
    pub place_category: String,
    pub confidence_basis: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingPolicyViolationDetectedEvent {
    pub child_device_id: String,
    pub child_profile_id: String,
    pub violation_id: String,
    pub policy_rule_ref: String,
    pub severity: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentNotificationRequestedEvent {
    pub child_device_id: String,
    pub child_profile_id: String,
    pub notification_id: String,
    pub source_policy_violation_id: String,
    pub channel: String,
    pub evidence_refs: Vec<String>,
}

impl DomainEvent for TrackingLocationObservedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        tracking_event_contract(constants::tracking_runtime::TRACKING_LOCATION_OBSERVED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        tracking_child_aggregate_key(&self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_idempotency_key(
            constants::tracking_runtime::TRACKING_LOCATION_OBSERVED_EVENT_TYPE,
            &self.observation_id,
        )
    }
}

impl DomainEvent for TrackingEvidenceRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        tracking_event_contract(constants::tracking_runtime::TRACKING_EVIDENCE_RECORDED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        tracking_child_aggregate_key(&self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_idempotency_key(
            constants::tracking_runtime::TRACKING_EVIDENCE_RECORDED_EVENT_TYPE,
            &self.evidence_ref,
        )
    }
}

impl DomainEvent for TrackingAiAnalysisRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        tracking_event_contract(
            constants::tracking_runtime::TRACKING_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
        )
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        tracking_child_aggregate_key(&self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_idempotency_key(
            constants::tracking_runtime::TRACKING_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            &self.ai_request_id,
        )
    }
}

impl DomainEvent for TrackingNearbyPlaceClassifiedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        tracking_event_contract(
            constants::tracking_runtime::TRACKING_NEARBY_PLACE_CLASSIFIED_EVENT_TYPE,
        )
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        tracking_child_aggregate_key(&self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_idempotency_key(
            constants::tracking_runtime::TRACKING_NEARBY_PLACE_CLASSIFIED_EVENT_TYPE,
            &self.source_ai_request_id,
        )
    }
}

impl DomainEvent for TrackingPolicyViolationDetectedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        tracking_event_contract(
            constants::tracking_runtime::TRACKING_POLICY_VIOLATION_DETECTED_EVENT_TYPE,
        )
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        tracking_child_aggregate_key(&self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_idempotency_key(
            constants::tracking_runtime::TRACKING_POLICY_VIOLATION_DETECTED_EVENT_TYPE,
            &self.violation_id,
        )
    }
}

impl DomainEvent for ParentNotificationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        tracking_event_contract(
            constants::tracking_runtime::PARENT_NOTIFICATION_REQUESTED_EVENT_TYPE,
        )
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        tracking_child_aggregate_key(&self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_idempotency_key(
            constants::tracking_runtime::PARENT_NOTIFICATION_REQUESTED_EVENT_TYPE,
            &self.notification_id,
        )
    }
}

pub fn default_tracking_runtime_config() -> TrackingRuntimeConfig {
    TrackingRuntimeConfig {
        tracking_enabled: true,
        tracking_mode: constants::tracking_runtime::TRACKING_MODE_OBSERVE_ONLY.to_string(),
        ai_boundary_mode: constants::tracking_runtime::AI_BOUNDARY_MODE_REQUEST_WHEN_UNCERTAIN
            .to_string(),
        notification_mode: constants::tracking_runtime::NOTIFICATION_MODE_PORTAL_ONLY.to_string(),
    }
}

pub fn policy_eligible_tracking_runtime_config() -> TrackingRuntimeConfig {
    TrackingRuntimeConfig {
        tracking_mode: constants::tracking_runtime::TRACKING_MODE_POLICY_ELIGIBLE.to_string(),
        ..default_tracking_runtime_config()
    }
}

fn tracking_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
    ))
}

fn tracking_child_aggregate_key(
    child_device_id: &str,
    child_profile_id: &str,
) -> Result<AggregateKey, EventingError> {
    AggregateKey::parse(format!(
        "{}{}{}",
        child_device_id,
        constants::tracking_runtime::IDEMPOTENCY_SEPARATOR,
        child_profile_id
    ))
}

fn tracking_idempotency_key(
    event_type: &str,
    unique_ref: &str,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type,
        constants::tracking_runtime::IDEMPOTENCY_SEPARATOR,
        unique_ref
    ))
}
