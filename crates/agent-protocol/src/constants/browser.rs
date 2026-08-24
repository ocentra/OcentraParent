pub const ACTIVE_STATE_KNOWN_ACTIVE: &str = "known-active";
pub const ACTIVE_STATE_KNOWN_INACTIVE: &str = "known-inactive";
pub const ACTIVE_STATE_UNKNOWN: &str = "unknown";
pub const ACTIVE_PROOF_SOURCE_CDP_FOCUS_ACTIVATION: &str = "cdp-focus-activation";
pub const ACTIVE_PROOF_SOURCE_FOREGROUND_CORRELATION: &str = "foreground-correlation";
pub const ACTIVE_PROOF_SOURCE_MANAGED_EXTENSION_EVENT: &str = "managed-extension-event";
pub const ACTIVE_PROOF_SOURCE_OWNED_SHELL_EVENT: &str = "owned-shell-event";
pub const ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY: &str = "target-list-only";
pub const ACTIVE_TAB_CAPABILITY_KNOWN_ACTIVE_SUPPORTED: &str = "known-active-supported";
pub const ACTIVE_TAB_CAPABILITY_MANUAL_REQUIRED: &str = "manual-required";
pub const ACTIVE_TAB_CAPABILITY_NOT_CLAIMED: &str = "not-claimed";
pub const ACTIVE_TAB_CAPABILITY_TARGET_LIST_ONLY: &str = "target-list-only";
pub const ACTIVE_TAB_CAPABILITY_UNAVAILABLE: &str = "unavailable";
pub const ACTIVE_TAB_CAPABILITY_UNSUPPORTED: &str = "unsupported";
pub const ADAPTER_ID_MANAGED_CHROMIUM_DEVTOOLS: &str = "managed-chromium-devtools-adapter";
pub const BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS: &str = "managed-loopback-devtools-redacted";
pub const BRIDGE_KIND_CHROMIUM_DEVTOOLS_PROTOCOL: &str = "chromium-devtools-protocol";
pub const CAPABILITY_STATUS_ADAPTER_ERROR: &str = "adapter-error";
pub const CAPABILITY_STATUS_AVAILABLE: &str = "available";
pub const CAPABILITY_STATUS_BRIDGE_MISSING: &str = "bridge-missing";
pub const CAPABILITY_STATUS_DISABLED_BY_PARENT: &str = "disabled-by-parent";
pub const CAPABILITY_STATUS_MANAGED_PROFILE_MISSING: &str = "managed-profile-missing";
pub const CAPABILITY_STATUS_PERMISSION_LIMITED: &str = "permission-limited";
pub const CAPABILITY_STATUS_STALE: &str = "stale";
pub const CAPABILITY_STATUS_TAB_LIST_ONLY: &str = "tab-list-only";
pub const CAPABILITY_STATUS_UNMANAGED_BROWSER: &str = "unmanaged-browser";
pub const CAPABILITY_STATUS_UNSUPPORTED_BROWSER: &str = "unsupported-browser";
pub const CHANNEL_BETA: &str = "beta";
pub const CHANNEL_CANARY: &str = "canary";
pub const CHANNEL_DEV: &str = "dev";
pub const CHANNEL_STABLE: &str = "stable";
pub const CHANNEL_UNKNOWN: &str = "unknown";
pub const CUSTODY_CHILD_DEVICE_LOCAL: &str = "child-device-local";
pub const CUSTODY_LOCAL_NETWORK_CHILD_AGENT: &str = "local-network-child-agent";
pub const CUSTODY_PARENT_CACHE: &str = "parent-cache";
pub const CUSTODY_PARENT_OWNED_EXPORT: &str = "parent-owned-export";
pub const CUSTODY_UNAVAILABLE: &str = "unavailable";
pub const EVIDENCE_ID_PREFIX: &str = "browser-evidence-";
pub const EVENT_ID_PREFIX: &str = "activity-browser-url-observed-";
pub const EVENT_SCHEMA_VERSION: u16 = 1;
pub const EVENT_BROWSER_EVIDENCE_OBSERVED: &str = "browser.evidence.observed";
pub const EVENT_BROWSER_EVIDENCE_JOURNALED: &str = "browser.evidence.journaled";
pub const EVENT_BROWSER_AI_ANALYSIS_REQUESTED: &str =
    super::child_domain_runtime::BROWSER_AI_ANALYSIS_REQUESTED_EVENT_TYPE;
pub const EVENT_BROWSER_AI_ANALYSIS_COMPLETED: &str = "browser.ai.analysis.completed";
pub const EVENT_BROWSER_POLICY_EVALUATION_REQUESTED: &str =
    super::child_domain_runtime::BROWSER_POLICY_EVALUATION_REQUESTED_EVENT_TYPE;
pub const EVENT_BROWSER_POLICY_DECISION_COMPLETED: &str = "browser.policy.decision.completed";
pub const EVENT_BROWSER_ACTION_INTENT_STATUS_REQUESTED: &str =
    "browser.action-intent.status.requested";
pub const EVENT_BROWSER_ACTION_INTENT_HANDOFF_REQUESTED: &str =
    "browser.action-intent.handoff.requested";
pub const EVENT_BROWSER_RUNTIME_STREAM_REPORT_REQUESTED: &str =
    "browser.runtime.stream.report.requested";
pub const EVENT_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_REQUESTED: &str =
    "browser.social.provider-receipt.status.requested";
pub const EVENT_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS_REQUESTED: &str =
    "browser.social.report-writer-delivery.status.requested";
pub const EVENT_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS_REQUESTED: &str =
    "browser.social.parent-notification-delivery.status.requested";
pub const EVENT_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_REQUESTED: &str =
    "browser.social.alert-report.parent-surface.status.requested";
pub const EVENT_BROWSER_INTERVENTION_COMMAND_ISSUED: &str = "browser.intervention.command.issued";
pub const EVENT_BROWSER_INTERVENTION_RESULT_OBSERVED: &str = "browser.intervention.result.observed";
pub const EVENT_BROWSER_AUDIT_ENTRY_COMMITTED: &str = "browser.audit.entry.committed";
pub const EVENT_BROWSER_READ_MODEL_PROJECTED: &str = "browser.read-model.projected";
pub const SUBSCRIBER_BROWSER_EVIDENCE_OBSERVER: &str = "browser-evidence-observer";
pub const SUBSCRIBER_BROWSER_EVIDENCE_JOURNAL: &str = "browser-evidence-journal";
pub const SUBSCRIBER_BROWSER_AI_REQUEST: &str = "browser-ai-request";
pub const SUBSCRIBER_BROWSER_AI_COMPLETE: &str = "browser-ai-complete";
pub const SUBSCRIBER_BROWSER_POLICY_REQUEST: &str = "browser-policy-request";
pub const SUBSCRIBER_BROWSER_POLICY_DECISION: &str = "browser-policy-decision";
pub const SUBSCRIBER_BROWSER_ACTION_INTENT_STATUS: &str = "browser-action-intent-status";
pub const SUBSCRIBER_BROWSER_ACTION_INTENT_HANDOFF: &str = "browser-action-intent-handoff";
pub const SUBSCRIBER_BROWSER_RUNTIME_STREAM_REPORT: &str = "browser-runtime-stream-report";
pub const SUBSCRIBER_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS: &str =
    "browser-social-provider-receipt-status";
pub const SUBSCRIBER_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS: &str =
    "browser-social-report-writer-delivery-status";
pub const SUBSCRIBER_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS: &str =
    "browser-social-parent-notification-delivery-status";
pub const SUBSCRIBER_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS: &str =
    "browser-social-alert-report-parent-surface-status";
pub const SUBSCRIBER_BROWSER_INTERVENTION_COMMAND: &str = "browser-intervention-command";
pub const SUBSCRIBER_BROWSER_INTERVENTION_RESULT: &str = "browser-intervention-result";
pub const SUBSCRIBER_BROWSER_AUDIT_ENTRY: &str = "browser-audit-entry";
pub const SUBSCRIBER_BROWSER_READ_MODEL: &str = "browser-read-model";
pub const TARGET_BROWSER_EVIDENCE_OBSERVER: &str = "browser-evidence-observer";
pub const TARGET_BROWSER_EVIDENCE_JOURNAL: &str = "browser-evidence-journal";
pub const TARGET_BROWSER_AI_ANALYZER: &str = "browser-ai-analyzer";
pub const TARGET_BROWSER_POLICY_ENGINE: &str = "browser-policy-engine";
pub const TARGET_BROWSER_ACTION_INTENT_STATUS: &str = "browser-action-intent-status";
pub const TARGET_BROWSER_ACTION_INTENT_HANDOFF: &str = "browser-action-intent-handoff";
pub const TARGET_BROWSER_RUNTIME_STREAM_REPORT: &str = "browser-runtime-stream-report";
pub const TARGET_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS: &str =
    "browser-social-provider-receipt-status";
