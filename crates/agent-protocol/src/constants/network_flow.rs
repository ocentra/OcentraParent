pub const INDICATOR_ADAPTER_UNAVAILABLE: &str = "adapter-unavailable";
pub const INDICATOR_ENCRYPTED_CONTENT_UNAVAILABLE: &str = "encrypted-content-unavailable";
pub const INDICATOR_HIGH_VOLUME: &str = "high-volume";
pub const INDICATOR_NEW_DESTINATION: &str = "new-destination";
pub const INDICATOR_REPEATED_FAILURE: &str = "repeated-failure";
pub const INDICATOR_UNUSUAL_UNKNOWN_PROCESS: &str = "unusual-unknown-process";
pub const INDICATOR_VPN_PROXY_TUNNEL: &str = "vpn-proxy-tunnel";

pub const INDICATOR_LABEL_ADAPTER_UNAVAILABLE: &str = "Network adapter unavailable";
pub const INDICATOR_LABEL_ENCRYPTED_CONTENT_UNAVAILABLE: &str = "Encrypted content unavailable";
pub const INDICATOR_LABEL_HIGH_VOLUME: &str = "High network volume";
pub const INDICATOR_LABEL_REPEATED_FAILURE: &str = "Repeated connection failure";
pub const INDICATOR_LABEL_UNUSUAL_UNKNOWN_PROCESS: &str = "Unknown process attribution";
pub const INDICATOR_LABEL_VPN_PROXY_TUNNEL: &str = "VPN, proxy, or tunnel likely in use";
pub const LABEL_DESTINATION_UNKNOWN: &str = "Unknown destination";
pub const LABEL_PROCESS_UNKNOWN: &str = "Unknown process";

pub const EVENT_SCHEMA_VERSION: u16 = 1;
pub const EVENT_NETWORK_FLOW_OBSERVED: &str = "network.flow.observed";
pub const EVENT_NETWORK_DOMAIN_OBSERVED: &str = "network.domain.observed";
pub const EVENT_NETWORK_ACTIVITY_CLASSIFIED: &str = "network.activity.classified";
pub const EVENT_NETWORK_REVIEW_REQUESTED: &str = "network.review.requested";
pub const EVENT_AI_ANALYSIS_REQUESTED: &str = "ai.analysis.requested";
pub const EVENT_AI_ANALYSIS_COMPLETED: &str = "ai.analysis.completed";
pub const EVENT_POLICY_EVALUATION_REQUESTED: &str = "policy.evaluation.requested";
pub const EVENT_POLICY_DECISION_COMPLETED: &str = "policy.decision.completed";
pub const EVENT_ENFORCEMENT_COMMAND_ISSUED: &str = "enforcement.command.issued";
pub const EVENT_ENFORCEMENT_RESULT_OBSERVED: &str = "enforcement.result.observed";
pub const EVENT_AUDIT_ENTRY_COMMITTED: &str = "audit.entry.committed";
pub const EVENT_PORTAL_READ_MODEL_UPDATED: &str = "portal.read_model.updated";

pub const TARGET_NETWORK_OBSERVER: &str = "network-observer";
pub const TARGET_DOMAIN_OBSERVER: &str = "network-domain-observer";
pub const TARGET_ACTIVITY_CLASSIFIER: &str = "network-activity-classifier";
pub const TARGET_NETWORK_REVIEW: &str = "network-review-request";
pub const TARGET_AI_ANALYZER: &str = "network-ai-analyzer";
pub const TARGET_POLICY_ENGINE: &str = "network-policy-engine";
pub const TARGET_ENFORCEMENT_DRY_RUN: &str = "network-enforcement-dry-run";
pub const TARGET_AUDIT_WRITER: &str = "network-audit-writer";
pub const TARGET_PORTAL_READ_MODEL: &str = "network-portal-read-model";

pub const SUBSCRIBER_NETWORK_OBSERVER: &str = "network-runtime-observer-subscriber";
pub const SUBSCRIBER_DOMAIN_OBSERVER: &str = "network-runtime-domain-subscriber";
pub const SUBSCRIBER_ACTIVITY_CLASSIFIER: &str = "network-runtime-classifier-subscriber";
pub const SUBSCRIBER_NETWORK_REVIEW: &str = "network-runtime-review-subscriber";
pub const SUBSCRIBER_AI_REQUEST: &str = "network-runtime-ai-request-subscriber";
pub const SUBSCRIBER_AI_COMPLETE: &str = "network-runtime-ai-complete-subscriber";
pub const SUBSCRIBER_POLICY_REQUEST: &str = "network-runtime-policy-request-subscriber";
pub const SUBSCRIBER_POLICY_DECISION: &str = "network-runtime-policy-decision-subscriber";
pub const SUBSCRIBER_ENFORCEMENT_COMMAND: &str = "network-runtime-enforcement-command-subscriber";
pub const SUBSCRIBER_ENFORCEMENT_RESULT: &str = "network-runtime-enforcement-result-subscriber";
pub const SUBSCRIBER_AUDIT_ENTRY: &str = "network-runtime-audit-entry-subscriber";
pub const SUBSCRIBER_PORTAL_READ_MODEL: &str = "network-runtime-portal-read-model-subscriber";

pub const RUNTIME_COMPONENT_NETWORK_SPINE: &str = "network-runtime-spine";
pub const RUNTIME_INSTANCE_LOCAL_CHILD_AGENT: &str = "local-child-agent";
pub const AGGREGATE_NETWORK_FLOW_PREFIX: &str = "network-flow-";
pub const CORRELATION_NETWORK_RUNTIME_PREFIX: &str = "network-runtime-correlation-";
pub const IDEMPOTENCY_NETWORK_RUNTIME_PREFIX: &str = "network-runtime-idempotency-";
pub const IDEMPOTENCY_NETWORK_REVIEW_PREFIX: &str = "network-review-idempotency-";
pub const REQUEST_NETWORK_REVIEW_PREFIX: &str = "network-review-request-";
pub const REQUEST_NETWORK_REVIEW_TIMEOUT_MS: u64 = 50;
pub const ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES: &str = "network runtime chain publishes";
pub const ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES_DEGRADED: &str =
    "network runtime chain publishes degraded state";
pub const ERROR_NETWORK_RUNTIME_QUEUE_DRAINS: &str = "network runtime queued flow drains";
pub const ERROR_NETWORK_RUNTIME_PAYLOAD_DECODES: &str = "network runtime payload decodes";
pub const ERROR_NETWORK_RUNTIME_REVIEW_COMPLETES: &str = "network runtime review request completes";
