use std::collections::BTreeSet;

use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::{
    AppGameServiceReadModel, APP_GAME_PRODUCT_NATIVE_APP, APP_GAME_PRODUCT_NATIVE_GAME,
    APP_GAME_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};
use ocentra_parent_agent_protocol::AppGameTimerParentSurfaceReadModel;
use ocentra_parent_agent_protocol::AppGameTimerParentSurfaceRow;
use ocentra_parent_agent_protocol::APP_GAME_TIMER_PARENT_SURFACE_CUSTODY_CHILD_DEVICE_QUERY_STORE;
use ocentra_parent_agent_protocol::APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_COMPILER_DECISION;
use ocentra_parent_agent_protocol::APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS;
use ocentra_parent_agent_protocol::APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE;
use ocentra_parent_agent_protocol::APP_GAME_TIMER_PARENT_SURFACE_STATE_RUNTIME_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::APP_GAME_TIMER_PARENT_SURFACE_STATUS_NO_ROWS;
use ocentra_parent_agent_protocol::APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL;
use ocentra_parent_agent_protocol::APP_GAME_TIMER_PARENT_SURFACE_STATUS_READY;
use ocentra_parent_agent_protocol::APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP;
use ocentra_parent_agent_protocol::APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME;

use crate::{
    activity_surface_store::load_app_game_model,
    enforcement_timer_state_file::read_active_timer_state,
    enforcement_timer_state_path::enforcement_timer_state_path, event_builder::build_event,
    fields::fields_from_pairs,
};

use super::activity_store_error_event::activity_store_error_event;
use super::app_game_timer_parent_surface_action_results::apply_timer_parent_surface_control_action_results;

struct TimerParentSurfaceRowSpec {
    row_id: String,
    target_domain: &'static str,
    timer_surface_state_index: usize,
    row_count: u64,
    evidence: Vec<ActivityEvidenceRef>,
}

const TIMER_PARENT_SURFACE_STATE_TEXTS: [&str; 4] = [
    APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_COMPILER_DECISION,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_RUNTIME_MANUAL_REQUIRED,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE,
];

const TIMER_PARENT_SURFACE_STATUS_TEXTS: [&str; 3] = [
    APP_GAME_TIMER_PARENT_SURFACE_STATUS_NO_ROWS,
    APP_GAME_TIMER_PARENT_SURFACE_STATUS_READY,
    APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL,
];

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
                &model,
                timer_state.as_ref(),
            );
            match app_game_timer_parent_surface_payload_result(&read_model) {
                Ok(payload) => build_event(
                    constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL_REPORTED,
                    &command.message_id,
                    command.source,
                    AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported,
                    LogLevel::Info,
                    payload,
                    None,
                ),
                Err(_error) => timer_parent_surface_serialization_error_event(command),
            }
        }
        None => activity_store_error_event(
            command,
            crate::activity_api::ActivityEventId(
                constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL_REPORTED,
            ),
            AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported,
        ),
    }
}

pub fn app_game_timer_parent_surface_from_service_model_with_timer_state(
    model: &AppGameServiceReadModel,
    active_timer_state: Option<&EnforcementActiveTimerState>,
) -> AppGameTimerParentSurfaceReadModel {
    let rows = timer_parent_surface_rows(model);
    let runtime_claims = TimerParentSurfaceRuntimeClaims {
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
    };
    let mut read_model = timer_parent_surface_read_model(model, rows, &runtime_claims);
    apply_timer_parent_surface_control_action_results(model, &mut read_model);
    // Action-result evidence can be surfaced as read-only handoff detail, but
    // this service read model does not own adapter dispatch or platform
    // enforcement. Keep those top-level ownership claims fail closed.
    read_model.adapter_dispatch_claimed = false;
    read_model.platform_enforcement_claimed = false;
    read_model
}

