use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;
use ocentra_parent_agent_protocol::app_game_policy_readiness::APP_GAME_POLICY_READINESS_KIND_CATEGORY_RISK_ROUTING;
use ocentra_parent_agent_protocol::AppGamePolicyReadinessRow;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_KIND_AI_CLASSIFIER_CONTEXT;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_KIND_APPROVAL_ACTION_RESULT;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_KIND_APPROVAL_AUTHORITY;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_KIND_CATEGORY_CANDIDATE;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_KIND_PLATFORM_AUTHORITY;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_KIND_UNKNOWN_REVIEW;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_STATE_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_STATE_MISSING;
use ocentra_parent_agent_protocol::APP_GAME_POLICY_READINESS_STATE_READY;
use ocentra_parent_agent_protocol::APP_GAME_SCHEMA_VERSION;

use super::super::app_game_policy_readiness_sources::{
    ai_classifier_refs, approval_action_result_refs, approval_authority_refs,
    category_candidate_refs, category_candidate_row_count, category_risk_routing,
    platform_authority_row_count, platform_authority_row_refs, policy_evidence_refs,
    unknown_review_refs, unknown_review_row_count,
};
use super::PolicyReadinessTextRef;

pub(super) fn readiness_rows(model: &AppGameServiceReadModel) -> Vec<AppGamePolicyReadinessRow> {
    let mut rows = policy_rows(model);
    rows.push(platform_authority_row(model));
    rows.push(ai_classifier_row(model));
    rows.extend(category_rows(model));
    rows.push(unknown_review_row(model));
    rows
}

fn policy_rows(model: &AppGameServiceReadModel) -> Vec<AppGamePolicyReadinessRow> {
    let policy_evidence_count =
        model.evidence_claim_rows.len() as u64 + model.identity_rows.len() as u64;
    let policy_evidence_state =
        if model.evidence_claim_rows.is_empty() || model.identity_rows.is_empty() {
            PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATE_MISSING)
        } else {
            PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATE_READY)
        };
    vec![
        readiness_row(
            PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE),
            policy_evidence_state,
            policy_evidence_count,
            policy_evidence_refs(model),
        ),
        readiness_row(
            PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_KIND_APPROVAL_AUTHORITY),
            nonempty_state(model.approval_authority_rows.is_empty()),
            model.approval_authority_rows.len() as u64,
            approval_authority_refs(model),
        ),
        readiness_row(
            PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_KIND_APPROVAL_ACTION_RESULT),
            approval_result_state(model.approval_action_result_rows.is_empty()),
            model.approval_action_result_rows.len() as u64,
            approval_action_result_refs(model),
        ),
    ]
}

fn platform_authority_row(model: &AppGameServiceReadModel) -> AppGamePolicyReadinessRow {
    let row_count = platform_authority_row_count(model);
    readiness_row(
        PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_KIND_PLATFORM_AUTHORITY),
        nonempty_state(row_count == 0),
        row_count,
        platform_authority_row_refs(model),
    )
}

fn ai_classifier_row(model: &AppGameServiceReadModel) -> AppGamePolicyReadinessRow {
    let row_count = model.ai_classifier_result_rows.len() as u64;
    readiness_row(
        PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_KIND_AI_CLASSIFIER_CONTEXT),
        if row_count == 0 {
            PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATE_MANUAL_REQUIRED)
        } else {
            PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATE_READY)
        },
        row_count,
        ai_classifier_refs(model),
    )
}

fn category_rows(model: &AppGameServiceReadModel) -> Vec<AppGamePolicyReadinessRow> {
    let candidate_count = category_candidate_row_count(model);
    let (risk_count, risk_refs) = category_risk_routing(model);
    vec![
        readiness_row(
            PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_KIND_CATEGORY_CANDIDATE),
            nonempty_state(candidate_count == 0),
            candidate_count,
            category_candidate_refs(model),
        ),
        readiness_row(
            PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_KIND_CATEGORY_RISK_ROUTING),
            if risk_count == 0 {
                PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATE_READY)
            } else {
                PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATE_MANUAL_REQUIRED)
            },
            risk_count,
            risk_refs,
        ),
    ]
}

fn unknown_review_row(model: &AppGameServiceReadModel) -> AppGamePolicyReadinessRow {
    let row_count = unknown_review_row_count(model);
    readiness_row(
        PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_KIND_UNKNOWN_REVIEW),
        if row_count == 0 {
            PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATE_READY)
        } else {
            PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATE_MANUAL_REQUIRED)
        },
        row_count,
        unknown_review_refs(model),
    )
}

fn nonempty_state(empty: bool) -> PolicyReadinessTextRef<'static> {
    if empty {
        PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATE_MISSING)
    } else {
        PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATE_READY)
    }
}

fn approval_result_state(empty: bool) -> PolicyReadinessTextRef<'static> {
    if empty {
        PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATE_MANUAL_REQUIRED)
    } else {
        PolicyReadinessTextRef(APP_GAME_POLICY_READINESS_STATE_READY)
    }
}

fn readiness_row(
    readiness_kind: PolicyReadinessTextRef<'static>,
    readiness_state: PolicyReadinessTextRef<'static>,
    row_count: u64,
    evidence: Vec<ActivityEvidenceRef>,
) -> AppGamePolicyReadinessRow {
    AppGamePolicyReadinessRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: readiness_kind.0.to_string(),
        readiness_kind: readiness_kind.0.to_string(),
        readiness_state: readiness_state.0.to_string(),
        row_count,
        evidence_reference_ids: evidence.iter().map(|row| row.evidence_id.clone()).collect(),
        evidence,
    }
}
