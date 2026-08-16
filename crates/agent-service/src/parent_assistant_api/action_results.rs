use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantActionConfirmResult;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantActionConfirmState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantActionPreview;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantActionPreviewKind;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantActionPreviewResult;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantActionPreviewState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantBackendState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantChildAgentValidationState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantEvidenceContext;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantProviderState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantRunCancelResult;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantRunCancelState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantRunState;
use ocentra_parent_agent_protocol::policy_constants as policy;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use crate::parent_assistant_evidence_context::evidence_contexts_from_command;
use crate::time::timestamp_now;

use super::payload_fields;
use super::ParentAssistantPayloadFieldName;
use super::ParentAssistantText;
use super::ParentAssistantTextRef;

pub(super) fn run_cancel_result_for_command(
    command: &AgentCommandEnvelope,
) -> ParentAssistantRunCancelResult {
    ParentAssistantRunCancelResult {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        backend_state: ParentAssistantBackendState::RuntimeBacked,
        thread_id: payload_fields::string_payload_field(
            command,
            ParentAssistantPayloadFieldName(constants::parent_assistant::FIELD_THREAD_ID),
        )
        .unwrap_or_else(|| {
            ParentAssistantTextRef(constants::parent_assistant::DEFAULT_THREAD_ID).into_text()
        })
        .0,
        run_id: payload_fields::string_payload_field(
            command,
            ParentAssistantPayloadFieldName(constants::parent_assistant::FIELD_RUN_ID),
        )
        .unwrap_or_else(|| {
            ParentAssistantTextRef(constants::parent_assistant::DEFAULT_RUN_ID).into_text()
        })
        .0,
        cancel_state: ParentAssistantRunCancelState::NotRunning,
        run_state: ParentAssistantRunState::Completed,
        provider_state: ParentAssistantProviderState::Unavailable,
        unavailable_reason: Some(
            ParentAssistantTextRef(constants::parent_assistant::RUN_NOT_RUNNING_REASON)
                .into_text()
                .0,
        ),
    }
}

struct ActionConfirmInputs {
    action_intent_id: ParentAssistantText,
    preview_id: Option<ParentAssistantText>,
    audit_reason: ParentAssistantText,
    source_refs: Vec<ParentEvidenceReference>,
    raw_prose_present: bool,
}

pub(super) fn action_confirm_result_for_command(
    command: &AgentCommandEnvelope,
) -> ParentAssistantActionConfirmResult {
    let inputs = action_confirm_inputs(command);

    if inputs.raw_prose_present {
        return rejected_action_confirm_result(
            inputs.action_intent_id,
            inputs.source_refs,
            inputs.audit_reason,
            ParentAssistantTextRef(
                constants::parent_assistant::ACTION_CONFIRM_RAW_PROSE_REJECTED_REASON,
            ),
        );
    }

    let Some(preview_id) = inputs.preview_id else {
        return rejected_action_confirm_result(
            inputs.action_intent_id,
            inputs.source_refs,
            inputs.audit_reason,
            ParentAssistantTextRef(
                constants::parent_assistant::ACTION_CONFIRM_PREVIEW_REQUIRED_REASON,
            ),
        );
    };

    ParentAssistantActionConfirmResult {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        backend_state: ParentAssistantBackendState::ContractRequired,
        action_intent_id: inputs.action_intent_id.0,
        preview_id: Some(preview_id.0),
        action_kind: ParentAssistantActionPreviewKind::PolicySuggestion,
        confirm_state: ParentAssistantActionConfirmState::ContractRequired,
        preview_required: true,
        preview_satisfied: true,
        raw_assistant_prose_accepted: false,
        parent_confirmation_required: true,
        parent_confirmation_recorded: false,
        child_agent_validation_state:
            ParentAssistantChildAgentValidationState::ChildAgentContractRequired,
        source_refs: inputs.source_refs,
        audit_reason: inputs.audit_reason.0,
        requires_controller_lease: true,
        child_agent_contract_required: true,
        enforcement_applied: false,
        policy_written: false,
        reason: ParentAssistantTextRef(
            constants::parent_assistant::ACTION_CONFIRM_CONTRACT_REQUIRED_REASON,
        )
        .into_text()
        .0,
    }
}

