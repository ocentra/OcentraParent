pub const ACTIVITY_CAPTURE_APP_GAME_ERROR: &str = "activity-capture-app-game-error";
pub const APP_GAME_CHILD_UX_LOCAL_HANDOFF_ARTIFACT_PREFIX: &str =
    "app-game-child-ux-local-handoff-";
pub const APP_GAME_CHILD_UX_PARENT_SURFACE_INTENT_PREFIX: &str =
    "app-game-child-ux-parent-surface-";
pub const APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_PREFIX: &str =
    "app-game-child-ux-parent-preference-setup-";
pub const ACTIVITY_CAPTURE_IO_ERROR: &str = "activity-capture-io-error";
pub const ACTIVITY_CAPTURE_INVALID_KEY_LENGTH: &str = "activity-capture-invalid-key-length";
pub const ACTIVITY_CAPTURE_JOURNAL_ERROR: &str = "activity-capture-journal-error";
pub const ACTIVITY_CAPTURE_STORE_ERROR: &str = "activity-capture-store-error";
pub const BROWSER_BRIDGE_INVALID_JSON: &str = "browser-bridge-invalid-json";
pub const BROWSER_BRIDGE_INVALID_RESPONSE: &str = "browser-bridge-invalid-response";
pub const BROWSER_BRIDGE_INVALID_TARGET_PAYLOAD: &str = "browser-bridge-invalid-target-payload";
pub const BROWSER_BRIDGE_IO_ERROR: &str = "browser-bridge-io-error";
pub const BROWSER_BRIDGE_NO_PAGE_TARGETS: &str = "browser-bridge-no-page-targets";
pub const BROWSER_BRIDGE_NON_LOOPBACK_ENDPOINT: &str = "browser-bridge-non-loopback-endpoint";
pub const BROWSER_BRIDGE_RESPONSE_TOO_LARGE: &str = "browser-bridge-response-too-large";
pub const BROWSER_BRIDGE_STALE_SESSION: &str = "browser-bridge-stale-session";
pub const BROWSER_BRIDGE_TIMEOUT: &str = "browser-bridge-timeout";
pub const BROWSER_BRIDGE_UNTRUSTED_BROWSER_IDENTITY: &str =
    "browser-bridge-untrusted-browser-identity";
pub const BROWSER_BRIDGE_UNTRUSTED_PORT: &str = "browser-bridge-untrusted-port";
pub const BROWSER_BRIDGE_UNTRUSTED_PROCESS: &str = "browser-bridge-untrusted-process";
pub const BROWSER_BRIDGE_UNTRUSTED_PROFILE: &str = "browser-bridge-untrusted-profile";
pub const BROWSER_BRIDGE_UNTRUSTED_SESSION: &str = "browser-bridge-untrusted-session";
pub const BROWSER_INTERVENTION_MANAGED_SESSION_REQUIRED: &str =
    "browser-intervention-managed-session-required";
pub const BROWSER_INTERVENTION_OS_APP_CONTROL_REQUIRED: &str =
    "browser-intervention-os-app-control-required";
pub const MANAGED_BROWSER_EXECUTABLE_MISSING: &str = "managed-browser-executable-missing";
pub const MANAGED_BROWSER_BRIDGE_CONNECT_PENDING: &str = "managed-browser-bridge-connect-pending";
pub const MANAGED_BROWSER_BRIDGE_PORT_UNAVAILABLE: &str = "managed-browser-bridge-port-unavailable";
pub const MANAGED_BROWSER_INVALID_BRIDGE_PORT: &str = "managed-browser-invalid-bridge-port";
pub const MANAGED_BROWSER_INVALID_PROFILE: &str = "managed-browser-invalid-profile";
pub const MANAGED_BROWSER_LAUNCH_ERROR: &str = "managed-browser-launch-error";
pub const MANAGED_BROWSER_LAUNCH_PENDING: &str = "managed-browser-launch-pending";
pub const MANAGED_BROWSER_PROFILE_DIR_MISSING: &str = "managed-browser-profile-dir-missing";
pub const MANAGED_BROWSER_PROFILE_METADATA_CORRUPT: &str =
    "managed-browser-profile-metadata-corrupt";
