use std::path::Path;

use ocentra_parent_agent_core::{
    activity_store::ActivityStore,
    screen_event_runtime::{
        publish_screen_capture_queue_events_for_input,
        publish_screen_degraded_event_chain_for_input, publish_screen_deletion_event_for_input,
        publish_screen_runtime_chain_for_input, ScreenRuntimeReport,
    },
    screen_event_runtime_input::{
        ScreenRuntimeCaptureInput, ScreenRuntimeDegradedInput, ScreenRuntimeDeletionInput,
        ScreenRuntimeInput,
    },
};
use ocentra_parent_agent_protocol::activity_surface::ActivityScreenReadModelRow;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_DELETION_DELETED;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_DELETION_EXPIRED_DELETED;

use crate::{
    activity_capture::ActivityCaptureError,
    activity_surface_read_models::activity_screen_row_from_result,
    screen_ai_service_event_subscription::{ActionRefText, ObservedAtText},
};

const COMPLETE_SCREEN_RETENTION_BATCH_LIMIT: u64 = i64::MAX as u64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiServiceEventBridgeRefs {
    pub(crate) action_ref: ActionRefText,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiQueueJobId(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScreenAiDeletionState(String);

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
    observed_at: ObservedAtText,
    refs: ScreenAiServiceEventBridgeRefs,
) -> Result<ScreenRuntimeReport, ScreenAiServiceEventBridgeError> {
    let input = screen_runtime_input_from_service_row(row, refs)?;
    publish_screen_runtime_chain_for_input(input, observed_at.0.as_str())
        .await
        .map_err(|_publish_error| ScreenAiServiceEventBridgeError::EventPublishFailed)
}

pub(crate) async fn publish_screen_capture_queue_event_chain(
    row: ActivityScreenReadModelRow,
    observed_at: ObservedAtText,
) -> Result<ScreenRuntimeReport, ScreenAiServiceEventBridgeError> {
    let input = screen_runtime_capture_input_from_service_row(row)?;
    publish_screen_capture_queue_events_for_input(input, observed_at.0.as_str())
        .await
        .map_err(|_publish_error| ScreenAiServiceEventBridgeError::EventPublishFailed)
}

pub(crate) async fn publish_screen_deletion_event_chain(
    row: ActivityScreenReadModelRow,
    observed_at: ObservedAtText,
) -> Result<ScreenRuntimeReport, ScreenAiServiceEventBridgeError> {
    let input = screen_runtime_deletion_input_from_service_row(row)?;
    publish_screen_deletion_event_for_input(input, observed_at.0.as_str())
        .await
        .map_err(|_publish_error| ScreenAiServiceEventBridgeError::EventPublishFailed)
}

pub(crate) async fn publish_screen_degraded_event_chain(
    row: ActivityScreenReadModelRow,
    observed_at: ObservedAtText,
) -> Result<ScreenRuntimeReport, ScreenAiServiceEventBridgeError> {
    let input = screen_runtime_degraded_input_from_service_row(row)?;
    publish_screen_degraded_event_chain_for_input(input, observed_at.0.as_str())
        .await
        .map_err(|_publish_error| ScreenAiServiceEventBridgeError::EventPublishFailed)
}

pub(crate) async fn publish_screen_capture_queue_events_for_queue_job(
    store_path: &Path,
    queue_job_id: ScreenAiQueueJobId,
    observed_at: ObservedAtText,
) -> Result<Option<ScreenRuntimeReport>, ActivityCaptureError> {
    let Some(row) = latest_screen_row_for_queue_job(store_path, &queue_job_id, &observed_at)?
    else {
        return Ok(None);
    };
    Ok(publish_screen_capture_queue_event_chain(row, observed_at)
        .await
        .ok())
}

pub(crate) async fn publish_screen_deletion_event_for_queue_job(
    store_path: &Path,
    queue_job_id: ScreenAiQueueJobId,
    observed_at: ObservedAtText,
) -> Result<Option<ScreenRuntimeReport>, ActivityCaptureError> {
    let Some(row) = latest_screen_row_for_queue_job(store_path, &queue_job_id, &observed_at)?
    else {
        return Ok(None);
    };
    Ok(publish_screen_deletion_event_chain(row, observed_at)
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
    if !deletion_state_is_safe(&ScreenAiDeletionState(row.image_deletion_state.clone())) {
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
        action_ref: refs.action_ref.0,
        deletion_proof_ref,
        portal_read_model_ref: row.row_id,
    })
}

pub(crate) fn screen_runtime_deletion_input_from_service_row(
    row: ActivityScreenReadModelRow,
) -> Result<ScreenRuntimeDeletionInput, ScreenAiServiceEventBridgeError> {
    if row.raw_image_retained {
        return Err(ScreenAiServiceEventBridgeError::RawImageRetained);
    }
    let deletion_proof_ref = row
        .deletion_reasons
        .first()
        .cloned()
        .ok_or(ScreenAiServiceEventBridgeError::MissingDeletionProof)?;
    Ok(ScreenRuntimeDeletionInput {
        queue_job_id: row.queue_job_id,
        screen_analysis_result_id: row.row_id,
        capture_reason: row.capture_reason,
        capture_scope: row.capture_scope,
        image_digest: row.image_digest,
        summary: row.label,
        model_runtime_ref: row.model_runtime_ref,
        model_id: row.model_id,
        prompt_or_template_version: row.prompt_or_template_version,
        deletion_proof_ref,
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

pub(crate) fn screen_runtime_degraded_input_from_service_row(
    row: ActivityScreenReadModelRow,
) -> Result<ScreenRuntimeDegradedInput, ScreenAiServiceEventBridgeError> {
    if row.raw_image_retained {
        return Err(ScreenAiServiceEventBridgeError::RawImageRetained);
    }
    if !deletion_state_is_safe(&ScreenAiDeletionState(row.image_deletion_state.clone())) {
        return Err(ScreenAiServiceEventBridgeError::UnsafeDeletionState);
    }
    let deletion_proof_ref = row
        .deletion_reasons
        .first()
        .cloned()
        .ok_or(ScreenAiServiceEventBridgeError::MissingDeletionProof)?;
    Ok(ScreenRuntimeDegradedInput {
        queue_job_id: row.queue_job_id,
        screen_analysis_result_id: row.row_id.clone(),
        capture_reason: row.capture_reason,
        capture_scope: row.capture_scope,
        image_digest: row.image_digest,
        summary: row.label,
        model_runtime_ref: row.model_runtime_ref,
        model_id: row.model_id,
        prompt_or_template_version: row.prompt_or_template_version,
        deletion_proof_ref,
        portal_read_model_ref: row.row_id,
    })
}

fn latest_screen_row_for_queue_job(
    store_path: &Path,
    queue_job_id: &ScreenAiQueueJobId,
    generated_at: &ObservedAtText,
) -> Result<Option<ActivityScreenReadModelRow>, ActivityCaptureError> {
    let store = ActivityStore::open(store_path)?;
    let summary = store.screen_evidence_recent_summary(
        // Retention publication is a durable outbox drain, not a UI summary.
        // Read the complete retained batch so a job outside the recent-ten
        // window is neither skipped nor acknowledged without publication.
        COMPLETE_SCREEN_RETENTION_BATCH_LIMIT,
        &generated_at.0,
    )?;
    Ok(summary
        .results
        .into_iter()
        .find(|result| result.queue_job_id == queue_job_id.0)
        .map(activity_screen_row_from_result))
}

fn deletion_state_is_safe(state: &ScreenAiDeletionState) -> bool {
    state.0 == SCREEN_DELETION_DELETED || state.0 == SCREEN_DELETION_EXPIRED_DELETED
}
