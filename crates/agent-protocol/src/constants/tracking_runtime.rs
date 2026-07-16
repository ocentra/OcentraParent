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
pub const TRACKING_ALERT_EVALUATED_EVENT_TYPE: &str = "tracking.alert.evaluated";
pub const TRACKING_PARENT_ACKNOWLEDGEMENT_RECORDED_EVENT_TYPE: &str =
    "tracking.parent-acknowledgement.recorded";
pub const TRACKING_CHILD_CHECK_IN_REQUESTED_EVENT_TYPE: &str = "tracking.child-check-in.requested";
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
pub const GEOFENCE_TRANSITION_MISSED_ARRIVAL: &str = "missed-arrival";
pub const GEOFENCE_TRANSITION_STALE_AT_PLACE: &str = "stale-at-place";
pub const GEOFENCE_TRANSITION_AMBIGUOUS: &str = "ambiguous";
pub const GEOFENCE_TRANSITION_UNCHANGED: &str = "unchanged";
pub const EXPECTED_PLACE_STATE_AT_EXPECTED_PLACE: &str = "at-expected-place";
pub const EXPECTED_PLACE_STATE_AWAY_FROM_EXPECTED_PLACE: &str = "away-from-expected-place";
pub const EXPECTED_PLACE_STATE_WHERE_EXPECTED: &str = "where-expected";
pub const EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE: &str = "left-expected-place";
pub const EXPECTED_PLACE_STATE_LATE_ARRIVAL: &str = "late-arrival";
pub const EXPECTED_PLACE_STATE_EARLY_EXIT: &str = "early-exit";
pub const EXPECTED_PLACE_STATE_UNKNOWN: &str = "unknown";
pub const EXPECTED_PLACE_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const LOCATION_VALIDATION_ACCEPTED: &str = "accepted";
pub const LOCATION_VALIDATION_REJECTED_LATITUDE: &str = "rejected-latitude";
pub const LOCATION_VALIDATION_REJECTED_LONGITUDE: &str = "rejected-longitude";
pub const LOCATION_VALIDATION_REJECTED_ACCURACY: &str = "rejected-accuracy";
pub const CAPABILITY_STATUS_LIVE: &str = "live";
pub const CAPABILITY_STATUS_RECENT: &str = "recent";
pub const CAPABILITY_STATUS_STALE: &str = "stale";
pub const CAPABILITY_STATUS_LAST_KNOWN: &str = "last-known";
pub const DEVICE_STATUS_LIVE: &str = "live";
pub const DEVICE_STATUS_STALE: &str = "stale";
pub const DEVICE_STATUS_OFFLINE_LAST_KNOWN_ONLY: &str = "offline-last-known-only";
pub const DEVICE_STATUS_PENDING_UPLOAD: &str = "pending-upload";
pub const DEVICE_STATUS_BATTERY_THROTTLED: &str = "battery-throttled";
pub const DEVICE_STATUS_SERVICE_DISABLED: &str = "service-disabled";
pub const DEVICE_STATUS_UNAVAILABLE: &str = "unavailable";
pub const CAPABILITY_STATUS_FOREGROUND_ONLY: &str = "foreground-only";
pub const CAPABILITY_STATUS_BACKGROUND_READY: &str = "background-ready";
pub const CAPABILITY_STATUS_APPROXIMATE_ONLY: &str = "approximate-only";
pub const CAPABILITY_STATUS_PERMISSION_REQUIRED: &str = "permission-required";
pub const CAPABILITY_STATUS_PERMISSION_DENIED: &str = "permission-denied";
pub const CAPABILITY_STATUS_BACKGROUND_PERMISSION_REQUIRED: &str = "background-permission-required";
pub const CAPABILITY_STATUS_SERVICE_DISABLED: &str = "service-disabled";
pub const CAPABILITY_STATUS_MANUAL_REQUIRED: &str = "manual-required";
pub const CAPABILITY_STATUS_PLATFORM_UNSUPPORTED: &str = "platform-unsupported";
pub const CAPABILITY_STATUS_OFFLINE_LAST_KNOWN_ONLY: &str = "offline-last-known-only";
pub const CAPABILITY_STATUS_BATTERY_THROTTLED: &str = "battery-throttled";
pub const CAPABILITY_STATUS_UNAVAILABLE: &str = "unavailable";
pub const CAPABILITY_STATUS_ADAPTER_ERROR: &str = "adapter-error";
pub const CAPABILITY_STATUS_DISABLED_BY_PARENT: &str = "disabled-by-parent";
pub const NEARBY_PROVIDER_KIND_GOOGLE_PLACES: &str = "google-places";
pub const NEARBY_PROVIDER_KIND_APPLE_MAPKIT: &str = "apple-mapkit";
pub const NEARBY_PROVIDER_KIND_OPENSTREETMAP: &str = "openstreetmap";
pub const NEARBY_PROVIDER_KIND_PARENT_DEFINED: &str = "parent-defined";
pub const NEARBY_PROVIDER_KIND_LOCAL_CACHE: &str = "local-cache";
pub const NEARBY_PROVIDER_KIND_UNAVAILABLE: &str = "unavailable";
pub const NEARBY_PLACE_AMBIGUITY_CLEAR: &str = "clear";
pub const NEARBY_PLACE_AMBIGUITY_MULTIPLE_CANDIDATES: &str = "multiple-candidates";
pub const NEARBY_PLACE_AMBIGUITY_LOW_ACCURACY: &str = "low-accuracy";
pub const NEARBY_PLACE_AMBIGUITY_PROVIDER_UNAVAILABLE: &str = "provider-unavailable";
pub const NEARBY_PLACE_AMBIGUITY_UNKNOWN: &str = "unknown";
pub const ALERT_SEVERITY_INFO: &str = "info";
pub const ALERT_SEVERITY_WATCH: &str = "watch";
pub const ALERT_SEVERITY_WARNING: &str = "warning";
pub const ALERT_SEVERITY_URGENT: &str = "urgent";
pub const ALERT_SEVERITY_CRITICAL: &str = "critical";
pub const TEMPORARY_LIVE_STATE_ACTIVE: &str = "active";
pub const TEMPORARY_LIVE_STATE_EXPIRED: &str = "expired";
pub const TEMPORARY_LIVE_STATE_AUTO_STOPPED: &str = "auto-stopped";
pub const MISSING_DEVICE_STATE_NOT_MISSING: &str = "not-missing";
pub const MISSING_DEVICE_STATE_LAST_KNOWN_ONLY: &str = "last-known-only";
pub const PARENT_DEFINED_PLACE_STATE_ACCEPTED: &str = "accepted";
pub const PARENT_DEFINED_PLACE_STATE_REJECTED_INVALID_RADIUS: &str = "rejected-invalid-radius";
pub const READ_MODEL_SCHEMA_COMPATIBLE: &str = "compatible";
pub const READ_MODEL_SCHEMA_MIGRATION_REQUIRED: &str = "migration-required";
pub const READ_MODEL_DIFFERENTIAL_CONSISTENT: &str = "consistent";
pub const READ_MODEL_DIFFERENTIAL_REJECTED_COUNT_DRIFT: &str = "rejected-count-drift";
pub const READ_MODEL_DIFFERENTIAL_REJECTED_TOMBSTONE_DRIFT: &str = "rejected-tombstone-drift";
pub const POLICY_RULE_EXPECTED_PLACE: &str = "policy.expected-place";
pub const POLICY_SEVERITY_REVIEW: &str = "review";
pub const POLICY_SEVERITY_WARNING: &str = "warning";
pub const POLICY_SEVERITY_URGENT: &str = "urgent";
pub const POLICY_SEVERITY_CRITICAL: &str = "critical";
pub const PARENT_NOTIFICATION_STATE_ALLOWED: &str = "allowed";
pub const PARENT_NOTIFICATION_STATE_SUPPRESSED_DUPLICATE: &str = "suppressed-duplicate";
pub const PARENT_NOTIFICATION_STATE_SUPPRESSED_MISSING_EVIDENCE: &str =
    "suppressed-missing-evidence";
