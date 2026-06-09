use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceKind, ActivityEvidenceRef, AgentCommandEnvelope, AgentEventEnvelope,
    AgentEventName, AppGameServiceReadModel, AppGameTimerParentSurfaceReadModel,
    AppGameTimerParentSurfaceRow, EnforcementActiveTimerState, LogFieldValue, LogFields, LogLevel,
    APP_GAME_PRODUCT_NATIVE_GAME, APP_GAME_SCHEMA_VERSION,
    APP_GAME_TIMER_PARENT_SURFACE_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_COMPILER_DECISION,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_RUNTIME_MANUAL_REQUIRED,
    APP_GAME_TIMER_PARENT_SURFACE_STATUS_NO_ROWS, APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL,
    APP_GAME_TIMER_PARENT_SURFACE_STATUS_READY, APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP,
    APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
};

use crate::{
    activity_surface_store::load_app_game_model,
    enforcement_timer_state_file::read_active_timer_state,
    enforcement_timer_state_path::enforcement_timer_state_path, event_builder::build_event,
    fields::fields_from_pairs,
};

use super::activity_store_error_event;
use super::app_game_timer_parent_surface_action_results::timer_parent_surface_control_action_results;

pub async fn build_activity_app_game_timer_parent_surface_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_app_game_model().await {
        Some(model) => {
            let timer_state = read_active_timer_state(&enforcement_timer_state_path())
                .await
                .ok()
                .flatten();
            let read_model = app_game_timer_parent_surface_from_service_model_with_timer_state(
                model,
                timer_state.as_ref(),
            );
            build_event(
                constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL_REPORTED,
                &command.message_id,
                command.source,
                AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported,
                LogLevel::Info,
                app_game_timer_parent_surface_payload(&read_model),
                None,
            )
        }
        None => activity_store_error_event(
            command,
            constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL_REPORTED,
            AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported,
        ),
    }
}

pub fn app_game_timer_parent_surface_from_service_model_with_timer_state(
    model: AppGameServiceReadModel,
    active_timer_state: Option<&EnforcementActiveTimerState>,
) -> AppGameTimerParentSurfaceReadModel {
    let rows = timer_parent_surface_rows(&model);
    let row_counts = timer_parent_surface_row_counts(&rows);
    let runtime_claims = timer_parent_surface_runtime_claims(active_timer_state);
    let control_action_results = timer_parent_surface_control_action_results(&model);
    let control_action_result_count = control_action_results.reference_ids.len() as u64;

    AppGameTimerParentSurfaceReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: model.generated_at,
        custody_label: APP_GAME_TIMER_PARENT_SURFACE_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status: timer_parent_surface_status(row_counts.returned, row_counts.ready),
        returned: row_counts.returned,
        ready_for_parent_surface_count: row_counts.ready,
        blocked_by_source_freshness_count: row_counts.blocked_by_source_freshness,
        blocked_by_compiler_decision_count: row_counts.blocked_by_compiler_decision,
        runtime_manual_required_count: row_counts.runtime_manual_required,
        control_action_result_count,
        control_action_result_reference_ids: control_action_results.reference_ids,
        control_action_result_statuses: control_action_results.statuses,
        control_action_result_capability_states: control_action_results.capability_states,
        control_action_result_enforcement_statuses: control_action_results.enforcement_statuses,
        child_facing_reason_reference_ids: control_action_results.child_reason_reference_ids,
        child_facing_status_reference_ids: control_action_results.child_status_reference_ids,
        child_ux_handoff_ready_count: control_action_results.child_ux_handoff_ready_count,
        child_ux_handoff_blocked_count: control_action_results.child_ux_handoff_blocked_count,
        child_ux_handoff_reference_ids: control_action_results.child_ux_handoff_reference_ids,
        child_ux_local_handoff_artifact_record_count: control_action_results
            .child_ux_local_handoff_artifact_record_count,
        child_ux_local_handoff_artifact_skipped_count: control_action_results
            .child_ux_local_handoff_artifact_skipped_count,
        child_ux_local_handoff_artifact_reference_ids: control_action_results
            .child_ux_local_handoff_artifact_reference_ids,
        child_ux_local_handoff_artifact_records: control_action_results
            .child_ux_local_handoff_artifact_records,
        child_ux_parent_surface_intent_manual_action_required_count: control_action_results
            .child_ux_parent_surface_intent_manual_action_required_count,
        child_ux_parent_surface_intent_unavailable_visible_count: control_action_results
            .child_ux_parent_surface_intent_unavailable_visible_count,
        child_ux_parent_surface_intent_history_visible_count: control_action_results
            .child_ux_parent_surface_intent_history_visible_count,
        child_ux_parent_surface_intent_preference_setup_required_count: control_action_results
            .child_ux_parent_surface_intent_preference_setup_required_count,
        child_ux_parent_surface_intent_reference_ids: control_action_results
            .child_ux_parent_surface_intent_reference_ids,
        child_ux_parent_surface_intent_records: control_action_results
            .child_ux_parent_surface_intent_records,
        child_ux_parent_preference_setup_draft_ready_count: control_action_results
            .child_ux_parent_preference_setup_draft_ready_count,
        child_ux_parent_preference_setup_unavailable_visible_count: control_action_results
            .child_ux_parent_preference_setup_unavailable_visible_count,
        child_ux_parent_preference_setup_reference_ids: control_action_results
            .child_ux_parent_preference_setup_reference_ids,
        child_ux_parent_preference_setup_request_ready_count: control_action_results
            .child_ux_parent_preference_setup_request_ready_count,
        child_ux_parent_preference_setup_request_unavailable_visible_count: control_action_results
            .child_ux_parent_preference_setup_request_unavailable_visible_count,
        child_ux_parent_preference_setup_request_reference_ids: control_action_results
            .child_ux_parent_preference_setup_request_reference_ids,
        child_ux_parent_preference_setup_records: control_action_results
            .child_ux_parent_preference_setup_records,
        timer_runtime_claimed: runtime_claims.active_timer_state_exists,
        scheduler_persistence_claimed: runtime_claims.active_timer_state_exists,
        durable_scheduler_storage_claimed: runtime_claims.active_timer_state_exists,
        audit_runtime_claimed: runtime_claims.audit_runtime_claimed,
        rollback_runtime_claimed: runtime_claims.rollback_runtime_claimed,
        adapter_dispatch_claimed: control_action_results.adapter_dispatch_claimed,
        child_delivery_claimed: false,
        platform_enforcement_claimed: control_action_results.platform_enforcement_claimed,
        raw_private_source_rows_included: false,
        rows,
    }
}

