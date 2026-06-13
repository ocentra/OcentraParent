pub const TRACKING_LOCATION_OBSERVED_EVENT_TYPE: &str = "tracking.location.observed";
pub const TRACKING_EVIDENCE_RECORDED_EVENT_TYPE: &str = "tracking.evidence.recorded";
pub const TRACKING_AI_ANALYSIS_REQUESTED_EVENT_TYPE: &str = "tracking.ai.analysis.requested";
pub const TRACKING_NEARBY_PLACE_CLASSIFIED_EVENT_TYPE: &str = "tracking.nearby-place.classified";
pub const TRACKING_GEOFENCE_TRANSITION_DETECTED_EVENT_TYPE: &str =
    "tracking.geofence.transition.detected";
pub const TRACKING_EXPECTED_PLACE_STATE_EVALUATED_EVENT_TYPE: &str =
    "tracking.expected-place.state.evaluated";
pub const TRACKING_POLICY_VIOLATION_DETECTED_EVENT_TYPE: &str =
    "tracking.policy.violation.detected";
pub const TRACKING_PARENT_ACKNOWLEDGEMENT_RECORDED_EVENT_TYPE: &str =
    "tracking.parent-acknowledgement.recorded";
pub const TRACKING_CHILD_CHECK_IN_RECORDED_EVENT_TYPE: &str = "tracking.child-check-in.recorded";
pub const PARENT_NOTIFICATION_REQUESTED_EVENT_TYPE: &str = "tracking.parent.notification.requested";

pub const TRACKING_MODE_OBSERVE_ONLY: &str = "observe-only";
pub const TRACKING_MODE_POLICY_ELIGIBLE: &str = "policy-eligible";
pub const AI_BOUNDARY_MODE_REQUEST_WHEN_UNCERTAIN: &str = "request-when-uncertain";
pub const NOTIFICATION_MODE_PORTAL_ONLY: &str = "portal-only";

pub const LOCATION_RELATION_UNCERTAIN_NEAR_EXPECTED_PLACE: &str = "uncertain-near-expected-place";
pub const LOCATION_RELATION_AT_EXPECTED_PLACE: &str = "at-expected-place";
pub const LOCATION_RELATION_AWAY_FROM_EXPECTED_PLACE: &str = "away-from-expected-place";
pub const UNCERTAINTY_CODE_NEARBY_PLACE_CLASSIFICATION_REQUIRED: &str =
    "nearby-place-classification-required";
pub const ALLOWED_AI_PURPOSE_NEARBY_PLACE_CLASSIFICATION: &str = "nearby-place-classification";
pub const PLACE_CATEGORY_HOSPITAL: &str = "hospital";
pub const CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT: &str = "ai-boundary-contract";
pub const GEOFENCE_TRANSITION_ENTER: &str = "enter";
pub const GEOFENCE_TRANSITION_EXIT: &str = "exit";
pub const GEOFENCE_TRANSITION_DWELL: &str = "dwell";
pub const GEOFENCE_TRANSITION_UNCHANGED: &str = "unchanged";
pub const EXPECTED_PLACE_STATE_AT_EXPECTED_PLACE: &str = "at-expected-place";
pub const EXPECTED_PLACE_STATE_AWAY_FROM_EXPECTED_PLACE: &str = "away-from-expected-place";
pub const EXPECTED_PLACE_STATE_UNKNOWN: &str = "unknown";
pub const LOCATION_VALIDATION_ACCEPTED: &str = "accepted";
pub const LOCATION_VALIDATION_REJECTED_LATITUDE: &str = "rejected-latitude";
pub const LOCATION_VALIDATION_REJECTED_LONGITUDE: &str = "rejected-longitude";
pub const LOCATION_VALIDATION_REJECTED_ACCURACY: &str = "rejected-accuracy";
pub const DEVICE_STATUS_ONLINE: &str = "online";
pub const DEVICE_STATUS_OFFLINE_LAST_KNOWN_ONLY: &str = "offline-last-known-only";
pub const DEVICE_STATUS_BATTERY_THROTTLED: &str = "battery-throttled";
pub const CAPABILITY_STATUS_GRANTED: &str = "granted";
pub const CAPABILITY_STATUS_MANUAL_REQUIRED: &str = "manual-required";
pub const CAPABILITY_STATUS_DEGRADED: &str = "degraded";
pub const NEARBY_PLACE_PROVIDER_REQUESTED: &str = "nearby-place-provider-requested";
pub const NEARBY_PLACE_PROVIDER_UNAVAILABLE: &str = "nearby-place-provider-unavailable";
pub const NEARBY_PLACE_AMBIGUITY_HIGH: &str = "high-ambiguity";
pub const NEARBY_PLACE_AMBIGUITY_LOW: &str = "low-ambiguity";
pub const ALERT_SEVERITY_NONE: &str = "none";
pub const ALERT_SEVERITY_REVIEW: &str = "review";
pub const ALERT_SEVERITY_URGENT: &str = "urgent";
pub const TEMPORARY_LIVE_STATE_ACTIVE: &str = "active";
pub const TEMPORARY_LIVE_STATE_EXPIRED: &str = "expired";
pub const TEMPORARY_LIVE_STATE_AUTO_STOPPED: &str = "auto-stopped";
pub const MISSING_DEVICE_STATE_NOT_MISSING: &str = "not-missing";
pub const MISSING_DEVICE_STATE_LAST_KNOWN_ONLY: &str = "last-known-only";
pub const PARENT_DEFINED_PLACE_STATE_ACCEPTED: &str = "accepted";
pub const PARENT_DEFINED_PLACE_STATE_REJECTED_INVALID_RADIUS: &str =
    "rejected-invalid-radius";