pub const MANAGED_BROWSER_PROFILE_STORE_IO_ERROR: &str = "managed-browser-profile-store-io-error";
pub const MANAGED_BROWSER_UNMANAGED_PROCESS: &str = "managed-browser-unmanaged-process";
pub const MANAGED_BROWSER_UNSUPPORTED_EXECUTABLE: &str = "managed-browser-unsupported-executable";
pub const ACTIVITY_JOURNAL_CIPHER: &str = "xchacha20poly1305";
pub const ACTIVITY_STORE_UNAVAILABLE: &str = "Activity store is unavailable.";
pub const DEV_MODE: &str = "dev";
pub const EMPTY: &str = "";
pub const LOCAL_NETWORK_MODE: &str = "lan";
pub const LOCALHOST_API_REACHABLE: &str = "Agent service localhost API is reachable.";
pub const LOOPBACK_MODE: &str = "loopback";
pub const LAN_CONTROL_ACCEPTED: &str = "accepted";
pub const LAN_CONTROL_REJECTED: &str = "rejected";
pub const LAN_CONTROL_DEGRADED: &str = "degraded";
pub const LAN_AUDIT_CONTROL_ACCEPTED: &str = "control-accepted";
pub const LAN_AUDIT_CONTROL_REJECTED: &str = "control-rejected";
pub const LAN_AUDIT_CONTROLLER_LEASE_RENEWED: &str = "controller-lease-renewed";
pub const LAN_AUDIT_CONTROLLER_LEASE_RELEASED: &str = "controller-lease-released";
pub const LAN_AUDIT_CONTROLLER_LEASE_TAKEOVER_ACCEPTED: &str = "controller-lease-takeover-accepted";
pub const LAN_AUDIT_CONTROLLER_LEASE_TAKEOVER_REJECTED: &str = "controller-lease-takeover-rejected";
pub const LAN_AUDIT_LAN_AI_PROVIDER_ADVERTISED: &str = "lan-ai-provider-advertised";
pub const LAN_AUDIT_LAN_AI_JOB_ACCEPTED: &str = "lan-ai-job-accepted";
pub const LAN_AUDIT_LAN_AI_JOB_REJECTED: &str = "lan-ai-job-rejected";
pub const LAN_AUDIT_LAN_AI_JOB_COMPLETED: &str = "lan-ai-job-completed";
pub const LAN_AUDIT_LAN_AI_JOB_DEGRADED: &str = "lan-ai-job-degraded";
pub const LAN_AUDIT_PAIRING_CHALLENGE_ISSUED: &str = "pairing-challenge-issued";
pub const LAN_AUDIT_PAIRING_PROOF_ACCEPTED: &str = "pairing-proof-accepted";
pub const LAN_AUDIT_PAIRING_PROOF_REJECTED: &str = "pairing-proof-rejected";
pub const LAN_AUDIT_PAIRING_REVOKED: &str = "pairing-revoked";
pub const LAN_AUDIT_ROUTE_SELECTED: &str = "route-selected";
pub const LAN_AUTH_UNAUTHENTICATED: &str = "unauthenticated";
pub const LAN_AUTH_UNPAIRED: &str = "unpaired";
pub const LAN_AUTH_PAIRED: &str = "paired";
pub const LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER: &str = "active-controller";
pub const LAN_PARENT_AUTHORITY_OBSERVER: &str = "observer";
pub const LAN_INTENT_APPROVAL_DECISION: &str = "approval-decision";
pub const LAN_INTENT_CONFIGURATION_UPDATE: &str = "configuration-update";
pub const LAN_INTENT_CONTROLLER_LEASE_RENEW: &str = "controller-lease-renew";
pub const LAN_INTENT_CONTROLLER_LEASE_RELEASE: &str = "controller-lease-release";
pub const LAN_INTENT_CONTROLLER_LEASE_TAKEOVER: &str = "controller-lease-takeover";
pub const LAN_INTENT_HEALTH_QUERY: &str = "health-query";
pub const LAN_INTENT_LAN_AI_PROVIDER_STATUS: &str = "lan-ai-provider-status";
pub const LAN_INTENT_LAN_AI_JOB_SUBMIT: &str = "lan-ai-job-submit";
pub const LAN_INTENT_RULE_QUERY: &str = "rule-query";
pub const LAN_INTENT_RULE_UPDATE: &str = "rule-update";
pub const LAN_REACHABILITY_OFFLINE: &str = "offline";
pub const LAN_REACHABILITY_ONLINE: &str = "online";
pub const LAN_REACHABILITY_STALE: &str = "stale";
pub const LAN_PAIRING_MALFORMED: &str = "malformed";
pub const LAN_SERVICE_IDENTITY_PROBE_HTTP_STATUS: &str = "http-status";
pub const LAN_SERVICE_IDENTITY_PROBE_HTML_TITLE: &str = "html-title";
pub const LAN_SERVICE_IDENTITY_PROBE_SERVER_HEADER: &str = "server-header";
pub const LAN_SERVICE_IDENTITY_PROBE_BANNER: &str = "banner";
pub const LAN_SERVICE_IDENTITY_PROBE_REDIRECT_LOCATION: &str = "redirect-location";
pub const LAN_SERVICE_IDENTITY_PROBE_CERTIFICATE_SUBJECT: &str = "certificate-subject";
pub const LAN_SERVICE_IDENTITY_PROBE_DESCRIPTOR_LINK: &str = "descriptor-link";
pub const LAN_PAIRING_UNPAIRED: &str = "unpaired";
pub const LAN_PAIRING_PAIRING: &str = "pairing";
pub const LAN_PAIRING_PAIRED: &str = "paired";
pub const LAN_PAIRING_REVOKED: &str = "revoked";
pub const LAN_PAIRING_EXPIRED: &str = "expired";
pub const LAN_PERSISTENCE_IN_MEMORY_FAIL_CLOSED: &str = "in-memory-fail-closed";
pub const LAN_PERSISTENCE_LOCAL_JSON_REGISTRY: &str = "local-json-registry";
pub const LAN_RESTART_FAIL_CLOSED_UNPAIRED: &str = "fail-closed-unpaired";
pub const LAN_RESTART_RESTORE_TRUSTED_REGISTRY_UNSELECTED: &str =
    "restore-trusted-registry-unselected";
