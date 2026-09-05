pub const EVENT_SCHEMA_VERSION: u16 = 1;
pub const EVENT_SCREEN_CAPTURE_OBSERVED: &str = "screen.capture.observed";
pub const EVENT_SCREEN_QUEUE_ENCRYPTED: &str = "screen.queue.encrypted";
pub const EVENT_SCREEN_AI_ANALYSIS_REQUESTED: &str =
    super::child_domain_runtime::SCREEN_AI_ANALYSIS_REQUESTED_EVENT_TYPE;
pub const EVENT_SCREEN_AI_ANALYSIS_COMPLETED: &str = "screen.ai.analysis.completed";
pub const EVENT_SCREEN_SUMMARY_COMMITTED: &str = "screen.summary.committed";
pub const EVENT_SCREEN_POLICY_DECISION_COMPLETED: &str = "screen.policy.decision.completed";
pub const EVENT_SCREEN_ACTION_DRY_RUN_RECORDED: &str = "screen.action.dry-run.recorded";
pub const EVENT_SCREEN_DELETION_COMMITTED: &str = "screen.deletion.committed";
pub const EVENT_SCREEN_PORTAL_READ_MODEL_UPDATED: &str = "screen.portal-read-model.updated";
pub const EVENT_SCREEN_SERVICE_ROW_READY: &str = "screen.service.row.ready";
pub const EVENT_SCREEN_MESH_WORK_QUEUED: &str = "screen.mesh.work.queued";
pub const EVENT_SCREEN_MESH_OFFER_PUBLISHED: &str = "screen.mesh.offer.published";
pub const EVENT_SCREEN_MESH_CLAIM_REQUESTED: &str = "screen.mesh.claim.requested";
pub const EVENT_SCREEN_MESH_CLAIM_GRANTED: &str = "screen.mesh.claim.granted";
pub const EVENT_SCREEN_MESH_LEASE_CREATED: &str = "screen.mesh.lease.created";
pub const EVENT_SCREEN_MESH_PROVIDER_RESULT_RETURNED: &str = "screen.mesh.provider-result.returned";
pub const EVENT_SCREEN_MESH_CHILD_RESULT_ACCEPTED: &str = "screen.mesh.child-result.accepted";
pub const EVENT_SCREEN_MESH_POLICY_REQUESTED: &str = "screen.mesh.policy.requested";

pub const TARGET_SCREEN_CAPTURE_OBSERVER: &str = "screen-capture-observer";
pub const TARGET_SCREEN_QUEUE_WRITER: &str = "screen-queue-writer";
pub const TARGET_SCREEN_AI_ANALYZER: &str = "screen-ai-analyzer";
pub const TARGET_SCREEN_SUMMARY_WRITER: &str = "screen-summary-writer";
pub const TARGET_SCREEN_POLICY_ENGINE: &str = "screen-policy-engine";
pub const TARGET_SCREEN_ACTION_DRY_RUN: &str = "screen-action-dry-run";
pub const TARGET_SCREEN_DELETION_WORKER: &str = "screen-deletion-worker";
pub const TARGET_SCREEN_PORTAL_READ_MODEL: &str = "screen-portal-read-model";
pub const TARGET_SCREEN_SERVICE_EVENT_SUBSCRIBER: &str = "screen-service-event-subscriber";
pub const TARGET_SCREEN_MESH_BRIDGE: &str = "screen-household-mesh-bridge";
pub const TARGET_SCREEN_MESH_CHILD_LEDGER: &str = "screen-household-child-ledger";
pub const TARGET_SCREEN_MESH_PROVIDER_WORKER: &str = "screen-household-provider-worker";
pub const TARGET_SCREEN_MESH_CHILD_VALIDATOR: &str = "screen-household-child-validator";

