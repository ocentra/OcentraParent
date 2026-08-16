pub const CONTRACT_SCHEMA_VERSION_V0_6: &str = "v0.6";

pub const ACTION_ALLOW: &str = "allow";
pub const ACTION_WARN: &str = "warn";
pub const ACTION_BLOCK: &str = "block";
pub const ACTION_TIME_LIMIT: &str = "time-limit";
pub const ACTION_ASK_PARENT: &str = "ask-parent";
pub const ACTION_UNKNOWN: &str = "unknown";

pub const HANDOFF_NOT_REQUESTED: &str = "not-requested";
pub const HANDOFF_DISABLED: &str = "disabled";
pub const HANDOFF_PENDING: &str = "pending";
pub const HANDOFF_HANDED_OFF: &str = "handed-off";

pub const TARGET_TYPE_APP: &str = "app";
pub const TARGET_TYPE_PROCESS: &str = "process";
pub const TARGET_TYPE_WINDOW: &str = "window";
pub const TARGET_TYPE_DOMAIN: &str = "domain";
pub const TARGET_TYPE_SITE: &str = "site";
pub const TARGET_TYPE_CATEGORY: &str = "category";
pub const TARGET_TYPE_VIDEO: &str = "video";
pub const TARGET_TYPE_CHANNEL: &str = "channel";
pub const TARGET_TYPE_ACTIVITY_TYPE: &str = "activity-type";
pub const TARGET_TYPE_DEVICE: &str = "device";

pub const ACTOR_ROLE_PARENT: &str = "parent";
pub const ACTOR_ROLE_GUARDIAN: &str = "guardian";
pub const ACTOR_ROLE_SYSTEM: &str = "system";

pub const EVIDENCE_KIND_JOURNAL_EVENT: &str = "journal-event";
pub const EVIDENCE_KIND_QUERY_STORE_SUMMARY: &str = "query-store-summary";
pub const EVIDENCE_KIND_ACTIVITY_EVENT: &str = "activity-event";
pub const EVIDENCE_KIND_POLICY_DECISION: &str = "policy-decision";
pub const EVIDENCE_KIND_LOCAL_AI_RESULT: &str = "local-ai-result";

pub const UNKNOWN_NONE: &str = "none";
pub const UNKNOWN_MISSING_EVIDENCE: &str = "missing-evidence";
pub const UNKNOWN_LOW_CONFIDENCE: &str = "low-confidence";
pub const UNKNOWN_MODEL_UNAVAILABLE: &str = "model-unavailable";
pub const UNKNOWN_POLICY_CONFLICT: &str = "policy-conflict";

pub const MEMORY_KIND_EVIDENCE_MEMORY: &str = "evidence-memory";
pub const MEMORY_KIND_RECENT_ACTIVITY: &str = "recent-activity";
pub const MEMORY_KIND_POLICY_MEMORY: &str = "policy-memory";
pub const MEMORY_KIND_SEMANTIC_MEMORY: &str = "semantic-memory";

pub const GRAPH_KIND_ENTITY: &str = "graph-entity";
pub const GRAPH_KIND_EDGE: &str = "graph-edge";

pub const REASON_LOCAL_AI_RESULT_MISSING: &str = "local-ai-result-missing";
pub const REASON_MISSING_EVIDENCE: &str = "missing-evidence";
pub const REASON_NO_MATCHING_PARENT_RULE: &str = "no-matching-parent-rule";
pub const REASON_POLICY_CONFLICT: &str = "policy-conflict";
pub const REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW: &str =
    "network-evidence-grade-parent-review";
pub const REASON_NETWORK_EVIDENCE_GRADE_OBSERVE_ONLY: &str = "network-evidence-grade-observe-only";

pub const NETWORK_EVIDENCE_GRADE_A: &str = "A";
pub const NETWORK_EVIDENCE_GRADE_B: &str = "B";
pub const NETWORK_EVIDENCE_GRADE_C: &str = "C";
pub const NETWORK_EVIDENCE_GRADE_D: &str = "D";
pub const NETWORK_POLICY_ACTION_NONE: &str = "none";
pub const NETWORK_POLICY_ACTION_MONITOR: &str = "monitor";
pub const NETWORK_POLICY_MAPPING_MODE_DRY_RUN: &str = "dry-run";
pub const NETWORK_POLICY_MAPPING_MODE_OBSERVE_ONLY: &str = "observe-only";
pub const NETWORK_POLICY_MAPPING_MODE_PARENT_REVIEW: &str = "parent-review";

