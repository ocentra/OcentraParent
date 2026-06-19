pub const PROMPT_VERSION_LOCAL_V1: &str = "parent-assistant-local-v1";
pub const DEFAULT_REQUEST_ID: &str = "parent-assistant-request-local";
pub const DEFAULT_THREAD_ID: &str = "parent-assistant-thread-local";
pub const DEFAULT_MESSAGE_ID: &str = "parent-assistant-message-local";
pub const DEFAULT_RUN_ID: &str = "parent-assistant-run-local";
pub const DEFAULT_ACTION_INTENT_ID: &str = "parent-assistant-action-intent-local";
pub const DEFAULT_FAMILY_ID: &str = "family-local";
pub const DEFAULT_PARENT_ACTOR_ID: &str = "parent-actor-local";
pub const DEFAULT_QUESTION: &str = "Summarize the recent child activity evidence.";
pub const DEFAULT_CITATION_LABEL: &str = "Recent activity";
pub const ACTIVITY_EVENT_CITATION_LABEL: &str = "Latest activity event";
pub const ACTIVITY_REPORT_CITATION_LABEL: &str = "Activity report";
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
pub const ACTIVITY_REPORT_SUMMARY_PREFIX: &str = "Activity report context: ";
pub const ACTIVITY_REPORT_SUMMARY_ID_LABEL: &str = "report=";
pub const ACTIVITY_REPORT_SUMMARY_STATE_LABEL: &str = ", savedState=";
pub const ACTIVITY_REPORT_SUMMARY_FILE_LABEL: &str = ", fileName=";
pub const ACTIVITY_REPORT_SUMMARY_SAVED_AT_LABEL: &str = ", savedAt=";
pub const ACTIVITY_REPORT_SUMMARY_STORAGE_REASON_LABEL: &str = ", storageReason=";
pub const ACTIVITY_REPORT_SUMMARY_SECTIONS_LABEL: &str = ", sections=";
pub const ACTIVITY_REPORT_SUMMARY_SOURCE_LABEL: &str = ", sources=";
pub const ACTIVITY_REPORT_SUMMARY_READY_SECTIONS_LABEL: &str = ", readySections=";
pub const ACTIVITY_REPORT_SUMMARY_OFFLINE_SOURCES_LABEL: &str = ", offlineSources=";
pub const ACTIVITY_REPORT_SUMMARY_STALE_SOURCES_LABEL: &str = ", staleSources=";
pub const ACTIVITY_REPORT_SUMMARY_UNAVAILABLE_SOURCES_LABEL: &str = ", unavailableSources=";
pub const ACTIVITY_REPORT_SUMMARY_UNREACHABLE_SOURCES_LABEL: &str = ", unreachableSources=";
pub const ACTIVITY_REPORT_SUMMARY_SECTION_KINDS_LABEL: &str = ", sectionKinds=";
pub const ACTIVITY_REPORT_SUMMARY_OFFLINE_SOURCE_IDS_LABEL: &str = ", offlineSourceIds=";
pub const ACTIVITY_REPORT_SUMMARY_STALE_SOURCE_IDS_LABEL: &str = ", staleSourceIds=";
pub const ACTIVITY_REPORT_SUMMARY_UNREACHABLE_SOURCE_IDS_LABEL: &str = ", unreachableSourceIds=";
pub const ACTIVITY_REPORT_SUMMARY_UNAVAILABLE_SOURCE_IDS_LABEL: &str = ", unavailableSourceIds=";
pub const ACTIVITY_REPORT_SUMMARY_CUSTODY_LABEL: &str = ", custodyLabel=";
pub const ACTIVITY_REPORT_SUMMARY_SOURCE_DATA_LABEL: &str = ", sourceLabel=";
pub const ACTIVITY_REPORT_SUMMARY_RAW_CHILD_EVIDENCE_LABEL: &str = ", rawChildEvidenceIncluded=";
pub const EVIDENCE_CUSTODY_ACTIVITY_SUMMARY: &str = "parent-owned-activity-summary";
pub const EVIDENCE_CUSTODY_ACTIVITY_EVENT: &str = "parent-owned-activity-event";
pub const EVIDENCE_CUSTODY_ACTIVITY_REPORT: &str = "parent-owned-activity-report";
pub const EVIDENCE_SOURCE_ACTIVITY_QUERY_STORE_SUMMARY: &str = "activity-query-store-summary";
pub const EVIDENCE_SOURCE_ACTIVITY_EVENT_CITATION: &str = "activity-event-citation";
pub const EVIDENCE_SOURCE_SAVED_ACTIVITY_REPORT_HISTORY: &str = "saved-activity-report-history";
pub const ACTIVITY_REPORT_SUMMARY_NONE: &str = "none";
pub const PROMPT_SYSTEM: &str =
    "Answer as a parent-facing assistant. Cite the provided evidence. Do not enforce directly.";
