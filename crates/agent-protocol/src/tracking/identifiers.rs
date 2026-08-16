use crate::constants;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use serde::{Deserialize, Serialize};

macro_rules! tracking_text_identifier {
    ($name:ident, $field:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                parse_tracking_identifier(value, $field).map(Self)
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

        impl PartialEq<&str> for $name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }
    };
}

tracking_text_identifier!(TrackingAcknowledgementId, "tracking.acknowledgement_id");
tracking_text_identifier!(
    TrackingAcknowledgementState,
    "tracking.acknowledgement_state"
);
tracking_text_identifier!(TrackingAcceptedAt, "tracking.accepted_at");
tracking_text_identifier!(TrackingAiPurpose, "tracking.ai_purpose");
tracking_text_identifier!(TrackingAiRequestId, "tracking.ai_request_id");
tracking_text_identifier!(TrackingAlertEvaluationId, "tracking.alert_evaluation_id");
tracking_text_identifier!(TrackingAlertSeverity, "tracking.alert_severity");
tracking_text_identifier!(TrackingCapabilityStatus, "tracking.capability_status");
tracking_text_identifier!(TrackingCheckInId, "tracking.check_in_id");
tracking_text_identifier!(TrackingCheckInState, "tracking.check_in_state");
tracking_text_identifier!(TrackingChildDeviceId, "tracking.child_device_id");
tracking_text_identifier!(TrackingChildProfileId, "tracking.child_profile_id");
tracking_text_identifier!(TrackingConfidenceBasis, "tracking.confidence_basis");
tracking_text_identifier!(
    TrackingDurableSettingsStoreRef,
    "tracking.durable_settings_store_ref"
);
tracking_text_identifier!(TrackingEvaluationId, "tracking.evaluation_id");
tracking_text_identifier!(TrackingEvidenceRef, "tracking.evidence_ref");
tracking_text_identifier!(TrackingExpectedPlaceRef, "tracking.expected_place_ref");
tracking_text_identifier!(TrackingExpectedPlaceState, "tracking.expected_place_state");
tracking_text_identifier!(TrackingGeofenceRuleRef, "tracking.geofence_rule_ref");
tracking_text_identifier!(
    TrackingLocalServiceStateSnapshotRef,
    "tracking.local_service_state_snapshot_ref"
);
tracking_text_identifier!(TrackingLocationRelation, "tracking.location_relation");
tracking_text_identifier!(
    TrackingMissingDeviceEvaluationId,
    "tracking.missing_device_evaluation_id"
);
tracking_text_identifier!(TrackingMissingDeviceState, "tracking.missing_device_state");
tracking_text_identifier!(TrackingMutationProofRef, "tracking.mutation_proof_ref");
tracking_text_identifier!(
    TrackingNearbyPlaceAmbiguityState,
    "tracking.nearby_place_ambiguity_state"
);
tracking_text_identifier!(
    TrackingNearbyPlaceProviderKind,
    "tracking.nearby_place_provider_kind"
);
tracking_text_identifier!(
    TrackingNearbyPlaceRequestId,
    "tracking.nearby_place_request_id"
);
tracking_text_identifier!(TrackingNotificationChannel, "tracking.notification_channel");
tracking_text_identifier!(TrackingNotificationId, "tracking.notification_id");
tracking_text_identifier!(TrackingObservationId, "tracking.observation_id");
tracking_text_identifier!(
    TrackingParentDefinedPlaceId,
    "tracking.parent_defined_place_id"
);
tracking_text_identifier!(
    TrackingParentDefinedPlaceState,
    "tracking.parent_defined_place_state"
);
tracking_text_identifier!(TrackingPlaceCategory, "tracking.place_category");
tracking_text_identifier!(TrackingPolicyRuleRef, "tracking.policy_rule_ref");
tracking_text_identifier!(TrackingPolicySeverity, "tracking.policy_severity");
tracking_text_identifier!(TrackingPolicyViolationId, "tracking.policy_violation_id");
tracking_text_identifier!(TrackingProviderRef, "tracking.provider_ref");
tracking_text_identifier!(TrackingReadModelProofRef, "tracking.read_model_proof_ref");
tracking_text_identifier!(
    TrackingReadModelCapabilityStatus,
    "tracking.read_model.capability_status"
);
tracking_text_identifier!(
    TrackingReadModelCountValue,
    "tracking.read_model.count_value"
);
tracking_text_identifier!(
    TrackingReadModelCustodyLabel,
    "tracking.read_model.custody_label"
);
tracking_text_identifier!(TrackingReadModelDeletedAt, "tracking.read_model.deleted_at");
tracking_text_identifier!(TrackingReadModelDeviceId, "tracking.read_model.device_id");
tracking_text_identifier!(TrackingReadModelEventId, "tracking.read_model.event_id");
tracking_text_identifier!(
    TrackingReadModelGeneratedAt,
    "tracking.read_model.generated_at"
);
tracking_text_identifier!(TrackingReadModelKind, "tracking.read_model.kind");
tracking_text_identifier!(
    TrackingReadModelObservedAt,
    "tracking.read_model.observed_at"
);
tracking_text_identifier!(TrackingReadModelObserver, "tracking.read_model.observer");
tracking_text_identifier!(TrackingReadModelPlatform, "tracking.read_model.platform");
tracking_text_identifier!(
    TrackingReadModelQueryVisibility,
    "tracking.read_model.query_visibility"
);
tracking_text_identifier!(
    TrackingReadModelSubjectDisplayName,
    "tracking.read_model.subject_display_name"
);
tracking_text_identifier!(TrackingReadModelSubjectId, "tracking.read_model.subject_id");
tracking_text_identifier!(
    TrackingReadModelSubjectKind,
    "tracking.read_model.subject_kind"
);
tracking_text_identifier!(TrackingRetentionCommandId, "tracking.retention_command_id");
tracking_text_identifier!(
    TrackingRetentionSettingsKind,
    "tracking.retention_settings_kind"
);
tracking_text_identifier!(
    TrackingRetentionWriteState,
    "tracking.retention_write_state"
);
tracking_text_identifier!(TrackingScheduleId, "tracking.schedule_id");
tracking_text_identifier!(TrackingReasonCode, "tracking.reason_code");
tracking_text_identifier!(TrackingSourceMessageId, "tracking.source_message_id");
tracking_text_identifier!(TrackingSourcePeerId, "tracking.source_peer_id");
tracking_text_identifier!(TrackingTargetDeviceId, "tracking.target_device_id");
tracking_text_identifier!(TrackingTargetPlatform, "tracking.target_platform");
tracking_text_identifier!(
    TrackingTemporaryLiveSessionId,
    "tracking.temporary_live_session_id"
);
tracking_text_identifier!(TrackingTemporaryLiveState, "tracking.temporary_live_state");
tracking_text_identifier!(TrackingTimestamp, "tracking.timestamp");
tracking_text_identifier!(TrackingTransitionId, "tracking.transition_id");
tracking_text_identifier!(TrackingTransitionKind, "tracking.transition_kind");
tracking_text_identifier!(TrackingUncertaintyCode, "tracking.uncertainty_code");
tracking_text_identifier!(TrackingWriterIntentRef, "tracking.writer_intent_ref");

