pub const TRACKING_LOCATION_OBSERVED_EVENT_TYPE: &str = "tracking.location.observed";
pub const TRACKING_EVIDENCE_RECORDED_EVENT_TYPE: &str = "tracking.evidence.recorded";
pub const TRACKING_AI_ANALYSIS_REQUESTED_EVENT_TYPE: &str = "tracking.ai.analysis.requested";
pub const TRACKING_NEARBY_PLACE_CLASSIFIED_EVENT_TYPE: &str = "tracking.nearby-place.classified";
pub const TRACKING_POLICY_VIOLATION_DETECTED_EVENT_TYPE: &str =
    "tracking.policy.violation.detected";
pub const PARENT_NOTIFICATION_REQUESTED_EVENT_TYPE: &str = "parent.notification.requested";

pub const TRACKING_MODE_OBSERVE_ONLY: &str = "observe-only";
pub const TRACKING_MODE_POLICY_ELIGIBLE: &str = "policy-eligible";
pub const AI_BOUNDARY_MODE_REQUEST_WHEN_UNCERTAIN: &str = "request-when-uncertain";
pub const NOTIFICATION_MODE_PORTAL_ONLY: &str = "portal-only";

pub const LOCATION_RELATION_UNCERTAIN_NEAR_EXPECTED_PLACE: &str = "uncertain-near-expected-place";
pub const UNCERTAINTY_CODE_NEARBY_PLACE_CLASSIFICATION_REQUIRED: &str =
    "nearby-place-classification-required";
pub const ALLOWED_AI_PURPOSE_NEARBY_PLACE_CLASSIFICATION: &str = "nearby-place-classification";
pub const PLACE_CATEGORY_HOSPITAL: &str = "hospital";
pub const CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT: &str = "ai-boundary-contract";
pub const POLICY_RULE_EXPECTED_PLACE: &str = "policy.expected-place";
pub const POLICY_SEVERITY_REVIEW: &str = "review";
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
pub const DEFAULT_POLICY_VIOLATION_ID: &str = "tracking-policy-violation-default";
pub const DEFAULT_NOTIFICATION_ID: &str = "parent-notification-default";
pub const DEFAULT_EXPECTED_PLACE_REF: &str = "expected-place-home";
pub const DEFAULT_OBSERVED_AT: &str = "2026-06-12T12:00:00Z";
pub const CORRELATION_PREFIX: &str = "tracking-runtime:";
pub const IDEMPOTENCY_SEPARATOR: &str = ":";
pub const ERROR_TRACKING_RUNTIME_FLOW_RECORDED: &str = "tracking runtime flow recorded";