pub const SUBSCRIBER_SCREEN_CAPTURE_OBSERVER: &str = "screen-runtime-capture-observer-subscriber";
pub const SUBSCRIBER_SCREEN_QUEUE_WRITER: &str = "screen-runtime-queue-writer-subscriber";
pub const SUBSCRIBER_SCREEN_AI_REQUEST: &str = "screen-runtime-ai-request-subscriber";
pub const SUBSCRIBER_SCREEN_AI_COMPLETE: &str = "screen-runtime-ai-complete-subscriber";
pub const SUBSCRIBER_SCREEN_SUMMARY_WRITER: &str = "screen-runtime-summary-writer-subscriber";
pub const SUBSCRIBER_SCREEN_POLICY_DECISION: &str = "screen-runtime-policy-decision-subscriber";
pub const SUBSCRIBER_SCREEN_ACTION_DRY_RUN: &str = "screen-runtime-action-dry-run-subscriber";
pub const SUBSCRIBER_SCREEN_DELETION_WORKER: &str = "screen-runtime-deletion-worker-subscriber";
pub const SUBSCRIBER_SCREEN_PORTAL_READ_MODEL: &str = "screen-runtime-portal-read-model-subscriber";
pub const SUBSCRIBER_SCREEN_SERVICE_ROW_READY: &str = "screen-service-row-ready-subscriber";
pub const SUBSCRIBER_SCREEN_MESH_WORK_QUEUE: &str = "screen-mesh-work-queue-subscriber";
pub const SUBSCRIBER_SCREEN_MESH_OFFER: &str = "screen-mesh-offer-subscriber";
pub const SUBSCRIBER_SCREEN_MESH_CLAIM_REQUEST: &str = "screen-mesh-claim-request-subscriber";
pub const SUBSCRIBER_SCREEN_MESH_CLAIM_GRANT: &str = "screen-mesh-claim-grant-subscriber";
pub const SUBSCRIBER_SCREEN_MESH_LEASE: &str = "screen-mesh-lease-subscriber";
pub const SUBSCRIBER_SCREEN_MESH_PROVIDER_RESULT: &str = "screen-mesh-provider-result-subscriber";
pub const SUBSCRIBER_SCREEN_MESH_CHILD_VALIDATION: &str = "screen-mesh-child-validation-subscriber";
pub const SUBSCRIBER_SCREEN_MESH_POLICY_REQUEST: &str = "screen-mesh-policy-request-subscriber";

pub const RUNTIME_COMPONENT_SCREEN_SPINE: &str = "screen-runtime-spine";
pub const RUNTIME_COMPONENT_SCREEN_SERVICE_SUBSCRIBER: &str = "screen-service-event-subscriber";
pub const RUNTIME_INSTANCE_LOCAL_CHILD_AGENT: &str = "local-child-agent";
pub const AGGREGATE_SCREEN_QUEUE_PREFIX: &str = "screen-queue-";
pub const CORRELATION_SCREEN_RUNTIME_PREFIX: &str = "screen-runtime-correlation-";
pub const IDEMPOTENCY_SCREEN_RUNTIME_PREFIX: &str = "screen-runtime-idempotency-";
pub const IDEMPOTENCY_SCREEN_SERVICE_ROW_READY_PREFIX: &str = "screen-service-row-ready-";
pub const SCREEN_CAPTURE_EVENT_REF: &str = "event.screen.capture.observed.1";
pub const SCREEN_QUEUE_EVENT_REF: &str = "event.screen.queue.encrypted.1";
pub const SCREEN_AI_REQUEST_EVENT_REF: &str = "event.screen.ai.analysis.requested.1";
pub const SCREEN_AI_RESULT_EVENT_REF: &str = "event.screen.ai.analysis.completed.1";
pub const SCREEN_SUMMARY_EVENT_REF: &str = "event.screen.summary.committed.1";
pub const SCREEN_POLICY_EVENT_REF: &str = "event.screen.policy.decision.completed.1";
pub const SCREEN_ACTION_EVENT_REF: &str = "event.screen.action.dry-run.recorded.1";
pub const SCREEN_DELETION_EVENT_REF: &str = "event.screen.deletion.committed.1";
pub const TEST_SCREEN_PORTAL_READ_MODEL_REF: &str = "activity-screen-read-model-row-1";
pub const TEST_SCREEN_ACTION_REF: &str = "screen-action-dry-run-1";
pub const TEST_SCREEN_POLICY_RULE_REF: &str = "policy-rule.screen.school.1";
pub const SCREEN_LIVE_VIEW_RUNTIME_ENABLED_ENV: &str =
    "OCENTRA_PARENT_SCREEN_LIVE_VIEW_RUNTIME_ENABLED";