pub const LAN_RESTART_RESTORE_TRUSTED_REGISTRY_SELECTED_ROUTE: &str =
    "restore-trusted-registry-selected-route";
pub const LAN_PROOF_DIRECT_PROOF_SUBMIT: &str = "direct-proof-submit";
pub const LAN_REASON_ANONYMOUS: &str = "anonymous";
pub const LAN_REASON_CONTROLLER_LEASE_EXPIRED: &str = "controller-lease-expired";
pub const LAN_REASON_CONTROLLER_LEASE_MISSING: &str = "controller-lease-missing";
pub const LAN_REASON_EXPIRED: &str = "expired";
pub const LAN_REASON_MALFORMED: &str = "malformed";
pub const LAN_REASON_OFFLINE: &str = "offline";
pub const LAN_REASON_OBSERVER_READ_ONLY: &str = "observer-read-only";
pub const LAN_REASON_PAYLOAD_TOO_LARGE: &str = "payload-too-large";
pub const LAN_REASON_REPLAYED: &str = "replayed";
pub const LAN_REASON_REVOKED: &str = "revoked";
pub const LAN_REASON_SIGNED_CHILD_AGENT_CONTEXT_UNAVAILABLE: &str =
    "signed-child-agent-context-unavailable";
pub const LAN_REASON_STALE: &str = "stale";
pub const LAN_REASON_TAKEOVER_DENIED: &str = "takeover-denied";
pub const LAN_REASON_UNSELECTED_DEVICE: &str = "unselected-device";
pub const LAN_REASON_UNSUPPORTED_ROUTE: &str = "unsupported-route";
pub const LAN_REASON_WRONG_DEVICE: &str = "wrong-device";
pub const LAN_REASON_WRONG_ORIGIN: &str = "wrong-origin";
pub const LAN_REASON_WRONG_CONTROLLER: &str = "wrong-controller";
pub const LAN_REASON_LAN_AI_PROVIDER_UNAVAILABLE: &str = "lan-ai-provider-unavailable";
pub const LAN_REASON_LAN_AI_JOB_UNAUTHORIZED: &str = "lan-ai-job-unauthorized";
pub const DEVICE_ROLE_PARENT_CONTROLLER: &str = "parent-controller";
pub const DEVICE_ROLE_PARENT_OBSERVER: &str = "parent-observer";
pub const DEVICE_ROLE_CHILD_AGENT: &str = "child-agent";
pub const DEVICE_ROLE_AI_PROVIDER: &str = "ai-provider";
pub const DEVICE_ROLE_STATE_IMPLEMENTED: &str = "implemented";
pub const DEVICE_ROLE_STATE_SCAFFOLD: &str = "scaffold";
pub const DEVICE_RUNTIME_SURFACE_PARENT_DESKTOP: &str = "parent-desktop";
pub const DEVICE_RUNTIME_SURFACE_PARENT_MOBILE: &str = "parent-mobile";
pub const DEVICE_RUNTIME_SURFACE_CHILD_DESKTOP: &str = "child-desktop";
pub const DEVICE_RUNTIME_SURFACE_CHILD_ANDROID: &str = "child-android";
pub const DEVICE_RUNTIME_SURFACE_CHILD_IOS: &str = "child-ios";
pub const DEVICE_RUNTIME_PLATFORM_IOS: &str = "ios";
pub const DEVICE_RUNTIME_ROUTE_LOCALHOST: &str = "localhost";
pub const DEVICE_RUNTIME_ROUTE_LOCAL_NETWORK: &str = "local-network";
pub const DEVICE_RUNTIME_ROUTE_CLOUD_RELAY: &str = "cloud-relay";
pub const DEVICE_RUNTIME_ROUTE_MANUAL_REQUIRED: &str = "manual-required";
pub const DEVICE_RUNTIME_AI_PROVIDER_AVAILABLE: &str = "available";
pub const DEVICE_RUNTIME_AI_PROVIDER_DEGRADED: &str = "degraded";
pub const DEVICE_RUNTIME_AI_PROVIDER_UNAVAILABLE: &str = "unavailable";
pub const DEVICE_RUNTIME_LOCAL_AI_CLAIM_NONE: &str = "none";
pub const DEVICE_RUNTIME_LOCAL_AI_CLAIM_SHARED_SINGLETON: &str = "shared-physical-device-singleton";
pub const PARENT_DESKTOP_BACKEND_RUST_SERVICE: &str = "rust-agent-service";
pub const PARENT_DESKTOP_SERVICE_CONNECTED: &str = "connected";
pub const PARENT_DESKTOP_SERVICE_UNAVAILABLE: &str = "unavailable";
pub const PARENT_DESKTOP_RUNTIME_READY: &str = "runtime-ready";
pub const PARENT_DESKTOP_RUNTIME_DEGRADED: &str = "runtime-degraded";
pub const PARENT_DESKTOP_FRONTEND_BUILT_PORTAL_DIST: &str = "built-portal-dist";
pub const PARENT_DESKTOP_HMR_BACKEND_NOT_USED: &str = "hmr-backend-not-used";
pub const PARENT_DESKTOP_PROCESS_OWNER_SHELL_ONLY: &str = "parent-desktop-shell-only";
pub const PARENT_DESKTOP_CONTROLLER_LEASE_MANUAL_REQUIRED: &str =
    "controller-lease-manual-required";
