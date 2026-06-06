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

pub const TEST_DEVICE_REF: &str = "device.child.windows-1";
pub const TEST_FLOW_EVENT_REF: &str = "event.network.flow.observed.1";
pub const TEST_DOMAIN_EVENT_REF: &str = "event.network.domain.observed.1";
pub const TEST_CLASSIFICATION_EVENT_REF: &str = "event.network.activity.classified.1";
pub const TEST_AI_REQUEST_REF: &str = "event.ai.analysis.requested.1";
pub const TEST_AI_ANALYSIS_REF: &str = "event.ai.analysis.completed.1";
pub const TEST_POLICY_EVALUATION_REF: &str = "event.policy.evaluation.requested.1";
pub const TEST_POLICY_DECISION_REF: &str = "event.policy.decision.completed.1";
pub const TEST_ENFORCEMENT_COMMAND_REF: &str = "event.enforcement.command.issued.1";
pub const TEST_ENFORCEMENT_RESULT_REF: &str = "event.enforcement.result.observed.1";
pub const TEST_AUDIT_ENTRY_REF: &str = "event.audit.entry.committed.1";
pub const TEST_PORTAL_READ_MODEL_REF: &str = "event.portal.read-model.updated.1";
pub const TEST_FLOW_EVIDENCE_REF: &str = "evidence.network.flow.1";
pub const TEST_DOMAIN_EVIDENCE_REF: &str = "evidence.network.domain.1";
pub const TEST_PARENT_RULE_REF: &str = "policy.rule.network-domain.1";
pub const TEST_ADAPTER_CAPABILITY_REF: &str = "adapter.capability.network.dry-run.1";
pub const TEST_ROLLBACK_REF: &str = "rollback.network.command.1";
pub const TEST_PROMPT_TEMPLATE_REF: &str = "prompt.network-ai-audit.v1";
pub const TEST_LIVE_CAPTURE_CUSTODY_STATUS_REF: &str = "network.live-capture.custody-status.13a";
pub const TEST_LIVE_CAPTURE_PROOF_REF: &str = "network.live-capture.proof.13";
pub const TEST_LIVE_CAPTURE_INTERFACE_REF: &str = "network.live-capture.interface.13";
pub const TEST_LIVE_CAPTURE_DRIVER_PROOF_REF: &str = "network.live-capture.driver-proof.13";
pub const TEST_LIVE_CAPTURE_PERMISSION_PROOF_REF: &str = "network.live-capture.permission-proof.13";
pub const TEST_LIVE_CAPTURE_BOUNDED_PROOF_REF: &str = "network.live-capture.bounded-proof.13";
pub const TEST_LIVE_CAPTURE_CLEAN_STOP_REF: &str = "network.live-capture.clean-stop.13";
pub const TEST_LIVE_CAPTURE_QUOTA_ROTATION_REF: &str = "network.live-capture.quota-rotation.13";
pub const TEST_LIVE_CAPTURE_RETENTION_DELETE_EXPORT_REF: &str =
    "network.live-capture.retention-delete-export.13";
pub const TEST_LIVE_CAPTURE_CUSTODY_REF: &str = "network.live-capture.custody.13";
pub const TEST_LIVE_CAPTURE_PRIVATE_TRAFFIC_EXCLUSION_REF: &str =
    "network.live-capture.private-traffic-exclusion.13";
pub const TEST_RAW_CAPTURE_STORAGE_PROOF_REF: &str = "network.raw-capture.storage-proof.03a";
pub const TEST_RAW_CAPTURE_ARTIFACT_MANIFEST_REF: &str =
    "network.raw-capture.artifact-manifest.03a";
pub const TEST_RAW_CAPTURE_STORAGE_LOCATION_REF: &str =
    "network.raw-capture.local-encrypted-storage.03a";
pub const TEST_RAW_CAPTURE_ENCRYPTION_AT_REST_REF: &str =
    "network.raw-capture.encryption-at-rest.03a";
pub const TEST_RAW_CAPTURE_QUOTA_ROTATION_REF: &str = "network.raw-capture.quota-rotation.03a";
pub const TEST_RAW_CAPTURE_RETENTION_POLICY_REF: &str = "network.raw-capture.retention-policy.03a";
pub const TEST_RAW_CAPTURE_DELETE_EXPORT_REF: &str = "network.raw-capture.delete-export.03a";
pub const TEST_RAW_CAPTURE_CUSTODY_CHAIN_REF: &str = "network.raw-capture.custody-chain.03a";
pub const TEST_RAW_CAPTURE_PRIVATE_TRAFFIC_EXCLUSION_REF: &str =
    "network.raw-capture.private-traffic-exclusion.03a";
pub const TEST_PRODUCT_READINESS_STATUS_REF: &str = "network.product-readiness.status.51a";
pub const TEST_PRODUCT_READINESS_PORTAL_READ_MODEL_REF: &str =
    "network.product-readiness.portal-read-model.51a";