pub const SCREEN_LIVE_VIEW_MODE_ENV: &str = "OCENTRA_PARENT_SCREEN_LIVE_VIEW_MODE";
pub const SCREEN_LIVE_VIEW_TRANSPORT_ENV: &str = "OCENTRA_PARENT_SCREEN_LIVE_VIEW_TRANSPORT";
pub const SCREEN_LIVE_VIEW_PERMISSION_ENV: &str = "OCENTRA_PARENT_SCREEN_LIVE_VIEW_PERMISSION";
pub const SCREEN_LIVE_VIEW_TRANSPORT_PROOF_ENV: &str =
    "OCENTRA_PARENT_SCREEN_LIVE_VIEW_TRANSPORT_PROOF";
pub const SCREEN_LIVE_VIEW_DELETION_PROOF_ENV: &str =
    "OCENTRA_PARENT_SCREEN_LIVE_VIEW_DELETION_PROOF";
pub const SCREEN_LIVE_VIEW_PARENT_UI_PERSISTENCE_ENV: &str =
    "OCENTRA_PARENT_SCREEN_LIVE_VIEW_PARENT_UI_PERSISTENCE";
pub const SCREEN_LIVE_VIEW_RELAY_CACHE_PROOF_ENV: &str =
    "OCENTRA_PARENT_SCREEN_LIVE_VIEW_RELAY_CACHE_PROOF";
pub const SCREEN_LIVE_VIEW_PLATFORM_PROMPT_ARTIFACT_ENV: &str =
    "OCENTRA_PARENT_SCREEN_LIVE_VIEW_PLATFORM_PROMPT_ARTIFACT";
pub const SCREEN_LIVE_VIEW_RELAY_CACHE_EXECUTION_ENV: &str =
    "OCENTRA_PARENT_SCREEN_LIVE_VIEW_RELAY_CACHE_EXECUTION";
pub const SCREEN_LIVE_VIEW_PHYSICAL_DEVICE_PARITY_ENV: &str =
    "OCENTRA_PARENT_SCREEN_LIVE_VIEW_PHYSICAL_DEVICE_PARITY";
pub const SCREEN_LIVE_VIEW_PRIVACY_LEGAL_APPROVAL_ENV: &str =
    "OCENTRA_PARENT_SCREEN_LIVE_VIEW_PRIVACY_LEGAL_APPROVAL";
pub const SCREEN_LIVE_VIEW_CACHE_RAW_FRAMES_ENV: &str =
    "OCENTRA_PARENT_SCREEN_LIVE_VIEW_CACHE_RAW_FRAMES";
pub const SCREEN_LIVE_VIEW_SESSION_RECORDING_ENV: &str =
    "OCENTRA_PARENT_SCREEN_LIVE_VIEW_SESSION_RECORDING";