fn derived_tracking_identifier_value(prefix: &str, segments: &[&str]) -> String {
    let mut value = String::from(prefix);
    for segment in segments {
        value.push_str(constants::tracking_runtime::IDEMPOTENCY_SEPARATOR);
        value.push_str(segment);
    }
    value
}

fn parse_tracking_identifier(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, EventingError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(EventingError::EmptyValue { field });
    }
    Ok(value)
}

fn parse_or_expect<T, E>(result: Result<T, E>, message: &'static str) -> T {
    result.expect_value(message)
}

fn tracking_policy_violation_id_from_source_and_rule_ref(
    source_ref: &str,
    policy_rule_ref: &TrackingPolicyRuleRef,
) -> TrackingPolicyViolationId {
    let value = derived_tracking_identifier_value(
        constants::tracking_runtime::TRACKING_POLICY_VIOLATION_DETECTED_EVENT_TYPE,
        &[source_ref, policy_rule_ref.as_str()],
    );
    parse_or_expect(
        TrackingPolicyViolationId::parse(value),
        constants::tracking_runtime::TRACKING_POLICY_VIOLATION_DETECTED_EVENT_TYPE,
    )
}

pub fn tracking_evidence_ref_from_observation_id(
    observation_id: &TrackingObservationId,
) -> TrackingEvidenceRef {
    let value = derived_tracking_identifier_value(
        constants::tracking_runtime::TRACKING_EVIDENCE_RECORDED_EVENT_TYPE,
        &[observation_id.as_str()],
    );
    parse_or_expect(
        TrackingEvidenceRef::parse(value),
        constants::tracking_runtime::TRACKING_EVIDENCE_RECORDED_EVENT_TYPE,
    )
}

