use ocentra_parent_agent_protocol::activity::policy_preview::PolicyAssistantConfirmationState as ProtocolPolicyAssistantConfirmationState;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestOrigin as ProtocolPolicyRequestOrigin;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus as ProtocolPolicyRequestStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmAction;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmActorRole;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmActorState;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmRequest;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmRequestKind;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmTargetKind;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PolicyRequestAssistantPreviewConfirmParseState {
    Accepted,
    Rejected,
}

struct PolicyRequestPayloadText<'a>(&'a str);

const DEFAULT_CONFIRM_COMMAND_ID: &str = "policy-request-assistant-preview-confirm-command";
const DEFAULT_CONFIRM_REQUEST_ID: &str = "policy-request-1";
const DEFAULT_CONFIRM_SUBMISSION_KEY: &str = "policy-request-submission-1";
const DEFAULT_CONFIRM_HOUSEHOLD_ID: &str = "family-local";
const DEFAULT_CONFIRM_CHILD_PROFILE_ID: &str = "child-profile-1";
const DEFAULT_CONFIRM_SOURCE_DOCUMENT_ID: &str = "policy-document-1";
const DEFAULT_CONFIRM_TARGET_REFERENCE_ID: &str = "example.test";
const DEFAULT_CONFIRM_RULE_ID: &str = "browser-rule-1";
const DEFAULT_CONFIRM_REQUESTED_AT: &str = "2026-06-18T00:00:00Z";
const DEFAULT_CONFIRM_EXPIRES_AT: &str = "2026-06-18T01:00:00Z";
const DEFAULT_CONFIRM_ASSISTANT_PREVIEW_ID: &str = "assistant-preview-1";
const DEFAULT_CONFIRM_AUDIT_REFERENCE_ID: &str = "audit.policy-request.preview";
const DEFAULT_CONFIRM_ACTOR_ID: &str = "parent-1";
const DEFAULT_CONFIRM_AUDIT_CONFIRMATION_ID: &str = "audit.policy-request.confirmed";
const DEFAULT_CONFIRM_CONFIRMED_AT: &str = "2026-06-18T00:05:00Z";

pub(super) fn parse_policy_request_assistant_preview_confirm_request(
    command: &AgentCommandEnvelope,
) -> (
    PolicyRequestAssistantPreviewConfirmRequest,
    PolicyRequestAssistantPreviewConfirmParseState,
) {
    command
        .payload
        .get(constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_REQUEST)
        .and_then(payload_string)
        .and_then(|text| deserialize_request(&text))
        .map(|request| {
            (
                request,
                PolicyRequestAssistantPreviewConfirmParseState::Accepted,
            )
        })
        .unwrap_or_else(rejected_request)
}

pub(super) fn default_policy_request_assistant_preview_confirm_request(
) -> PolicyRequestAssistantPreviewConfirmRequest {
    PolicyRequestAssistantPreviewConfirmRequest {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: DEFAULT_CONFIRM_COMMAND_ID.to_string(),
        request_id: DEFAULT_CONFIRM_REQUEST_ID.to_string(),
        submission_key: DEFAULT_CONFIRM_SUBMISSION_KEY.to_string(),
        household_id: DEFAULT_CONFIRM_HOUSEHOLD_ID.to_string(),
        child_profile_id: DEFAULT_CONFIRM_CHILD_PROFILE_ID.to_string(),
        device_id: Some(constants::peer::LOCAL_DEV_AGENT.to_string()),
        source_document_id: DEFAULT_CONFIRM_SOURCE_DOCUMENT_ID.to_string(),
        policy_version: 1,
        request_kind: PolicyRequestAssistantPreviewConfirmRequestKind::AskParent,
        target_kind: PolicyRequestAssistantPreviewConfirmTargetKind::Site,
        target_reference_id: DEFAULT_CONFIRM_TARGET_REFERENCE_ID.to_string(),
        requested_action: PolicyRequestAssistantPreviewConfirmAction::AskParent,
        rule_id: Some(DEFAULT_CONFIRM_RULE_ID.to_string()),
        requested_bonus_minutes: None,
        requested_at: DEFAULT_CONFIRM_REQUESTED_AT.to_string(),
        expires_at: DEFAULT_CONFIRM_EXPIRES_AT.to_string(),
        origin: ProtocolPolicyRequestOrigin::AssistantDraft,
        assistant_preview_id: DEFAULT_CONFIRM_ASSISTANT_PREVIEW_ID.to_string(),
        assistant_confirmation_state:
            ProtocolPolicyAssistantConfirmationState::ParentConfirmationRequired,
        request_status: ProtocolPolicyRequestStatus::PreviewOnly,
        audit_reference_ids: vec![DEFAULT_CONFIRM_AUDIT_REFERENCE_ID.to_string()],
        confirmation_actor_id: DEFAULT_CONFIRM_ACTOR_ID.to_string(),
        confirmation_actor_role: PolicyRequestAssistantPreviewConfirmActorRole::Parent,
        confirmation_actor_state: PolicyRequestAssistantPreviewConfirmActorState::Active,
        confirmation_audit_reference_id: DEFAULT_CONFIRM_AUDIT_CONFIRMATION_ID.to_string(),
        confirmed_at: DEFAULT_CONFIRM_CONFIRMED_AT.to_string(),
    }
}

fn payload_string(value: &LogFieldValue) -> Option<PolicyRequestPayloadText<'_>> {
    if let LogFieldValue::String(text) = value {
        Some(PolicyRequestPayloadText(text.as_str()))
    } else {
        None
    }
}

fn deserialize_request(
    text: &PolicyRequestPayloadText<'_>,
) -> Option<PolicyRequestAssistantPreviewConfirmRequest> {
    serde_json::from_str(text.0).ok()
}

fn rejected_request() -> (
    PolicyRequestAssistantPreviewConfirmRequest,
    PolicyRequestAssistantPreviewConfirmParseState,
) {
    (
        super::default_policy_request_assistant_preview_confirm_request(),
        PolicyRequestAssistantPreviewConfirmParseState::Rejected,
    )
}