pub const PARENT_DESKTOP_CONTROLLER_ROUTE_ACTIVE_CONTROLLER: &str = "active-controller-route";
pub const PARENT_DESKTOP_CONTROLLER_ROUTE_MANUAL_REQUIRED: &str =
    "controller-route-manual-required";
pub const PARENT_DESKTOP_OBSERVER_READ_ONLY: &str = "observer-read-only";
pub const PARENT_DESKTOP_AUTHENTICATION_MANUAL_REQUIRED: &str = "authentication-manual-required";
pub const PARENT_DESKTOP_SOURCE_CUSTODY_LIVE_LOCAL_NETWORK: &str = "live-local-network-custody";
pub const PARENT_DESKTOP_SOURCE_CUSTODY_MANUAL_REQUIRED: &str = "source-custody-manual-required";
pub const PARENT_DESKTOP_RELAY_ROUTE_UNAVAILABLE: &str = "relay-route-unavailable";
pub const PARENT_DESKTOP_PARENT_CACHE_UNAVAILABLE: &str = "parent-cache-unavailable";
pub const PARENT_DESKTOP_PARENT_STORAGE_UNAVAILABLE: &str = "parent-owned-storage-unavailable";
pub const PARENT_DESKTOP_SERVICE_LAUNCH_OWNER_PACKAGE_SERVICE: &str = "package-service-manager";
pub const PARENT_DESKTOP_SERVICE_LAUNCH_STRATEGY_CONNECT_OR_DEGRADE: &str =
    "connect-existing-service-or-degrade";
