use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogFieldValue,
    LogFields, LogLevel,
    PolicyAssistantConfirmationState as ProtocolPolicyAssistantConfirmationState,
    PolicyRequestAssistantPreviewConfirmAction, PolicyRequestAssistantPreviewConfirmActorRole,
    PolicyRequestAssistantPreviewConfirmActorState, PolicyRequestAssistantPreviewConfirmClaimState,
    PolicyRequestAssistantPreviewConfirmRequest, PolicyRequestAssistantPreviewConfirmRequestKind,
    PolicyRequestAssistantPreviewConfirmResult, PolicyRequestAssistantPreviewConfirmResultState,
    PolicyRequestAssistantPreviewConfirmTargetKind,
    PolicyRequestOrigin as ProtocolPolicyRequestOrigin,
    PolicyRequestStatus as ProtocolPolicyRequestStatus, PolicySourceStatus, PolicySourceSurface,
    PolicyTargetType, ACTIVITY_SCHEMA_VERSION, AGENT_PROTOCOL_SCHEMA_VERSION,
};
use ocentra_policy_control_core::policy_request::{
    confirm_assistant_policy_request_preview, policy_request_schema_version,
    AssistantPolicyRequestConfirmation, ChildPolicyRequest,
    PolicyAssistantConfirmationState as CorePolicyAssistantConfirmationState,
    PolicyDurationMinutes, PolicyRequestId, PolicyRequestKind as CorePolicyRequestKind,
    PolicyRequestOrigin as CorePolicyRequestOrigin, PolicyRequestScope,
    PolicyRequestStatus as CorePolicyRequestStatus, PolicyRequestSubmissionKey,
    PolicyRequestTarget, PolicyRequestTimestamp,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyActorRole as CoreParentPolicyActorRole, ParentPolicyDocumentId, PolicyActorId,
    PolicyAuditReferenceId, PolicyChildProfileId, PolicyDeviceId, PolicyHouseholdId,
    PolicyRuleAction as CorePolicyRuleAction, PolicyRuleId,
    PolicySourceActorState as CorePolicySourceActorState, PolicyTargetKind as CorePolicyTargetKind,
    PolicyTargetReferenceId, PolicyVersion,
};