pub fn tracking_ai_request_id_from_evidence_ref(
    evidence_ref: &TrackingEvidenceRef,
) -> TrackingAiRequestId {
    let value = derived_tracking_identifier_value(
        constants::tracking_runtime::TRACKING_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
        &[evidence_ref.as_str()],
    );
    parse_or_expect(
        TrackingAiRequestId::parse(value),
        constants::tracking_runtime::TRACKING_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
    )
}

pub fn tracking_nearby_place_request_id_from_evidence_ref(
    evidence_ref: &TrackingEvidenceRef,
) -> TrackingNearbyPlaceRequestId {
    let value = derived_tracking_identifier_value(
        constants::tracking_runtime::TRACKING_NEARBY_PLACE_PROVIDER_REQUEST_ID_PREFIX,
        &[evidence_ref.as_str()],
    );
    parse_or_expect(
        TrackingNearbyPlaceRequestId::parse(value),
        constants::tracking_runtime::TRACKING_NEARBY_PLACE_PROVIDER_REQUEST_ID_PREFIX,
    )
}

pub fn tracking_transition_id_from_observation_id(
    observation_id: &TrackingObservationId,
) -> TrackingTransitionId {
    let value = derived_tracking_identifier_value(
        constants::tracking_runtime::TRACKING_GEOFENCE_TRANSITION_DETECTED_EVENT_TYPE,
        &[observation_id.as_str()],
    );
    parse_or_expect(
        TrackingTransitionId::parse(value),
        constants::tracking_runtime::TRACKING_GEOFENCE_TRANSITION_DETECTED_EVENT_TYPE,
    )
}

pub fn tracking_evaluation_id_from_observation_id(
    observation_id: &TrackingObservationId,
) -> TrackingEvaluationId {
    let value = derived_tracking_identifier_value(
        constants::tracking_runtime::TRACKING_EXPECTED_PLACE_STATE_EVALUATED_EVENT_TYPE,
        &[observation_id.as_str()],
    );
    parse_or_expect(
        TrackingEvaluationId::parse(value),
        constants::tracking_runtime::TRACKING_EXPECTED_PLACE_STATE_EVALUATED_EVENT_TYPE,
    )
}

pub fn tracking_check_in_id_from_observation_id(
    observation_id: &TrackingObservationId,
) -> TrackingCheckInId {
    let value = derived_tracking_identifier_value(
        constants::tracking_runtime::TRACKING_CHILD_CHECK_IN_RECORDED_EVENT_TYPE,
        &[observation_id.as_str()],
    );
    parse_or_expect(
        TrackingCheckInId::parse(value),
        constants::tracking_runtime::TRACKING_CHILD_CHECK_IN_RECORDED_EVENT_TYPE,
    )
}

