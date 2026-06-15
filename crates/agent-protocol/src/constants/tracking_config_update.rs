pub const PARENT_EVENT_TYPE: &str = "tracking.config.updated.parent";
pub const CHILD_EVENT_TYPE: &str = "tracking.config.updated.child";
pub const RESPONSE_STATE_APPLIED: &str = "applied";
pub const RESPONSE_STATE_REJECTED: &str = "rejected";
pub const EFFECTIVE_STATE_ENABLED: &str = "enabled";
pub const EFFECTIVE_STATE_DISABLED: &str = "disabled";
pub const EFFECTIVE_STATE_DEGRADED: &str = "degraded";
pub const REQUEST_TIMEOUT_MS: u64 = 5000;
pub const TARGET_SCOPE_CHILD_DEVICE: &str = "child-device";
pub const TARGET_SCOPE_CHILD_PROFILE: &str = "child-profile";
pub const TARGET_SCOPE_FAMILY: &str = "family";
pub const TARGET_SCOPE_DEVICE_GROUP: &str = "device-group";
pub const ROUTE_LOCALHOST: &str = "localhost";
pub const ROUTE_LOCAL_NETWORK: &str = "local-network";
pub const ROUTE_CLOUD_RELAY: &str = "cloud-relay";
pub const SOURCE_COMPONENT_PARENT_PORTAL: &str = "parent-portal";
pub const SOURCE_COMPONENT_PARENT_AGENT_SERVICE: &str = "parent-agent-service";
pub const SOURCE_COMPONENT_CHILD_TRACKING_RUNTIME: &str = "child-tracking-runtime";
pub const SUBSCRIBER_PARENT_TRACKING_CONFIG_RELAY: &str = "subscriber.tracking-config.parent-relay";
pub const SUBSCRIBER_CHILD_TRACKING_CONFIG_APPLIER: &str =
    "subscriber.tracking-config.child-applier";
pub const TARGET_HANDLER_CHILD_TRACKING_RUNTIME: &str = "child-tracking-runtime";
pub const TARGET_HANDLER_PARENT_TRACKING_CONFIG_RELAY: &str = "target.tracking-config.parent-relay";
pub const TARGET_HANDLER_CHILD_TRACKING_CONFIG_APPLIER: &str =
    "target.tracking-config.child-applier";
pub const CORRELATION_PREFIX: &str = "tracking-config-update:";
pub const ERROR_PARENT_CONFIG_EVENT_APPLIED: &str = "parent tracking config event applied";
