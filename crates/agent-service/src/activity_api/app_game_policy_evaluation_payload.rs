use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceRef, AppGamePolicyEvaluationReadModel, AppGamePolicyEvaluationRow,
    AppGamePolicyReadinessReadModel, AppGamePolicyReadinessRow, AppGameServiceReadModel,
    LogFieldValue, LogFields, APP_GAME_POLICY_EVALUATION_ADAPTER_NOT_DISPATCHED,
    APP_GAME_POLICY_EVALUATION_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    APP_GAME_POLICY_EVALUATION_DECISION_DRY_RUN_READY,
    APP_GAME_POLICY_EVALUATION_DECISION_MANUAL_REQUIRED,
    APP_GAME_POLICY_EVALUATION_HANDOFF_DISABLED, APP_GAME_POLICY_EVALUATION_KIND_APPROVAL_REQUEST,
    APP_GAME_POLICY_EVALUATION_KIND_BLOCK_LAUNCH,
    APP_GAME_POLICY_EVALUATION_KIND_CATEGORY_RISK_REVIEW,
    APP_GAME_POLICY_EVALUATION_KIND_TIME_LIMIT,
    APP_GAME_POLICY_EVALUATION_POLICY_ACTION_ASK_PARENT,
    APP_GAME_POLICY_EVALUATION_POLICY_ACTION_BLOCK,
    APP_GAME_POLICY_EVALUATION_POLICY_ACTION_TIME_LIMIT,
    APP_GAME_POLICY_EVALUATION_POLICY_ACTION_WARN,
    APP_GAME_POLICY_EVALUATION_REASON_ADAPTER_DISPATCH_DISABLED,
    APP_GAME_POLICY_EVALUATION_REASON_MANUAL_REQUIRED, APP_GAME_POLICY_EVALUATION_REASON_READY,
    APP_GAME_POLICY_EVALUATION_REJECTION_BLOCK_LAUNCH_MANUAL_REQUIRED,
    APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_APPROVAL_AUTHORITY,
    APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_CLASSIFIER_CONTEXT,
    APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_PLATFORM_AUTHORITY,
    APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_POLICY_EVIDENCE,
    APP_GAME_POLICY_EVALUATION_REJECTION_NONE,
    APP_GAME_POLICY_EVALUATION_REQUESTED_ACTION_ASK_PARENT,
    APP_GAME_POLICY_EVALUATION_REQUESTED_ACTION_BLOCK_LAUNCH,
    APP_GAME_POLICY_EVALUATION_REQUESTED_ACTION_TIME_LIMIT,
    APP_GAME_POLICY_EVALUATION_REQUESTED_ACTION_WARN,
    APP_GAME_POLICY_EVALUATION_STATUS_MANUAL_REQUIRED, APP_GAME_POLICY_EVALUATION_STATUS_NO_ROWS,
    APP_GAME_POLICY_EVALUATION_STATUS_READY, APP_GAME_POLICY_READINESS_KIND_AI_CLASSIFIER_CONTEXT,
    APP_GAME_POLICY_READINESS_KIND_APPROVAL_AUTHORITY,
    APP_GAME_POLICY_READINESS_KIND_PLATFORM_AUTHORITY,
    APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE, APP_GAME_POLICY_READINESS_STATE_READY,
    APP_GAME_SCHEMA_VERSION,
};

use super::app_game_policy_readiness_payload::app_game_policy_readiness_from_service_model;
use crate::fields::fields_from_pairs;