pub fn tracking_temporary_live_session_id_from_child_device_id(
    child_device_id: &TrackingChildDeviceId,
) -> TrackingTemporaryLiveSessionId {
    let value = derived_tracking_identifier_value(
        constants::tracking_runtime::TRACKING_TEMPORARY_LIVE_SESSION_ID_PREFIX,
        &[child_device_id.as_str()],
    );
    parse_or_expect(
        TrackingTemporaryLiveSessionId::parse(value),
        constants::tracking_runtime::TRACKING_TEMPORARY_LIVE_SESSION_ID_PREFIX,
    )
}

pub fn tracking_missing_device_evaluation_id_from_child_device_id(
    child_device_id: &TrackingChildDeviceId,
) -> TrackingMissingDeviceEvaluationId {
    let value = derived_tracking_identifier_value(
        constants::tracking_runtime::TRACKING_MISSING_DEVICE_EVALUATION_ID_PREFIX,
        &[child_device_id.as_str()],
    );
    parse_or_expect(
        TrackingMissingDeviceEvaluationId::parse(value),
        constants::tracking_runtime::TRACKING_MISSING_DEVICE_EVALUATION_ID_PREFIX,
    )
}

pub fn tracking_parent_defined_place_id_from_evidence_ref(
    evidence_ref: &TrackingEvidenceRef,
) -> TrackingParentDefinedPlaceId {
    let value = derived_tracking_identifier_value(
        constants::tracking_runtime::TRACKING_PARENT_DEFINED_PLACE_ID_PREFIX,
        &[evidence_ref.as_str()],
    );
    parse_or_expect(
        TrackingParentDefinedPlaceId::parse(value),
        constants::tracking_runtime::TRACKING_PARENT_DEFINED_PLACE_ID_PREFIX,
    )
}

pub fn tracking_violation_id_from_ai_request_and_rule_ref(
    ai_request_id: &TrackingAiRequestId,
    policy_rule_ref: &TrackingPolicyRuleRef,
) -> TrackingPolicyViolationId {
    tracking_policy_violation_id_from_source_and_rule_ref(ai_request_id.as_str(), policy_rule_ref)
}

pub fn tracking_violation_id_from_evaluation_and_rule_ref(
    evaluation_id: &TrackingEvaluationId,
    policy_rule_ref: &TrackingPolicyRuleRef,
) -> TrackingPolicyViolationId {
    tracking_policy_violation_id_from_source_and_rule_ref(evaluation_id.as_str(), policy_rule_ref)
}

pub fn tracking_notification_id_from_violation_id(
    violation_id: &TrackingPolicyViolationId,
) -> TrackingNotificationId {
    let value = derived_tracking_identifier_value(
        constants::tracking_runtime::PARENT_NOTIFICATION_REQUESTED_EVENT_TYPE,
        &[violation_id.as_str()],
    );
    parse_or_expect(
        TrackingNotificationId::parse(value),
        constants::tracking_runtime::PARENT_NOTIFICATION_REQUESTED_EVENT_TYPE,
    )
}

pub fn tracking_acknowledgement_id_from_violation_id(
    violation_id: &TrackingPolicyViolationId,
) -> TrackingAcknowledgementId {
    let value = derived_tracking_identifier_value(
        constants::tracking_runtime::TRACKING_PARENT_ACKNOWLEDGEMENT_RECORDED_EVENT_TYPE,
        &[violation_id.as_str()],
    );
    parse_or_expect(
        TrackingAcknowledgementId::parse(value),
        constants::tracking_runtime::TRACKING_PARENT_ACKNOWLEDGEMENT_RECORDED_EVENT_TYPE,
    )
}

pub fn tracking_alert_evaluation_id_from_violation_id(
    violation_id: &TrackingPolicyViolationId,
) -> TrackingAlertEvaluationId {
    let value = derived_tracking_identifier_value(
        constants::tracking_runtime::TRACKING_ALERT_EVALUATED_EVENT_TYPE,
        &[violation_id.as_str()],
    );
    parse_or_expect(
        TrackingAlertEvaluationId::parse(value),
        constants::tracking_runtime::TRACKING_ALERT_EVALUATED_EVENT_TYPE,
    )
}
