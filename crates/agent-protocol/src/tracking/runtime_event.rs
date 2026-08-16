use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use ocentra_evidence::PrivatePayloadState;
use serde::{Deserialize, Serialize};

use super::identifiers::{
    TrackingAcknowledgementId, TrackingAcknowledgementState, TrackingAiPurpose,
    TrackingAiRequestId, TrackingAlertEvaluationId, TrackingAlertSeverity,
    TrackingCapabilityStatus, TrackingCheckInId, TrackingCheckInState, TrackingChildDeviceId,
    TrackingChildProfileId, TrackingConfidenceBasis, TrackingEvaluationId, TrackingEvidenceRef,
    TrackingExpectedPlaceRef, TrackingExpectedPlaceState, TrackingGeofenceRuleRef,
    TrackingLocationRelation, TrackingNearbyPlaceAmbiguityState, TrackingNearbyPlaceProviderKind,
    TrackingNotificationChannel, TrackingNotificationId, TrackingObservationId,
    TrackingPlaceCategory, TrackingPolicyRuleRef, TrackingPolicySeverity,
    TrackingPolicyViolationId, TrackingProviderRef, TrackingReasonCode, TrackingScheduleId,
    TrackingTimestamp, TrackingTransitionId, TrackingTransitionKind, TrackingUncertaintyCode,
};
use crate::{constants, AGENT_PROTOCOL_SCHEMA_VERSION};