pub const ACKNOWLEDGEMENT_STATE_ACKNOWLEDGED: &str = "acknowledged";
pub const CHILD_CHECK_IN_REQUEST_STATE_PENDING: &str = "pending";
pub const CHILD_CHECK_IN_REQUEST_STATE_SENT: &str = "sent";
pub const CHILD_CHECK_IN_REQUEST_STATE_ANSWERED: &str = "answered";
pub const CHILD_CHECK_IN_REQUEST_STATE_EXPIRED: &str = "expired";
pub const CHILD_CHECK_IN_REQUEST_STATE_CANCELLED: &str = "cancelled";
pub const CHILD_CHECK_IN_REQUEST_STATE_ESCALATED: &str = "escalated";
pub const CHILD_CHECK_IN_DELIVERY_STATE_QUEUED: &str = "queued";
pub const CHILD_CHECK_IN_DELIVERY_STATE_REQUESTED: &str = "requested";
pub const CHILD_CHECK_IN_DELIVERY_STATE_DUPLICATE: &str = "duplicate";
pub const CHILD_CHECK_IN_DELIVERY_STATE_STALE: &str = "stale";
pub const CHILD_CHECK_IN_DELIVERY_STATE_UNSUPPORTED_DELIVERY: &str = "unsupported-delivery";
pub const CHECK_IN_STATE_RECEIVED: &str = "received";
pub const AI_RESULT_ACCEPTED_AS_EVIDENCE: &str = "accepted-as-evidence";
pub const AI_RESULT_REJECTED_MISSING_EVIDENCE_REF: &str = "rejected-missing-evidence-ref";
pub const AI_RESULT_REJECTED_HALLUCINATED_EVIDENCE_REF: &str = "rejected-hallucinated-evidence-ref";
pub const AI_RESULT_REJECTED_WRONG_CHILD_OR_DEVICE_REF: &str = "rejected-wrong-child-or-device-ref";
pub const AI_RESULT_REJECTED_STALE_CORRELATION: &str = "rejected-stale-correlation";
pub const NOTIFICATION_CHANNEL_PARENT_PORTAL: &str = "parent-portal";
pub const TRACKING_CHILD_CHECK_IN_REQUEST_TIMEOUT_MS: u64 = 5000;