struct TimerParentSurfaceRuntimeClaims {
    audit_runtime_claimed: bool,
    rollback_runtime_claimed: bool,
}

pub fn app_game_timer_parent_surface_payload(
    read_model: &AppGameTimerParentSurfaceReadModel,
) -> LogFields {
    app_game_timer_parent_surface_payload_result(read_model).unwrap_or_else(|_error| {
        fields_from_pairs(vec![(
            constants::field::REASON,
            LogFieldValue::String(TIMER_PARENT_SURFACE_SERIALIZATION_ERROR.to_string()),
        )])
    })
}

fn app_game_timer_parent_surface_payload_result(
    read_model: &AppGameTimerParentSurfaceReadModel,
) -> Result<LogFields, serde_json::Error> {
    let serialized_read_model = serde_json::to_string(read_model)?;
    Ok(fields_from_pairs(vec![
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
            LogFieldValue::String(serialized_read_model),
        ),
    ]))
}

const TIMER_PARENT_SURFACE_SERIALIZATION_ERROR: &str =
    "app-game timer parent surface read model serialization failed";

fn timer_parent_surface_serialization_error_event(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_event(
        constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported,
        LogLevel::Error,
        fields_from_pairs(vec![(
            constants::field::REASON,
            LogFieldValue::String(TIMER_PARENT_SURFACE_SERIALIZATION_ERROR.to_string()),
        )]),
        None,
    )
}

fn timer_parent_surface_rows(model: &AppGameServiceReadModel) -> Vec<AppGameTimerParentSurfaceRow> {
    let policy_evidence = policy_evidence_refs(model);
    let platform_evidence = platform_authority_row_refs(model);
    let approval_evidence = approval_authority_refs(model);
    let timer_surface_state_index = timer_surface_state_index(model);
    let rows: Vec<_> = model
        .identity_rows
        .iter()
        .filter_map(|identity| {
            let target_domain = timer_parent_surface_target_domain(&identity.product_kind)?;
            let mut evidence = identity.evidence.clone();
            push_evidence(
                &mut evidence,
                vec![ActivityEvidenceRef {
                    evidence_id: identity.identity_id.clone(),
                    kind: ActivityEvidenceKind::LocalDbRow,
                    digest: None,
                    uri: None,
                }],
            );
            push_evidence(&mut evidence, policy_evidence.clone());
            push_evidence(&mut evidence, platform_evidence.clone());
            push_evidence(&mut evidence, approval_evidence.clone());

            timer_parent_surface_row(TimerParentSurfaceRowSpec {
                row_id: identity.identity_id.clone(),
                target_domain,
                timer_surface_state_index,
                row_count: evidence.len() as u64,
                evidence,
            })
        })
        .collect();
    let has_rows = !rows.is_empty();
    let has_identity_rows = !model.identity_rows.is_empty();

    rows.into_iter()
        .chain(
            (!has_rows && !has_identity_rows && !model.evidence_claim_rows.is_empty()).then(|| {
                timer_parent_surface_row(TimerParentSurfaceRowSpec {
                    row_id: APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP.to_string(),
                    target_domain: APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP,
                    timer_surface_state_index: 0,
                    row_count: model.evidence_claim_rows.len() as u64,
                    evidence: evidence_claim_refs(model),
                })
            }),
        )
        .collect()
}

fn timer_parent_surface_target_domain(product_kind: &str) -> Option<&'static str> {
    match product_kind {
        APP_GAME_PRODUCT_NATIVE_APP => Some(APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP),
        APP_GAME_PRODUCT_NATIVE_GAME => Some(APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME),
        _ => None,
    }
}

struct TimerParentSurfaceRowCounts {
    returned: u64,
    ready_for_parent_surface_count: u64,
    blocked_by_source_freshness_count: u64,
    blocked_by_compiler_decision_count: u64,
    runtime_manual_required_count: u64,
}