pub const PROMPT_QUESTION_LABEL: &str = "Question:";
pub const PROMPT_EVIDENCE_LABEL: &str = "Evidence:";
pub const PROMPT_SEPARATOR: &str = "\n";
pub const DEFAULT_PREVIEW_ID: &str = "parent-assistant-action-preview-local";
pub const ACTION_PREVIEW_SUMMARY: &str =
    "Preview only. Child-agent contract approval is required before any action.";
pub const ACTION_PREVIEW_POLICY_SUMMARY: &str = "Policy suggestion preview only. Controller lease and child-agent contract execution are required before any rule changes.";
pub const ACTION_PREVIEW_SCHEDULE_SUMMARY: &str = "Schedule-change preview only. Controller lease and child-agent contract execution are required before any schedule changes.";
pub const ACTION_PREVIEW_TIME_LIMIT_SUMMARY: &str = "Time-limit preview only. Controller lease and child-agent contract execution are required before any limit changes.";
pub const ACTION_PREVIEW_NONE_SUMMARY: &str = "No backend action is prepared from this answer.";
pub const ACTION_PREVIEW_DRAFT_REASON: &str = "Action preview is a draft only. Controller lease and child-agent contract are required before changes.";
pub const ACTION_PREVIEW_AUDIT_REASON: &str =
    "Preview generated from cited parent-owned evidence; child-agent validation is still required.";
pub const THREAD_TITLE_DEFAULT: &str = "Parent Assistant local thread";
pub const THREAD_STORAGE_DIR: &str = "parent-assistant-threads";
pub const THREAD_STORAGE_FILE: &str = "threads.json";
pub const THREAD_DURABLE_REASON: &str =
    "Parent Assistant thread state is persisted in the local Parent Assistant store.";
pub const THREAD_VOLATILE_REASON: &str =
    "Parent Assistant thread state is service-backed and volatile until durable sync is wired.";
pub const THREAD_ARCHIVED_REASON: &str =
    "Parent Assistant thread is archived in the local Parent Assistant store.";
pub const THREAD_STORAGE_UNAVAILABLE_REASON: &str = "parent-assistant-thread-storage-unavailable";
pub const RUN_NOT_RUNNING_REASON: &str = "parent-assistant-run-not-running";
pub const ACTION_CONFIRM_CONTRACT_REQUIRED_REASON: &str =
    "Controller lease and child-agent policy contract are required before applying this action.";
pub const ACTION_CONFIRM_PREVIEW_REQUIRED_REASON: &str =
    "Action confirmation rejected because no matching preview id was provided.";
pub const ACTION_CONFIRM_RAW_PROSE_REJECTED_REASON: &str =
    "Action confirmation rejected because raw assistant prose is not an executable action intent.";
pub const ACTION_CONFIRM_AUDIT_REASON: &str =
    "Parent confirmation cannot write policy until child-agent validation is wired.";
pub const REQUIRED_CHILD_CONTRACT_POLICY_WRITE: &str = "child-agent-policy-write-contract";
pub const QUESTION_POLICY_HINT: &str = "policy";
pub const QUESTION_RULE_HINT: &str = "rule";
pub const QUESTION_SCHEDULE_HINT: &str = "schedule";
pub const QUESTION_BEDTIME_HINT: &str = "bedtime";
pub const QUESTION_TIME_LIMIT_HINT: &str = "time limit";
pub const QUESTION_LIMIT_HINT: &str = "limit";
pub const TEST_POLICY_QUESTION: &str = "Suggest a policy rule for evening games.";
pub const TEST_THREAD_ID: &str = "parent-assistant-thread-proof";
pub const TEST_MESSAGE_ID: &str = "parent-assistant-message-proof";
pub const API_PROVIDER_ID_NOT_AUTHORIZED: &str = "api-provider-not-authorized";
pub const API_PROVIDER_ID_AUTHORIZED: &str = "api-provider-authorized";
pub const API_PROVIDER_AUTHORIZED_ENV: &str = "OCENTRA_PARENT_PARENT_ASSISTANT_API_AI_AUTHORIZED";
pub const API_PROVIDER_DEGRADED_ENV: &str = "OCENTRA_PARENT_PARENT_ASSISTANT_API_AI_DEGRADED";
pub const API_PROVIDER_AUTHORIZATION_AUTHORIZED: &str = "authorized";
pub const API_PROVIDER_AUTHORIZATION_NOT_AUTHORIZED: &str = "not-authorized";
pub const API_PROVIDER_ACCESS_NOT_AUTHORIZED: &str = "not-authorized";
pub const API_PROVIDER_ACCESS_AUTHORIZED_UNAVAILABLE: &str = "authorized-unavailable";
pub const API_PROVIDER_ACCESS_AUTHORIZED_DEGRADED: &str = "authorized-degraded";
pub const API_PROVIDER_CUSTODY_LABEL: &str = "parent-authorized-api-ai";
pub const API_PROVIDER_CUSTODY_STATE: &str = "parent-owned-citations-only";
pub const API_PROVIDER_RETENTION_POLICY: &str = "no-retention-without-parent-authorization";
pub const API_PROVIDER_RETENTION_NO_AUTHORIZATION: &str =
    "no-retention-without-parent-authorization";