pub const SOURCE_COMPONENT_PARENT_RUNTIME: &str = "parent-runtime-core";
pub const SOURCE_COMPONENT_CHILD_TRACKING_RUNTIME: &str = "child-tracking-runtime";
pub const SOURCE_COMPONENT_CHILD_AI_RUNTIME: &str = "child-ai-runtime";
pub const SOURCE_COMPONENT_CHILD_POLICY_RUNTIME: &str = "child-policy-runtime";
pub const SOURCE_COMPONENT_CHILD_NOTIFICATION_RUNTIME: &str = "child-notification-runtime";

pub const SUBSCRIBER_CHILD_TRACKING_OBSERVER: &str = "subscriber.tracking.location-observer";
pub const SUBSCRIBER_CHILD_TRACKING_CHECK_IN_REQUESTER: &str =
    "subscriber.tracking.check-in-requester";
pub const SUBSCRIBER_CHILD_AI_TRACKING_ANALYZER: &str = "subscriber.child-ai.tracking-analyzer";
pub const SUBSCRIBER_CHILD_POLICY_TRACKING_ANALYZER: &str =
    "subscriber.child-policy.tracking-analyzer";
pub const SUBSCRIBER_CHILD_POLICY_EXPECTED_PLACE_EVALUATOR: &str =
    "subscriber.child-policy.expected-place-evaluator";
pub const SUBSCRIBER_CHILD_NOTIFICATION_POLICY_BRIDGE: &str =
    "subscriber.child-notification.policy-bridge";

pub const TARGET_HANDLER_CHILD_TRACKING_OBSERVER: &str = "target.tracking.location-observer";
pub const TARGET_HANDLER_CHILD_TRACKING_CHECK_IN_REQUESTER: &str =
    "target.tracking.check-in-requester";
pub const TARGET_HANDLER_CHILD_AI_TRACKING_ANALYZER: &str = "target.child-ai.tracking-analyzer";
pub const TARGET_HANDLER_CHILD_POLICY_TRACKING_ANALYZER: &str =
    "target.child-policy.tracking-analyzer";
pub const TARGET_HANDLER_CHILD_POLICY_EXPECTED_PLACE_EVALUATOR: &str =
    "target.child-policy.expected-place-evaluator";
pub const TARGET_HANDLER_CHILD_NOTIFICATION_POLICY_BRIDGE: &str =
    "target.child-notification.policy-bridge";

