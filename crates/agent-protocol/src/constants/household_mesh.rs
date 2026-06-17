pub const EVENT_SCHEMA_VERSION: u16 = 1;
pub const EVENT_BRIDGE_LOCAL_SELECTED: &str = "household.mesh.bridge.local-event.selected";
pub const EVENT_BRIDGE_LAN_EXPORTED: &str = "household.mesh.bridge.lan-message.exported";
pub const EVENT_BRIDGE_LAN_RECEIVED: &str = "household.mesh.bridge.lan-message.received";
pub const EVENT_BRIDGE_LOCAL_REPUBLISHED: &str = "household.mesh.bridge.local-event.republished";
pub const MESSAGE_AI_WORK_OFFER: &str = "household.mesh.ai-work-offer";
pub const MESSAGE_AI_WORK_RESULT: &str = "household.mesh.ai-work-result";
pub const TARGET_BRIDGE_EXPORT_VALIDATOR: &str = "household-mesh-bridge-export-validator";
pub const TARGET_BRIDGE_LAN_TRANSPORT: &str = "household-mesh-lan-transport";
pub const TARGET_BRIDGE_IMPORT_VALIDATOR: &str = "household-mesh-bridge-import-validator";
pub const TARGET_LOCAL_EVENT_REPUBLISHER: &str = "household-mesh-local-event-republisher";
pub const SUBSCRIBER_BRIDGE_LOCAL_SELECTED: &str =
    "household-mesh-bridge-local-selected-subscriber";
pub const SUBSCRIBER_BRIDGE_LAN_EXPORTED: &str = "household-mesh-bridge-lan-exported-subscriber";
pub const SUBSCRIBER_BRIDGE_LAN_RECEIVED: &str = "household-mesh-bridge-lan-received-subscriber";
pub const SUBSCRIBER_BRIDGE_LOCAL_REPUBLISHED: &str =
    "household-mesh-bridge-local-republished-subscriber";