fn timer_parent_surface_row_counts(
    rows: &[AppGameTimerParentSurfaceRow],
) -> TimerParentSurfaceRowCounts {
    TimerParentSurfaceRowCounts {
        returned: rows.len() as u64,
        ready_for_parent_surface_count: rows
            .iter()
            .filter(|row| {
                row.timer_surface_state
                    == APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE
            })
            .count() as u64,
        blocked_by_source_freshness_count: rows
            .iter()
            .filter(|row| {
                row.timer_surface_state
                    == APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS
            })
            .count() as u64,
        blocked_by_compiler_decision_count: rows
            .iter()
            .filter(|row| {
                row.timer_surface_state
                    == APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_COMPILER_DECISION
            })
            .count() as u64,
        runtime_manual_required_count: rows
            .iter()
            .filter(|row| {
                row.timer_surface_state
                    == APP_GAME_TIMER_PARENT_SURFACE_STATE_RUNTIME_MANUAL_REQUIRED
            })
            .count() as u64,
    }
}

fn timer_parent_surface_status_text_index(counts: &TimerParentSurfaceRowCounts) -> usize {
    {
        let has_rows = (counts.returned != 0) as usize;
        let ready_is_full = (counts.ready_for_parent_surface_count == counts.returned) as usize;
        has_rows * ready_is_full + has_rows * (1 - ready_is_full) * 2
    }
}

fn timer_parent_surface_read_model(
    model: &AppGameServiceReadModel,
    rows: Vec<AppGameTimerParentSurfaceRow>,
    runtime_claims: &TimerParentSurfaceRuntimeClaims,
) -> AppGameTimerParentSurfaceReadModel {
    let counts = timer_parent_surface_row_counts(&rows);
    let capability_status = TIMER_PARENT_SURFACE_STATUS_TEXTS
        [timer_parent_surface_status_text_index(&counts)]
    .to_string();

    AppGameTimerParentSurfaceReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: model.generated_at.clone(),
        custody_label: APP_GAME_TIMER_PARENT_SURFACE_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status,
        returned: counts.returned,
        ready_for_parent_surface_count: counts.ready_for_parent_surface_count,
        blocked_by_source_freshness_count: counts.blocked_by_source_freshness_count,
        blocked_by_compiler_decision_count: counts.blocked_by_compiler_decision_count,
        runtime_manual_required_count: counts.runtime_manual_required_count,
        control_action_result_count: 0,
        control_action_result_reference_ids: Vec::new(),
        control_action_result_statuses: Vec::new(),
        control_action_result_capability_states: Vec::new(),
        control_action_result_enforcement_statuses: Vec::new(),
        child_facing_reason_reference_ids: Vec::new(),
        child_facing_status_reference_ids: Vec::new(),
        child_ux_handoff_ready_count: 0,
        child_ux_handoff_blocked_count: 0,
        child_ux_handoff_reference_ids: Vec::new(),
        child_ux_local_handoff_artifact_record_count: 0,
        child_ux_local_handoff_artifact_skipped_count: 0,
        child_ux_local_handoff_artifact_reference_ids: Vec::new(),
        child_ux_local_handoff_artifact_records: Vec::new(),
        child_ux_parent_surface_intent_manual_action_required_count: 0,
        child_ux_parent_surface_intent_unavailable_visible_count: 0,
        child_ux_parent_surface_intent_history_visible_count: 0,
        child_ux_parent_surface_intent_preference_setup_required_count: 0,
        child_ux_parent_surface_intent_reference_ids: Vec::new(),
        child_ux_parent_surface_intent_records: Vec::new(),
        child_ux_parent_preference_setup_draft_ready_count: 0,
        child_ux_parent_preference_setup_unavailable_visible_count: 0,
        child_ux_parent_preference_setup_reference_ids: Vec::new(),
        child_ux_parent_preference_setup_request_ready_count: 0,
        child_ux_parent_preference_setup_request_unavailable_visible_count: 0,
        child_ux_parent_preference_setup_request_reference_ids: Vec::new(),
        child_ux_parent_preference_setup_records: Vec::new(),
        // A persisted active-state file proves state visibility only. It does
        // not prove a running timer service, scheduler ownership, or durable
        // scheduler storage/event handoff.
        timer_runtime_claimed: false,
        scheduler_persistence_claimed: false,
        durable_scheduler_storage_claimed: false,
        audit_runtime_claimed: runtime_claims.audit_runtime_claimed,
        rollback_runtime_claimed: runtime_claims.rollback_runtime_claimed,
        adapter_dispatch_claimed: false,
        child_delivery_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
        rows,
    }
}

