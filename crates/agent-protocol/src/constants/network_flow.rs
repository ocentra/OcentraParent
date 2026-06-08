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
pub const TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF: &str =
    "network.remote-delivery.event-chain-journal.10c";
pub const TEST_REMOTE_EVENT_CHAIN_REPLAY_REF: &str =
    "network.remote-delivery.event-chain-replay.10c";
pub const TEST_REMOTE_EVENT_CHAIN_EXPORT_REF: &str =
    "network.remote-delivery.event-chain-export.10c";
pub const TEST_REMOTE_EVENT_CHAIN_SUPPORT_STATUS_REF: &str =
    "network.remote-delivery.event-chain.support-status.10c";
pub const TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF: &str =
    "network.remote-delivery.event-chain.receipt-ledger.10d";
pub const TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF: &str =
    "network.remote-delivery.event-chain.local-receipt-ack.10d";
pub const TEST_REMOTE_EVENT_CHAIN_RECEIPT_REPLAY_REF: &str =
    "network.remote-delivery.event-chain.receipt-replay.10d";
pub const TEST_REMOTE_EVENT_CHAIN_RECEIPT_SUPPORT_STATUS_REF: &str =
    "network.remote-delivery.event-chain.receipt-support-status.10d";
pub const TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF: &str =
    "network.remote-delivery.durable-envelope.10e";
pub const TEST_REMOTE_DELIVERY_DURABLE_STORE_REF: &str =
    "network.remote-delivery.durable-envelope-store.10e";
pub const TEST_REMOTE_DELIVERY_DURABLE_REPLAY_REF: &str =
    "network.remote-delivery.durable-envelope-replay.10e";
pub const TEST_REMOTE_DELIVERY_DURABLE_DELETE_EXPORT_REF: &str =
    "network.remote-delivery.durable-envelope-delete-export.10e";
pub const TEST_REMOTE_DELIVERY_DURABLE_SUPPORT_STATUS_REF: &str =
    "network.remote-delivery.durable-envelope-support-status.10e";
pub const TEST_REMOTE_DELIVERY_STATUS_BRIDGE_REF: &str =
    "network.remote-delivery.status-bridge.10f";
pub const TEST_REMOTE_DELIVERY_OUTBOX_STATUS_BRIDGE_REF: &str =
    "network.remote-delivery.outbox-status-bridge.10h";
pub const TEST_REMOTE_DELIVERY_OUTBOX_REF: &str = "network.remote-delivery.outbox.10g";
pub const TEST_REMOTE_DELIVERY_OUTBOX_HANDOFF_REF: &str =
    "network.remote-delivery.outbox-handoff.10g";
pub const TEST_REMOTE_DELIVERY_OUTBOX_REPLAY_REF: &str =
    "network.remote-delivery.outbox-replay.10g";
pub const TEST_REMOTE_DELIVERY_OUTBOX_SUPPORT_STATUS_REF: &str =
    "network.remote-delivery.outbox-support-status.10g";
pub const TEST_REMOTE_DELIVERY_DISPATCH_READINESS_REF: &str =
    "network.remote-delivery.dispatch-readiness.10i";
pub const TEST_REMOTE_DELIVERY_TRANSPORT_REQUIREMENTS_REF: &str =
    "network.remote-delivery.transport-requirements.10i";
pub const TEST_REMOTE_DELIVERY_BROKER_DISPATCH_GATE_REF: &str =
    "network.remote-delivery.broker-dispatch-gate.10i";
pub const TEST_REMOTE_DELIVERY_FAMILY_HUB_DISPATCH_GATE_REF: &str =
    "network.remote-delivery.family-hub-dispatch-gate.10i";
pub const TEST_REMOTE_DELIVERY_NO_ENFORCEMENT_INVARIANT_REF: &str =
    "network.remote-delivery.no-enforcement-invariant.10j";
pub const TEST_REMOTE_DELIVERY_AVAILABLE_METADATA_REF: &str =
    "network.remote-delivery.available-metadata.10j";
pub const TEST_REMOTE_DELIVERY_TRANSPORT_DISPATCH_STATE_REF: &str =
    "network.remote-delivery.transport-dispatch-state.10k";
pub const TEST_REMOTE_DELIVERY_DISPATCH_BLOCKED_MANUAL_REF: &str =
    "network.remote-delivery.dispatch-blocked-manual-required.10k";
pub const TEST_REMOTE_DELIVERY_FUTURE_TRANSPORT_SEAM_REF: &str =
    "network.remote-delivery.future-transport-seam.10k";
pub const TEST_REMOTE_DELIVERY_FIXTURE_TRANSPORT_REF: &str =
    "network.remote-delivery.fixture-transport.10l";
pub const TEST_REMOTE_DELIVERY_FIXTURE_DISPATCH_ATTEMPT_REF: &str =
    "network.remote-delivery.fixture-dispatch-attempt.10l";
pub const TEST_REMOTE_DELIVERY_FIXTURE_ACK_REF: &str = "network.remote-delivery.fixture-ack.10l";
pub const TEST_REMOTE_DELIVERY_DELETE_EXPORT_PROPAGATION_REF: &str =
    "network.remote-delivery.delete-export-propagation-readiness.10m";
pub const TEST_REMOTE_DELIVERY_REMOTE_DELETE_REF: &str =
    "network.remote-delivery.remote-delete-readiness.10m";
pub const TEST_REMOTE_DELIVERY_REMOTE_EXPORT_REF: &str =
    "network.remote-delivery.remote-export-readiness.10m";
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
pub const ERROR_NETWORK_RUNTIME_REMOTE_DURABLE_ENVELOPE: &str =
    "network runtime remote durable envelope proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_OUTBOX_HANDOFF: &str =
    "network runtime remote outbox handoff proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_OUTBOX_STATUS_BRIDGE: &str =
    "network runtime remote outbox status bridge proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_DISPATCH_READINESS: &str =
    "network runtime remote dispatch readiness proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_NO_ENFORCEMENT_INVARIANT: &str =
    "network runtime remote no-enforcement invariant proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_TRANSPORT_DISPATCH_STATE: &str =
    "network runtime remote transport dispatch state proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_FIXTURE_TRANSPORT: &str =
    "network runtime remote fixture transport proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_DELETE_EXPORT_PROPAGATION: &str =
    "network runtime remote delete export propagation proof";