pub const PARENT_DESKTOP_PACKAGE_SERVICE_AUTO_START: &str = "package-installs-auto-start-service";
pub const PARENT_DESKTOP_PACKAGE_HEALTH_PROBE_REQUIRED: &str = "package-health-probe-required";
pub const PARENT_DESKTOP_PORT_CONFLICT_POLICY_NO_FOREIGN_RECLAIM: &str = "no-foreign-port-reclaim";
pub const PARENT_DESKTOP_PORT_OWNERSHIP_FIXED_LOOPBACK: &str = "fixed-agent-4477-portal-4478";
pub const PARENT_DESKTOP_BLANK_WINDOW_GUARD_FRONTEND_DIST: &str = "frontend-dist-configured";
pub const PARENT_DESKTOP_PACKAGE_PREVIEW_UNSIGNED: &str = "unsigned-package-preview";
pub const PARENT_DESKTOP_UPDATE_CHANNEL_SCAFFOLD: &str = "update-channel-scaffold";
pub const PARENT_DESKTOP_ROLLBACK_UNAVAILABLE: &str = "rollback-unavailable";
pub const PARENT_DESKTOP_SIGNING_MANUAL_REQUIRED: &str = "signing-manual-required";
pub const PARENT_DESKTOP_NOTARIZATION_MANUAL_REQUIRED: &str = "notarization-manual-required";
pub const PARENT_DESKTOP_STORE_DISTRIBUTION_MANUAL_REQUIRED: &str =
    "store-distribution-manual-required";
pub const PARENT_DESKTOP_SUPPORT_DIAGNOSTICS_REDACTED: &str = "support-diagnostics-redacted";
pub const PARENT_DESKTOP_SUPPORT_OUTPUT_ALLOWED_FIELDS: &str =
    "support-fields-version-commit-platform-package-service-route";
pub const PARENT_DESKTOP_PLATFORM_MATRIX_SPLIT_PROOF_ROWS: &str =
    "platform-matrix-split-proof-rows";