pub const SCREEN_LIVE_VIEW_REMOTE_INPUT_ENV: &str = "OCENTRA_PARENT_SCREEN_LIVE_VIEW_REMOTE_INPUT";
pub const SCREEN_LIVE_VIEW_MODE_LAN_ONLY: &str = "lan-only";
pub const SCREEN_LIVE_VIEW_MODE_RELAY_BACKED: &str = "relay-backed";
pub const SCREEN_LIVE_VIEW_TRANSPORT_LAN_MUTUAL_AUTH: &str = "lan-mutual-auth";
pub const SCREEN_LIVE_VIEW_TRANSPORT_RELAY_E2EE: &str = "relay-e2ee";
pub const SCREEN_LIVE_VIEW_PERMISSION_CAPTURE_ONLY: &str = "screen-capture-only";
pub const SCREEN_LIVE_VIEW_PERMISSION_LIVE_VIEW: &str = "live-view";
pub const ENV_TRUE: &str = "1";
pub const ERROR_SCREEN_RUNTIME_CHAIN_PUBLISHES: &str = "screen runtime chain publishes";
pub const ERROR_SCREEN_RUNTIME_PAYLOAD_DECODES: &str = "screen runtime payload decodes";
pub const ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES: &str =
    "screen service event bridge publishes";
pub const ERROR_SCREEN_SERVICE_EVENT_BRIDGE_REJECTS: &str = "screen service event bridge rejects";
pub const ERROR_SCREEN_SERVICE_EVENT_BRIDGE_MAPS: &str = "screen service event bridge maps";
pub const ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBES: &str = "screen service event subscriber registers";
pub const ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBER_PUBLISHES: &str =
    "screen service event subscriber publishes";
pub const ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBER_REJECTS: &str =
    "screen service event subscriber rejects";
pub const ERROR_SCREEN_RUNTIME_OWNER_UNAVAILABLE_MANUAL_REQUIRED: &str =
    "screen runtime owner unavailable; manual-required";
pub const ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBER_RECORDS: &str =
    "screen service event subscriber records";
pub const ERROR_SCREEN_LIVE_VIEW_ENV_LOCKS: &str = "screen live view env locks";
pub const FIELD_SCREEN_SERVICE_ROW_READY: &str = "screenServiceRowReady";
pub const IDEMPOTENCY_SCREEN_MESH_PREFIX: &str = "screen-mesh-idempotency-";
pub const SCREEN_MESH_WORK_EVENT_REF: &str = "event.screen.mesh.work.queued.1";
pub const SCREEN_MESH_OFFER_EVENT_REF: &str = "event.screen.mesh.offer.published.1";
pub const SCREEN_MESH_CLAIM_REQUEST_EVENT_REF: &str = "event.screen.mesh.claim.requested.1";
pub const SCREEN_MESH_CLAIM_GRANT_EVENT_REF: &str = "event.screen.mesh.claim.granted.1";
pub const SCREEN_MESH_LEASE_EVENT_REF: &str = "event.screen.mesh.lease.created.1";
pub const SCREEN_MESH_PROVIDER_RESULT_EVENT_REF: &str =
    "event.screen.mesh.provider-result.returned.1";
pub const SCREEN_MESH_CHILD_ACCEPTED_EVENT_REF: &str = "event.screen.mesh.child-result.accepted.1";
pub const TEST_SCREEN_MESH_PROVIDER_PEER_ID: &str = "trusted-parent-desktop-ai-provider-1";
pub const TEST_SCREEN_MESH_CLAIM_ID: &str = "screen-mesh-claim-1";
pub const TEST_SCREEN_MESH_LEASE_ID: &str = "screen-mesh-lease-1";
pub const TEST_SCREEN_MESH_RESULT_REF: &str = "screen-mesh-provider-result-1";
pub const TEST_SCREEN_MESH_PAYLOAD_REF: &str = "screen-mesh-redacted-summary-payload-1";
pub const TEST_SCREEN_MESH_WRONG_SUFFIX: &str = "-wrong";
pub const ERROR_SCREEN_MESH_CHAIN_PUBLISHES: &str = "screen mesh runtime chain publishes";
pub const ERROR_SCREEN_MESH_PAYLOAD_DECODES: &str = "screen mesh payload decodes";