pub const TARGET_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS: &str =
    "browser-social-report-writer-delivery-status";
pub const TARGET_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS: &str =
    "browser-social-parent-notification-delivery-status";
pub const TARGET_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS: &str =
    "browser-social-alert-report-parent-surface-status";
pub const TARGET_BROWSER_INTERVENTION_ADAPTER: &str = "browser-intervention-adapter";
pub const TARGET_BROWSER_AUDIT_WRITER: &str = "browser-audit-writer";
pub const TARGET_BROWSER_READ_MODEL: &str = "browser-read-model";
pub const RUNTIME_COMPONENT_BROWSER_SPINE: &str = "browser-event-runtime-spine";
pub const RUNTIME_INSTANCE_LOCAL_BROWSER_RUNTIME: &str = "local-browser-runtime";
pub const AGGREGATE_BROWSER_RUNTIME_PREFIX: &str = "browser-runtime-";
pub const CORRELATION_BROWSER_RUNTIME_PREFIX: &str = "browser-runtime-correlation-";
pub const IDEMPOTENCY_BROWSER_RUNTIME_PREFIX: &str = "browser-runtime-idempotency-";
pub const IDEMPOTENCY_BROWSER_ACTION_INTENT_STATUS_PREFIX: &str =
    "browser-action-intent-status-idempotency-";
pub const IDEMPOTENCY_BROWSER_ACTION_INTENT_HANDOFF_PREFIX: &str =
    "browser-action-intent-handoff-idempotency-";
pub const IDEMPOTENCY_BROWSER_RUNTIME_STREAM_REPORT_PREFIX: &str =
    "browser-runtime-stream-report-idempotency-";
pub const IDEMPOTENCY_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_PREFIX: &str =
    "browser-social-provider-receipt-status-idempotency-";
pub const IDEMPOTENCY_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS_PREFIX: &str =
    "browser-social-report-writer-delivery-status-idempotency-";
pub const IDEMPOTENCY_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS_PREFIX: &str =
    "browser-social-parent-notification-delivery-status-idempotency-";
pub const IDEMPOTENCY_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_PREFIX: &str =
    "browser-social-alert-report-parent-surface-status-idempotency-";
pub const REQUEST_BROWSER_ACTION_INTENT_STATUS_PREFIX: &str =
    "browser-action-intent-status-request-";
pub const REQUEST_BROWSER_ACTION_INTENT_HANDOFF_PREFIX: &str =
    "browser-action-intent-handoff-request-";
pub const REQUEST_BROWSER_RUNTIME_STREAM_REPORT_PREFIX: &str =
    "browser-runtime-stream-report-request-";
pub const REQUEST_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_PREFIX: &str =
    "browser-social-provider-receipt-status-request-";
pub const REQUEST_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS_PREFIX: &str =
    "browser-social-report-writer-delivery-status-request-";
pub const REQUEST_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS_PREFIX: &str =
    "browser-social-parent-notification-delivery-status-request-";
pub const REQUEST_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_PREFIX: &str =
    "browser-social-alert-report-parent-surface-status-request-";
pub const REQUEST_BROWSER_ACTION_INTENT_STATUS_TIMEOUT_MS: u64 = 50;
pub const REQUEST_BROWSER_ACTION_INTENT_HANDOFF_TIMEOUT_MS: u64 = 50;
pub const REQUEST_BROWSER_RUNTIME_STREAM_REPORT_TIMEOUT_MS: u64 = 250;
pub const REQUEST_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_TIMEOUT_MS: u64 = 50;
pub const REQUEST_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS_TIMEOUT_MS: u64 = 50;
pub const REQUEST_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS_TIMEOUT_MS: u64 = 50;
pub const REQUEST_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_TIMEOUT_MS: u64 = 50;
pub const TEST_BROWSER_RUNTIME_EVIDENCE_REF: &str = "browser-evidence-ref-test";
pub const TEST_BROWSER_RUNTIME_JOURNAL_REF: &str = "browser-journal-ref-test";
pub const TEST_BROWSER_RUNTIME_AI_REQUEST_REF: &str = "browser-ai-request-ref-test";
pub const TEST_BROWSER_RUNTIME_AI_ANALYSIS_REF: &str = "browser-ai-analysis-ref-test";
pub const TEST_BROWSER_RUNTIME_POLICY_EVALUATION_REF: &str = "browser-policy-evaluation-ref-test";
pub const TEST_BROWSER_RUNTIME_POLICY_DECISION_REF: &str = "browser-policy-decision-ref-test";
pub const TEST_BROWSER_RUNTIME_POLICY_PREVIEW_ID: &str = "browser-policy-preview-test";
pub const TEST_BROWSER_RUNTIME_ACTION_INTENT_ID: &str = "browser-action-intent-test";
pub const ACTION_INTENT_ID_PREFIX: &str = "browser-action-intent-";
pub const ACTION_INTENT_OUTBOX_REF_PREFIX: &str = "browser-action-intent-outbox-";
pub const ACTION_INTENT_HANDOFF_REF_PREFIX: &str = "browser-action-intent-handoff-";
pub const TEST_BROWSER_RUNTIME_ACTION_INTENT_OUTBOX_REF: &str = "browser-action-intent-outbox-test";
pub const TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REF: &str =
    "browser-action-intent-handoff-test";
pub const TEST_BROWSER_RUNTIME_ACTION_INTENT_DURABLE_RESULT_REF: &str =
    "browser-action-intent-durable-result-test";
pub const TEST_BROWSER_RUNTIME_ACTION_INTENT_DURABLE_STORE_REF: &str =
    "browser-action-intent-durable-store-test";
pub const TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_READ_MODEL_REF: &str =
    "browser-action-intent-handoff-read-model-test";
pub const TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_SUPPORT_STATUS_REF: &str =
    "browser-action-intent-handoff-support-status-test";
pub const ERROR_BROWSER_RUNTIME_ACTION_INTENT_DURABLE_HANDOFF: &str =
    "browser action-intent durable handoff proof failed";
pub const SOCIAL_PROVIDER_RECEIPT_STATE_PROVIDER_DISPATCH_REQUIRED: &str =
    "provider-dispatch-required";
pub const SOCIAL_PROVIDER_RECEIPT_STATE_MANUAL_REQUIRED: &str = "manual-receipt-required";
pub const SOCIAL_PROVIDER_RECEIPT_RUNTIME_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_ATTEMPT_REF: &str =
    "browser-social-provider-attempt-test";
pub const TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_PROOF_REF: &str =
    "browser-social-provider-receipt-proof-required-test";
pub const TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_DURABLE_RESULT_REF: &str =
    "browser-social-provider-receipt-durable-result-test";
pub const TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_DURABLE_STORE_REF: &str =
    "browser-social-provider-receipt-durable-store-test";
pub const TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_READ_MODEL_REF: &str =
    "browser-social-provider-receipt-read-model-test";
pub const TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_SUPPORT_STATUS_REF: &str =
    "browser-social-provider-receipt-support-status-test";
pub const ERROR_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_DURABLE: &str =
    "browser social provider receipt durable proof failed";
pub const TEST_BROWSER_RUNTIME_INTERVENTION_COMMAND_REF: &str =
    "browser-intervention-command-ref-test";
pub const TEST_BROWSER_RUNTIME_INTERVENTION_RESULT_REF: &str =
    "browser-intervention-result-ref-test";