pub(super) fn action_preview_result_for_command(
    command: &AgentCommandEnvelope,
) -> ParentAssistantActionPreviewResult {
    let previewed_at: String = timestamp_now();
    let question = payload_fields::string_payload_field(
        command,
        ParentAssistantPayloadFieldName(constants::field::PARENT_ASSISTANT_QUESTION),
    )
    .unwrap_or_else(|| {
        ParentAssistantTextRef(constants::parent_assistant::DEFAULT_QUESTION).into_text()
    });
    let preview = preview_only_action(&question);
    let evidence_context = evidence_contexts_from_command(command, None, None, previewed_at);
    let source_refs = source_refs_from_contexts(&evidence_context);
    ParentAssistantActionPreviewResult {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        backend_state: ParentAssistantBackendState::RuntimeBacked,
        action_intent_id: payload_fields::string_payload_field(
            command,
            ParentAssistantPayloadFieldName(constants::parent_assistant::FIELD_ACTION_INTENT_ID),
        )
        .unwrap_or_else(|| {
            ParentAssistantTextRef(constants::parent_assistant::DEFAULT_ACTION_INTENT_ID)
                .into_text()
        })
        .0,
        preview_state: ParentAssistantActionPreviewState::Draft,
        evidence_context,
        preview_required: true,
        preview_satisfied: true,
        raw_assistant_prose_accepted: false,
        parent_confirmation_required: true,
        parent_confirmation_recorded: false,
        child_agent_validation_state:
            ParentAssistantChildAgentValidationState::ChildAgentContractRequired,
        source_refs,
        audit_reason: ParentAssistantTextRef(
            constants::parent_assistant::ACTION_PREVIEW_AUDIT_REASON,
        )
        .into_text()
        .0,
        requires_controller_lease: preview.requires_controller_lease,
        child_agent_contract_required: true,
        enforcement_applied: false,
        policy_written: false,
        preview,
        reason: ParentAssistantTextRef(constants::parent_assistant::ACTION_PREVIEW_DRAFT_REASON)
            .into_text()
            .0,
    }
}

fn rejected_action_confirm_result(
    action_intent_id: ParentAssistantText,
    source_refs: Vec<ParentEvidenceReference>,
    audit_reason: ParentAssistantText,
    reason: ParentAssistantTextRef<'static>,
) -> ParentAssistantActionConfirmResult {
    ParentAssistantActionConfirmResult {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        backend_state: ParentAssistantBackendState::ContractRequired,
        action_intent_id: action_intent_id.0,
        preview_id: None,
        action_kind: ParentAssistantActionPreviewKind::PolicySuggestion,
        confirm_state: ParentAssistantActionConfirmState::Rejected,
        preview_required: true,
        preview_satisfied: false,
        raw_assistant_prose_accepted: false,
        parent_confirmation_required: true,
        parent_confirmation_recorded: false,
        child_agent_validation_state:
            ParentAssistantChildAgentValidationState::ChildAgentUnavailable,
        source_refs,
        audit_reason: audit_reason.0,
        requires_controller_lease: true,
        child_agent_contract_required: true,
        enforcement_applied: false,
        policy_written: false,
        reason: reason.into_text().0,
    }
}

fn source_refs_from_contexts(
    evidence_context: &[ParentAssistantEvidenceContext],
) -> Vec<ParentEvidenceReference> {
    evidence_context
        .iter()
        .map(|context| context.evidence.clone())
        .collect()
}