pub const PREVIEW_CUSTODY_ACTIVITY_STORE: &str = "child-device-activity-store";
pub const PREVIEW_CAPABILITY_READY: &str = "ready";
pub const PREVIEW_CAPABILITY_NO_EVIDENCE: &str = "no-stored-evidence";
pub const PREVIEW_ID_PREFIX: &str = "policy-preview-";
pub const PREVIEW_DECISION_ID_PREFIX: &str = "policy-preview-decision-";
pub const PARENT_RULE_CONTEXT_REFERENCE_COUNT_FIELD: &str = "parentRuleContextReferenceCount";
pub const PARENT_RULE_CONTEXT_REF_IDS_FIELD: &str = "parentRuleContextRefIds";

pub const TEST_DECISION_ID: &str = "decision-1";
pub const TEST_EVALUATED_AT: &str = "2026-05-20T20:45:00.000Z";
pub const TEST_EXPIRES_AT: &str = "2026-05-20T21:00:00.000Z";
pub const TEST_EVIDENCE_ID: &str = "evidence-1";
pub const TEST_AI_RESULT_ID: &str = "ai-result-1";
pub const TEST_AI_REQUEST_ID: &str = "request-1";
pub const TEST_TARGET_ID: &str = "target-1";
pub const TEST_TARGET_VALUE: &str = "video.example";
pub const TEST_PARENT_ACTOR_ID: &str = "parent-1";
pub const TEST_POLICY_VERSION: &str = "policy-v1";
pub const TEST_PARENT_RULE_CONTEXT_REF_ID: &str = "parent-rule-context-1";
pub const TEST_PARENT_RULE_CONTEXT_CUSTODY: &str = "child-device-query-store";
pub const TEST_FAMILY_ID: &str = "family-1";
pub const TEST_CHILD_PROFILE_ID: &str = "child-profile-1";
pub const TEST_CHILD_PROFILE_DISPLAY_NAME: &str = "Child One";
pub const TEST_PARENT_DEVICE_ID: &str = "device-1";
pub const TEST_PARENT_DEVICE_LABEL: &str = "Windows laptop";
pub const TEST_PARENT_DEVICE_PLATFORM_WINDOWS: &str = "windows";
pub const TEST_BLOCK_RULE_ID: &str = "rule-block";
pub const TEST_ALLOW_RULE_ID: &str = "rule-allow";
pub const TEST_TIME_LIMIT_RULE_ID: &str = "rule-time-limit";
pub const TEST_ASK_PARENT_RULE_ID: &str = "rule-ask-parent";
pub const TEST_DISABLED_RULE_ID: &str = "rule-disabled";
pub const TEST_EXPIRED_RULE_ID: &str = "rule-expired";
pub const TEST_REASON_PARENT_BLOCK: &str = "parent-explicit-block";
pub const TEST_REASON_PARENT_ALLOW: &str = "parent-explicit-allow";
pub const TEST_REASON_PARENT_TIME_LIMIT: &str = "parent-time-budget";
pub const TEST_REASON_PARENT_ASK: &str = "parent-permission-required";
pub const TEST_REASON_DISABLED: &str = "disabled-rule";
pub const TEST_REASON_EXPIRED: &str = "expired-rule";
pub const TEST_REASON_AI_ALLOW: &str = "local-ai-suggested-allow";
pub const TEST_REASON_AI_BLOCK: &str = "local-ai-suggested-block";
pub const TEST_RUNTIME_REFERENCE_ID: &str = "runtime-1";
pub const TEST_PROVIDER_ID: &str = "local-provider";
pub const TEST_MODEL_ID: &str = "safety-model";
pub const TEST_MODEL_REFERENCE: &str = "local-model-cache/safety-model";
pub const TEST_PROMPT_VERSION: &str = "prompt-v1";
pub const TEST_MEMORY_REFERENCE_ID: &str = "memory-1";
pub const TEST_GRAPH_REFERENCE_ID: &str = "graph-1";
pub const TEST_DERIVED_INDEX_VERSION: &str = "derived-index-v1";
pub const TEST_PREVIEW_ID: &str = "policy-preview-1";