fn timer_parent_surface_row(spec: TimerParentSurfaceRowSpec) -> AppGameTimerParentSurfaceRow {
    let TimerParentSurfaceRowSpec {
        row_id,
        target_domain,
        timer_surface_state_index,
        row_count,
        evidence,
    } = spec;
    let evidence_reference_ids = evidence.iter().map(|row| row.evidence_id.clone()).collect();
    AppGameTimerParentSurfaceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id,
        target_domain: target_domain.to_string(),
        timer_surface_state: TIMER_PARENT_SURFACE_STATE_TEXTS[timer_surface_state_index]
            .to_string(),
        row_count,
        evidence_reference_ids,
        evidence,
    }
}

fn timer_surface_state_index(model: &AppGameServiceReadModel) -> usize {
    let evidence_missing = model.evidence_claim_rows.is_empty() as usize;
    let platform_missing = (platform_authority_row_count(model) == 0) as usize;
    let approval_missing = model.approval_authority_rows.is_empty() as usize;
    (1 - evidence_missing) * platform_missing
        + (1 - evidence_missing) * (1 - platform_missing) * approval_missing * 2
        + (1 - evidence_missing) * (1 - platform_missing) * (1 - approval_missing) * 3
}

fn policy_evidence_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = evidence_claim_refs(model);
    push_evidence(&mut evidence, identity_refs(model));
    evidence
}

fn evidence_claim_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    model
        .evidence_claim_rows
        .iter()
        .flat_map(|row| {
            let mut refs = row.evidence.clone();
            refs.push(ActivityEvidenceRef {
                evidence_id: row.claim_id.clone(),
                kind: ActivityEvidenceKind::LocalDbRow,
                digest: None,
                uri: None,
            });
            refs
        })
        .collect()
}

fn identity_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    model
        .identity_rows
        .iter()
        .flat_map(|row| {
            let mut refs = row.evidence.clone();
            refs.push(ActivityEvidenceRef {
                evidence_id: row.identity_id.clone(),
                kind: ActivityEvidenceKind::LocalDbRow,
                digest: None,
                uri: None,
            });
            refs
        })
        .collect()
}

fn approval_authority_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    model
        .approval_authority_rows
        .iter()
        .map(|row| ActivityEvidenceRef {
            evidence_id: row.authority_id.clone(),
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        })
        .collect()
}

fn platform_authority_row_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    model
        .platform_authority_matrices
        .iter()
        .flat_map(|matrix| matrix.rows.iter())
        .map(|row| ActivityEvidenceRef {
            evidence_id: row.row_id.clone(),
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        })
        .collect()
}

fn platform_authority_row_count(model: &AppGameServiceReadModel) -> u64 {
    model
        .platform_authority_matrices
        .iter()
        .map(|matrix| matrix.rows.len() as u64)
        .sum()
}

fn push_evidence(target: &mut Vec<ActivityEvidenceRef>, rows: Vec<ActivityEvidenceRef>) {
    let mut seen: BTreeSet<String> = target
        .iter()
        .map(|candidate| candidate.evidence_id.clone())
        .collect();
    target.extend(
        rows.into_iter()
            .filter(|evidence| seen.insert(evidence.evidence_id.clone())),
    );
}
