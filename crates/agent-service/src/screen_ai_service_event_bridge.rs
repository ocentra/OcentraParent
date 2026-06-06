use std::path::Path;

use ocentra_parent_agent_core::{
    ActivityStore, ScreenRuntimeCaptureInput, ScreenRuntimeInput, ScreenRuntimeReport,
};
use ocentra_parent_agent_protocol::{
    constants, ActivityScreenReadModelRow, SCREEN_DELETION_DELETED, SCREEN_DELETION_EXPIRED_DELETED,
};

use crate::{
    activity_capture::ActivityCaptureError,
    activity_surface_read_models::activity_screen_row_from_result,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiServiceEventBridgeRefs {
    pub(crate) action_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ScreenAiServiceEventBridgeError {
    RawImageRetained,
    MissingPolicyDecision,
    MissingPolicyAction,
    MissingParentRule,
    MissingDeletionProof,
    UnsafeDeletionState,
    EventPublishFailed,
}

pub(crate) async fn publish_screen_service_row_event_chain(
    row: ActivityScreenReadModelRow,
    observed_at: &str,
    refs: ScreenAiServiceEventBridgeRefs,
) -> Result<ScreenRuntimeReport, ScreenAiServiceEventBridgeError> {
    let input = screen_runtime_input_from_service_row(row, refs)?;
    ocentra_parent_agent_core::publish_screen_runtime_chain_for_input(input, observed_at)
        .await
        .map_err(|_| ScreenAiServiceEventBridgeError::EventPublishFailed)
}

pub(crate) async fn publish_screen_capture_queue_event_chain(
    row: ActivityScreenReadModelRow,
    observed_at: &str,
) -> Result<ScreenRuntimeReport, ScreenAiServiceEventBridgeError> {
    let input = screen_runtime_capture_input_from_service_row(row)?;
    ocentra_parent_agent_core::publish_screen_capture_queue_events_for_input(input, observed_at)
        .await
        .map_err(|_| ScreenAiServiceEventBridgeError::EventPublishFailed)
}

pub(crate) async fn publish_screen_capture_queue_events_for_queue_job(
    store_path: &Path,
    queue_job_id: &str,
    observed_at: &str,
) -> Result<Option<ScreenRuntimeReport>, ActivityCaptureError> {
    let Some(row) = latest_screen_row_for_queue_job(store_path, queue_job_id, observed_at)? else {
        return Ok(None);
    };
    Ok(publish_screen_capture_queue_event_chain(row, observed_at)
        .await
        .ok())
}

pub(crate) fn screen_runtime_input_from_service_row(
    row: ActivityScreenReadModelRow,
    refs: ScreenAiServiceEventBridgeRefs,
) -> Result<ScreenRuntimeInput, ScreenAiServiceEventBridgeError> {
    if row.raw_image_retained {
        return Err(ScreenAiServiceEventBridgeError::RawImageRetained);
    }
    if !deletion_state_is_safe(&row.image_deletion_state) {
        return Err(ScreenAiServiceEventBridgeError::UnsafeDeletionState);
    }
    let policy_decision_ref = row
        .policy_decision_ref
        .ok_or(ScreenAiServiceEventBridgeError::MissingPolicyDecision)?;
    let policy_action = row
        .policy_action
        .ok_or(ScreenAiServiceEventBridgeError::MissingPolicyAction)?;
    let parent_rule_ref = row
        .parent_rule_refs
        .first()
        .cloned()
        .ok_or(ScreenAiServiceEventBridgeError::MissingParentRule)?;
    let deletion_proof_ref = row
        .deletion_reasons
        .first()
        .cloned()
        .ok_or(ScreenAiServiceEventBridgeError::MissingDeletionProof)?;

    Ok(ScreenRuntimeInput {
        queue_job_id: row.queue_job_id,
        screen_analysis_result_id: row.row_id.clone(),
        capture_reason: row.capture_reason,
        capture_scope: row.capture_scope,
        image_digest: row.image_digest,
        summary: row.label,
        model_runtime_ref: row.model_runtime_ref,
        model_id: row.model_id,
        prompt_or_template_version: row.prompt_or_template_version,
        policy_decision_ref,
        policy_action,
        parent_rule_ref,
        action_ref: refs.action_ref,
        deletion_proof_ref,
        portal_read_model_ref: row.row_id,
    })
}

pub(crate) fn screen_runtime_capture_input_from_service_row(
    row: ActivityScreenReadModelRow,
) -> Result<ScreenRuntimeCaptureInput, ScreenAiServiceEventBridgeError> {
    if row.raw_image_retained {
        return Err(ScreenAiServiceEventBridgeError::RawImageRetained);
    }
    Ok(ScreenRuntimeCaptureInput {
        queue_job_id: row.queue_job_id,
        screen_analysis_result_id: row.row_id,
        capture_reason: row.capture_reason,
        capture_scope: row.capture_scope,
        image_digest: row.image_digest,
        summary: row.label,
        model_runtime_ref: row.model_runtime_ref,
        model_id: row.model_id,
        prompt_or_template_version: row.prompt_or_template_version,
    })
}

fn latest_screen_row_for_queue_job(
    store_path: &Path,
    queue_job_id: &str,
    generated_at: &str,
) -> Result<Option<ActivityScreenReadModelRow>, ActivityCaptureError> {
    let store = ActivityStore::open(store_path)?;
    let summary = store.screen_evidence_recent_summary(
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        generated_at,
    )?;
    Ok(summary
        .results
        .into_iter()
        .find(|result| result.queue_job_id == queue_job_id)
        .map(activity_screen_row_from_result))
}

fn deletion_state_is_safe(state: &str) -> bool {
    state == SCREEN_DELETION_DELETED || state == SCREEN_DELETION_EXPIRED_DELETED
}