pub const PARENT_DESKTOP_RELEASE_BRANCH_PRODUCTION_PROMOTION_REQUIRED: &str =
    "production-promotion-required";
pub const PARENT_DESKTOP_ARTIFACT_PROOF_CI_PREVIEW: &str = "ci-package-preview-artifact-proof";
pub const LAN_AI_PROVIDER_STATUS_UNAVAILABLE: &str = "lan-ai-provider-unavailable";
pub const LAN_AI_PROVIDER_STATUS_AVAILABLE: &str = "lan-ai-provider-available";
pub const LAN_AI_PROVIDER_STATUS_DEGRADED: &str = "lan-ai-provider-degraded";
pub const LAN_AI_PROVIDER_STATUS_BUSY: &str = "lan-ai-provider-busy";
pub const LAN_AI_PROVIDER_ROUTING_AUTHORIZED_RESULT: &str = "authorized-result";
pub const LAN_AI_PROVIDER_ROUTING_BUSY: &str = "busy";
pub const LAN_AI_PROVIDER_ROUTING_DEGRADED: &str = "degraded";
pub const LAN_AI_PROVIDER_ROUTING_UNAVAILABLE: &str = "unavailable";
pub const LAN_AI_PROVIDER_ROUTING_UNSUPPORTED_CAPABILITY: &str = "unsupported-capability";
pub const LAN_AI_PROVIDER_RESULT_REDACTED: &str = "lan-ai-provider-result-redacted";
pub const LAN_AI_JOB_STATE_ACCEPTED: &str = "accepted";
pub const LAN_AI_JOB_STATE_REJECTED: &str = "rejected";
pub const LAN_AI_JOB_STATE_COMPLETED: &str = "completed";
pub const LAN_AI_JOB_STATE_DEGRADED: &str = "degraded";
pub const LAN_SIGNED_CHILD_AGENT_VERIFICATION_ACCEPTED: &str = "accepted";
pub const LAN_AI_LEASE_STATE_CLAIMED: &str = "claimed";
pub const LAN_AI_LEASE_STATE_COMPLETED: &str = "completed";
pub const LAN_AI_LEASE_STATE_DUPLICATE_REJECTED: &str = "duplicate-rejected";
pub const LAN_AI_LEASE_STATE_EXPIRED_REQUEUED: &str = "expired-requeued";
pub const LAN_AI_LEASE_STATE_DEAD_LETTERED: &str = "dead-lettered";
pub const LAN_AI_DEAD_LETTER_REASON_MAX_ATTEMPTS: &str = "max-attempts";
pub const LAN_DISCOVERY_STATE_DISCOVERED: &str = "discovered";
pub const LAN_DISCOVERY_STATE_PENDING: &str = "pending";
pub const LAN_DISCOVERY_STATE_PAIRED: &str = "paired";
pub const LAN_DISCOVERY_STATE_REJECTED: &str = "rejected";
pub const LAN_DISCOVERY_STATE_EXPIRED: &str = "expired";
pub const LAN_DISCOVERY_STATE_REVOKED: &str = "revoked";
pub const LAN_DISCOVERY_STATE_STALE: &str = "stale";
pub const LAN_DISCOVERY_STATE_OFFLINE: &str = "offline";
pub const LAN_DISCOVERY_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const LAN_DISCOVERY_STATE_UNAVAILABLE: &str = "unavailable";
pub const LAN_DISCOVERY_SOURCE_LOCAL_SERVICE: &str = "local-service";
pub const LAN_NON_CLAIM_PHYSICAL_HOUSEHOLD_MANUAL_REQUIRED: &str =
    "physical-household-lan-manual-required";