use crate::{
    activity_store_path::activity_db_path, event_builder::build_event, fields::fields_from_pairs,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PolicyRequestAssistantPreviewConfirmParseState {
    Accepted,
    Rejected,
}

const POLICY_REQUEST_CONFIRM_SOURCE_ID: &str = "policy-request-assistant-preview-confirm";

struct PolicyRequestAssistantPreviewConfirmStoreOutcome {
    activity_store_mutation_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
    upstream_writer_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
    read_model_projection_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
}

impl PolicyRequestAssistantPreviewConfirmStoreOutcome {
    fn claimed() -> Self {
        Self {
            activity_store_mutation_claim_state:
                PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
            upstream_writer_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
            read_model_projection_claim_state:
                PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        }
    }

    fn unclaimed() -> Self {
        Self {
            activity_store_mutation_claim_state:
                PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
            upstream_writer_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
            read_model_projection_claim_state:
                PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        }
    }
}

#[derive(Clone)]
struct SupportedPolicyPreviewTarget {
    subject_kind: ActivitySubjectKind,
    target_type: PolicyTargetType,
    target_value: String,
    subject_display_name: String,
    subject_field: Option<(&'static str, String)>,
}

pub(crate) async fn build_policy_request_assistant_preview_confirm_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let (request, parse_state) = parse_policy_request_assistant_preview_confirm_request(&command);
    let result =
        execute_policy_request_assistant_preview_confirm(&command, &request, parse_state).await;
    let result_text =
        serde_json::to_string(&result).expect(constants::error::AGENT_EVENT_SERIALIZES);

    build_event(
        constants::event_id::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentPolicyRequestAssistantPreviewConfirmReported,
        LogLevel::Info,
        fields_from_pairs(vec![(
            constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_RESULT,
            LogFieldValue::String(result_text),
        )]),
        None,
    )
}

fn parse_policy_request_assistant_preview_confirm_request(
    command: &AgentCommandEnvelope,
) -> (
    PolicyRequestAssistantPreviewConfirmRequest,
    PolicyRequestAssistantPreviewConfirmParseState,
) {
    match command
        .payload
        .get(constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_REQUEST)
    {
        Some(LogFieldValue::String(text)) => match serde_json::from_str(text) {
            Ok(request) => (
                request,
                PolicyRequestAssistantPreviewConfirmParseState::Accepted,
            ),
            Err(_) => (
                default_policy_request_assistant_preview_confirm_request(),
                PolicyRequestAssistantPreviewConfirmParseState::Rejected,
            ),
        },
        _ => (
            default_policy_request_assistant_preview_confirm_request(),
            PolicyRequestAssistantPreviewConfirmParseState::Rejected,
        ),
    }
}

async fn execute_policy_request_assistant_preview_confirm(
    command: &AgentCommandEnvelope,
    request: &PolicyRequestAssistantPreviewConfirmRequest,
    parse_state: PolicyRequestAssistantPreviewConfirmParseState,
) -> PolicyRequestAssistantPreviewConfirmResult {
    if parse_state == PolicyRequestAssistantPreviewConfirmParseState::Rejected {
        return rejected_result(request, "invalid-request".to_string());
    }

    match build_core_child_policy_request(request).and_then(|core_request| {
        let confirmation = build_core_confirmation(request)?;
        confirm_assistant_policy_request_preview(&core_request, confirmation)
    }) {
        Ok(confirmed_request) => {
            let store_outcome =
                persist_supported_policy_preview_confirmation(command, request, &confirmed_request)
                    .await;
            confirmed_result(request, &confirmed_request, store_outcome)
        }
        Err(error) => rejected_result(request, error.to_string()),
    }
}

fn build_core_child_policy_request(
    request: &PolicyRequestAssistantPreviewConfirmRequest,
) -> Result<ChildPolicyRequest, EventingError> {
    if request.schema_version != AGENT_PROTOCOL_SCHEMA_VERSION {
        return Err(EventingError::InvalidValue {
            field: constants::field::SCHEMA_VERSION,
            value: request.schema_version.to_string(),
        });
    }

    let rule_id = request
        .rule_id
        .clone()
        .map(PolicyRuleId::parse)
        .transpose()?;
    let requested_bonus_minutes = request
        .requested_bonus_minutes
        .map(PolicyDurationMinutes::new)
        .transpose()?;

    Ok(ChildPolicyRequest {
        schema_version: policy_request_schema_version()?,
        request_id: PolicyRequestId::parse(request.request_id.clone())?,
        submission_key: PolicyRequestSubmissionKey::parse(request.submission_key.clone())?,
        household_id: PolicyHouseholdId::parse(request.household_id.clone())?,
        child_profile_id: PolicyChildProfileId::parse(request.child_profile_id.clone())?,
        device_id: request
            .device_id
            .clone()
            .map(PolicyDeviceId::parse)
            .transpose()?,
        source_document_id: ParentPolicyDocumentId::parse(request.source_document_id.clone())?,
        policy_version: PolicyVersion::new(request.policy_version)?,
        origin: map_request_origin(request.origin),
        assistant_preview_id: Some(
            ocentra_policy_control_core::policy_request::PolicyAssistantPreviewId::parse(
                request.assistant_preview_id.clone(),
            )?,
        ),
        assistant_confirmation_state: map_confirmation_state(request.assistant_confirmation_state),
        status: map_request_status(request.request_status),
        scope: PolicyRequestScope {
            request_kind: map_request_kind(request.request_kind),
            target: PolicyRequestTarget {
                kind: map_target_kind(request.target_kind),
                reference_id: PolicyTargetReferenceId::parse(request.target_reference_id.clone())?,
            },
            requested_action: map_requested_action(request.requested_action),
            rule_id,
            requested_bonus_minutes,
        },
        requested_at: PolicyRequestTimestamp::parse(request.requested_at.clone())?,
        expires_at: PolicyRequestTimestamp::parse(request.expires_at.clone())?,
        audit_reference_ids: request
            .audit_reference_ids
            .iter()
            .cloned()
            .map(PolicyAuditReferenceId::parse)
            .collect::<Result<Vec<_>, _>>()?,
        resolved_approval_id: None,
        resolved_at: None,
    })
}

fn build_core_confirmation(
    request: &PolicyRequestAssistantPreviewConfirmRequest,
) -> Result<AssistantPolicyRequestConfirmation, EventingError> {
    Ok(AssistantPolicyRequestConfirmation {
        actor_id: PolicyActorId::parse(request.confirmation_actor_id.clone())?,
        actor_role: map_actor_role(request.confirmation_actor_role),
        actor_state: map_actor_state(request.confirmation_actor_state),
        confirmed_at: PolicyRequestTimestamp::parse(request.confirmed_at.clone())?,
        audit_reference_id: PolicyAuditReferenceId::parse(
            request.confirmation_audit_reference_id.clone(),
        )?,
    })
}

fn confirmed_result(
    request: &PolicyRequestAssistantPreviewConfirmRequest,
    confirmed_request: &ChildPolicyRequest,
    store_outcome: PolicyRequestAssistantPreviewConfirmStoreOutcome,
) -> PolicyRequestAssistantPreviewConfirmResult {
    PolicyRequestAssistantPreviewConfirmResult {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: request.command_id.clone(),
        request_id: request.request_id.clone(),
        assistant_preview_id: Some(request.assistant_preview_id.clone()),
        result_state: PolicyRequestAssistantPreviewConfirmResultState::Confirmed,
        policy_request_status: map_protocol_request_status(confirmed_request.status),
        policy_assistant_confirmation_state: map_protocol_confirmation_state(
            confirmed_request.assistant_confirmation_state,
        ),
        policy_audit_reference_id: confirmed_request
            .audit_reference_ids
            .last()
            .map(|reference| reference.as_str().to_string()),
        confirmed_at: Some(request.confirmed_at.clone()),
        rejection_reason: None,
        command_transport_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        service_validation_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        activity_store_mutation_claim_state: store_outcome.activity_store_mutation_claim_state,
        upstream_writer_claim_state: store_outcome.upstream_writer_claim_state,
        read_model_projection_claim_state: store_outcome.read_model_projection_claim_state,
        portal_writable_ui_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        child_device_delivery_claim_state:
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        provider_delivery_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        platform_enforcement_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        product_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
    }
}

fn rejected_result(
    request: &PolicyRequestAssistantPreviewConfirmRequest,
    rejection_reason: String,
) -> PolicyRequestAssistantPreviewConfirmResult {
    PolicyRequestAssistantPreviewConfirmResult {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: request.command_id.clone(),
        request_id: request.request_id.clone(),
        assistant_preview_id: Some(request.assistant_preview_id.clone()),
        result_state: PolicyRequestAssistantPreviewConfirmResultState::Rejected,
        policy_request_status: request.request_status,
        policy_assistant_confirmation_state: request.assistant_confirmation_state,
        policy_audit_reference_id: None,
        confirmed_at: None,
        rejection_reason: Some(rejection_reason),
        command_transport_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        service_validation_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        activity_store_mutation_claim_state:
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        upstream_writer_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        read_model_projection_claim_state:
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        portal_writable_ui_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        child_device_delivery_claim_state:
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        provider_delivery_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        platform_enforcement_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        product_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
    }
}

async fn persist_supported_policy_preview_confirmation(
    command: &AgentCommandEnvelope,
    request: &PolicyRequestAssistantPreviewConfirmRequest,
    confirmed_request: &ChildPolicyRequest,
) -> PolicyRequestAssistantPreviewConfirmStoreOutcome {
    let Some(event) = confirmed_policy_preview_activity_event(command, request, confirmed_request)
    else {
        return PolicyRequestAssistantPreviewConfirmStoreOutcome::unclaimed();
    };

    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(&path).map_err(|_| ())?;
        store.ingest_events(&[event]).map_err(|_| ())?;
        Ok::<(), ()>(())
    })
    .await
    .ok()
    .and_then(Result::ok)
    .map(|_| PolicyRequestAssistantPreviewConfirmStoreOutcome::claimed())
    .unwrap_or_else(PolicyRequestAssistantPreviewConfirmStoreOutcome::unclaimed)
}

