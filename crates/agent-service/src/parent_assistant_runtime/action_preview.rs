use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantActionPreview;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantActionPreviewKind;

use super::ParentAssistantText;
use super::ParentAssistantTextRef;

pub(super) fn preview_only_action(question: &ParentAssistantText) -> ParentAssistantActionPreview {
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