type FieldPair = (&'static str, LogFieldValue);

pub fn app_game_policy_evaluation_from_service_model(
    model: AppGameServiceReadModel,
) -> AppGamePolicyEvaluationReadModel {
    let readiness = app_game_policy_readiness_from_service_model(model);
    app_game_policy_evaluation_from_readiness(readiness)
}

pub fn app_game_policy_evaluation_payload(
    read_model: &AppGamePolicyEvaluationReadModel,
) -> LogFields {
    fields_from_pairs(read_model_pairs(read_model))
}

fn app_game_policy_evaluation_from_readiness(
    readiness: AppGamePolicyReadinessReadModel,
) -> AppGamePolicyEvaluationReadModel {
    let rows = evaluation_rows(&readiness);
    let returned = rows.len() as u64;
    let policy_evaluation_ready = rows
        .iter()
        .any(|row| row.decision_state == APP_GAME_POLICY_EVALUATION_DECISION_DRY_RUN_READY);
    let manual_review_required = rows
        .iter()
        .any(|row| row.decision_state == APP_GAME_POLICY_EVALUATION_DECISION_MANUAL_REQUIRED);
    let capability_status = if app_game_boundary_row_count(&readiness) == 0 {
        APP_GAME_POLICY_EVALUATION_STATUS_NO_ROWS
    } else if policy_evaluation_ready {
        APP_GAME_POLICY_EVALUATION_STATUS_READY
    } else {
        APP_GAME_POLICY_EVALUATION_STATUS_MANUAL_REQUIRED
    };

    AppGamePolicyEvaluationReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: readiness.generated_at,
        custody_label: APP_GAME_POLICY_EVALUATION_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status: capability_status.to_string(),
        returned,
        policy_evaluation_ready,
        manual_review_required,
        dry_run: true,
        enforcement_handoff_state: APP_GAME_POLICY_EVALUATION_HANDOFF_DISABLED.to_string(),
        adapter_dispatch_claimed: false,
        readiness_row_count: readiness.rows.len() as u64,
        evaluated_row_count: returned,
        evidence_claim_row_count: readiness.evidence_claim_row_count,
        identity_row_count: readiness.identity_row_count,
        approval_authority_row_count: readiness.approval_authority_row_count,
        approval_action_result_row_count: readiness.approval_action_result_row_count,
        platform_authority_row_count: readiness.platform_authority_row_count,
        ai_classifier_result_row_count: readiness.ai_classifier_result_row_count,
        rows,
    }
}