struct TimerParentSurfaceRuntimeClaims {
    active_timer_state_exists: bool,
    audit_runtime_claimed: bool,
    rollback_runtime_claimed: bool,
}

fn timer_parent_surface_runtime_claims(
    active_timer_state: Option<&EnforcementActiveTimerState>,
) -> TimerParentSurfaceRuntimeClaims {
    TimerParentSurfaceRuntimeClaims {
        active_timer_state_exists: active_timer_state.is_some(),
        audit_runtime_claimed: active_timer_state
            .and_then(|state| state.audit_event.journal_sequence.as_ref())
            .is_some(),
        rollback_runtime_claimed: active_timer_state
            .and_then(|state| state.action.rollback_token.as_ref())
            .or_else(|| active_timer_state.and_then(|state| state.result.rollback_token.as_ref()))
            .or_else(|| {
                active_timer_state.and_then(|state| state.timer_event.rollback_token.as_ref())
            })
            .is_some(),
    }
}

struct TimerParentSurfaceRowCounts {
    returned: u64,
    ready: u64,
    blocked_by_source_freshness: u64,
    blocked_by_compiler_decision: u64,
    runtime_manual_required: u64,
}

fn timer_parent_surface_row_counts(
    rows: &[AppGameTimerParentSurfaceRow],
) -> TimerParentSurfaceRowCounts {
    TimerParentSurfaceRowCounts {
        returned: rows.len() as u64,
        ready: count_rows_with_state(
            rows,
            APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE,
        ),
        blocked_by_source_freshness: count_rows_with_state(
            rows,
            APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS,
        ),
        blocked_by_compiler_decision: count_rows_with_state(
            rows,
            APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_COMPILER_DECISION,
        ),
        runtime_manual_required: count_rows_with_state(
            rows,
            APP_GAME_TIMER_PARENT_SURFACE_STATE_RUNTIME_MANUAL_REQUIRED,
        ),
    }
}

pub fn app_game_timer_parent_surface_payload(
    read_model: &AppGameTimerParentSurfaceReadModel,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(read_model.custody_label.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(read_model.capability_status.clone()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned as f64),
        ),
        (
            constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL,
            LogFieldValue::String(
                serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ])
}