fn confirmed_policy_preview_activity_event(
    command: &AgentCommandEnvelope,
    request: &PolicyRequestAssistantPreviewConfirmRequest,
    confirmed_request: &ChildPolicyRequest,
) -> Option<ActivityEvent> {
    let target = supported_policy_preview_target(request)?;
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(target.target_type.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String(target.target_value.clone()),
    );
    fields.insert(
        constants::field::POLICY_SOURCE_STATUS.to_string(),
        LogFieldValue::String(PolicySourceStatus::Confirmed.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::POLICY_SOURCE_SURFACE.to_string(),
        LogFieldValue::String(PolicySourceSurface::AiPreview.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::POLICY_REQUEST_ORIGIN.to_string(),
        LogFieldValue::String(request.origin.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::POLICY_ASSISTANT_CONFIRMATION_STATE.to_string(),
        LogFieldValue::String(
            map_protocol_confirmation_state(confirmed_request.assistant_confirmation_state)
                .as_protocol_str()
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::POLICY_REQUEST_STATUS.to_string(),
        LogFieldValue::String(
            map_protocol_request_status(confirmed_request.status)
                .as_protocol_str()
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::POLICY_REVIEWED_BY_ACTOR_ID.to_string(),
        LogFieldValue::String(request.confirmation_actor_id.clone()),
    );
    fields.insert(
        constants::field::POLICY_REVIEWED_BY_ACTOR_ROLE.to_string(),
        LogFieldValue::String(actor_role_protocol(request.confirmation_actor_role).to_string()),
    );
    fields.insert(
        constants::field::POLICY_REVIEWED_AT.to_string(),
        LogFieldValue::String(request.confirmed_at.clone()),
    );
    if let Some(audit_reference_id) = confirmed_request.audit_reference_ids.last() {
        fields.insert(
            constants::field::POLICY_AUDIT_REFERENCE_ID.to_string(),
            LogFieldValue::String(audit_reference_id.as_str().to_string()),
        );
    }
    if let Some((field, value)) = target.subject_field.clone() {
        fields.insert(field.to_string(), LogFieldValue::String(value));
    }

    Some(ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: request.confirmation_audit_reference_id.clone(),
        observed_at: request.confirmed_at.clone(),
        source: ActivitySource {
            device_id: command.target.device_id.clone(),
            platform: command.target.platform.clone(),
            observer: ActivityObserver::AgentService,
            source_id: POLICY_REQUEST_CONFIRM_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: target.subject_kind,
            subject_id: request.target_reference_id.clone(),
            display_name: Some(target.subject_display_name),
        },
        fields,
        evidence: Vec::new(),
    })
}

fn supported_policy_preview_target(
    request: &PolicyRequestAssistantPreviewConfirmRequest,
) -> Option<SupportedPolicyPreviewTarget> {
    let target_value = request.target_reference_id.clone();
    let subject_display_name = target_value.clone();

    match request.target_kind {
        PolicyRequestAssistantPreviewConfirmTargetKind::App => Some(SupportedPolicyPreviewTarget {
            subject_kind: ActivitySubjectKind::Process,
            target_type: PolicyTargetType::App,
            target_value: target_value.clone(),
            subject_display_name,
            subject_field: Some((constants::field::PROCESS_NAME, target_value)),
        }),
        PolicyRequestAssistantPreviewConfirmTargetKind::Site => {
            Some(SupportedPolicyPreviewTarget {
                subject_kind: ActivitySubjectKind::Url,
                target_type: PolicyTargetType::Site,
                target_value: target_value.clone(),
                subject_display_name,
                subject_field: Some((constants::field::URL, target_value)),
            })
        }
        PolicyRequestAssistantPreviewConfirmTargetKind::Category => {
            Some(SupportedPolicyPreviewTarget {
                subject_kind: ActivitySubjectKind::Device,
                target_type: PolicyTargetType::Category,
                target_value: target_value.clone(),
                subject_display_name,
                subject_field: Some((constants::field::SCREEN_PRIMARY_CATEGORY, target_value)),
            })
        }
        PolicyRequestAssistantPreviewConfirmTargetKind::Device => {
            Some(SupportedPolicyPreviewTarget {
                subject_kind: ActivitySubjectKind::Device,
                target_type: PolicyTargetType::Device,
                target_value,
                subject_display_name,
                subject_field: None,
            })
        }
        PolicyRequestAssistantPreviewConfirmTargetKind::ChildProfile
        | PolicyRequestAssistantPreviewConfirmTargetKind::Resource => None,
    }
}

fn actor_role_protocol(role: PolicyRequestAssistantPreviewConfirmActorRole) -> &'static str {
    match role {
        PolicyRequestAssistantPreviewConfirmActorRole::Parent => "parent",
        PolicyRequestAssistantPreviewConfirmActorRole::CoParent => "co-parent",
        PolicyRequestAssistantPreviewConfirmActorRole::Observer => "observer",
        PolicyRequestAssistantPreviewConfirmActorRole::Child => "child",
        PolicyRequestAssistantPreviewConfirmActorRole::Support => "support",
    }
}

fn map_request_origin(origin: ProtocolPolicyRequestOrigin) -> CorePolicyRequestOrigin {
    match origin {
        ProtocolPolicyRequestOrigin::Child => CorePolicyRequestOrigin::Child,
        ProtocolPolicyRequestOrigin::AssistantDraft => CorePolicyRequestOrigin::AssistantDraft,
    }
}

fn map_request_kind(
    kind: PolicyRequestAssistantPreviewConfirmRequestKind,
) -> CorePolicyRequestKind {
    match kind {
        PolicyRequestAssistantPreviewConfirmRequestKind::AskParent => {
            CorePolicyRequestKind::AskParent
        }
        PolicyRequestAssistantPreviewConfirmRequestKind::BonusTime => {
            CorePolicyRequestKind::BonusTime
        }
        PolicyRequestAssistantPreviewConfirmRequestKind::TemporaryOverride => {
            CorePolicyRequestKind::TemporaryOverride
        }
    }
}

fn map_target_kind(kind: PolicyRequestAssistantPreviewConfirmTargetKind) -> CorePolicyTargetKind {
    match kind {
        PolicyRequestAssistantPreviewConfirmTargetKind::ChildProfile => {
            CorePolicyTargetKind::ChildProfile
        }
        PolicyRequestAssistantPreviewConfirmTargetKind::Device => CorePolicyTargetKind::Device,
        PolicyRequestAssistantPreviewConfirmTargetKind::App => CorePolicyTargetKind::App,
        PolicyRequestAssistantPreviewConfirmTargetKind::Site => CorePolicyTargetKind::Site,
        PolicyRequestAssistantPreviewConfirmTargetKind::Category => CorePolicyTargetKind::Category,
        PolicyRequestAssistantPreviewConfirmTargetKind::Resource => CorePolicyTargetKind::Resource,
    }
}

fn map_requested_action(
    action: PolicyRequestAssistantPreviewConfirmAction,
) -> CorePolicyRuleAction {
    match action {
        PolicyRequestAssistantPreviewConfirmAction::Allow => CorePolicyRuleAction::Allow,
        PolicyRequestAssistantPreviewConfirmAction::Warn => CorePolicyRuleAction::Warn,
        PolicyRequestAssistantPreviewConfirmAction::AskParent => CorePolicyRuleAction::AskParent,
        PolicyRequestAssistantPreviewConfirmAction::TimeLimit => CorePolicyRuleAction::TimeLimit,
        PolicyRequestAssistantPreviewConfirmAction::Block => CorePolicyRuleAction::Block,
    }
}

fn map_actor_role(
    role: PolicyRequestAssistantPreviewConfirmActorRole,
) -> CoreParentPolicyActorRole {
    match role {
        PolicyRequestAssistantPreviewConfirmActorRole::Parent => CoreParentPolicyActorRole::Parent,
        PolicyRequestAssistantPreviewConfirmActorRole::CoParent => {
            CoreParentPolicyActorRole::CoParent
        }
        PolicyRequestAssistantPreviewConfirmActorRole::Observer => {
            CoreParentPolicyActorRole::Observer
        }
        PolicyRequestAssistantPreviewConfirmActorRole::Child => CoreParentPolicyActorRole::Child,
        PolicyRequestAssistantPreviewConfirmActorRole::Support => {
            CoreParentPolicyActorRole::Support
        }
    }
}

fn map_actor_state(
    state: PolicyRequestAssistantPreviewConfirmActorState,
) -> CorePolicySourceActorState {
    match state {
        PolicyRequestAssistantPreviewConfirmActorState::Active => {
            CorePolicySourceActorState::Active
        }
        PolicyRequestAssistantPreviewConfirmActorState::Revoked => {
            CorePolicySourceActorState::Revoked
        }
    }
}

fn map_confirmation_state(
    state: ProtocolPolicyAssistantConfirmationState,
) -> CorePolicyAssistantConfirmationState {
    match state {
        ProtocolPolicyAssistantConfirmationState::NotRequired => {
            CorePolicyAssistantConfirmationState::NotRequired
        }
        ProtocolPolicyAssistantConfirmationState::ParentConfirmationRequired => {
            CorePolicyAssistantConfirmationState::ParentConfirmationRequired
        }
        ProtocolPolicyAssistantConfirmationState::ParentConfirmed => {
            CorePolicyAssistantConfirmationState::ParentConfirmed
        }
    }
}

fn map_request_status(status: ProtocolPolicyRequestStatus) -> CorePolicyRequestStatus {
    match status {
        ProtocolPolicyRequestStatus::PreviewOnly => CorePolicyRequestStatus::PreviewOnly,
        ProtocolPolicyRequestStatus::PendingParentReview => {
            CorePolicyRequestStatus::PendingParentReview
        }
        ProtocolPolicyRequestStatus::Approved => CorePolicyRequestStatus::Approved,
        ProtocolPolicyRequestStatus::Denied => CorePolicyRequestStatus::Denied,
        ProtocolPolicyRequestStatus::Modified => CorePolicyRequestStatus::Modified,
        ProtocolPolicyRequestStatus::Expired => CorePolicyRequestStatus::Expired,
        ProtocolPolicyRequestStatus::ReplayRejected => CorePolicyRequestStatus::PreviewOnly,
    }
}

fn map_protocol_request_status(status: CorePolicyRequestStatus) -> ProtocolPolicyRequestStatus {
    match status {
        CorePolicyRequestStatus::PreviewOnly => ProtocolPolicyRequestStatus::PreviewOnly,
        CorePolicyRequestStatus::PendingParentReview => {
            ProtocolPolicyRequestStatus::PendingParentReview
        }
        CorePolicyRequestStatus::Approved => ProtocolPolicyRequestStatus::Approved,
        CorePolicyRequestStatus::Denied => ProtocolPolicyRequestStatus::Denied,
        CorePolicyRequestStatus::Modified => ProtocolPolicyRequestStatus::Modified,
        CorePolicyRequestStatus::Expired => ProtocolPolicyRequestStatus::Expired,
        CorePolicyRequestStatus::ReplayRejected => ProtocolPolicyRequestStatus::ReplayRejected,
    }
}

fn map_protocol_confirmation_state(
    state: CorePolicyAssistantConfirmationState,
) -> ProtocolPolicyAssistantConfirmationState {
    match state {
        CorePolicyAssistantConfirmationState::NotRequired => {
            ProtocolPolicyAssistantConfirmationState::NotRequired
        }
        CorePolicyAssistantConfirmationState::ParentConfirmationRequired => {
            ProtocolPolicyAssistantConfirmationState::ParentConfirmationRequired
        }
        CorePolicyAssistantConfirmationState::ParentConfirmed => {
            ProtocolPolicyAssistantConfirmationState::ParentConfirmed
        }
    }
}

fn default_policy_request_assistant_preview_confirm_request(
) -> PolicyRequestAssistantPreviewConfirmRequest {
    PolicyRequestAssistantPreviewConfirmRequest {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: "policy-request-assistant-preview-confirm-command".to_string(),
        request_id: "policy-request-1".to_string(),
        submission_key: "policy-request-submission-1".to_string(),
        household_id: "family-local".to_string(),
        child_profile_id: "child-profile-1".to_string(),
        device_id: Some(constants::peer::LOCAL_DEV_AGENT.to_string()),
        source_document_id: "policy-document-1".to_string(),
        policy_version: 1,
        request_kind: PolicyRequestAssistantPreviewConfirmRequestKind::AskParent,
        target_kind: PolicyRequestAssistantPreviewConfirmTargetKind::Site,
        target_reference_id: "example.test".to_string(),
        requested_action: PolicyRequestAssistantPreviewConfirmAction::AskParent,
        rule_id: Some("browser-rule-1".to_string()),
        requested_bonus_minutes: None,
        requested_at: "2026-06-18T00:00:00Z".to_string(),
        expires_at: "2026-06-18T01:00:00Z".to_string(),
        origin: ProtocolPolicyRequestOrigin::AssistantDraft,
        assistant_preview_id: "assistant-preview-1".to_string(),
        assistant_confirmation_state:
            ProtocolPolicyAssistantConfirmationState::ParentConfirmationRequired,
        request_status: ProtocolPolicyRequestStatus::PreviewOnly,
        audit_reference_ids: vec!["audit.policy-request.preview".to_string()],
        confirmation_actor_id: "parent-1".to_string(),
        confirmation_actor_role: PolicyRequestAssistantPreviewConfirmActorRole::Parent,
        confirmation_actor_state: PolicyRequestAssistantPreviewConfirmActorState::Active,
        confirmation_audit_reference_id: "audit.policy-request.confirmed".to_string(),
        confirmed_at: "2026-06-18T00:05:00Z".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use ocentra_parent_agent_core::activity_store::ActivityStore;
    use ocentra_parent_agent_protocol::{
        AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentPeer, AgentPeerRole,
        AgentRoute, LogFields, PolicyAssistantConfirmationState, PolicyRequestStatus,
        PolicySourceStatus, PolicySourceSurface, PolicyTargetType,
    };

    use super::*;
    use crate::{
        activity_report_env_lock::REPORT_ENV_LOCK, lan_pairing::LanPairingRuntime,
        websocket::handle_command_text_for_test,
    };

    #[tokio::test]
    async fn policy_request_assistant_preview_confirm_accepts_valid_parent_confirmation() {
        let _guard = REPORT_ENV_LOCK.lock().await;
        let store_path = temp_path("policy-request-confirm");
        cleanup_path(&store_path);
        std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

        let body = serde_json::to_string(&command_envelope(
            default_policy_request_assistant_preview_confirm_request(),
        ))
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
        let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
        let result = result_payload(
            &event.payload[constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_RESULT],
        );
        let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
        let read_model = store
            .policy_preview_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                "2026-06-18T00:10:30Z",
            )
            .expect(constants::error::ACTIVITY_STORE_QUERIES);

        std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
        cleanup_path(&store_path);

        assert_eq!(
            event.event,
            AgentEventName::AgentPolicyRequestAssistantPreviewConfirmReported
        );
        assert_eq!(
            result.result_state,
            PolicyRequestAssistantPreviewConfirmResultState::Confirmed
        );
        assert_eq!(
            result.policy_request_status,
            PolicyRequestStatus::PendingParentReview
        );
        assert_eq!(
            result.policy_assistant_confirmation_state,
            PolicyAssistantConfirmationState::ParentConfirmed
        );
        assert_eq!(
            result.command_transport_claim_state,
            PolicyRequestAssistantPreviewConfirmClaimState::Claimed
        );
        assert_eq!(
            result.service_validation_claim_state,
            PolicyRequestAssistantPreviewConfirmClaimState::Claimed
        );
        assert_eq!(
            result.activity_store_mutation_claim_state,
            PolicyRequestAssistantPreviewConfirmClaimState::Claimed
        );
        assert_eq!(
            result.upstream_writer_claim_state,
            PolicyRequestAssistantPreviewConfirmClaimState::Claimed
        );
        assert_eq!(
            result.read_model_projection_claim_state,
            PolicyRequestAssistantPreviewConfirmClaimState::Claimed
        );
        assert_eq!(read_model.returned, 1);
        assert_eq!(
            read_model.rows[0].target.target_type,
            PolicyTargetType::Site
        );
        assert_eq!(read_model.rows[0].target.target_value, "example.test");
        assert_eq!(
            read_model.rows[0].policy_source_status,
            Some(PolicySourceStatus::Confirmed)
        );
        assert_eq!(
            read_model.rows[0].policy_source_surface,
            Some(PolicySourceSurface::AiPreview)
        );
        assert_eq!(
            read_model.rows[0].policy_request_origin,
            Some(ProtocolPolicyRequestOrigin::AssistantDraft)
        );
        assert_eq!(
            read_model.rows[0].policy_assistant_confirmation_state,
            Some(PolicyAssistantConfirmationState::ParentConfirmed)
        );
        assert_eq!(
            read_model.rows[0].policy_request_status,
            Some(PolicyRequestStatus::PendingParentReview)
        );
        assert_eq!(
            read_model.rows[0].policy_reviewed_by_actor_id.as_deref(),
            Some("parent-1")
        );
        assert_eq!(
            read_model.rows[0].policy_audit_reference_id.as_deref(),
            Some("audit.policy-request.confirmed")
        );
        assert_eq!(
            result.product_claim_state,
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed
        );
        assert!(result.rejection_reason.is_none());
    }

    #[tokio::test]
    async fn policy_request_assistant_preview_confirm_leaves_unsupported_targets_unclaimed() {
        let _guard = REPORT_ENV_LOCK.lock().await;
        let store_path = temp_path("policy-request-confirm-unsupported");
        cleanup_path(&store_path);
        std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

        let mut request = default_policy_request_assistant_preview_confirm_request();
        request.target_kind = PolicyRequestAssistantPreviewConfirmTargetKind::ChildProfile;
        request.target_reference_id = "child-profile-1".to_string();

        let body = serde_json::to_string(&command_envelope(request))
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
        let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
        let result = result_payload(
            &event.payload[constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_RESULT],
        );
        let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
        let read_model = store
            .policy_preview_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                "2026-06-18T00:10:30Z",
            )
            .expect(constants::error::ACTIVITY_STORE_QUERIES);

        std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
        cleanup_path(&store_path);

        assert_eq!(
            result.result_state,
            PolicyRequestAssistantPreviewConfirmResultState::Confirmed
        );
        assert_eq!(
            result.activity_store_mutation_claim_state,
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed
        );
        assert_eq!(
            result.upstream_writer_claim_state,
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed
        );
        assert_eq!(
            result.read_model_projection_claim_state,
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed
        );
        assert_eq!(read_model.returned, 0);
    }

    #[tokio::test]
    async fn policy_request_assistant_preview_confirm_rejects_missing_typed_request_payload() {
        let body = serde_json::to_string(&command_envelope_without_request())
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
        let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
        let result = result_payload(
            &event.payload[constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_RESULT],
        );

        assert_eq!(
            result.result_state,
            PolicyRequestAssistantPreviewConfirmResultState::Rejected
        );
        assert_eq!(
            result.policy_request_status,
            PolicyRequestStatus::PreviewOnly
        );
        assert_eq!(
            result.policy_assistant_confirmation_state,
            PolicyAssistantConfirmationState::ParentConfirmationRequired
        );
        assert_eq!(result.rejection_reason.as_deref(), Some("invalid-request"));
    }

    #[tokio::test]
    async fn policy_request_assistant_preview_confirm_rejects_invalid_parent_authority() {
        let mut request = default_policy_request_assistant_preview_confirm_request();
        request.confirmation_actor_role = PolicyRequestAssistantPreviewConfirmActorRole::Observer;
        let body = serde_json::to_string(&command_envelope(request))
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
        let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
        let result = result_payload(
            &event.payload[constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_RESULT],
        );

        assert_eq!(
            result.result_state,
            PolicyRequestAssistantPreviewConfirmResultState::Rejected
        );
        assert!(result.rejection_reason.is_some());
    }

    fn command_envelope(
        request: PolicyRequestAssistantPreviewConfirmRequest,
    ) -> AgentCommandEnvelope {
        AgentCommandEnvelope {
            schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
            message_id: "cmd-policy-request-confirm-1".to_string(),
            sent_at: "2026-06-18T00:10:00Z".to_string(),
            source: AgentPeer {
                peer_id: constants::peer::PORTAL_DEV.to_string(),
                role: AgentPeerRole::Portal,
            },
            target: AgentMessageTarget {
                device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                platform: "windows".to_string(),
                route: AgentRoute::Localhost,
            },
            command: AgentCommandName::AgentPolicyRequestAssistantPreviewConfirm,
            payload: fields_from_pairs(vec![(
                constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_REQUEST,
                LogFieldValue::String(
                    serde_json::to_string(&request)
                        .expect(constants::error::AGENT_EVENT_SERIALIZES),
                ),
            )]),
        }
    }

    fn command_envelope_without_request() -> AgentCommandEnvelope {
        AgentCommandEnvelope {
            payload: LogFields::new(),
            ..command_envelope(default_policy_request_assistant_preview_confirm_request())
        }
    }

    fn result_payload(value: &LogFieldValue) -> PolicyRequestAssistantPreviewConfirmResult {
        match value {
            LogFieldValue::String(text) => {
                serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
            }
            _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
        }
    }

    fn temp_path(suffix: &str) -> std::path::PathBuf {
        let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(suffix);

        let mut path = std::env::temp_dir();
        path.push(name);
        path.set_extension(constants::activity_store::FILE_EXTENSION);
        path
    }

    fn cleanup_path(path: &std::path::PathBuf) {
        let _ = remove_file(path);
        let mut wal_path = path.clone();
        wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
        let _ = remove_file(wal_path);
        let mut shm_path = path.clone();
        shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
        let _ = remove_file(shm_path);
    }
}