fn read_model_pairs(read_model: &AppGamePolicyEvaluationReadModel) -> Vec<FieldPair> {
    vec![
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
            constants::field::APP_GAME_POLICY_EVALUATION_READ_MODEL,
            LogFieldValue::String(
                serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ]
}

fn evaluation_rows(readiness: &AppGamePolicyReadinessReadModel) -> Vec<AppGamePolicyEvaluationRow> {
    vec![
        evaluation_row(
            readiness,
            APP_GAME_POLICY_EVALUATION_KIND_TIME_LIMIT,
            APP_GAME_POLICY_EVALUATION_REQUESTED_ACTION_TIME_LIMIT,
            APP_GAME_POLICY_EVALUATION_POLICY_ACTION_TIME_LIMIT,
            &[
                APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE,
                APP_GAME_POLICY_READINESS_KIND_APPROVAL_AUTHORITY,
                APP_GAME_POLICY_READINESS_KIND_PLATFORM_AUTHORITY,
            ],
            None,
        ),
        evaluation_row(
            readiness,
            APP_GAME_POLICY_EVALUATION_KIND_APPROVAL_REQUEST,
            APP_GAME_POLICY_EVALUATION_REQUESTED_ACTION_ASK_PARENT,
            APP_GAME_POLICY_EVALUATION_POLICY_ACTION_ASK_PARENT,
            &[
                APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE,
                APP_GAME_POLICY_READINESS_KIND_APPROVAL_AUTHORITY,
            ],
            None,
        ),
        evaluation_row(
            readiness,
            APP_GAME_POLICY_EVALUATION_KIND_CATEGORY_RISK_REVIEW,
            APP_GAME_POLICY_EVALUATION_REQUESTED_ACTION_WARN,
            APP_GAME_POLICY_EVALUATION_POLICY_ACTION_WARN,
            &[
                APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE,
                APP_GAME_POLICY_READINESS_KIND_AI_CLASSIFIER_CONTEXT,
            ],
            None,
        ),
        evaluation_row(
            readiness,
            APP_GAME_POLICY_EVALUATION_KIND_BLOCK_LAUNCH,
            APP_GAME_POLICY_EVALUATION_REQUESTED_ACTION_BLOCK_LAUNCH,
            APP_GAME_POLICY_EVALUATION_POLICY_ACTION_BLOCK,
            &[
                APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE,
                APP_GAME_POLICY_READINESS_KIND_PLATFORM_AUTHORITY,
            ],
            Some(APP_GAME_POLICY_EVALUATION_REJECTION_BLOCK_LAUNCH_MANUAL_REQUIRED),
        ),
    ]
}

fn evaluation_row(
    readiness: &AppGamePolicyReadinessReadModel,
    evaluation_kind: &'static str,
    requested_action: &'static str,
    policy_action: &'static str,
    required_kinds: &[&'static str],
    forced_rejection: Option<&'static str>,
) -> AppGamePolicyEvaluationRow {
    let rejection_reason =
        forced_rejection.unwrap_or_else(|| missing_rejection(readiness, required_kinds));
    let dry_run_ready = rejection_reason == APP_GAME_POLICY_EVALUATION_REJECTION_NONE;
    let evidence = collect_evidence(readiness, required_kinds);

    AppGamePolicyEvaluationRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        evaluation_id: evaluation_kind.to_string(),
        evaluation_kind: evaluation_kind.to_string(),
        requested_action: requested_action.to_string(),
        policy_action: policy_action.to_string(),
        decision_state: if dry_run_ready {
            APP_GAME_POLICY_EVALUATION_DECISION_DRY_RUN_READY
        } else {
            APP_GAME_POLICY_EVALUATION_DECISION_MANUAL_REQUIRED
        }
        .to_string(),
        rejection_reason: rejection_reason.to_string(),
        reason_codes: if dry_run_ready {
            vec![
                APP_GAME_POLICY_EVALUATION_REASON_READY.to_string(),
                APP_GAME_POLICY_EVALUATION_REASON_ADAPTER_DISPATCH_DISABLED.to_string(),
            ]
        } else {
            vec![
                APP_GAME_POLICY_EVALUATION_REASON_MANUAL_REQUIRED.to_string(),
                rejection_reason.to_string(),
                APP_GAME_POLICY_EVALUATION_REASON_ADAPTER_DISPATCH_DISABLED.to_string(),
            ]
        },
        required_readiness_kinds: required_kinds
            .iter()
            .map(|kind| (*kind).to_string())
            .collect(),
        evidence_reference_ids: evidence.iter().map(|row| row.evidence_id.clone()).collect(),
        evidence,
        dry_run: true,
        enforcement_handoff_state: APP_GAME_POLICY_EVALUATION_HANDOFF_DISABLED.to_string(),
        adapter_dispatch_state: APP_GAME_POLICY_EVALUATION_ADAPTER_NOT_DISPATCHED.to_string(),
    }
}

fn missing_rejection(
    readiness: &AppGamePolicyReadinessReadModel,
    required_kinds: &[&'static str],
) -> &'static str {
    for kind in required_kinds {
        if readiness_ready(readiness, kind) {
            continue;
        }
        return match *kind {
            APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE => {
                APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_POLICY_EVIDENCE
            }
            APP_GAME_POLICY_READINESS_KIND_APPROVAL_AUTHORITY => {
                APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_APPROVAL_AUTHORITY
            }
            APP_GAME_POLICY_READINESS_KIND_PLATFORM_AUTHORITY => {
                APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_PLATFORM_AUTHORITY
            }
            APP_GAME_POLICY_READINESS_KIND_AI_CLASSIFIER_CONTEXT => {
                APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_CLASSIFIER_CONTEXT
            }
            _ => APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_POLICY_EVIDENCE,
        };
    }
    APP_GAME_POLICY_EVALUATION_REJECTION_NONE
}

fn readiness_ready(readiness: &AppGamePolicyReadinessReadModel, readiness_kind: &str) -> bool {
    readiness.rows.iter().any(|row| {
        row.readiness_kind == readiness_kind
            && row.readiness_state == APP_GAME_POLICY_READINESS_STATE_READY
    })
}

fn collect_evidence(
    readiness: &AppGamePolicyReadinessReadModel,
    required_kinds: &[&'static str],
) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &readiness.rows {
        if required_kinds
            .iter()
            .any(|kind| row.readiness_kind == *kind)
        {
            push_evidence(&mut evidence, row);
        }
    }
    evidence
}

fn push_evidence(target: &mut Vec<ActivityEvidenceRef>, row: &AppGamePolicyReadinessRow) {
    for evidence in &row.evidence {
        if target
            .iter()
            .any(|candidate| candidate.evidence_id == evidence.evidence_id)
        {
            continue;
        }
        target.push(evidence.clone());
    }
}

fn app_game_boundary_row_count(readiness: &AppGamePolicyReadinessReadModel) -> u64 {
    readiness.evidence_claim_row_count
        + readiness.identity_row_count
        + readiness.approval_authority_row_count
        + readiness.approval_action_result_row_count
        + readiness.platform_authority_row_count
        + readiness.ai_classifier_result_row_count
}