pub const LAN_NON_CLAIM_CLOUD_RELAY_NOT_IMPLEMENTED: &str = "cloud-relay-not-implemented";
pub const LAN_NON_CLAIM_REMOTE_DESKTOP_NOT_IMPLEMENTED: &str = "remote-desktop-not-implemented";
pub const LAN_READ_MODEL_JSON_EXPECTATION: &str = "read model payload is JSON";
pub const LAN_HONEST_NON_CLAIMS_ARRAY_EXPECTATION: &str = "honest non claims are an array";
pub const LAN_PROVIDER_CUSTODY_LOCAL_NETWORK_AI_PROVIDER: &str = "local-network-ai-provider";
pub const APP_GAME_CHILD_UX_PARENT_SURFACE_HISTORY_ROW_VISIBLE: &str = "history-row-visible";
pub const APP_GAME_CHILD_UX_PARENT_SURFACE_MANUAL_ACTION_REQUIRED: &str = "manual-action-required";
pub const APP_GAME_CHILD_UX_PARENT_SURFACE_PREFERENCE_SETUP_REQUIRED: &str =
    "preference-setup-required";
pub const APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_DRAFT_READY: &str = "draft-ready";
pub const APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_REQUEST_READY: &str = "request-ready";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_REQUEST_SCHEMA_VERSION: &str =
    "app-game-timer-parent-preference-setup-request-proof";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_REQUEST_ACCEPTED: &str = "accepted";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_PERSISTED: &str = "persisted";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_UNAVAILABLE: &str = "unavailable";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_PERSISTED: &str = "persisted";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_UNAVAILABLE: &str = "unavailable";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_SUFFIX: &str = "mutation-receipt";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_READY: &str =
    "handoff-ready";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_UNAVAILABLE: &str =
    "unavailable";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_SUFFIX: &str =
    "child-runtime-delivery-handoff";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_QUEUED: &str = "queued";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_UNAVAILABLE: &str =
    "unavailable";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_SUFFIX: &str =
    "child-runtime-delivery-queue";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_READY: &str =
    "dispatch-ready";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_UNAVAILABLE: &str =
    "unavailable";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_SUFFIX: &str =
    "child-runtime-delivery-dispatch";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIRED: &str =
    "receipt-required";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_UNAVAILABLE: &str =
    "unavailable";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIREMENT_SUFFIX: &str =
    "child-runtime-delivery-receipt-required";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_PENDING: &str =
    "receipt-pending";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_PENDING_SUFFIX: &str =
    "child-runtime-delivery-receipt-pending";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_INGESTED: &str =
    "receipt-ingested";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_INGESTED_SUFFIX: &str =
    "child-runtime-delivery-receipt-ingested";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_RECORDED: &str = "outbox-recorded";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_SUFFIX: &str = "durable-local-outbox";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_FILE_EXTENSION: &str =
    "setup-outbox.jsonl";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_MANUAL_REQUIRED: &str =
    "provider-manual-required";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_READINESS_SUFFIX: &str =
    "provider-delivery-readiness";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ATTEMPT_MANUAL_REQUIRED: &str =
    "provider-delivery-manual-required";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ATTEMPT_SUFFIX: &str =
    "provider-delivery-attempt";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ADAPTER_REQUIRED: &str =
    "provider-adapter-required";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_ADAPTER_REQUIREMENT_SUFFIX: &str =
    "provider-adapter-required";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_CREDENTIAL_PROOF_REQUIRED: &str =
    "provider-credential-proof-required";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_CREDENTIAL_REQUIREMENT_SUFFIX: &str =
    "provider-credential-proof-required";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_QUEUE_QUEUED: &str =
    "provider-delivery-queued";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_QUEUE_SUFFIX: &str =
    "provider-delivery-local-queue";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_REQUIRED: &str =
    "provider-delivery-receipt-required";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_REQUIREMENT_SUFFIX: &str =
    "provider-delivery-receipt-required";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_PENDING: &str =
    "provider-delivery-receipt-pending";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_PENDING_SUFFIX: &str =
    "provider-delivery-receipt-pending";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_INGESTED: &str =
    "provider-delivery-receipt-ingested";