fn timer_parent_surface_rows(model: &AppGameServiceReadModel) -> Vec<AppGameTimerParentSurfaceRow> {
    let mut rows = Vec::new();
    let policy_evidence = policy_evidence_refs(model);
    let platform_evidence = platform_authority_row_refs(model);
    let approval_evidence = approval_authority_refs(model);

    for identity in &model.identity_rows {
        let mut evidence = identity.evidence.clone();
        push_local_db_row_evidence(&mut evidence, &identity.identity_id);
        push_evidence(&mut evidence, policy_evidence.clone());
        push_evidence(&mut evidence, platform_evidence.clone());
        push_evidence(&mut evidence, approval_evidence.clone());

        rows.push(timer_parent_surface_row(
            &identity.identity_id,
            if identity.product_kind == APP_GAME_PRODUCT_NATIVE_GAME {
                APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME
            } else {
                APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP
            },
            timer_surface_state(model),
            evidence.len() as u64,
            evidence,
        ));
    }

    if rows.is_empty() && !model.evidence_claim_rows.is_empty() {
        rows.push(timer_parent_surface_row(
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP,
            APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP,
            APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS,
            model.evidence_claim_rows.len() as u64,
            evidence_claim_refs(model),
        ));
    }

    rows
}

fn timer_parent_surface_row(
    row_id: &str,
    target_domain: &'static str,
    timer_surface_state: &'static str,
    row_count: u64,
    evidence: Vec<ActivityEvidenceRef>,
) -> AppGameTimerParentSurfaceRow {
    AppGameTimerParentSurfaceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: row_id.to_string(),
        target_domain: target_domain.to_string(),
        timer_surface_state: timer_surface_state.to_string(),
        row_count,
        evidence_reference_ids: evidence.iter().map(|row| row.evidence_id.clone()).collect(),
        evidence,
    }
}

fn timer_surface_state(model: &AppGameServiceReadModel) -> &'static str {
    if model.evidence_claim_rows.is_empty() {
        APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS
    } else if platform_authority_row_count(model) == 0 {
        APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_COMPILER_DECISION
    } else if model.approval_authority_rows.is_empty() {
        APP_GAME_TIMER_PARENT_SURFACE_STATE_RUNTIME_MANUAL_REQUIRED
    } else {
        APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE
    }
}

fn timer_parent_surface_status(returned: u64, ready_count: u64) -> String {
    if returned == 0 {
        APP_GAME_TIMER_PARENT_SURFACE_STATUS_NO_ROWS.to_string()
    } else if ready_count == returned {
        APP_GAME_TIMER_PARENT_SURFACE_STATUS_READY.to_string()
    } else {
        APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL.to_string()
    }
}

fn policy_evidence_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = evidence_claim_refs(model);
    push_evidence(&mut evidence, identity_refs(model));
    evidence
}

fn evidence_claim_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.evidence_claim_rows {
        push_evidence(&mut evidence, row.evidence.clone());
        push_local_db_row_evidence(&mut evidence, &row.claim_id);
    }
    evidence
}

fn identity_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.identity_rows {
        push_evidence(&mut evidence, row.evidence.clone());
        push_local_db_row_evidence(&mut evidence, &row.identity_id);
    }
    evidence
}

fn approval_authority_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.approval_authority_rows {
        push_local_db_row_evidence(&mut evidence, &row.authority_id);
    }
    evidence
}

fn platform_authority_row_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for matrix in &model.platform_authority_matrices {
        for row in &matrix.rows {
            push_local_db_row_evidence(&mut evidence, &row.row_id);
        }
    }
    evidence
}

fn platform_authority_row_count(model: &AppGameServiceReadModel) -> u64 {
    model
        .platform_authority_matrices
        .iter()
        .map(|matrix| matrix.rows.len() as u64)
        .sum()
}

fn count_rows_with_state(rows: &[AppGameTimerParentSurfaceRow], state: &str) -> u64 {
    rows.iter()
        .filter(|row| row.timer_surface_state == state)
        .count() as u64
}

fn push_evidence(target: &mut Vec<ActivityEvidenceRef>, rows: Vec<ActivityEvidenceRef>) {
    for evidence in rows {
        if target
            .iter()
            .any(|candidate| candidate.evidence_id == evidence.evidence_id)
        {
            continue;
        }
        target.push(evidence);
    }
}

fn push_local_db_row_evidence(target: &mut Vec<ActivityEvidenceRef>, evidence_id: &str) {
    if evidence_id.is_empty() {
        return;
    }
    push_evidence(
        target,
        vec![ActivityEvidenceRef {
            evidence_id: evidence_id.to_string(),
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        }],
    );
}