pub const TEST_BROWSER_RUNTIME_AUDIT_ENTRY_REF: &str = "browser-audit-entry-ref-test";
pub const TEST_BROWSER_RUNTIME_READ_MODEL_REF: &str = "browser-read-model-ref-test";
pub const EXACT_URL_CAPABILITY_MANAGED_EXACT_URL_AVAILABLE: &str = "managed-exact-url-available";
pub const EXACT_URL_CAPABILITY_MANAGED_TARGET_LIST_ONLY: &str = "managed-target-list-only";
pub const EXACT_URL_CAPABILITY_MANUAL_REQUIRED: &str = "manual-required";
pub const EXACT_URL_CAPABILITY_NOT_CLAIMED: &str = "not-claimed";
pub const EXACT_URL_CAPABILITY_UNAVAILABLE: &str = "unavailable";
pub const EXACT_URL_CAPABILITY_UNSUPPORTED: &str = "unsupported";
pub const FAMILY_BRAVE: &str = "brave";
pub const FAMILY_CHROME: &str = "chrome";
pub const FAMILY_EDGE: &str = "edge";
pub const FAMILY_FIREFOX: &str = "firefox";
pub const FAMILY_OPERA: &str = "opera";
pub const FAMILY_UNKNOWN: &str = "unknown";
pub const FAMILY_UNKNOWN_CHROMIUM: &str = "unknown-chromium";
pub const HTTP_BODY_SEPARATOR: &str = "\r\n\r\n";
pub const HTTP_CONNECTION_CLOSE: &str = "Connection: close";
pub const HTTP_HEADER_CONTENT_LENGTH: &str = "content-length:";
pub const HTTP_GET_JSON_LIST: &str = "GET /json/list HTTP/1.1";
pub const HTTP_GET_JSON_VERSION: &str = "GET /json/version HTTP/1.1";
pub const HTTP_HEADER_HOST_LOOPBACK: &str = "Host: 127.0.0.1";
pub const HTTP_LINE_SEPARATOR: &str = "\r\n";
pub const HTTP_OK_PREFIX: &str = "HTTP/1.1 200 OK";
pub const INTERVENTION_ACTION_ALLOW: &str = "allow";
pub const INTERVENTION_ACTION_APPROVAL_HOLD: &str = "approval-hold";
pub const INTERVENTION_ACTION_ASK_PARENT: &str = "parent-review";
pub const INTERVENTION_ACTION_BLOCK: &str = "block";
pub const INTERVENTION_ACTION_CHECKING_HOLD: &str = "checking-hold";
pub const INTERVENTION_ACTION_MONITOR: &str = "monitor";
pub const INTERVENTION_ACTION_REDIRECT: &str = "redirect";
pub const INTERVENTION_ACTION_RELAUNCH_MANAGED: &str = "relaunch-managed";
pub const INTERVENTION_ACTION_TERMINATE_PROCESS: &str = "terminate-process";
pub const INTERVENTION_ACTION_TIME_LIMIT: &str = "time-limit";
pub const INTERVENTION_ACTION_UNKNOWN: &str = "unknown";
pub const INTERVENTION_ACTION_WARN: &str = "warn";
pub const INTERVENTION_CAPABILITY_ADAPTER_ERROR: &str = "adapter-error";
pub const INTERVENTION_CAPABILITY_DISABLED_BY_PARENT: &str = "disabled-by-parent";
pub const INTERVENTION_CAPABILITY_NEEDS_MANAGED_EXTENSION: &str = "needs-managed-extension";
pub const INTERVENTION_CAPABILITY_NEEDS_MANAGED_SESSION: &str = "needs-managed-session";
pub const INTERVENTION_CAPABILITY_NEEDS_OS_APP_CONTROL: &str = "needs-os-app-control";
pub const INTERVENTION_CAPABILITY_READY: &str = "ready";
pub const INTERVENTION_CAPABILITY_UNSUPPORTED_BROWSER: &str = "unsupported-browser";
pub const INTERVENTION_DECISION_SOURCE_LOCAL_AI: &str = "local-ai";
pub const INTERVENTION_DECISION_SOURCE_MANUAL_TEST: &str = "manual-test";
pub const INTERVENTION_DECISION_SOURCE_PARENT_PORTAL: &str = "parent-portal";
pub const INTERVENTION_DECISION_SOURCE_PARENT_RULE: &str = "parent-rule";
pub const INTERVENTION_DECISION_SOURCE_SYSTEM: &str = "system";
pub const INTERVENTION_DECISION_SOURCE_UNKNOWN: &str = "unknown";
pub const INTERVENTION_EVENT_ID_PREFIX: &str = "activity-browser-intervention-applied-";
pub const INTERVENTION_ID_PREFIX: &str = "browser-intervention-";
pub const INTERVENTION_READ_MODEL_REPORTED_EVENT_ID: &str =
    "browser-intervention-read-model-reported";
pub const INTERVENTION_MECHANISM_CHROMIUM_CDP_FETCH: &str = "chromium-cdp-fetch";
pub const INTERVENTION_MECHANISM_APPROVAL_HOLD_PAGE: &str = "approval-hold-page";
pub const INTERVENTION_MECHANISM_CHECKING_HOLD_PAGE: &str = "checking-hold-page";
pub const INTERVENTION_MECHANISM_MANAGED_EXTENSION: &str = "managed-extension";
pub const INTERVENTION_MECHANISM_MANAGED_BLOCK_PAGE: &str = "managed-block-page";
pub const INTERVENTION_MECHANISM_MONITOR_ONLY: &str = "monitor-only";
pub const INTERVENTION_MECHANISM_NONE: &str = "none";
pub const INTERVENTION_MECHANISM_OS_APP_CONTROL: &str = "os-app-control";
pub const INTERVENTION_MECHANISM_OWNED_WEBVIEW: &str = "owned-webview";
pub const INTERVENTION_MECHANISM_WEBDRIVER_BIDI_NETWORK: &str = "webdriver-bidi-network";
pub const INTERVENTION_OUTCOME_ALLOWED: &str = "allowed";
pub const INTERVENTION_OUTCOME_APPLIED: &str = "applied";
pub const INTERVENTION_OUTCOME_APPROVAL_REQUIRED: &str = "approval-required";
pub const INTERVENTION_OUTCOME_BLOCKED: &str = "blocked";
pub const INTERVENTION_OUTCOME_FAILED: &str = "failed";
pub const INTERVENTION_OUTCOME_HELD: &str = "held";
pub const INTERVENTION_OUTCOME_MANUAL_REQUIRED: &str = "manual-required";
pub const INTERVENTION_OUTCOME_MONITOR_ONLY: &str = "monitor-only";
pub const INTERVENTION_OUTCOME_REDIRECTED: &str = "redirected";
pub const INTERVENTION_OUTCOME_RELAUNCH_STARTED: &str = "relaunch-started";
pub const INTERVENTION_OUTCOME_TERMINATED: &str = "terminated";
pub const INTERVENTION_OUTCOME_UNSUPPORTED: &str = "unsupported";
pub const INTERVENTION_OUTCOME_WARNED: &str = "warned";
pub const INTERVENTION_BOUNDARY_MANAGED_SESSION: &str = "managed-session";
pub const INTERVENTION_BOUNDARY_UNMANAGED_BROWSER_PROCESS: &str = "unmanaged-browser-process";
pub const INTERVENTION_BOUNDARY_BROWSER_LIKE_PROCESS: &str = "browser-like-process";
pub const INTERVENTION_BOUNDARY_UNSUPPORTED: &str = "unsupported";
pub const INTERVENTION_BOUNDARY_UNKNOWN: &str = "unknown";
pub const INTERVENTION_EXACT_URL_PROVEN: &str = "exact-url-proven";
pub const INTERVENTION_EXACT_URL_NOT_CLAIMED: &str = "not-claimed";
pub const INTERVENTION_EXACT_URL_UNAVAILABLE: &str = "unavailable";
pub const INTERVENTION_UNMANAGED_DETECTION_NONE: &str = "none";
pub const INTERVENTION_UNMANAGED_DETECTION_DETECTED: &str = "detected";
pub const INTERVENTION_UNMANAGED_DETECTION_WARNED: &str = "warned";
pub const INTERVENTION_UNMANAGED_DETECTION_TERMINATED: &str = "terminated";
pub const INTERVENTION_UNMANAGED_DETECTION_MANUAL_REQUIRED: &str = "manual-required";
pub const INTERVENTION_UNMANAGED_DETECTION_UNAVAILABLE: &str = "unavailable";
pub const INTERVENTION_FIELD_BROWSER_BOUNDARY_STATE: &str = "browserBoundaryState";
pub const INTERVENTION_FIELD_EXACT_URL_CLAIM_STATE: &str = "exactUrlClaimState";
pub const INTERVENTION_FIELD_UNMANAGED_DETECTION_STATE: &str = "unmanagedDetectionState";
pub const INTERVENTION_DELIVERY_NOT_DELIVERED: &str = "not-delivered";
pub const INTERVENTION_DELIVERY_WARN_PAGE_RENDERED: &str = "warn-page-rendered";
pub const INTERVENTION_DELIVERY_BLOCK_PAGE_RENDERED: &str = "block-page-rendered";
pub const INTERVENTION_DELIVERY_APPROVAL_HOLD_RENDERED: &str = "approval-hold-rendered";
pub const INTERVENTION_DELIVERY_CHECKING_HOLD_RENDERED: &str = "checking-hold-rendered";
pub const INTERVENTION_DELIVERY_PORTAL_ROW_ONLY: &str = "portal-row-only";
pub const INTERVENTION_DELIVERY_MANUAL_REQUIRED: &str = "manual-required";
pub const INTERVENTION_SOURCE_ID_MANAGED_BROWSER: &str = "managed-browser-intervention";
pub const INTERVENTION_SUBJECT_ID_PREFIX: &str = "browser-intervention-target-";
pub const INTERVENTION_TARGET_TYPE_BROWSER_GAME: &str = "browser-game";
pub const INTERVENTION_TARGET_TYPE_BROWSER_PROCESS: &str = "browser-process";
pub const INTERVENTION_TARGET_TYPE_BROWSER_SESSION: &str = "browser-session";
pub const INTERVENTION_TARGET_TYPE_CLOUD_GAMING: &str = "cloud-gaming";
pub const INTERVENTION_TARGET_TYPE_DOMAIN: &str = "domain";
pub const INTERVENTION_TARGET_TYPE_GAME_ACCOUNT: &str = "game-account";
pub const INTERVENTION_TARGET_TYPE_GAME_PURCHASE: &str = "game-purchase";
pub const INTERVENTION_TARGET_TYPE_SITE: &str = "site";
pub const INTERVENTION_TARGET_TYPE_SOCIAL_ACCOUNT_CREATION: &str = "social-account-creation";
pub const INTERVENTION_TARGET_TYPE_SOCIAL_FEED: &str = "social-feed";
pub const INTERVENTION_TARGET_TYPE_SOCIAL_SHORT_VIDEO_FEED: &str = "social-short-video-feed";
pub const INTERVENTION_TARGET_TYPE_SOCIAL_MESSAGING: &str = "social-messaging";
pub const INTERVENTION_TARGET_TYPE_SOCIAL_UPLOAD_POST: &str = "social-upload-post";
pub const INTERVENTION_TARGET_TYPE_SOCIAL_LIVESTREAM: &str = "social-livestream";
pub const INTERVENTION_TARGET_TYPE_UNBLOCKED_GAME_SITE: &str = "unblocked-game-site";
pub const INTERVENTION_TARGET_TYPE_UNKNOWN_GAME: &str = "unknown-game";
pub const INTERVENTION_TARGET_TYPE_UNKNOWN_SOCIAL_SITE: &str = "unknown-social-site";
pub const INTERVENTION_TARGET_TYPE_UNKNOWN: &str = "unknown";
pub const INTERVENTION_TARGET_TYPE_URL: &str = "url";
pub const INTERVENTION_TARGET_TYPE_VIDEO: &str = "video";
pub const INVENTORY_INSTALL_STATE_CANDIDATE_RUNNING: &str = "candidate-running";
pub const INVENTORY_INSTALL_STATE_INSTALLED: &str = "installed";
pub const INVENTORY_INSTALL_STATE_NOT_INSTALLED: &str = "not-installed";
pub const INVENTORY_INSTALL_STATE_PACKAGED: &str = "packaged";
pub const INVENTORY_INSTALL_STATE_PORTABLE: &str = "portable";
pub const INVENTORY_INSTALL_STATE_UNKNOWN: &str = "unknown";
pub const INVENTORY_RUNNING_STATE_NOT_RUNNING: &str = "not-running";
pub const INVENTORY_RUNNING_STATE_RUNNING_MANAGED: &str = "running-managed";
pub const INVENTORY_RUNNING_STATE_RUNNING_UNMANAGED: &str = "running-unmanaged";
pub const INVENTORY_RUNNING_STATE_RUNNING_UNKNOWN: &str = "running-unknown";
pub const INVENTORY_RUNNING_STATE_UNKNOWN: &str = "unknown";
pub const INVENTORY_REASON_MANAGED_TARGET_LIST_ACTIVE_TAB_UNPROVED: &str =
    "managed-target-list-active-tab-unproved";
