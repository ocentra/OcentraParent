pub const PARENT_EVENT_TYPE: &str = "tracking.config.updated.parent";
pub const CHILD_EVENT_TYPE: &str = "tracking.config.updated.child";
pub const APPLIED_EVENT_TYPE: &str = "tracking.config.applied.child";
pub const CHANGE_REQUESTED_EVENT_TYPE: &str = "tracking.config.change_requested";
pub const CHANGE_APPROVED_EVENT_TYPE: &str = "tracking.config.change_approved";
pub const CHANGE_REJECTED_EVENT_TYPE: &str = "tracking.config.change_rejected";
pub const RESPONSE_STATE_APPLIED: &str = "applied";
pub const RESPONSE_STATE_REJECTED: &str = "rejected";
pub const POLICY_DECISION_APPROVED: &str = "approved";
pub const POLICY_DECISION_REJECTED: &str = "rejected";
pub const EFFECTIVE_STATE_ENABLED: &str = "enabled";
pub const EFFECTIVE_STATE_DISABLED: &str = "disabled";
pub const EFFECTIVE_STATE_DEGRADED: &str = "degraded";
pub const AUDIT_OUTCOME_COMMITTED: &str = "committed";
pub const AUDIT_OUTCOME_FAILED: &str = "failed";
pub const READ_MODEL_UPDATE_KIND_TRACKING_CONFIG_STATE: &str = "tracking-config-state";
pub const READ_MODEL_UPDATE_KIND_MANUAL_REQUIRED_STATE: &str = "manual-required-state";
pub const REQUEST_TIMEOUT_MS: u64 = 5000;
pub const TARGET_SCOPE_CHILD_DEVICE: &str = "child-device";
pub const TARGET_SCOPE_CHILD_PROFILE: &str = "child-profile";
pub const TARGET_SCOPE_FAMILY: &str = "family";
pub const TARGET_SCOPE_DEVICE_GROUP: &str = "device-group";
pub const ROUTE_LOCALHOST: &str = super::value::DEVICE_RUNTIME_ROUTE_LOCALHOST;
pub const ROUTE_LOCAL_NETWORK: &str = super::value::DEVICE_RUNTIME_ROUTE_LOCAL_NETWORK;
pub const ROUTE_CLOUD_RELAY: &str = super::value::DEVICE_RUNTIME_ROUTE_CLOUD_RELAY;
pub const SOURCE_COMPONENT_PARENT_PORTAL: &str = "parent-portal";
pub const SOURCE_COMPONENT_PARENT_AGENT_SERVICE: &str = "parent-agent-service";
pub const SOURCE_COMPONENT_PARENT_RUNTIME: &str = "parent-runtime-core";
pub const SOURCE_COMPONENT_CHILD_TRACKING_RUNTIME: &str = "child-tracking-runtime";
pub const SUBSCRIBER_PARENT_TRACKING_CONFIG_RELAY: &str = "subscriber.tracking-config.parent-relay";
pub const SUBSCRIBER_PARENT_TRACKING_CONFIG_CHANGE_REQUESTER: &str =
    "subscriber.tracking-config.change-requester";
pub const SUBSCRIBER_PARENT_TRACKING_CONFIG_POLICY_REQUESTER: &str =
    "subscriber.tracking-config.policy-requester";
pub const SUBSCRIBER_PARENT_TRACKING_CONFIG_POLICY_DECIDER: &str =
    "subscriber.tracking-config.policy-decider";
pub const SUBSCRIBER_PARENT_TRACKING_CONFIG_DECISION_APPLIER: &str =
    "subscriber.tracking-config.decision-applier";
pub const SUBSCRIBER_CHILD_TRACKING_CONFIG_APPLIER: &str =
    "subscriber.tracking-config.child-applier";
pub const SUBSCRIBER_CHILD_TRACKING_CONFIG_APPLIED_RECORDER: &str =
    "subscriber.tracking-config.child-applied-recorder";
pub const TARGET_HANDLER_CHILD_TRACKING_RUNTIME: &str = "child-tracking-runtime";
pub const TARGET_HANDLER_PARENT_TRACKING_CONFIG_RELAY: &str = "target.tracking-config.parent-relay";
pub const TARGET_HANDLER_PARENT_TRACKING_CONFIG_CHANGE_REQUESTER: &str =
    "target.tracking-config.change-requester";
pub const TARGET_HANDLER_PARENT_TRACKING_CONFIG_POLICY_REQUESTER: &str =
    "target.tracking-config.policy-requester";
pub const TARGET_HANDLER_PARENT_TRACKING_CONFIG_POLICY_DECIDER: &str =
    "target.tracking-config.policy-decider";
pub const TARGET_HANDLER_PARENT_TRACKING_CONFIG_DECISION_APPLIER: &str =
    "target.tracking-config.decision-applier";
pub const TARGET_HANDLER_CHILD_TRACKING_CONFIG_APPLIER: &str =
    "target.tracking-config.child-applier";
pub const TARGET_HANDLER_CHILD_TRACKING_CONFIG_APPLIED_RECORDER: &str =
    "target.tracking-config.child-applied-recorder";
pub const CORRELATION_PREFIX: &str = "tracking-config-update:";
pub const POLICY_RULE_LOCAL_CHILD_RUNTIME: &str = "policy.rule.tracking.local-child-runtime";
pub const POLICY_RULE_REMOTE_SYNC_DISABLED: &str = "policy.rule.tracking.remote-sync-disabled";
pub const POLICY_RULE_REMOTE_AI_DISABLED: &str = "policy.rule.tracking.remote-ai-disabled";
pub const REJECTION_REASON_INVALID_REQUEST: &str = "invalid-tracking-config-request";
pub const REJECTION_REASON_CHILD_RUNTIME_DISPATCH_BLOCKED: &str = "child-runtime-dispatch-blocked";
pub const ERROR_PARENT_CONFIG_EVENT_APPLIED: &str = "parent tracking config event applied";
pub const ERROR_CHILD_CONFIG_APPLIED_EVENT_RECORDED: &str =
    "child tracking config applied event recorded";