pub const API_PROVIDER_RETENTION_PARENT_AUTHORIZED: &str = "parent-authorized-no-default-retention";
pub const API_PROVIDER_DELETION_POLICY: &str = "delete-provider-cache-on-parent-request";
pub const API_PROVIDER_DELETION_STATE: &str = "delete-provider-cache-on-parent-request";
pub const API_PROVIDER_NOT_AUTHORIZED_REASON: &str = "api-ai-provider-not-authorized";
pub const API_PROVIDER_AUTHORIZED_UNAVAILABLE_REASON: &str =
    "api-ai-provider-adapter-not-configured";
pub const API_PROVIDER_AUTHORIZED_DEGRADED_REASON: &str = "api-ai-provider-authorized-degraded";
pub const PROVIDER_ROUTE_LOCAL_READY_REASON: &str =
    "Local parent-assistant provider is configured and selected.";
pub const PROVIDER_ROUTE_LOCAL_DEGRADED_REASON: &str = "Local parent-assistant provider is degraded; API AI remains optional and unavailable for safety decisions.";
pub const PROVIDER_ROUTE_LOCAL_UNAVAILABLE_REASON: &str =
    "Local parent-assistant provider is unavailable and no API provider is selected.";
pub const PROVIDER_ROUTE_API_UNAVAILABLE_REASON: &str =
    "API AI is parent-authorized but no configured API adapter is available.";
pub const PROVIDER_ROUTE_API_DEGRADED_REASON: &str =
    "API AI is parent-authorized but degraded; it remains outside child-safety decisions.";
pub const PROVIDER_ROUTE_NONE_REASON: &str = "No parent-assistant provider is available; cited source refs remain available for degraded answers.";
pub const LOCAL_PROVIDER_DEGRADED: &str = "local provider returned degraded output";
pub const LOCAL_PROVIDER_BUSY: &str = "local provider is busy";
pub const BACKEND_STATE_SCAFFOLD_ONLY: &str = "scaffold-only";
pub const BACKEND_NOT_CONNECTED: &str = "parent-assistant-backend-not-connected";
pub const BACKEND_STATE_RUNTIME_BACKED: &str = "runtime-backed";
pub const BACKEND_STATE_DURABLE_LOCAL: &str = "durable-local";
pub const BACKEND_STATE_VOLATILE_LOCAL: &str = "volatile-local";
pub const BACKEND_STATE_CONTRACT_REQUIRED: &str = "contract-required";
pub const BACKEND_STATE_UNAVAILABLE: &str = "unavailable";
pub const FIELD_THREAD: &str = "parentAssistantThread";
pub const FIELD_THREAD_RESPONSE: &str = "parentAssistantThreadResponse";
pub const FIELD_THREADS: &str = "parentAssistantThreads";
pub const FIELD_PROVIDER_STATUS: &str = "parentAssistantProviderStatus";
pub const FIELD_PROVIDER_ROUTE: &str = "parentAssistantProviderRoute";
pub const FIELD_RUN_CANCEL_RESULT: &str = "parentAssistantRunCancelResult";
pub const FIELD_ACTION_CONFIRM_RESULT: &str = "parentAssistantActionConfirmResult";
pub const FIELD_THREAD_ID: &str = "assistantThreadId";
pub const FIELD_MESSAGE_ID: &str = "assistantMessageId";
pub const FIELD_RUN_ID: &str = "assistantRunId";
pub const FIELD_ACTION_INTENT_ID: &str = "assistantActionIntentId";
pub const FIELD_REQUIRED_CHILD_CONTRACTS: &str = "requiredChildContracts";
pub const PROVIDER_CONFIGURED: &str = "configured";
pub const PROVIDER_DEGRADED: &str = "degraded";
pub const PROVIDER_UNAVAILABLE: &str = "unavailable";
pub const ANSWER_ANSWERED: &str = "answered";
pub const ANSWER_QUEUED: &str = "queued";
pub const ANSWER_DEGRADED: &str = "degraded";
pub const ANSWER_UNAVAILABLE: &str = "unavailable";
pub const RUN_STATE_QUEUED: &str = "queued";
pub const RUN_STATE_ACTIVE: &str = "active";
pub const RUN_STATE_COMPLETED: &str = "completed";
pub const RUN_STATE_FAILED: &str = "failed";
pub const RUN_STATE_CANCELLED: &str = "cancelled";
pub const RUN_STATE_DEGRADED: &str = "degraded";
pub const RUN_STATE_UNAVAILABLE: &str = "unavailable";