fn preview_only_action(question: &ParentAssistantText) -> ParentAssistantActionPreview {
    let normalized_question = ParentAssistantText(question.0.to_ascii_lowercase());
    let (action_kind, summary) = preview_kind_and_summary(&normalized_question);

    ParentAssistantActionPreview {
        preview_id: Some(constants::parent_assistant::DEFAULT_PREVIEW_ID.to_string()),
        action_kind,
        summary: Some(summary.into_text().0),
        action_reference: None,
        requires_controller_lease: action_kind != ParentAssistantActionPreviewKind::None,
        child_agent_contract_required: true,
        enforcement_applied: false,
    }
}

fn preview_kind_and_summary(
    normalized_question: &ParentAssistantText,
) -> (
    ParentAssistantActionPreviewKind,
    ParentAssistantTextRef<'static>,
) {
    if has_policy_hint(normalized_question) {
        return (
            ParentAssistantActionPreviewKind::PolicySuggestion,
            ParentAssistantTextRef(constants::parent_assistant::ACTION_PREVIEW_POLICY_SUMMARY),
        );
    }

    if has_schedule_hint(normalized_question) {
        return (
            ParentAssistantActionPreviewKind::ScheduleChange,
            ParentAssistantTextRef(constants::parent_assistant::ACTION_PREVIEW_SCHEDULE_SUMMARY),
        );
    }

    if has_time_limit_hint(normalized_question) {
        return (
            ParentAssistantActionPreviewKind::TimeLimitChange,
            ParentAssistantTextRef(constants::parent_assistant::ACTION_PREVIEW_TIME_LIMIT_SUMMARY),
        );
    }

    (
        ParentAssistantActionPreviewKind::None,
        ParentAssistantTextRef(constants::parent_assistant::ACTION_PREVIEW_NONE_SUMMARY),
    )
}

fn has_policy_hint(normalized_question: &ParentAssistantText) -> bool {
    normalized_question
        .0
        .contains(constants::parent_assistant::QUESTION_POLICY_HINT)
        || normalized_question
            .0
            .contains(constants::parent_assistant::QUESTION_RULE_HINT)
}

fn has_schedule_hint(normalized_question: &ParentAssistantText) -> bool {
    normalized_question
        .0
        .contains(constants::parent_assistant::QUESTION_SCHEDULE_HINT)
        || normalized_question
            .0
            .contains(constants::parent_assistant::QUESTION_BEDTIME_HINT)
}

fn has_time_limit_hint(normalized_question: &ParentAssistantText) -> bool {
    normalized_question
        .0
        .contains(constants::parent_assistant::QUESTION_TIME_LIMIT_HINT)
        || normalized_question
            .0
            .contains(constants::parent_assistant::QUESTION_LIMIT_HINT)
}

fn action_confirm_inputs(command: &AgentCommandEnvelope) -> ActionConfirmInputs {
    ActionConfirmInputs {
        action_intent_id: payload_fields::string_payload_field(
            command,
            ParentAssistantPayloadFieldName(constants::parent_assistant::FIELD_ACTION_INTENT_ID),
        )
        .unwrap_or_else(|| {
            ParentAssistantTextRef(constants::parent_assistant::DEFAULT_ACTION_INTENT_ID)
                .into_text()
        }),
        preview_id: payload_fields::string_payload_field(
            command,
            ParentAssistantPayloadFieldName(constants::field::PARENT_ASSISTANT_ACTION_PREVIEW_ID),
        ),
        audit_reason: payload_fields::string_payload_field(
            command,
            ParentAssistantPayloadFieldName(constants::field::PARENT_ASSISTANT_ACTION_AUDIT_REASON),
        )
        .unwrap_or_else(|| {
            ParentAssistantTextRef(constants::parent_assistant::ACTION_CONFIRM_AUDIT_REASON)
                .into_text()
        }),
        source_refs: source_refs_from_contexts(&evidence_contexts_from_command(
            command,
            None,
            None,
            timestamp_now::<String>(),
        )),
        raw_prose_present: payload_fields::string_payload_field(
            command,
            ParentAssistantPayloadFieldName(constants::field::PARENT_ASSISTANT_ACTION_RAW_PROSE),
        )
        .is_some(),
    }
}