pub const RUNTIME_COMPONENT_HOUSEHOLD_MESH_BRIDGE: &str = "household-mesh-bridge";
pub const RUNTIME_INSTANCE_CHILD_MESH_BRIDGE: &str = "child-household-mesh-bridge";
pub const RUNTIME_INSTANCE_PROVIDER_MESH_BRIDGE: &str = "provider-household-mesh-bridge";
pub const AGGREGATE_HOUSEHOLD_MESH_PREFIX: &str = "household-mesh-";
pub const IDEMPOTENCY_HOUSEHOLD_MESH_PREFIX: &str = "household-mesh-idempotency-";
pub const TEST_BRIDGE_CORRELATION_ID: &str = "household-mesh-bridge-correlation-1";
pub const TEST_BRIDGE_OUTBOUND_MESSAGE_ID: &str = "household-mesh-outbound-message-1";
pub const TEST_BRIDGE_INBOUND_MESSAGE_ID: &str = "household-mesh-inbound-message-1";
pub const TEST_BRIDGE_CHILD_AGENT_PEER_ID: &str = "child-agent-peer-1";
pub const TEST_BRIDGE_PROVIDER_PEER_ID: &str = "trusted-parent-desktop-provider-peer-1";
pub const TEST_BRIDGE_PAYLOAD_REF: &str = "household-mesh-redacted-payload-ref-1";
pub const TEST_BRIDGE_FAMILY_ID: &str = "household-mesh-family-1";
pub const TEST_BRIDGE_OTHER_FAMILY_ID: &str = "household-mesh-family-2";
pub const TEST_BRIDGE_TARGET_CHILD_DEVICE_ID: &str = "household-mesh-child-device-1";
pub const TEST_BRIDGE_OTHER_CHILD_DEVICE_ID: &str = "household-mesh-child-device-2";
pub const TEST_BRIDGE_IDEMPOTENCY_KEY: &str = "household-mesh-idempotency-1";
pub const TEST_BRIDGE_SENT_AT_EPOCH_SECONDS: u64 = 1_720_000_000;
pub const TEST_BRIDGE_RECEIVED_AT_EPOCH_SECONDS: u64 = 1_720_000_030;
pub const TEST_BRIDGE_STALE_AFTER_SECONDS: u64 = 60;
pub const TEST_BRIDGE_SELECTED_EVENT_REF: &str = "event.household.mesh.local-selected.1";
pub const TEST_BRIDGE_EXPORTED_MESSAGE_REF: &str = "message.household.mesh.exported.1";
pub const TEST_BRIDGE_RECEIVED_MESSAGE_REF: &str = "message.household.mesh.received.1";
pub const ERROR_BRIDGE_CHAIN_PUBLISHES: &str = "household mesh bridge chain publishes";
pub const ERROR_BRIDGE_PAYLOAD_DECODES: &str = "household mesh bridge payload decodes";
pub const ERROR_BRIDGE_TOPOLOGY_PROVES: &str = "household mesh bridge topology proves";
pub const TEST_ROUTE_JOB_ID: &str = "household-ai-route-job-1";
pub const TEST_PARENT_DESKTOP_PROVIDER_ID: &str = "parent-desktop-ai-provider-1";
pub const TEST_OTHER_LAPTOP_PROVIDER_ID: &str = "household-laptop-ai-provider-1";
pub const TEST_CHILD_DESKTOP_PROVIDER_ID: &str = "child-desktop-ai-provider-1";
pub const TEST_PARENT_MOBILE_PROVIDER_ID: &str = "parent-mobile-ai-provider-1";
pub const TEST_REVOKED_PROVIDER_ID: &str = "revoked-ai-provider-1";
pub const ROUTE_REASON_SELECTED_DESKTOP: &str = "selected-desktop-provider";
pub const ROUTE_REASON_SELECTED_LAPTOP: &str = "selected-laptop-provider";
pub const ROUTE_REASON_SELECTED_CHILD_DESKTOP: &str = "selected-child-desktop-provider";
pub const ROUTE_REASON_MOBILE_DORMANT_DESKTOP_AVAILABLE: &str = "mobile-dormant-desktop-available";
pub const ROUTE_REASON_MOBILE_FALLBACK_ALLOWED: &str = "mobile-fallback-allowed";
pub const ROUTE_REASON_MOBILE_FALLBACK_DENIED: &str = "mobile-fallback-denied";
pub const ROUTE_REASON_STALE_PROVIDER: &str = "stale-provider";
pub const ROUTE_REASON_OFFLINE_PROVIDER: &str = "offline-provider";
pub const ROUTE_REASON_REVOKED_PROVIDER: &str = "revoked-provider";
pub const ROUTE_REASON_CUSTODY_MISMATCH: &str = "custody-mismatch";
pub const ROUTE_REASON_UNSUPPORTED_CAPABILITY: &str = "unsupported-capability";
pub const ROUTE_REASON_PROVIDER_DEGRADED: &str = "provider-degraded";
pub const ROUTE_REASON_NO_PROVIDER: &str = "no-provider";
pub const ERROR_ROUTE_SELECTS_PROVIDER: &str = "household provider route selects provider";
pub const LOCAL_EVENT_DEVICE_DISCOVERY: &str = "household.mesh.local-event.device-discovery";
pub const LOCAL_EVENT_PROVIDER_ADVERTISEMENT: &str =
    "household.mesh.local-event.provider-advertisement";
pub const LOCAL_EVENT_PROVIDER_HEARTBEAT: &str = "household.mesh.local-event.provider-heartbeat";
pub const LOCAL_EVENT_PROVIDER_CAPABILITY: &str = "household.mesh.local-event.provider-capability";
pub const LOCAL_EVENT_AI_WORK_OFFER: &str = "household.mesh.local-event.ai-work-offer";
pub const LOCAL_EVENT_AI_WORK_CLAIM_REQUEST: &str =
    "household.mesh.local-event.ai-work-claim-request";
pub const LOCAL_EVENT_AI_WORK_CLAIM_DECISION: &str =
    "household.mesh.local-event.ai-work-claim-decision";
pub const LOCAL_EVENT_AI_WORK_LEASE_STATE: &str = "household.mesh.local-event.ai-work-lease-state";
pub const LOCAL_EVENT_AI_JOB_PAYLOAD_TRANSFER: &str =
    "household.mesh.local-event.ai-job-payload-transfer";
pub const LOCAL_EVENT_AI_RESULT_RETURN: &str = "household.mesh.local-event.ai-result-return";
pub const LOCAL_EVENT_CONFIG_COMMAND: &str = "household.mesh.local-event.config-command";
pub const LOCAL_EVENT_APPROVAL_OVERRIDE_COMMAND: &str =
    "household.mesh.local-event.approval-override-command";
pub const LOCAL_EVENT_READ_MODEL_QUERY_REQUEST: &str =
    "household.mesh.local-event.read-model-query-request";
pub const LOCAL_EVENT_RAW_CAPTURE_INTERNAL: &str =
    "household.mesh.local-event.raw-capture-internal";