pub const INVENTORY_REASON_CROSS_PLATFORM_MANUAL_REQUIRED: &str =
    "cross-platform-browser-manual-required";
pub const INVENTORY_REASON_UNMANAGED_BROWSER_PROCESS_ONLY: &str = "unmanaged-browser-process-only";
pub const INVENTORY_REASON_WINDOWS_BROWSER_PROCESS_UNSUPPORTED: &str =
    "windows-browser-process-unsupported";
pub const INVENTORY_REASON_WINDOWS_CHROMIUM_FORK_MANUAL_REQUIRED: &str =
    "windows-chromium-fork-manual-required";
pub const INVENTORY_REASON_WINDOWS_MANAGED_PROFILE_REQUIRED: &str =
    "windows-managed-profile-required";
pub const INVENTORY_REASON_WINDOWS_PACKAGE_MANUAL_REQUIRED: &str =
    "windows-package-browser-manual-required";
pub const INVENTORY_REASON_WINDOWS_UNSUPPORTED_LATER_ADAPTER: &str =
    "windows-unsupported-later-adapter";
pub const INVENTORY_ROW_ID_EDGE_STABLE: &str = "browser-inventory-edge-stable";
pub const INVENTORY_ROW_ID_MANAGED_CHROME: &str = "browser-inventory-chrome-managed";
pub const INVENTORY_ROW_ID_PREFIX_WINDOWS: &str = "browser-inventory-windows";
pub const INVENTORY_ROW_ID_PREFIX_PLATFORM: &str = "browser-inventory-platform";
pub const INVENTORY_ROW_ID_UNKNOWN_BROWSER: &str = "browser-inventory-unknown-browser";
pub const INVENTORY_ROW_ID_UNMANAGED_CHROME: &str = "browser-inventory-chrome-unmanaged";
pub const MANAGEMENT_TIER_MANAGED: &str = "managed";
pub const MANAGEMENT_TIER_MANAGED_PROFILE_EXTENSION: &str = "managed-profile-extension";
pub const MANAGEMENT_TIER_MANUAL_REQUIRED: &str = "manual-required";
pub const MANAGEMENT_TIER_OWNED_SHELL: &str = "owned-shell";
pub const MANAGEMENT_TIER_UNMANAGED: &str = "unmanaged";
pub const MANAGEMENT_TIER_UNKNOWN: &str = "unknown";
pub const MANAGEMENT_TIER_UNSUPPORTED: &str = "unsupported";
pub const MANAGED_PROFILE_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const MANAGED_PROFILE_STATE_MISSING: &str = "missing";
pub const MANAGED_PROFILE_STATE_NOT_APPLICABLE: &str = "not-applicable";
pub const MANAGED_PROFILE_STATE_READY: &str = "ready";
pub const MANAGED_PROFILE_STATE_REPAIR_REQUIRED: &str = "repair-required";
pub const MANAGED_PROFILE_STATE_UNAVAILABLE: &str = "unavailable";
pub const MANAGED_STATE_BRIDGE_CONNECTED: &str = "bridge-connected";
pub const MANAGED_STATE_BRIDGE_DISCONNECTED: &str = "bridge-disconnected";
pub const MANAGED_STATE_ERROR: &str = "error";
pub const MANAGED_STATE_INSTALLED_SUPPORTED: &str = "installed-supported";
pub const MANAGED_STATE_INSTALLED_UNSUPPORTED: &str = "installed-unsupported";
pub const MANAGED_STATE_LAUNCH_PENDING: &str = "launch-pending";
pub const MANAGED_STATE_MANAGED_PROFILE_READY: &str = "managed-profile-ready";
pub const MANAGED_STATE_NOT_INSTALLED: &str = "not-installed";
pub const MANAGED_STATE_PERMISSION_REQUIRED: &str = "permission-required";
pub const MANAGED_STATE_RUNNING_MANAGED: &str = "running-managed";
pub const MANAGED_STATE_STOPPED: &str = "stopped";
pub const NATIVE_HOST_ALLOWED_ORIGIN: &str = "chrome-extension://ocentra-managed-extension";
pub const NATIVE_HOST_EXTENSION_ID: &str = "ocentra-managed-extension";
pub const NATIVE_HOST_ID: &str = "ocentra-managed-native-host";
pub const NATIVE_HOST_MAX_MESSAGE_BYTES: usize = 1_048_576;
pub const NATIVE_HOST_MESSAGE_TYPE_TAB_STATE: &str = "tab-state";
pub const NATIVE_HOST_SCHEMA_VERSION: u64 = 1;
pub const NATIVE_HOST_STALE_HEARTBEAT_MS: u64 = 30_000;
pub const PERFORMANCE_BUDGET_BROWSER_GAME_RUNTIME_SIGNAL_COLLECTION: &str =
    "browser-game-runtime-signal-collection";