pub const APP_GAME_PARENT_PREFERENCE_SETUP_PROVIDER_DELIVERY_RECEIPT_INGESTED_SUFFIX: &str =
    "provider-delivery-receipt-ingested";
pub const APP_GAME_TEST_CHILD_RUNTIME_TRANSPORT_RECEIPT_GENERATED_AT: &str =
    "2026-06-08T23:25:00.000Z";
pub const APP_GAME_TEST_CHILD_RUNTIME_TRANSPORT_RECEIPT_INVENTORY_ENTRY_ID: &str =
    "inventory-entry-app-game-child-runtime";
pub const APP_GAME_TEST_CHILD_RUNTIME_TRANSPORT_RECEIPT_PERMISSION_LIMITED_RUNTIME_ID: &str =
    "runtime-evidence-app-game-child-runtime-permission-limited";
pub const APP_GAME_TEST_CHILD_RUNTIME_TRANSPORT_RECEIPT_PERMISSION_LIMITED_FOREGROUND_ID: &str =
    "foreground-evidence-app-game-child-runtime-permission-limited";
pub const APP_GAME_TEST_CHILD_RUNTIME_TRANSPORT_RECEIPT_UNAVAILABLE_INVENTORY_ENTRY_ID: &str =
    "inventory-entry-app-game-child-runtime-unavailable";
pub const APP_GAME_TEST_CHILD_RUNTIME_TRANSPORT_RECEIPT_TEMP_SUFFIX: &str =
    "child-runtime-transport-receipt";
pub const APP_GAME_TEST_LOCAL_OUTBOX_RECORD_JSON_LINE: &str =
    "{\"recordId\":\"app-game-test-local-outbox-record\"}\n";
pub const APP_GAME_NOTIFICATION_READINESS_LOCAL_OUTBOX: &str =
    "notification-readiness-local-outbox";
pub const APP_GAME_TEST_PLATFORM_PROOF_STATUS_GENERATED_AT: &str = "2026-06-08T16:50:00.000Z";
pub const APP_GAME_TEST_PLATFORM_PROOF_STATUS_REPARSES: &str =
    "platform proof status read model reparses";
pub const APP_GAME_TEST_PLATFORM_PROOF_STATUS_ROW_EXISTS: &str = "platform row exists";
pub const APP_GAME_TEST_PLATFORM_PROOF_STATUS_MISSING_PROOF_REF: &str = "missing proof ref ";
pub const APP_GAME_TEST_PLATFORM_PROOF_STATUS_MISSING_OPEN_GAP: &str = "missing open gap ";
pub const APP_GAME_TEST_POLICY_READINESS_CATEGORY_EVIDENCE_ID: &str =
    "evidence-category-native-game";
pub const APP_GAME_TEST_POLICY_READINESS_CATEGORY_INVENTORY_ENTRY_ID: &str =
    "inventory-native-game-category";
pub const APP_GAME_TEST_POLICY_READINESS_CATEGORY_SOURCE_REF: &str = "source-category-native-game";
pub const APP_GAME_TEST_POLICY_READINESS_UNKNOWN_INVENTORY_ENTRY_ID: &str =
    "inventory-unknown-executable-review";
pub const APP_GAME_TEST_POLICY_READINESS_UNKNOWN_DISPLAY_LABEL: &str = "Unknown executable";
pub const APP_GAME_TEST_POLICY_READINESS_UNKNOWN_EVIDENCE_ID: &str =
    "evidence-unknown-executable-review";
pub const APP_GAME_CHILD_UX_PARENT_PREFERENCE_SETUP_UNAVAILABLE_VISIBLE: &str =
    "unavailable-visible";
pub const TRANSPORT_WEBSOCKET: &str = "websocket";
pub const TRUE: &str = "true";
pub const UNKNOWN_HOST: &str = "unknown-host";
pub const WATCHER_STATUS_ONLY: &str =
    "Watcher status endpoint is available; watcher runtime is not active.";