pub const LOCAL_EVENT_ADAPTER_INTERNAL: &str = "household.mesh.local-event.adapter-internal";
pub const LOCAL_EVENT_PRIVATE_QUEUE_MECHANIC: &str =
    "household.mesh.local-event.private-queue-mechanic";
pub const LOCAL_EVENT_POLICY_DECISION: &str = "household.mesh.local-event.policy-decision";
pub const LOCAL_EVENT_ENFORCEMENT_COMMAND: &str = "household.mesh.local-event.enforcement-command";

pub const LAN_MESSAGE_DEVICE_DISCOVERY: &str = "household.mesh.lan-message.device-discovery";
pub const LAN_MESSAGE_PROVIDER_ADVERTISEMENT: &str =
    "household.mesh.lan-message.provider-advertisement";
pub const LAN_MESSAGE_PROVIDER_HEARTBEAT: &str = "household.mesh.lan-message.provider-heartbeat";
pub const LAN_MESSAGE_PROVIDER_CAPABILITY: &str = "household.mesh.lan-message.provider-capability";
pub const LAN_MESSAGE_AI_WORK_OFFER: &str = "household.mesh.lan-message.ai-work-offer";
pub const LAN_MESSAGE_AI_WORK_CLAIM_REQUEST: &str =
    "household.mesh.lan-message.ai-work-claim-request";
pub const LAN_MESSAGE_AI_WORK_CLAIM_DECISION: &str =
    "household.mesh.lan-message.ai-work-claim-decision";
pub const LAN_MESSAGE_AI_WORK_LEASE_STATE: &str = "household.mesh.lan-message.ai-work-lease-state";
pub const LAN_MESSAGE_AI_JOB_PAYLOAD_TRANSFER: &str =
    "household.mesh.lan-message.ai-job-payload-transfer";
pub const LAN_MESSAGE_AI_RESULT_RETURN: &str = "household.mesh.lan-message.ai-result-return";
pub const LAN_MESSAGE_CONFIG_COMMAND: &str = "household.mesh.lan-message.config-command";
pub const LAN_MESSAGE_APPROVAL_OVERRIDE_COMMAND: &str =
    "household.mesh.lan-message.approval-override-command";
pub const LAN_MESSAGE_READ_MODEL_QUERY_REQUEST: &str =
    "household.mesh.lan-message.read-model-query-request";

pub const AUTHENTICATION_PAIRED_TRUSTED_DEVICE: &str = "paired-trusted-device";
pub const AUTHENTICATION_ANONYMOUS: &str = "anonymous";
pub const AUTHENTICATION_STALE_OR_REVOKED: &str = "stale-or-revoked";
pub const POLICY_AUTHORITY_CHILD_AGENT_ONLY: &str = "child-agent-only";
pub const POLICY_AUTHORITY_PROVIDER_CLAIMED: &str = "provider-claimed";
pub const POLICY_AUTHORITY_PARENT_UI_CLAIMED: &str = "parent-ui-claimed";
pub const BRIDGE_STATE_LOCAL_REPUBLISH_REQUIRED: &str = "local-republish-required";
pub const BRIDGE_STATE_EXPORT_SELECTED: &str = "export-selected";
pub const REJECTION_UNSELECTED_LOCAL_EVENT: &str = "unselected-local-event";
pub const REJECTION_UNAUTHENTICATED_MESSAGE: &str = "unauthenticated-message";
pub const REJECTION_DIRECT_REMOTE_PUBLISH: &str = "direct-remote-publish";
pub const REJECTION_POLICY_AUTHORITY_ESCALATION: &str = "policy-authority-escalation";
pub const REJECTION_RAW_PAYLOAD: &str = "raw-payload";
pub const REJECTION_MISMATCHED_MESSAGE_REF: &str = "mismatched-message-ref";
pub const REJECTION_REPLAYED_MESSAGE: &str = "replayed-message";
pub const REJECTION_STALE_MESSAGE: &str = "stale-message";
pub const REJECTION_FAMILY_MISMATCH: &str = "family-mismatch";
pub const REJECTION_WRONG_TARGET_DEVICE: &str = "wrong-target-device";

pub const TEST_SELECTED_EXPORTS_EXPECT: &str = "selected local event exports";
pub const TEST_UNSELECTED_REJECTS_EXPECT: &str = "unselected local event rejects";
pub const TEST_INCOMING_VALIDATES_EXPECT: &str = "incoming message validates before republish";
pub const TEST_DIRECT_REMOTE_REJECTS_EXPECT: &str = "direct remote publish rejects";
pub const TEST_PROVIDER_POLICY_REJECTS_EXPECT: &str = "provider policy authority rejects";