pub const PERFORMANCE_BUDGET_CDP_TARGET_MAPPING_100_TABS: &str = "cdp-target-mapping-100-tabs";
pub const PERFORMANCE_BUDGET_CLOUD_GAMING_HEURISTIC_TIMEOUT: &str =
    "cloud-gaming-heuristic-timeout";
pub const PERFORMANCE_BUDGET_INVENTORY_SCAN: &str = "inventory-scan";
pub const PERFORMANCE_BUDGET_JOURNAL_WRITE_PER_EVENT: &str = "journal-write-per-event";
pub const PERFORMANCE_BUDGET_LOCAL_AI_QUEUE_TIMEOUT: &str = "local-ai-queue-timeout";
pub const PERFORMANCE_BUDGET_MEMORY_CACHE_LOOKUP_INVALIDATION: &str =
    "memory-cache-lookup-invalidation";
pub const PERFORMANCE_BUDGET_MS_BROWSER_GAME_RUNTIME_SIGNAL_COLLECTION: u64 = 100;
pub const PERFORMANCE_BUDGET_MS_CDP_TARGET_MAPPING_100_TABS: u64 = 100;
pub const PERFORMANCE_BUDGET_MS_CLOUD_GAMING_HEURISTIC_TIMEOUT: u64 = 1_000;
pub const PERFORMANCE_BUDGET_MS_INVENTORY_SCAN: u64 = 250;
pub const PERFORMANCE_BUDGET_MS_JOURNAL_WRITE_PER_EVENT: u64 = 20;
pub const PERFORMANCE_BUDGET_MS_LOCAL_AI_QUEUE_TIMEOUT: u64 = 30_000;
pub const PERFORMANCE_BUDGET_MS_MEMORY_CACHE_LOOKUP_INVALIDATION: u64 = 20;
pub const PERFORMANCE_BUDGET_MS_PORTAL_RENDER_100_TABS: u64 = 500;
pub const PERFORMANCE_BUDGET_MS_RAPID_BRIDGE_RECONNECT: u64 = 500;
pub const PERFORMANCE_BUDGET_MS_SQLITE_REPLAY_10000_EVENTS: u64 = 2_000;
pub const PERFORMANCE_BUDGET_MS_SUPPORT_MATRIX_DERIVATION: u64 = 50;
pub const PERFORMANCE_BUDGET_MS_UNMANAGED_PROCESS_SCAN: u64 = 250;
pub const PERFORMANCE_BUDGET_MS_URL_SHAPE_METADATA_EXTRACTION: u64 = 100;
pub const PERFORMANCE_BUDGET_PORTAL_RENDER_100_TABS: &str = "portal-render-100-tabs";
pub const PERFORMANCE_BUDGET_RAPID_BRIDGE_RECONNECT: &str = "rapid-bridge-reconnect";
pub const PERFORMANCE_BUDGET_SQLITE_REPLAY_10000_EVENTS: &str = "sqlite-replay-10000-events";
pub const PERFORMANCE_BUDGET_SUPPORT_MATRIX_DERIVATION: &str = "support-matrix-derivation";
pub const PERFORMANCE_BUDGET_UNMANAGED_PROCESS_SCAN: &str = "unmanaged-process-scan";
pub const PERFORMANCE_BUDGET_URL_SHAPE_METADATA_EXTRACTION: &str = "url-shape-metadata-extraction";
pub const PERFORMANCE_SAMPLE_SIZE_1: usize = 1;
pub const PERFORMANCE_SAMPLE_SIZE_100: usize = 100;
pub const PERFORMANCE_SAMPLE_SIZE_10000: usize = 10_000;
pub const PROCESS_ID_UNKNOWN: u32 = 0;
pub const PRODUCT_NAME_ARC_BROWSER: &str = "Arc Browser";
pub const PRODUCT_NAME_BRAVE_BROWSER: &str = "Brave Browser";
pub const PRODUCT_NAME_CHROME_FOR_TESTING: &str = "Chrome for Testing";
pub const PRODUCT_NAME_CHROMIUM: &str = "Chromium";
pub const PRODUCT_NAME_DUCKDUCKGO_BROWSER: &str = "DuckDuckGo Browser";
pub const PRODUCT_NAME_FIREFOX_DEVELOPER_EDITION: &str = "Firefox Developer Edition";
pub const PRODUCT_NAME_GOOGLE_CHROME: &str = "Google Chrome";
pub const PRODUCT_NAME_MICROSOFT_EDGE: &str = "Microsoft Edge";
pub const PRODUCT_NAME_MOZILLA_FIREFOX: &str = "Mozilla Firefox";
pub const PRODUCT_NAME_OPERA_BROWSER: &str = "Opera Browser";
pub const PRODUCT_NAME_OPERA_GX_BROWSER: &str = "Opera GX Browser";
pub const PRODUCT_NAME_SAFARI_BROWSER: &str = "Safari";
pub const PRODUCT_NAME_TOR_BROWSER: &str = "Tor Browser";
pub const PRODUCT_NAME_VIVALDI_BROWSER: &str = "Vivaldi Browser";
pub const PROFILE_ID_DEV: &str = "managed-browser-profile-dev";
pub const PROFILE_SCOPE_ID_DEV: &str = "managed-profile-scope-dev";
pub const PROFILE_POLICY_REVISION_DEV: &str = "browser-policy-revision-dev";
pub const PROFILE_ROOT_REF_MANAGED: &str = "managed-profile-root-redacted";
pub const PROFILE_PATH_REF_MANAGED: &str = "managed-profile-redacted";
pub const PROFILE_STORE_METADATA_SUFFIX: &str = ".profile-store.json";
pub const PROFILE_STORE_MAX_METADATA_BYTES: usize = 64 * 1024;
pub const PROFILE_STORE_MAX_PROFILE_ID_BYTES: usize = 256;
pub const PROFILE_STORE_MAX_PROFILE_SCOPE_ID_BYTES: usize = 256;
pub const PROFILE_STORE_MAX_DEVICE_ID_BYTES: usize = 256;
pub const PROFILE_STORE_MAX_POLICY_REVISION_BYTES: usize = 256;
pub const PROFILE_STORE_MAX_TIMESTAMP_BYTES: usize = 64;
pub const PROFILE_STORE_MAX_REPAIR_REASON_BYTES: usize = 128;
pub const PROFILE_STORE_LIFECYCLE_READY: &str = "ready";
pub const PROFILE_STORE_LIFECYCLE_MISSING: &str = "missing";
pub const PROFILE_STORE_LIFECYCLE_REPAIR_REQUIRED: &str = "repair-required";
pub const PROFILE_STORE_LIFECYCLE_DELETED: &str = "deleted";
pub const PROFILE_STORE_LIFECYCLE_UNSAFE_DEFAULT_PROFILE: &str = "unsafe-default-profile";
pub const PROFILE_STORE_LIFECYCLE_UNOWNED_PROFILE: &str = "unowned-profile";
pub const PROFILE_STORE_LIFECYCLE_UNAVAILABLE: &str = "unavailable";
pub const PROFILE_STORE_REASON_CREATED: &str = "managed-profile-created";
pub const PROFILE_STORE_REASON_DELETED: &str = "managed-profile-deleted";
pub const PROFILE_STORE_REASON_DELETE_PENDING: &str = "managed-profile-delete-pending";
pub const PROFILE_STORE_REASON_METADATA_CORRUPT: &str = "managed-profile-metadata-corrupt";
pub const PROFILE_STORE_REASON_METADATA_MISSING: &str = "managed-profile-metadata-missing";
pub const PROFILE_STORE_REASON_PROFILE_DIR_MISSING: &str = "managed-profile-dir-missing";
pub const PROFILE_STORE_REASON_PROTECTED_CUSTODY_ADAPTER_UNAVAILABLE: &str =
    "managed-profile-protected-custody-adapter-unavailable";