pub const READ_MODEL_SCHEMA_COMPATIBLE: &str = "compatible";
pub const READ_MODEL_SCHEMA_MIGRATION_REQUIRED: &str = "migration-required";
pub const READ_MODEL_DIFFERENTIAL_CONSISTENT: &str = "consistent";
pub const READ_MODEL_DIFFERENTIAL_REJECTED_COUNT_DRIFT: &str = "rejected-count-drift";
pub const READ_MODEL_DIFFERENTIAL_REJECTED_TOMBSTONE_DRIFT: &str =
    "rejected-tombstone-drift";
pub const POLICY_RULE_EXPECTED_PLACE: &str = "policy.expected-place";
pub const POLICY_SEVERITY_REVIEW: &str = "review";
pub const ACKNOWLEDGEMENT_STATE_ACKNOWLEDGED: &str = "acknowledged";
pub const CHECK_IN_STATE_RECEIVED: &str = "received";
pub const AI_RESULT_ACCEPTED_AS_EVIDENCE: &str = "accepted-as-evidence";
pub const AI_RESULT_REJECTED_MISSING_EVIDENCE_REF: &str = "rejected-missing-evidence-ref";
pub const AI_RESULT_REJECTED_HALLUCINATED_EVIDENCE_REF: &str =
    "rejected-hallucinated-evidence-ref";
pub const AI_RESULT_REJECTED_WRONG_CHILD_OR_DEVICE_REF: &str =
    "rejected-wrong-child-or-device-ref";
pub const AI_RESULT_REJECTED_STALE_CORRELATION: &str = "rejected-stale-correlation";
pub const NOTIFICATION_CHANNEL_PARENT_PORTAL: &str = "parent-portal";

pub const SOURCE_COMPONENT_CHILD_TRACKING_RUNTIME: &str = "child-tracking-runtime";
pub const SOURCE_COMPONENT_CHILD_AI_RUNTIME: &str = "child-ai-runtime";
pub const SOURCE_COMPONENT_CHILD_POLICY_RUNTIME: &str = "child-policy-runtime";
pub const SOURCE_COMPONENT_CHILD_NOTIFICATION_RUNTIME: &str = "child-notification-runtime";

pub const SUBSCRIBER_CHILD_TRACKING_OBSERVER: &str = "subscriber.tracking.location-observer";
pub const SUBSCRIBER_CHILD_AI_TRACKING_ANALYZER: &str = "subscriber.child-ai.tracking-analyzer";
pub const SUBSCRIBER_CHILD_POLICY_TRACKING_ANALYZER: &str =
    "subscriber.child-policy.tracking-analyzer";
pub const SUBSCRIBER_CHILD_NOTIFICATION_POLICY_BRIDGE: &str =
    "subscriber.child-notification.policy-bridge";

pub const TARGET_HANDLER_CHILD_TRACKING_OBSERVER: &str = "target.tracking.location-observer";
pub const TARGET_HANDLER_CHILD_AI_TRACKING_ANALYZER: &str = "target.child-ai.tracking-analyzer";
pub const TARGET_HANDLER_CHILD_POLICY_TRACKING_ANALYZER: &str =
    "target.child-policy.tracking-analyzer";
pub const TARGET_HANDLER_CHILD_NOTIFICATION_POLICY_BRIDGE: &str =
    "target.child-notification.policy-bridge";

pub const DEFAULT_CHILD_DEVICE_ID: &str = "child-device-default";
pub const DEFAULT_CHILD_PROFILE_ID: &str = "child-profile-default";
pub const DEFAULT_OBSERVATION_ID: &str = "tracking-observation-default";
pub const DEFAULT_EVIDENCE_REF: &str = "tracking-evidence-default";
pub const DEFAULT_AI_REQUEST_ID: &str = "tracking-ai-request-default";
pub const DEFAULT_GEOFENCE_RULE_REF: &str = "geofence-rule-home";
pub const DEFAULT_GEOFENCE_TRANSITION_ID: &str = "tracking-geofence-transition-default";
pub const DEFAULT_EXPECTED_PLACE_EVALUATION_ID: &str =
    "tracking-expected-place-evaluation-default";
pub const DEFAULT_NEARBY_PLACE_REQUEST_ID: &str = "tracking-nearby-place-request-default";
pub const DEFAULT_ALERT_EVALUATION_ID: &str = "tracking-alert-evaluation-default";
pub const DEFAULT_TEMPORARY_LIVE_SESSION_ID: &str = "tracking-temporary-live-session-default";
pub const DEFAULT_MISSING_DEVICE_EVALUATION_ID: &str =
    "tracking-missing-device-evaluation-default";
pub const DEFAULT_PARENT_DEFINED_PLACE_ID: &str = "parent-defined-place-home";
pub const DEFAULT_POLICY_VIOLATION_ID: &str = "tracking-policy-violation-default";
pub const DEFAULT_PARENT_ACKNOWLEDGEMENT_ID: &str = "tracking-parent-acknowledgement-default";
pub const DEFAULT_CHILD_CHECK_IN_ID: &str = "tracking-child-check-in-default";
pub const DEFAULT_NOTIFICATION_ID: &str = "parent-notification-default";
pub const DEFAULT_EXPECTED_PLACE_REF: &str = "expected-place-home";
pub const DEFAULT_OBSERVED_AT: &str = "2026-06-12T12:00:00Z";
pub const CORRELATION_PREFIX: &str = "tracking-runtime:";
pub const IDEMPOTENCY_SEPARATOR: &str = ":";
pub const ERROR_TRACKING_RUNTIME_FLOW_RECORDED: &str = "tracking runtime flow recorded";