pub const DEFAULT_CHILD_DEVICE_ID: &str = "child-device-default";
pub const DEFAULT_CHILD_PROFILE_ID: &str = "child-profile-default";
pub const DEFAULT_OBSERVATION_ID: &str = "tracking-observation-default";
pub const DEFAULT_EVIDENCE_REF: &str = "tracking-evidence-default";
pub const DEFAULT_AI_REQUEST_ID: &str = "tracking-ai-request-default";
pub const DEFAULT_GEOFENCE_RULE_REF: &str = "geofence-rule-home";
pub const DEFAULT_GEOFENCE_TRANSITION_ID: &str = "tracking-geofence-transition-default";
pub const DEFAULT_EXPECTED_PLACE_EVALUATION_ID: &str = "tracking-expected-place-evaluation-default";
pub const DEFAULT_EXPECTED_PLACE_SCHEDULE_ID: &str = "tracking-expected-place-schedule-default";
pub const DEFAULT_EXPECTED_PLACE_DISTANCE_TOLERANCE_METERS: u32 = 150;
pub const DEFAULT_EXPECTED_PLACE_LATE_GRACE_SECONDS: u32 = 600;
pub const DEFAULT_EXPECTED_PLACE_EARLY_EXIT_GRACE_SECONDS: u32 = 600;
pub const DEFAULT_NEARBY_PLACE_REQUEST_ID: &str = "tracking-nearby-place-request-default";
pub const TRACKING_NEARBY_PLACE_PROVIDER_REQUEST_ID_PREFIX: &str =
    "tracking.nearby-place.provider.request";
pub const DEFAULT_TRACKING_PROVIDER_REF: &str = "parent-local-place-store";
pub const DEFAULT_ALERT_EVALUATION_ID: &str = "tracking-alert-evaluation-default";
pub const DEFAULT_TEMPORARY_LIVE_SESSION_ID: &str = "tracking-temporary-live-session-default";
pub const DEFAULT_MISSING_DEVICE_EVALUATION_ID: &str = "tracking-missing-device-evaluation-default";
pub const DEFAULT_PARENT_DEFINED_PLACE_ID: &str = "parent-defined-place-home";
pub const TRACKING_TEMPORARY_LIVE_SESSION_ID_PREFIX: &str = "tracking.temporary-live.session";
pub const TRACKING_MISSING_DEVICE_EVALUATION_ID_PREFIX: &str = "tracking.missing-device.evaluation";
pub const TRACKING_PARENT_DEFINED_PLACE_ID_PREFIX: &str = "tracking.parent-defined-place";
pub const DEFAULT_POLICY_VIOLATION_ID: &str = "tracking-policy-violation-default";
pub const DEFAULT_PARENT_ACKNOWLEDGEMENT_ID: &str = "tracking-parent-acknowledgement-default";
pub const DEFAULT_CHILD_CHECK_IN_ID: &str = "tracking-child-check-in-default";
pub const DEFAULT_NOTIFICATION_ID: &str = "parent-notification-default";
pub const DEFAULT_EXPECTED_PLACE_REF: &str = "expected-place-home";
pub const DEFAULT_OBSERVED_AT: &str = "2026-06-12T12:00:00Z";
pub const DEFAULT_NEARBY_QUERY_RADIUS_METERS: u32 = 250;
pub const DEFAULT_NEARBY_DISTANCE_METERS: u32 = 42;
pub const DEFAULT_NEARBY_PLACE_CONFIDENCE: f64 = 0.91;
pub const CORRELATION_PREFIX: &str = "tracking-runtime:";
pub const IDEMPOTENCY_SEPARATOR: &str = ":";
pub const FIELD_LOCATION_VALIDATION: &str = "tracking.location.validation";
pub const ERROR_TRACKING_RUNTIME_FLOW_RECORDED: &str = "tracking runtime flow recorded";
pub const REASON_ADAPTER_ERROR: &str = "adapter-error";
pub const REASON_BACKGROUND_PERMISSION_REQUIRED: &str = "background-permission-required";
pub const REASON_BACKGROUND_PLATFORM_UNSUPPORTED: &str = "background-platform-unsupported";
pub const REASON_BATTERY_LOW: &str = "battery-low";
pub const REASON_BATTERY_THROTTLED: &str = "battery-throttled";
pub const REASON_CONNECTIVITY_OFFLINE: &str = "connectivity-offline";
pub const REASON_CONNECTIVITY_METERED: &str = "connectivity-metered";
pub const REASON_DISABLED_BY_PARENT: &str = "disabled-by-parent";
pub const REASON_EXPECTED_PLACE_AMBIGUOUS: &str = "expected-place-ambiguous";
pub const REASON_EXPECTED_PLACE_EARLY_EXIT_GRACE_ACTIVE: &str =
    "expected-place-early-exit-grace-active";