pub const PROFILE_STORE_REASON_REPAIRED: &str = "managed-profile-repaired";
pub const PROFILE_STORE_TEST_ROOT_DIR: &str = "managed-browser-profile-store-test";
pub const PROFILE_STORE_TEST_DEVICE_ID: &str = "local-dev-agent";
pub const PROFILE_STORE_TEST_CREATE_SUFFIX: &str = "create";
pub const PROFILE_STORE_TEST_DELETE_SUFFIX: &str = "delete";
pub const PROFILE_STORE_TEST_LAUNCH_SUFFIX: &str = "launch";
pub const PROFILE_STORE_TEST_MISSING_SUFFIX: &str = "missing";
pub const PROFILE_STORE_TEST_REJECT_SUFFIX: &str = "reject";
pub const QUERY_VISIBILITY_LIVE_LAN: &str = "live-lan";
pub const QUERY_VISIBILITY_LIVE_LOCAL: &str = "live-local";
pub const QUERY_VISIBILITY_PARENT_CACHE: &str = "parent-cache";
pub const QUERY_VISIBILITY_PARENT_OWNED_EXPORT: &str = "parent-owned-export";
pub const QUERY_VISIBILITY_UNAVAILABLE: &str = "unavailable";
pub const SESSION_ID_DEV: &str = "managed-browser-session-dev";
pub const SESSION_ID_PREFIX_MANAGED: &str = "managed-browser-session";
pub const SOURCE_ID_MANAGED_CHROMIUM_DEVTOOLS: &str = "managed-chromium-devtools";
pub const SUPPORT_TIER_CANDIDATE: &str = "candidate";
pub const SUPPORT_TIER_MANAGED_TARGET_LIST: &str = "managed-target-list";
pub const SUPPORT_TIER_MANAGED_URL_TAB: &str = "managed-url-tab";
pub const SUPPORT_TIER_MANUAL_REQUIRED: &str = "manual-required";
pub const SUPPORT_TIER_UNMANAGED_PROCESS_ONLY: &str = "unmanaged-process-only";
pub const SUPPORT_TIER_UNKNOWN: &str = "unknown";
pub const SUPPORT_TIER_UNSUPPORTED: &str = "unsupported";
pub const SUBJECT_ID_PREFIX: &str = "browser-url-";
pub const TAB_ID_PREFIX: &str = "browser-tab-";
pub const UNMANAGED_FALLBACK_OS_BLOCK_MANUAL_REQUIRED: &str = "os-block-manual-required";
pub const UNMANAGED_FALLBACK_RELAUNCH_MANAGED: &str = "relaunch-managed";
pub const UNMANAGED_FALLBACK_REPORT_ONLY: &str = "report-only";
pub const UNMANAGED_FALLBACK_TERMINATE_PROCESS: &str = "terminate-process";
pub const UNMANAGED_FALLBACK_UNAVAILABLE: &str = "unavailable";
pub const UNMANAGED_FALLBACK_UNSUPPORTED: &str = "unsupported";
pub const UNMANAGED_FALLBACK_WARN_CHILD: &str = "warn-child";
pub const UNMANAGED_DETECTION_CONFIDENCE_HIGH: &str = "high";
pub const UNMANAGED_DETECTION_CONFIDENCE_LOW: &str = "low";
pub const UNMANAGED_DETECTION_CONFIDENCE_MEDIUM: &str = "medium";
pub const UNMANAGED_DETECTION_REASON_BROWSER_LIKE_PROCESS: &str = "browser-like-process";
pub const UNMANAGED_DETECTION_REASON_PACKAGED_BROWSER_PROCESS: &str = "packaged-browser-process";
pub const UNMANAGED_DETECTION_REASON_PORTABLE_BROWSER_PROCESS: &str = "portable-browser-process";
pub const UNMANAGED_DETECTION_REASON_POSSIBLE_BROWSER_GAME_BYPASS: &str =
    "possible-browser-game-bypass";
pub const UNMANAGED_DETECTION_REASON_POSSIBLE_CLOUD_GAMING_BYPASS: &str =
    "possible-cloud-gaming-bypass";
pub const UNMANAGED_DETECTION_REASON_POSSIBLE_SOCIAL_BYPASS: &str = "possible-social-bypass";
pub const UNMANAGED_DETECTION_REASON_SUPPORTED_BROWSER_OUTSIDE_MANAGED_SESSION: &str =
    "supported-browser-outside-managed-session";
pub const UNMANAGED_DETECTION_REASON_TOR_PRIVACY_BROWSER_PROCESS: &str =
    "tor-privacy-browser-process";
pub const UNMANAGED_DETECTION_REASON_UNSUPPORTED_BROWSER_PROCESS: &str =
    "unsupported-browser-process";
pub const UNMANAGED_ENFORCEMENT_BLOCKED_AND_RELAUNCHED_MANAGED: &str =
    "blocked-and-relaunched-managed";
pub const UNMANAGED_ENFORCEMENT_ALLOWED_UNMANAGED_EXCEPTION: &str = "allowed-unmanaged-exception";
pub const UNMANAGED_ENFORCEMENT_ASK_PARENT: &str = "parent-review";
pub const UNMANAGED_ENFORCEMENT_DEGRADED: &str = "degraded";
pub const UNMANAGED_ENFORCEMENT_MONITOR_ONLY: &str = "monitor-only";
pub const UNMANAGED_ENFORCEMENT_OS_BLOCK_CONFIGURED: &str = "os-block-configured";
pub const UNMANAGED_ENFORCEMENT_OS_BLOCK_MANUAL_REQUIRED: &str = "os-block-manual-required";
pub const UNMANAGED_ENFORCEMENT_READY_TO_BLOCK: &str = "ready-to-block";
pub const UNMANAGED_ENFORCEMENT_RELAUNCH_MANAGED_BROWSER: &str = "relaunch-managed-browser";
pub const UNMANAGED_ENFORCEMENT_REPORT_ONLY: &str = "report-only";
pub const UNMANAGED_ENFORCEMENT_REQUIRES_OS_APP_CONTROL: &str = "requires-os-app-control";
pub const UNMANAGED_ENFORCEMENT_TERMINATE_PROCESS: &str = "terminate-process";
pub const UNMANAGED_ENFORCEMENT_UNSUPPORTED: &str = "unsupported";
pub const UNMANAGED_ENFORCEMENT_UNAVAILABLE: &str = "unavailable";
pub const UNMANAGED_ENFORCEMENT_WARN_CHILD: &str = "warn-child";
pub const UNMANAGED_FALLBACK_ACTION_ALLOWED_UNMANAGED_EXCEPTION: &str =
    "allowed-unmanaged-exception";
pub const UNMANAGED_FALLBACK_ACTION_ASK_PARENT: &str = "parent-review";
pub const UNMANAGED_FALLBACK_ACTION_DEGRADED: &str = "degraded";
pub const UNMANAGED_FALLBACK_ACTION_OS_BLOCK_CONFIGURED: &str = "os-block-configured";
pub const UNMANAGED_FALLBACK_ACTION_OS_BLOCK_MANUAL_REQUIRED: &str = "os-block-manual-required";
pub const UNMANAGED_FALLBACK_ACTION_RELAUNCH_MANAGED_BROWSER: &str = "relaunch-managed-browser";
pub const UNMANAGED_FALLBACK_ACTION_REPORT_ONLY: &str = "report-only";
pub const UNMANAGED_FALLBACK_ACTION_TERMINATE_PROCESS: &str = "terminate-process";
pub const UNMANAGED_FALLBACK_ACTION_UNAVAILABLE: &str = "unavailable";
pub const UNMANAGED_FALLBACK_ACTION_WARN_CHILD: &str = "warn-child";
pub const UNMANAGED_PROCESS_HASH_REF_WINDOWS_REDACTED: &str =
    "windows-browser-process-hash-redacted";
pub const UNMANAGED_PROCESS_KIND_EMBEDDED_BROWSER_LIKE: &str = "embedded-browser-like";
pub const UNMANAGED_PROCESS_KIND_PACKAGED_BROWSER: &str = "packaged-browser";
pub const UNMANAGED_PROCESS_KIND_PORTABLE_BROWSER: &str = "portable-browser";
pub const UNMANAGED_PROCESS_KIND_POSSIBLE_BROWSER_GAME_BYPASS: &str =
    "possible-browser-game-bypass";
pub const UNMANAGED_PROCESS_KIND_POSSIBLE_CLOUD_GAMING_BYPASS: &str =
    "possible-cloud-gaming-bypass";
