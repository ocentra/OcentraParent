use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::{
    APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP,
    APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
};

use crate::app_game_child_ux_outbox_types::AppGameChildUxOutboxInput;
use crate::app_game_child_ux_types::{AppGameChildUxAction, AppGameChildUxNoticeState};

const INVALID_ARTIFACT_FIELD: &str = "app_game.child_ux_outbox.artifact";
const INVALID_CONTEXT_FIELD: &str = "app_game.child_ux_outbox.context";

pub(super) fn validate_input(input: &AppGameChildUxOutboxInput) -> Result<(), EventingError> {
    validate_artifact(input)?;
    validate_context(input)?;
    validate_bound_refs(input)
}

pub(super) fn is_deliverable(state: AppGameChildUxNoticeState) -> bool {
    !matches!(
        state,
        AppGameChildUxNoticeState::NoNotice
            | AppGameChildUxNoticeState::ManualRequired
            | AppGameChildUxNoticeState::Unavailable
    )
}

fn validate_artifact(input: &AppGameChildUxOutboxInput) -> Result<(), EventingError> {
    let artifact = &input.artifact;
    let invalid_claim = artifact.child_delivery_claimed
        || artifact.notification_delivery_claimed
        || artifact.adapter_dispatch_claimed
        || artifact.platform_enforcement_claimed
        || artifact.raw_private_source_rows_included
        || input.notice.adapter_dispatch_claimed;
    let invalid_identity = artifact.schema_version == 0
        || artifact.artifact_reference_id.trim().is_empty()
        || artifact.source_result_id.trim().is_empty()
        || !matches!(
            artifact.target_domain.as_str(),
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP
                | APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME
        );
    if invalid_claim || invalid_identity {
        return Err(invalid(
            INVALID_ARTIFACT_FIELD,
            &artifact.artifact_reference_id,
        ));
    }
    Ok(())
}

fn validate_context(input: &AppGameChildUxOutboxInput) -> Result<(), EventingError> {
    if context_has_empty_refs(input) {
        return Err(invalid(INVALID_CONTEXT_FIELD, input.entry_id.as_str()));
    }
    Ok(())
}

fn context_has_empty_refs(input: &AppGameChildUxOutboxInput) -> bool {
    input.entry_id.as_str().trim().is_empty()
        || input.alert_ref.as_str().trim().is_empty()
        || input.family.family_id.trim().is_empty()
        || input.device.device_id.as_str().trim().is_empty()
        || input.parent_action.action_reference_id.trim().is_empty()
        || input.parent_action.actor.actor_id.trim().is_empty()
        || input.parent_action.policy_version.trim().is_empty()
        || input.parent_action.created_at.trim().is_empty()
        || input.observed_at.as_str().trim().is_empty()
        || input.outbox_file_ref.as_str().trim().is_empty()
        || input.local_data_path_ref.as_str().trim().is_empty()
        || input.audit_refs.is_empty()
        || input
            .audit_refs
            .iter()
            .any(|reference| reference.as_str().trim().is_empty())
}

fn validate_bound_refs(input: &AppGameChildUxOutboxInput) -> Result<(), EventingError> {
    let reason_refs = input
        .notice
        .child_reason_refs
        .iter()
        .map(|reference| reference.as_str())
        .collect::<Vec<_>>();
    let status_refs = input
        .notice
        .child_status_refs
        .iter()
        .map(|reference| reference.as_str())
        .collect::<Vec<_>>();
    let artifact_reason_refs = input
        .artifact
        .child_reason_reference_ids
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let artifact_status_refs = input
        .artifact
        .child_status_reference_ids
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let ask_parent_missing_refs = input.notice.action == AppGameChildUxAction::AskParent
        && (reason_refs.is_empty() || status_refs.is_empty());
    let deliverable_missing_evidence =
        is_deliverable(input.notice.state) && input.notice.evidence_refs.is_empty();
    if ask_parent_missing_refs
        || reason_refs != artifact_reason_refs
        || status_refs != artifact_status_refs
        || deliverable_missing_evidence
    {
        return Err(invalid(
            INVALID_ARTIFACT_FIELD,
            &input.artifact.artifact_reference_id,
        ));
    }
    Ok(())
}

fn invalid(field: &'static str, value: &str) -> EventingError {
    EventingError::InvalidValue {
        field,
        value: value.to_owned(),
    }
}