pub const REASON_EXPECTED_PLACE_HOLIDAY_EXCEPTION_ACTIVE: &str =
    "expected-place-holiday-exception-active";
pub const REASON_EXPECTED_PLACE_LATE_GRACE_ACTIVE: &str = "expected-place-late-grace-active";
pub const REASON_EXPECTED_PLACE_SCHEDULE_DISABLED: &str = "expected-place-schedule-disabled";
pub const REASON_EXPECTED_PLACE_TRIP_EXCEPTION_ACTIVE: &str =
    "expected-place-trip-exception-active";
pub const REASON_FOREGROUND_PERMISSION_REQUIRED: &str = "foreground-permission-required";
pub const REASON_FRESH_LOCATION_REQUIRED: &str = "fresh-location-required";
pub const REASON_GEOFENCE_GRACE_ACTIVE: &str = "geofence-grace-active";
pub const REASON_INSIDE_GEOFENCE_WITH_ACCURACY: &str = "inside-geofence-with-accuracy";
pub const REASON_INSIDE_EXPECTED_PLACE_WINDOW: &str = "inside-expected-place-window";
pub const REASON_LAST_LOCATION_SAMPLE_STALE: &str = "last-location-sample-stale";
pub const REASON_LOCATION_ACCURACY_BELOW_RULE_THRESHOLD: &str =
    "location-accuracy-below-rule-threshold";
pub const REASON_LOW_POWER_MODE: &str = "low-power-mode";
pub const REASON_MANAGED_DEVICE_PROOF_REQUIRED: &str = "managed-device-proof-required";
pub const REASON_MISSED_EXPECTED_PLACE_ARRIVAL: &str = "missed-expected-place-arrival";
pub const REASON_OUTSIDE_EXPECTED_PLACE_WINDOW: &str = "outside-expected-place-window";
pub const REASON_OUTSIDE_GEOFENCE_WITH_ACCURACY: &str = "outside-geofence-with-accuracy";
pub const REASON_PARENT_DEFINED_PLACE_MATCH: &str = "parent-defined-place-match";
pub const REASON_PARENT_SYNC_LATE: &str = "parent-sync-late";
pub const REASON_PENDING_UPLOAD_BACKLOG: &str = "pending-upload-backlog";
pub const REASON_PLATFORM_UNSUPPORTED: &str = "platform-unsupported";
pub const REASON_PRECISE_LOCATION_UNAVAILABLE: &str = "precise-location-unavailable";
pub const REASON_RADIO_DISABLED: &str = "radio-disabled";
pub const REASON_NEARBY_PLACE_AMBIGUITY_PRESERVED: &str = "nearby-place-ambiguity-preserved";
pub const REASON_NEARBY_PLACE_NO_CANDIDATES: &str = "nearby-place-no-candidates";
pub const REASON_NEARBY_PLACE_PROVIDER_UNAVAILABLE: &str = "provider-unavailable";
pub const REASON_NEARBY_PLACE_SINGLE_CANDIDATE: &str = "nearby-place-single-candidate";
pub const REASON_SERVICE_DISABLED: &str = "service-disabled";
pub const REASON_STALE_LOCATION_REJECTED: &str = "stale-location-rejected";
pub const REASON_TRACKING_HEARTBEAT_STALE: &str = "tracking-heartbeat-stale";
pub const REASON_TRACKING_RUNTIME_UNAVAILABLE: &str = "tracking-runtime-unavailable";
pub const REASON_EXITED_EXPECTED_PLACE_WINDOW: &str = "exited-expected-place-window";
pub const REASON_DUPLICATE_CHECK_IN_REQUEST: &str = "duplicate-check-in-request";
pub const REASON_STALE_CHECK_IN_REQUEST: &str = "stale-check-in-request";
pub const REASON_UNSUPPORTED_CHECK_IN_DELIVERY: &str = "unsupported-check-in-delivery";