pub const UNMANAGED_PROCESS_KIND_POSSIBLE_SOCIAL_BYPASS: &str = "possible-social-bypass";
pub const UNMANAGED_PROCESS_KIND_SUPPORTED_BROWSER: &str = "supported-browser";
pub const UNMANAGED_PROCESS_KIND_TOR_PRIVACY_BROWSER: &str = "tor-privacy-browser";
pub const UNMANAGED_PROCESS_KIND_UNKNOWN_BROWSER_LIKE: &str = "unknown-browser-like";
pub const UNMANAGED_PROCESS_KIND_UNSUPPORTED_BROWSER: &str = "unsupported-browser";
pub const UNMANAGED_SIGNATURE_REF_WINDOWS_REDACTED: &str = "windows-browser-signature-redacted";
pub const CHROMIUM_ARG_NO_DEFAULT_BROWSER_CHECK: &str = "--no-default-browser-check";
pub const CHROMIUM_ARG_NO_FIRST_RUN: &str = "--no-first-run";
pub const CHROMIUM_ARG_PROFILE_DIRECTORY_PREFIX: &str = "--profile-directory=";
pub const CHROMIUM_ARG_REMOTE_DEBUGGING_ADDRESS_PREFIX: &str = "--remote-debugging-address=";
pub const CHROMIUM_ARG_REMOTE_DEBUGGING_PORT_PREFIX: &str = "--remote-debugging-port=";
pub const CHROMIUM_ARG_USER_DATA_DIR_PREFIX: &str = "--user-data-dir=";
pub const CHROMIUM_DEFAULT_URL: &str = "about:blank";
pub const CHROMIUM_INTERNAL_CHROME_PREFIX: &str = "chrome://";
pub const CHROMIUM_INTERNAL_DEVTOOLS_PREFIX: &str = "devtools://";
pub const CHROMIUM_INTERNAL_EDGE_PREFIX: &str = "edge://";
pub const CHROMIUM_REMOTE_DEBUGGING_LOOPBACK: &str = "127.0.0.1";
pub const DEVTOOLS_FIELD_BROWSER: &str = "Browser";
pub const DEVTOOLS_FIELD_ID: &str = "id";
pub const DEVTOOLS_FIELD_TAB_ID: &str = "tabId";
pub const DEVTOOLS_FIELD_TITLE: &str = "title";
pub const DEVTOOLS_FIELD_TYPE: &str = "type";
pub const DEVTOOLS_FIELD_URL: &str = "url";
pub const DEVTOOLS_FIELD_WEB_SOCKET_DEBUGGER_URL: &str = "webSocketDebuggerUrl";
pub const DEVTOOLS_FIELD_WINDOW_ID: &str = "windowId";
pub const DEVTOOLS_TARGET_TYPE_PAGE: &str = "page";
pub const DEVTOOLS_DEFAULT_BRIDGE_PORT: u16 = 9222;
pub const DEVTOOLS_MAX_RESPONSE_BYTES: usize = 262_144;
pub const DEVTOOLS_PORT_UNRESERVED: u16 = 0;
pub const DEVTOOLS_TEST_BRIDGE_PORT: u16 = 4242;
pub const DEVTOOLS_TEST_BROWSER_VERSION: &str = "Chrome/125.0.0.0";
pub const DEVTOOLS_TEST_BROWSER_LIKE_PROCESS: &str = "embedded-browser-shell.exe";
pub const DEVTOOLS_TEST_BLANK_AND_INTERNAL_LIST_BODY: &str = r#"[{"id":"target-blank","type":"page","url":"about:blank","title":"Blank"},{"id":"target-internal","type":"page","url":"chrome://settings","title":"Settings"}]"#;
pub const DEVTOOLS_TEST_EMPTY_LIST_BODY: &str = "[]";
pub const DEVTOOLS_TEST_EXECUTABLE_PATH: &str = "browser.exe";
pub const DEVTOOLS_TEST_EDGE_SHORTCUT_FILE_NAME: &str = "Microsoft Edge.lnk";
pub const DEVTOOLS_TEST_EDGE_STORE_PACKAGE_APPLICATION_ID: &str = "App";
pub const DEVTOOLS_TEST_EDGE_STORE_PACKAGE_DISPLAY_NAME: &str = "Microsoft Edge";
pub const DEVTOOLS_TEST_EDGE_STORE_PACKAGE_NAME: &str = "Microsoft.MicrosoftEdge.Stable";
pub const DEVTOOLS_TEST_EDGE_STORE_PACKAGE_USER_MODEL_ID: &str =
    "Microsoft.MicrosoftEdge.Stable!App";
pub const DEVTOOLS_TEST_EDGE_STORE_PACKAGE_MANIFEST_XML: &str = r#"<Package xmlns:uap="http://schemas.microsoft.com/appx/manifest/uap/windows10">
  <Identity Name="Microsoft.MicrosoftEdge.Stable" Publisher="CN=Microsoft Corporation" Version="1.0.0.0" />
  <Properties>
    <DisplayName>Microsoft Edge</DisplayName>
  </Properties>
  <Applications>
    <Application Id="App">
      <uap:VisualElements DisplayName="Microsoft Edge" />
    </Application>
  </Applications>
</Package>"#;
pub const DEVTOOLS_TEST_INVALID_JSON_BODY: &str = "{";
pub const DEVTOOLS_TEST_INVALID_LIST_BODY: &str = r#"{"id":"not-array"}"#;
pub const DEVTOOLS_TEST_WINDOWS_BROWSER_INVENTORY_DIR: &str = "windows-browser-inventory-test";
pub const DEVTOOLS_TEST_WINDOWS_BROWSER_INVENTORY_SOURCE_DIR: &str =
    "windows-browser-inventory-source-test";
pub const DEVTOOLS_TEST_INSTALLED_BROWSER_DIR: &str = "managed-browser-discovery-test";
pub const DEVTOOLS_TEST_UNMANAGED_PROCESS_ID: u32 = 5150;
pub const DEVTOOLS_TEST_LIST_BODY: &str = r#"[{"id":"target-1","type":"page","url":"https://example.test/learn","title":"Example learning page"}]"#;
pub const DEVTOOLS_TEST_LIST_BODY_WITH_TAB_WINDOW: &str = r#"[{"id":"target-1","type":"page","url":"https://example.test/learn","title":"Example learning page","tabId":"tab-1","windowId":"window-1"}]"#;
pub const DEVTOOLS_TEST_LIST_BODY_MISSING_ID: &str =
    r#"[{"type":"page","url":"https://example.test/learn","title":"Example learning page"}]"#;
pub const DEVTOOLS_TEST_LIST_BODY_MISSING_URL: &str =
    r#"[{"id":"target-1","type":"page","title":"Example learning page"}]"#;
pub const DEVTOOLS_TEST_LIST_BODY_WITH_DEBUGGER_URL: &str = r#"[{"id":"target-1","type":"page","url":"https://example.test/learn","title":"Example learning page","webSocketDebuggerUrl":"ws://127.0.0.1:4242/devtools/page/target-1"}]"#;
pub const DEVTOOLS_TEST_MSEDGE_BETA_PATH: &str =
    "C:\\Program Files (x86)\\Microsoft\\Edge Beta\\Application\\msedge.exe";
pub const DEVTOOLS_TEST_RAW_DEBUGGER_URL: &str = "ws://127.0.0.1:4242/devtools/page/target-1";
pub const DEVTOOLS_TEST_UNSUPPORTED_EXECUTABLE_PATH: &str = "unsupported-browser.exe";
pub const DEVTOOLS_TEST_UNOWNED_PROFILE_DIR: &str = "child-browser-profile";
pub const DEVTOOLS_TEST_PAGE_TITLE: &str = "Example learning page";
pub const DEVTOOLS_TEST_PAGE_URL: &str = "https://example.test/learn";
pub const DEVTOOLS_TEST_TARGET_ID: &str = "target-1";
pub const DEVTOOLS_TEST_OVERSIZED_BODY_UNIT: &str = "{}";
pub const DEVTOOLS_TEST_OVERSIZED_REPEAT_COUNT: usize = 140_000;
pub const DEVTOOLS_TEST_VERSION_BODY: &str = r#"{"Browser":"Chrome/125.0.0.0"}"#;
pub const DEVTOOLS_TIMEOUT_MS: u64 = 2000;
pub const EXECUTABLE_CHROME_LINUX: &str = "chrome";
pub const EXECUTABLE_CHROME_WINDOWS: &str = "chrome.exe";
pub const EXECUTABLE_ARC_WINDOWS: &str = "arc.exe";
pub const EXECUTABLE_BRAVE_WINDOWS: &str = "brave.exe";
pub const EXECUTABLE_CHROMIUM_WINDOWS: &str = "chromium.exe";
pub const EXECUTABLE_DUCKDUCKGO_WINDOWS: &str = "duckduckgo.exe";
pub const EXECUTABLE_FIREFOX_WINDOWS: &str = "firefox.exe";
pub const EXECUTABLE_GOOGLE_CHROME_LINUX: &str = "google-chrome";
pub const EXECUTABLE_MICROSOFT_EDGE_LINUX: &str = "microsoft-edge";
pub const EXECUTABLE_MSEDGE_LINUX: &str = "msedge";
pub const EXECUTABLE_MSEDGE_WINDOWS: &str = "msedge.exe";
pub const EXECUTABLE_OPERA_GX_WINDOWS: &str = "opera_gx.exe";
pub const EXECUTABLE_OPERA_WINDOWS: &str = "opera.exe";
pub const EXECUTABLE_TOR_WINDOWS: &str = "tor.exe";
pub const EXECUTABLE_VIVALDI_WINDOWS: &str = "vivaldi.exe";
pub const INVENTORY_EXECUTABLE_PATH_REF_WINDOWS_REDACTED: &str =
    "windows-browser-executable-redacted";
