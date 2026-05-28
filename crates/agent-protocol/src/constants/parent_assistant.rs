pub const PROMPT_VERSION_LOCAL_V1: &str = "parent-assistant-local-v1";
pub const DEFAULT_REQUEST_ID: &str = "parent-assistant-request-local";
pub const DEFAULT_THREAD_ID: &str = "parent-assistant-thread-local";
pub const DEFAULT_MESSAGE_ID: &str = "parent-assistant-message-local";
pub const DEFAULT_FAMILY_ID: &str = "family-local";
pub const DEFAULT_PARENT_ACTOR_ID: &str = "parent-actor-local";
pub const DEFAULT_QUESTION: &str = "Summarize the recent child activity evidence.";
pub const DEFAULT_CITATION_LABEL: &str = "Recent activity";
pub const ACTIVITY_EVENT_CITATION_LABEL: &str = "Latest activity event";
pub const DEFAULT_ALLOWED_SUMMARY: &str =
    "Recent local activity summary is allowed parent evidence.";
pub const ACTIVITY_CONTEXT_READY: &str =
    "Recent local Activity report/read-model evidence is available for parent review.";
pub const ACTIVITY_CONTEXT_EMPTY: &str =
    "The local Activity query store is reachable but has no recent rows for this scope.";
pub const ACTIVITY_CONTEXT_UNAVAILABLE: &str =
    "The local Activity query store is unavailable for Parent Assistant evidence context.";
pub const ACTIVITY_CONTEXT_PREFIX: &str = "Activity read models: ";
pub const ACTIVITY_CONTEXT_RECENT_LABEL: &str = "appUse=";
pub const ACTIVITY_CONTEXT_SCREEN_LABEL: &str = ", screen=";
pub const ACTIVITY_CONTEXT_BROWSER_LABEL: &str = ", browser=";
pub const ACTIVITY_CONTEXT_GAMES_LABEL: &str = ", games=";
pub const ACTIVITY_CONTEXT_NETWORK_LABEL: &str = ", network=";
pub const ACTIVITY_EVENT_SUMMARY_PREFIX: &str = "Latest cited activity event: ";
pub const PROMPT_SYSTEM: &str =
    "Answer as a parent-facing assistant. Cite the provided evidence. Do not enforce directly.";
pub const PROMPT_QUESTION_LABEL: &str = "Question:";
pub const PROMPT_EVIDENCE_LABEL: &str = "Evidence:";
pub const PROMPT_SEPARATOR: &str = "\n";
pub const DEFAULT_PREVIEW_ID: &str = "parent-assistant-action-preview-local";
pub const ACTION_PREVIEW_SUMMARY: &str =
    "Preview only. Child-agent contract approval is required before any action.";
pub const ACTION_PREVIEW_POLICY_SUMMARY: &str =
    "Policy suggestion preview only. Controller lease and child-agent contract execution are required before any rule changes.";
pub const ACTION_PREVIEW_SCHEDULE_SUMMARY: &str =
    "Schedule-change preview only. Controller lease and child-agent contract execution are required before any schedule changes.";
pub const ACTION_PREVIEW_TIME_LIMIT_SUMMARY: &str =
    "Time-limit preview only. Controller lease and child-agent contract execution are required before any limit changes.";
pub const ACTION_PREVIEW_NONE_SUMMARY: &str = "No backend action is prepared from this answer.";
pub const QUESTION_POLICY_HINT: &str = "policy";
pub const QUESTION_RULE_HINT: &str = "rule";
pub const QUESTION_SCHEDULE_HINT: &str = "schedule";
pub const QUESTION_BEDTIME_HINT: &str = "bedtime";
pub const QUESTION_TIME_LIMIT_HINT: &str = "time limit";
pub const QUESTION_LIMIT_HINT: &str = "limit";
pub const TEST_POLICY_QUESTION: &str = "Suggest a policy rule for evening games.";
pub const API_PROVIDER_ID_NOT_AUTHORIZED: &str = "api-provider-not-authorized";
pub const API_PROVIDER_CUSTODY_LABEL: &str = "parent-authorized-api-ai";
pub const API_PROVIDER_RETENTION_POLICY: &str = "no-retention-without-parent-authorization";
pub const API_PROVIDER_DELETION_POLICY: &str = "delete-provider-cache-on-parent-request";
pub const API_PROVIDER_NOT_AUTHORIZED_REASON: &str = "api-ai-provider-not-authorized";
pub const LOCAL_PROVIDER_DEGRADED: &str = "local provider returned degraded output";
pub const LOCAL_PROVIDER_BUSY: &str = "local provider is busy";
pub const BACKEND_STATE_SCAFFOLD_ONLY: &str = "scaffold-only";
pub const BACKEND_NOT_CONNECTED: &str = "parent-assistant-backend-not-connected";
pub const PROVIDER_CONFIGURED: &str = "configured";
pub const PROVIDER_DEGRADED: &str = "degraded";
pub const PROVIDER_UNAVAILABLE: &str = "unavailable";
pub const ANSWER_ANSWERED: &str = "answered";
pub const ANSWER_QUEUED: &str = "queued";
pub const ANSWER_DEGRADED: &str = "degraded";
pub const ANSWER_UNAVAILABLE: &str = "unavailable";