pub const TEST_PRODUCT_READINESS_RETENTION_EXPORT_REF: &str =
    "network.product-readiness.retention-export.51a";
pub const TEST_REMOTE_DELIVERY_STATUS_REF: &str = "network.remote-delivery.status.10c";
pub const TEST_LOCAL_AI_RUNTIME_RESULT_STATUS_REF: &str =
    "network.local-ai.runtime-result.status.33b";
pub const TEST_LOCAL_AI_TRIGGER_REF: &str = "network.local-ai.trigger.33b";
pub const TEST_LOCAL_AI_QUEUE_JOB_REF: &str = "network.local-ai.queue-job.33b";
pub const TEST_LOCAL_AI_QUEUE_REF: &str = "network.local-ai.queue.33b";
pub const TEST_LOCAL_AI_MODEL_RUNTIME_REF: &str = "network.local-ai.model-runtime.33b";
pub const TEST_LOCAL_AI_RESULT_REF: &str = "network.local-ai.result.33b";
pub const TEST_LOCAL_AI_RUNTIME_REFERENCE_ID: &str = "network.local-ai.runtime-ref.33b";
pub const TEST_LOCAL_AI_MODEL_REF: &str = "network.local-ai.model.33b";
pub const TEST_LOCAL_AI_MODEL_VERSION_REF: &str = "network.local-ai.model-version.33b";
pub const TEST_LOCAL_AI_OUTPUT_SUMMARY_REF: &str = "network.local-ai.output-summary.33b";
pub const TEST_LOCAL_AI_PROMPT_TEMPLATE_REF: &str = "network.local-ai.prompt-template.33b";
pub const TEST_LOCAL_AI_POLICY_CONTEXT_REF: &str = "network.local-ai.policy-context.33b";
pub const TEST_LOCAL_AI_NETWORK_SUMMARY_REF: &str = "network.local-ai.network-summary.33b";
pub const TEST_LOCAL_AI_SCREEN_SUMMARY_REF: &str = "network.local-ai.screen-summary.33b";
pub const TEST_LOCAL_AI_MANAGED_BROWSER_EXACT_URL_EVIDENCE_REF: &str =
    "network.local-ai.managed-browser-exact-url-evidence.33b";
pub const TEST_RISK_EVALUATION_REF: &str = "network.risk-evaluation.51a";
pub const TEST_CHILD_PROFILE_REF: &str = "child-profile.network.51a";
pub const TEST_HOUSEHOLD_POLICY_REF: &str = "household-policy.network.51a";
pub const TEST_RISK_BUDGET_REF: &str = "network.risk-budget.51a";
pub const TEST_CASCADE_REF: &str = "network.cascade.51a";
pub const TEST_RISK_SIGNAL_REF: &str = "network.signal.51a";
pub const TEST_RISK_AUDIT_REF: &str = "network.audit.51a";
pub const TEST_PERFORMANCE_BENCHMARK_REF: &str = "network.performance.benchmark.51a";
pub const TEST_PERFORMANCE_FIXTURE_SET_REF: &str = "network.performance.fixtures.51a";
pub const TEST_PERFORMANCE_EVENT_HISTORY_REF: &str = "network.performance.event-history.51a";
pub const TEST_PERFORMANCE_RESOURCE_SNAPSHOT_REF: &str =
    "network.performance.resource-snapshot.51a";
pub const TEST_PLATFORM_MANIFEST_REF: &str = "network.platform-claim.manifest.51a";
pub const TEST_PLATFORM_MANUAL_FOLLOWUP_REF: &str = "network.platform-claim.manual-followup.51a";
pub const TEST_BROKER_CUSTODY_PROOF_REF: &str = "broker.network.custody-proof.1";
pub const TEST_BROKER_PUBLISHER_AUTH_REF: &str = "broker.network.publisher-auth.1";
pub const TEST_BROKER_SUBSCRIBER_AUTH_REF: &str = "broker.network.subscriber-auth.1";
pub const TEST_BROKER_ENCRYPTION_REF: &str = "broker.network.encryption.1";
pub const TEST_BROKER_RETENTION_POLICY_REF: &str = "broker.network.retention-policy.1";
pub const TEST_BROKER_REPLAY_PLAN_REF: &str = "broker.network.replay-plan.1";
pub const TEST_BROKER_DELETION_PLAN_REF: &str = "broker.network.deletion-plan.1";
pub const TEST_BROKER_OFFSET_POLICY_REF: &str = "broker.network.offset-policy.1";
pub const TEST_BROKER_DEDUPE_POLICY_REF: &str = "broker.network.dedupe-policy.1";
pub const TEST_BROKER_CONFIG_REF: &str = "broker.network.config.1";
pub const TEST_BROKER_DROPPED_EVENT_AUDIT_REF: &str = "broker.network.dropped-event-audit.1";
pub const TEST_BROKER_ADAPTER_ACTION_LEDGER_REF: &str = "broker.network.adapter-action-ledger.1";
pub const TEST_FAMILY_HUB_IDENTITY_REF: &str = "family-hub.network.identity.1";
pub const TEST_FAMILY_HUB_RELAY_POLICY_REF: &str = "family-hub.network.relay-policy.1";
pub const TEST_REMOTE_LIFECYCLE_CROSS_PROCESS_REPLAY_REF: &str =
    "broker.network.cross-process-replay.manual-required.10d";