pub const INVENTORY_EXECUTABLE_PATH_REF_PLATFORM_REDACTED: &str =
    "platform-browser-executable-redacted";
pub const INVENTORY_FILE_HASH_REF_WINDOWS_REDACTED: &str = "windows-browser-file-hash-redacted";
pub const INVENTORY_PUBLISHER_SIGNATURE_REF_WINDOWS_REDACTED: &str =
    "windows-browser-publisher-signature-redacted";
pub const PATH_SEGMENT_APPLICATION: &str = "Application";
pub const PATH_SEGMENT_ARC: &str = "Arc";
pub const PATH_SEGMENT_BRAVE_BROWSER: &str = "Brave-Browser";
pub const PATH_SEGMENT_BRAVE_SOFTWARE: &str = "BraveSoftware";
pub const PATH_SEGMENT_BROWSER: &str = "Browser";
pub const PATH_SEGMENT_CHROME: &str = "Chrome";
pub const PATH_SEGMENT_CHROME_BETA: &str = "chrome beta";
pub const PATH_SEGMENT_CHROME_DEV: &str = "chrome dev";
pub const PATH_SEGMENT_CHROME_FOR_TESTING: &str = "Chrome for Testing";
pub const PATH_SEGMENT_CHROME_FOR_TESTING_NORMALIZED: &str = "chrome for testing";
pub const PATH_SEGMENT_CHROME_SXS: &str = "chrome sxs";
pub const PATH_SEGMENT_CHROMIUM: &str = "Chromium";
pub const PATH_SEGMENT_CHROMIUM_NORMALIZED: &str = "chromium";
pub const PATH_SEGMENT_DEFAULT: &str = "Default";
pub const PATH_SEGMENT_DEFAULT_NORMALIZED: &str = "default";
pub const PATH_SEGMENT_DUCKDUCKGO: &str = "DuckDuckGo";
pub const PATH_SEGMENT_EDGE: &str = "Edge";
pub const PATH_SEGMENT_EDGE_BETA: &str = "edge beta";
pub const PATH_SEGMENT_EDGE_DEV: &str = "edge dev";
pub const PATH_SEGMENT_EDGE_SXS: &str = "edge sxs";
pub const PATH_SEGMENT_FIREFOX_DEVELOPER_EDITION: &str = "Firefox Developer Edition";
pub const PATH_SEGMENT_FIREFOX_DEVELOPER_NORMALIZED: &str = "firefox developer edition";
pub const PATH_SEGMENT_FIREFOX_NIGHTLY: &str = "Firefox Nightly";
pub const PATH_SEGMENT_FIREFOX_NIGHTLY_NORMALIZED: &str = "firefox nightly";
pub const PATH_SEGMENT_GOOGLE: &str = "Google";
pub const PATH_SEGMENT_MANAGED_BROWSER: &str = "ManagedBrowser";
pub const PATH_SEGMENT_MICROSOFT: &str = "Microsoft";
pub const PATH_SEGMENT_MOZILLA_FIREFOX: &str = "Mozilla Firefox";
pub const PATH_SEGMENT_OCENTRA_PARENT: &str = "OcentraParent";
pub const PATH_SEGMENT_OPERA_GX_STABLE: &str = "Opera GX Stable";
pub const PATH_SEGMENT_OPERA_SOFTWARE: &str = "Opera Software";
pub const PATH_SEGMENT_OPERA_STABLE: &str = "Opera Stable";
pub const PATH_SEGMENT_PORTABLE_NORMALIZED: &str = "portable";
pub const PATH_SEGMENT_PROGRAMS: &str = "Programs";
pub const PATH_SEGMENT_START_MENU: &str = "Start Menu";
pub const PATH_SEGMENT_TOR_BROWSER: &str = "Tor Browser";
pub const PATH_SEGMENT_TOR_BROWSER_NORMALIZED: &str = "tor browser";
pub const PATH_SEGMENT_USER_DATA: &str = "User Data";
pub const PATH_SEGMENT_USER_DATA_NORMALIZED: &str = "user data";
pub const PATH_SEGMENT_VIVALDI: &str = "Vivaldi";
pub const PATH_SEGMENT_WEBVIEW_NORMALIZED: &str = "webview";
pub const PATH_SEGMENT_WINDOWS: &str = "Windows";
pub const PATH_SEGMENT_WINDOWS_APPS: &str = "WindowsApps";
pub const PATH_SEGMENT_WINDOWS_APPS_NORMALIZED: &str = "windowsapps";
pub const PACKAGE_FRAGMENT_ARC: &str = "arc";
pub const PACKAGE_FRAGMENT_BRAVE: &str = "brave";
pub const PACKAGE_FRAGMENT_CHROME: &str = "chrome";
pub const PACKAGE_FRAGMENT_CHROMIUM: &str = "chromium";
pub const PACKAGE_FRAGMENT_DUCKDUCKGO: &str = "duckduckgo";
pub const PACKAGE_FRAGMENT_EDGE: &str = "edge";
pub const PACKAGE_FRAGMENT_FIREFOX: &str = "firefox";
pub const PACKAGE_FRAGMENT_MICROSOFT_EDGE: &str = "microsoftedge";
pub const PACKAGE_FRAGMENT_OPERA: &str = "opera";
pub const PACKAGE_FRAGMENT_TOR: &str = "tor";
pub const PACKAGE_FRAGMENT_VIVALDI: &str = "vivaldi";
pub const PACKAGE_SCAN_LIMIT_BROWSER_DISCOVERY: usize = 128;
pub const SHORTCUT_LINK_FLAGS_HAS_LINK_INFO: u32 = 0x0000_0002;
pub const SHORTCUT_LINK_FLAGS_OFFSET: usize = 0x0000_0014;
pub const SHORTCUT_LINK_HEADER_SIZE: u32 = 0x0000_004c;
pub const SHORTCUT_LINK_INFO_FLAGS_OFFSET: usize = 0x0000_0008;
pub const SHORTCUT_LINK_INFO_HEADER_SIZE: u32 = 0x0000_001c;
pub const SHORTCUT_LINK_INFO_LOCAL_BASE_PATH_FLAG: u32 = 0x0000_0001;
pub const SHORTCUT_LINK_INFO_LOCAL_BASE_PATH_OFFSET: usize = 0x0000_0010;
pub const SHORTCUT_LINK_INFO_MIN_SIZE: usize = 0x0000_001c;
pub const SHORTCUT_LINK_INFO_SECTION_OFFSET: usize = 0x0000_004c;
pub const SHORTCUT_LINK_INFO_SIZE_OFFSET: usize = 0x0000_0000;
pub const SHORTCUT_SCAN_LIMIT_BROWSER_DISCOVERY: usize = 128;
pub const WINDOWS_SHORTCUT_EXTENSION: &str = "lnk";
pub const PATH_SEPARATOR_COLON: &str = ":";
pub const PATH_SEPARATOR_FORWARD: &str = "/";
pub const PATH_SEPARATOR_BACKSLASH: &str = "\\";
pub const PROCESS_SCAN_LIMIT_BROWSER_DISCOVERY: usize = 256;
pub const PROFILE_DIRECTORY_MANAGED_CHILD: &str = "OcentraManagedChild";
pub const PROFILE_ID_PREFIX_MANAGED: &str = "managed-browser-profile";
pub const URL_SCHEME_SEPARATOR: &str = "://";