pub const TRACKING_RUNTIME_SCHEMA_VERSION: u16 = crate::AGENT_PROTOCOL_SCHEMA_VERSION;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingRuntimeMode {
    #[serde(rename = "observe-only")]
    ObserveOnly,
    #[serde(rename = "policy-eligible")]
    PolicyEligible,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingRuntimeEnabledState {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingAiBoundaryMode {
    #[serde(rename = "request-when-uncertain")]
    RequestWhenUncertain,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingNotificationMode {
    #[serde(rename = "portal-only")]
    ParentPortalOnly,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingAiAnalysisRequirement {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingParentActionRequirement {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingExpectedPlaceExceptionState {
    #[serde(rename = "holiday-mode")]
    HolidayMode,
    #[serde(rename = "trip-exception")]
    TripException,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingRuntimeConfig {
    pub tracking_enabled_state: TrackingRuntimeEnabledState,
    pub tracking_mode: TrackingRuntimeMode,
    pub ai_boundary_mode: TrackingAiBoundaryMode,
    pub notification_mode: TrackingNotificationMode,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingLocationObservedEvent {
    pub child_device_id: TrackingChildDeviceId,
    pub child_profile_id: TrackingChildProfileId,
    pub observation_id: TrackingObservationId,
    pub observed_at: TrackingTimestamp,
    pub latitude_e7: i32,
    pub longitude_e7: i32,
    pub horizontal_accuracy_meters: u16,
    pub expected_place_ref: TrackingExpectedPlaceRef,
    pub config: TrackingRuntimeConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingEvidenceRecordedEvent {
    pub child_device_id: TrackingChildDeviceId,
    pub child_profile_id: TrackingChildProfileId,
    pub evidence_ref: TrackingEvidenceRef,
    pub source_observation_id: TrackingObservationId,
    pub source_observed_at: TrackingTimestamp,
    pub expected_place_ref: TrackingExpectedPlaceRef,
    pub location_relation: TrackingLocationRelation,
    pub ai_analysis_requirement: TrackingAiAnalysisRequirement,
    pub parent_action_requirement: TrackingParentActionRequirement,
    pub allowed_ai_purpose: TrackingAiPurpose,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingAiAnalysisRequestedEvent {
    pub child_device_id: TrackingChildDeviceId,
    pub child_profile_id: TrackingChildProfileId,
    pub ai_request_id: TrackingAiRequestId,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
    pub source_observed_at: TrackingTimestamp,
    pub uncertainty_code: TrackingUncertaintyCode,
    pub allowed_analysis_purpose: TrackingAiPurpose,
    pub parent_action_requirement: TrackingParentActionRequirement,
    pub private_payload_state: PrivatePayloadState,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingNearbyPlaceClassifiedEvent {
    pub child_device_id: TrackingChildDeviceId,
    pub child_profile_id: TrackingChildProfileId,
    pub source_ai_request_id: TrackingAiRequestId,
    pub source_location_evidence_ref: TrackingEvidenceRef,
    pub source_observed_at: TrackingTimestamp,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
    pub provider_kind: TrackingNearbyPlaceProviderKind,
    pub provider_ref: Option<TrackingProviderRef>,
    pub query_radius_meters: u32,
    pub distance_meters: Option<u32>,
    pub place_category: TrackingPlaceCategory,
    pub confidence: f64,
    pub confidence_basis: TrackingConfidenceBasis,
    pub ambiguity_state: TrackingNearbyPlaceAmbiguityState,
    pub reason_codes: Vec<TrackingReasonCode>,
    pub parent_action_requirement: TrackingParentActionRequirement,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingGeofenceTransitionDetectedEvent {
    pub child_device_id: TrackingChildDeviceId,
    pub child_profile_id: TrackingChildProfileId,
    pub transition_id: TrackingTransitionId,
    pub geofence_rule_ref: TrackingGeofenceRuleRef,
    pub source_observation_id: TrackingObservationId,
    pub source_observed_at: TrackingTimestamp,
    pub transition_kind: TrackingTransitionKind,
    pub capability_status: TrackingCapabilityStatus,
    pub distance_meters: Option<u32>,
    pub reason_codes: Vec<TrackingReasonCode>,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingExpectedPlaceStateEvaluatedEvent {
    pub child_device_id: TrackingChildDeviceId,
    pub child_profile_id: TrackingChildProfileId,
    pub evaluation_id: TrackingEvaluationId,
    pub schedule_id: TrackingScheduleId,
    pub expected_place_ref: TrackingExpectedPlaceRef,
    pub source_observation_id: TrackingObservationId,
    pub source_observed_at: TrackingTimestamp,
    pub expected_place_state: TrackingExpectedPlaceState,
    pub distance_tolerance_meters: Option<u32>,
    pub late_grace_seconds: u32,
    pub early_exit_grace_seconds: u32,
    pub exception_state: Option<TrackingExpectedPlaceExceptionState>,
    pub reason_codes: Vec<TrackingReasonCode>,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
    pub parent_action_requirement: TrackingParentActionRequirement,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingPolicyViolationDetectedEvent {
    pub child_device_id: TrackingChildDeviceId,
    pub child_profile_id: TrackingChildProfileId,
    pub violation_id: TrackingPolicyViolationId,
    pub policy_rule_ref: TrackingPolicyRuleRef,
    pub severity: TrackingPolicySeverity,
    pub detected_at: TrackingTimestamp,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingParentNotificationState {
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "suppressed-duplicate")]
    SuppressedDuplicate,
    #[serde(rename = "suppressed-missing-evidence")]
    SuppressedMissingEvidence,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingAlertEvaluatedEvent {
    pub child_device_id: TrackingChildDeviceId,
    pub child_profile_id: TrackingChildProfileId,
    pub alert_evaluation_id: TrackingAlertEvaluationId,
    pub source_policy_violation_id: TrackingPolicyViolationId,
    pub policy_rule_ref: TrackingPolicyRuleRef,
    pub severity: TrackingAlertSeverity,
    pub parent_notification_state: TrackingParentNotificationState,
    pub evaluated_at: TrackingTimestamp,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingParentAcknowledgementRecordedEvent {
    pub child_device_id: TrackingChildDeviceId,
    pub child_profile_id: TrackingChildProfileId,
    pub acknowledgement_id: TrackingAcknowledgementId,
    pub source_policy_violation_id: TrackingPolicyViolationId,
    pub acknowledged_at: TrackingTimestamp,
    pub acknowledgement_state: TrackingAcknowledgementState,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingChildCheckInRequestState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "answered")]
    Answered,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "escalated")]
    Escalated,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingChildCheckInDeliveryState {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "unsupported-delivery")]
    UnsupportedDelivery,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingChildCheckInRequestedEvent {
    pub child_device_id: TrackingChildDeviceId,
    pub child_profile_id: TrackingChildProfileId,
    pub check_in_id: TrackingCheckInId,
    pub requested_at: TrackingTimestamp,
    pub request_state: TrackingChildCheckInRequestState,
    pub delivery_state: TrackingChildCheckInDeliveryState,
    pub related_alert_id: TrackingPolicyViolationId,
    pub include_location_if_permitted: bool,
    pub expires_at: TrackingTimestamp,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
    pub audit_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingChildCheckInRequestReceipt {
    pub schema_version: u16,
    pub child_device_id: TrackingChildDeviceId,
    pub child_profile_id: TrackingChildProfileId,
    pub check_in_id: TrackingCheckInId,
    pub related_alert_id: TrackingPolicyViolationId,
    pub request_state: TrackingChildCheckInRequestState,
    pub delivery_state: TrackingChildCheckInDeliveryState,
    pub receipt_recorded_at: TrackingTimestamp,
    pub reason_code: Option<TrackingReasonCode>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingChildCheckInRecordedEvent {
    pub child_device_id: TrackingChildDeviceId,
    pub child_profile_id: TrackingChildProfileId,
    pub check_in_id: TrackingCheckInId,
    pub source_observation_id: TrackingObservationId,
    pub checked_in_at: TrackingTimestamp,
    pub check_in_state: TrackingCheckInState,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentNotificationRequestedEvent {
    pub child_device_id: TrackingChildDeviceId,
    pub child_profile_id: TrackingChildProfileId,
    pub notification_id: TrackingNotificationId,
    pub source_policy_violation_id: TrackingPolicyViolationId,
    pub channel: TrackingNotificationChannel,
    pub requested_at: TrackingTimestamp,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
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

impl DomainEvent for TrackingGeofenceTransitionDetectedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        tracking_event_contract(
            constants::tracking_runtime::TRACKING_GEOFENCE_TRANSITION_DETECTED_EVENT_TYPE,
        )
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        tracking_child_aggregate_key(&self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_idempotency_key(
            constants::tracking_runtime::TRACKING_GEOFENCE_TRANSITION_DETECTED_EVENT_TYPE,
            &self.transition_id,
        )
    }
}

impl DomainEvent for TrackingExpectedPlaceStateEvaluatedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        tracking_event_contract(
            constants::tracking_runtime::TRACKING_EXPECTED_PLACE_STATE_EVALUATED_EVENT_TYPE,
        )
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        tracking_child_aggregate_key(&self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_idempotency_key(
            constants::tracking_runtime::TRACKING_EXPECTED_PLACE_STATE_EVALUATED_EVENT_TYPE,
            &self.evaluation_id,
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

impl DomainEvent for TrackingAlertEvaluatedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        tracking_event_contract(constants::tracking_runtime::TRACKING_ALERT_EVALUATED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        tracking_child_aggregate_key(&self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_idempotency_key(
            constants::tracking_runtime::TRACKING_ALERT_EVALUATED_EVENT_TYPE,
            &self.alert_evaluation_id,
        )
    }
}

impl DomainEvent for TrackingParentAcknowledgementRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        tracking_event_contract(
            constants::tracking_runtime::TRACKING_PARENT_ACKNOWLEDGEMENT_RECORDED_EVENT_TYPE,
        )
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        tracking_child_aggregate_key(&self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_idempotency_key(
            constants::tracking_runtime::TRACKING_PARENT_ACKNOWLEDGEMENT_RECORDED_EVENT_TYPE,
            &self.acknowledgement_id,
        )
    }
}

pub fn default_tracking_runtime_config() -> TrackingRuntimeConfig {
    TrackingRuntimeConfig {
        tracking_enabled_state: TrackingRuntimeEnabledState::Enabled,
        tracking_mode: TrackingRuntimeMode::ObserveOnly,
        ai_boundary_mode: TrackingAiBoundaryMode::RequestWhenUncertain,
        notification_mode: TrackingNotificationMode::ParentPortalOnly,
    }
}

pub fn policy_eligible_tracking_runtime_config() -> TrackingRuntimeConfig {
    TrackingRuntimeConfig {
        tracking_mode: TrackingRuntimeMode::PolicyEligible,
        ..default_tracking_runtime_config()
    }
}

pub(crate) fn tracking_event_contract(
    event_type: impl std::fmt::Display,
) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type.to_string())?,
        SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
    ))
}

pub(crate) fn tracking_child_aggregate_key(
    child_device_id: &TrackingChildDeviceId,
    child_profile_id: &TrackingChildProfileId,
) -> Result<AggregateKey, EventingError> {
    AggregateKey::parse(format!(
        "{}{}{}",
        child_device_id.as_str(),
        constants::tracking_runtime::IDEMPOTENCY_SEPARATOR,
        child_profile_id.as_str()
    ))
}

pub(crate) fn tracking_idempotency_key(
    event_type: impl std::fmt::Display,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type,
        constants::tracking_runtime::IDEMPOTENCY_SEPARATOR,
        unique_ref
    ))
}
