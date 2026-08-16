use ocentra_parent_agent_protocol::app_game::{AppGameServiceReadModel, APP_GAME_SCHEMA_VERSION};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    APP_GAME_CONTROL_ACTION_STATUS_ENFORCED, APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::AppGamePolicyReadinessReadModel;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_STATE_READY;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_STATUS_NO_ROWS;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_STATUS_PARTIAL;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_STATUS_READY;

#[path = "app_game_policy_readiness_payload/rows.rs"]
mod rows;

use super::app_game_policy_readiness_sources::{
    app_game_boundary_row_count, category_candidate_row_count, platform_authority_row_count,
    unknown_review_row_count,
};
use crate::fields::fields_from_pairs;

#[derive(Clone, Copy)]
pub(super) struct PolicyReadinessTextRef<'a>(&'a str);

pub fn app_game_policy_readiness_from_service_model(
    model: AppGameServiceReadModel,
) -> AppGamePolicyReadinessReadModel {
    let platform_authority_row_count = platform_authority_row_count(&model);
    let category_candidate_row_count = category_candidate_row_count(&model);
    let unknown_review_row_count = unknown_review_row_count(&model);
    let rows = rows::readiness_rows(&model);
    let returned = rows.len() as u64;
    let policy_evaluation_ready = !model.evidence_claim_rows.is_empty()
        && !model.identity_rows.is_empty()
        && !model.approval_authority_rows.is_empty()
        && platform_authority_row_count > 0;
    let category_routing_ready = category_candidate_row_count > 0;
    let unknown_review_required = unknown_review_row_count > 0;
    let manual_review_required = rows
        .iter()
        .any(|row| row.readiness_state != APP_GAME_POLICY_READINESS_STATE_READY);
    let capability_status = policy_readiness_status(&model, policy_evaluation_ready)
        .0
        .to_string();
    let adapter_dispatch_claimed = adapter_dispatch_claimed(&model);

    AppGamePolicyReadinessReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: model.generated_at,
        custody_label: APP_GAME_POLICY_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status,
        returned,
        policy_evaluation_ready,
        category_routing_ready,
        unknown_review_required,
        manual_review_required,
        adapter_dispatch_claimed,
        evidence_claim_row_count: model.evidence_claim_rows.len() as u64,
        identity_row_count: model.identity_rows.len() as u64,
        approval_authority_row_count: model.approval_authority_rows.len() as u64,
        approval_action_result_row_count: model.approval_action_result_rows.len() as u64,
        platform_authority_row_count,
        ai_classifier_result_row_count: model.ai_classifier_result_rows.len() as u64,
        category_candidate_row_count,
        unknown_review_row_count,
        rows,
    }
}

fn adapter_dispatch_claimed(model: &AppGameServiceReadModel) -> bool {
    model.approval_action_result_rows.iter().any(|row| {
        row.result_status == APP_GAME_CONTROL_ACTION_STATUS_ENFORCED
            && row.enforcement_result.as_ref().is_some_and(|result| {
                result.status == APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED
            })
    })
}

pub fn app_game_policy_readiness_payload(
    read_model: &AppGamePolicyReadinessReadModel,
) -> LogFields {
    read_model_fields(read_model)
}

fn read_model_fields(read_model: &AppGamePolicyReadinessReadModel) -> LogFields {
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
            constants::field::APP_GAME_POLICY_READINESS_READ_MODEL,
            LogFieldValue::String(serde_json::to_string(read_model).unwrap_or_default()),
        ),
    ])
}

fn policy_readiness_status(
    model: &AppGameServiceReadModel,
    policy_evaluation_ready: bool,
) -> PolicyReadinessTextRef<'static> {
    if app_game_boundary_row_count(model) == 0 {
        PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATUS_NO_ROWS)
    } else if policy_evaluation_ready {
        PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATUS_READY)
    } else {
        PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATUS_PARTIAL)
    }
}
