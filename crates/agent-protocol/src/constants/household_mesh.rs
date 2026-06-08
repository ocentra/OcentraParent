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

pub const TEST_SELECTED_EXPORTS_EXPECT: &str = "selected local event exports";
pub const TEST_UNSELECTED_REJECTS_EXPECT: &str = "unselected local event rejects";
pub const TEST_INCOMING_VALIDATES_EXPECT: &str = "incoming message validates before republish";
pub const TEST_DIRECT_REMOTE_REJECTS_EXPECT: &str = "direct remote publish rejects";
pub const TEST_PROVIDER_POLICY_REJECTS_EXPECT: &str = "provider policy authority rejects";