pub const TEST_REMOTE_LIFECYCLE_RETENTION_DELETE_EXPORT_REF: &str =
    "broker.network.remote-retention-delete-export.manual-required.10d";
pub const TEST_REMOTE_LIFECYCLE_DELIVERY_ACK_REF: &str =
    "family-hub.network.delivery-ack.manual-required.10d";
pub const TEST_REMOTE_LIFECYCLE_FOLLOWUP_REF: &str =
    "network.remote-delivery.lifecycle-followup.10d";
pub const TEST_REMOTE_DURABLE_ENVELOPE_SCHEMA_REF: &str =
    "broker.network.durable-envelope.schema.10e";
pub const TEST_REMOTE_DURABLE_ENVELOPE_JOURNAL_REF: &str =
    "broker.network.durable-envelope.journal-readiness.10e";
pub const TEST_REMOTE_DURABLE_ENVELOPE_REPLAY_REF: &str =
    "broker.network.durable-envelope.replay-readiness.10e";
pub const TEST_REMOTE_DURABLE_ENVELOPE_DELETE_EXPORT_REF: &str =
    "broker.network.durable-envelope.delete-export-readiness.10e";
pub const TEST_REMOTE_DURABLE_ENVELOPE_SUPPORT_STATUS_REF: &str =
    "network.remote-delivery.durable-envelope.support-status.10e";
pub const TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF: &str =
    "network.remote-delivery.event-chain-journal.10f";
pub const TEST_REMOTE_EVENT_CHAIN_REPLAY_REF: &str =
    "network.remote-delivery.event-chain-replay.10f";
pub const TEST_REMOTE_EVENT_CHAIN_EXPORT_REF: &str =
    "network.remote-delivery.event-chain-export.10f";
pub const TEST_REMOTE_EVENT_CHAIN_SUPPORT_STATUS_REF: &str =
    "network.remote-delivery.event-chain.support-status.10f";
pub const TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF: &str =
    "network.remote-delivery.event-chain.receipt-ledger.10g";
pub const TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF: &str =
    "network.remote-delivery.event-chain.local-receipt-ack.10g";
pub const TEST_REMOTE_EVENT_CHAIN_RECEIPT_REPLAY_REF: &str =
    "network.remote-delivery.event-chain.receipt-replay.10g";
pub const TEST_REMOTE_EVENT_CHAIN_RECEIPT_SUPPORT_STATUS_REF: &str =
    "network.remote-delivery.event-chain.receipt-support-status.10g";
pub const TEST_REMOTE_EVENT_CHAIN_JOURNAL_PATH_PREFIX: &str =
    "ocentra-network-remote-event-chain-journal";
pub const TEST_REMOTE_EVENT_CHAIN_JOURNAL_EXTENSION: &str = "ndjson";
pub const UNCERTAINTY_NETWORK_ONLY_NO_EXACT_URL: &str = "network-only-no-exact-url";
pub const UNSUPPORTED_CLAIM_DECRYPTED_HTTPS_PAYLOAD: &str = "decrypted-https-payload";
pub const UNAVAILABLE_REASON_MANUAL_REQUIRED: &str = "manual-required";

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
pub const ERROR_NETWORK_RUNTIME_QUEUE_OVERFLOW_DEAD_LETTERS: &str =
    "network runtime queue overflow dead letters";
pub const ERROR_NETWORK_RUNTIME_QUEUE_TTL_EXPIRES: &str =
    "network runtime queue ttl expires before dispatch";
pub const ERROR_NETWORK_RUNTIME_QUEUE_IDEMPOTENCY_REJECTS: &str =
    "network runtime queue idempotency rejects duplicates";
pub const ERROR_NETWORK_RUNTIME_PAYLOAD_DECODES: &str = "network runtime payload decodes";
pub const ERROR_NETWORK_RUNTIME_REVIEW_COMPLETES: &str = "network runtime review request completes";
pub const ERROR_NETWORK_RUNTIME_BROKER_DELIVERY_SEMANTICS: &str =
    "network runtime broker delivery semantics proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_DELIVERY_STATUS: &str =
    "network runtime remote delivery status proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_EVENT_CHAIN_JOURNAL: &str =
    "network runtime remote event-chain journal proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_RECEIPT_LEDGER: &str =
    "network runtime remote receipt ledger proof";
