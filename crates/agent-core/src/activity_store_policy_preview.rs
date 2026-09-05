use ocentra_parent_agent_protocol::activity::policy::POLICY_DRY_RUN_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::policy_context::LocalAiParentRuleContextRef;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyPreviewReadModel, PolicyPreviewReadModelRow,
};
use ocentra_parent_agent_protocol::policy_constants as policy;
use rusqlite::Connection;

use crate::activity_store_error::ActivityStoreError;
use crate::activity_store_parent_rule_context::parent_rule_contexts;
use crate::activity_store_policy_preview_confirmation::confirmation_context_projection;
use crate::activity_store_policy_preview_evidence::evidence_references_from_row;
use crate::activity_store_policy_preview_lifecycle::policy_lifecycle_projection_from_row;
use crate::activity_store_policy_preview_network::grade_mapped_network_decision;
use crate::activity_store_policy_preview_parent_rules::parent_rule_contexts_for_row;
use crate::activity_store_policy_preview_rows::{policy_preview_rows, PolicyPreviewStoreRow};
use crate::activity_store_policy_preview_target_state::{
    target_explanation_code_from_row, target_finding_kinds, target_state_from_row,
};
use crate::activity_store_policy_preview_targets::targets_from_row;
use crate::policy_dry_run_evaluator::{evaluate_policy_dry_run, PolicyDryRunEvaluationInput};

pub(crate) fn policy_preview_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> Result<PolicyPreviewReadModel, ActivityStoreError> {
    let rows = policy_preview_rows(connection, limit)?;
    let parent_rule_contexts = parent_rule_contexts(connection)?;
    let preview_rows = rows
        .into_iter()
        .filter_map(|row| preview_row(&row, generated_at, &parent_rule_contexts))
        .collect::<Vec<_>>();

    let capability_status = if preview_rows.is_empty() {
        policy::PREVIEW_CAPABILITY_NO_EVIDENCE
    } else {
        policy::PREVIEW_CAPABILITY_READY
    };

    Ok(PolicyPreviewReadModel {
        schema_version: POLICY_DRY_RUN_SCHEMA_VERSION.to_string(),
        generated_at: generated_at.to_string(),
        custody: policy::PREVIEW_CUSTODY_ACTIVITY_STORE.to_string(),
        limit,
        returned: preview_rows.len() as u64,
        capability_status: capability_status.to_string(),
        rows: preview_rows,
    })
}

fn preview_row(
    row: &PolicyPreviewStoreRow,
    generated_at: &str,
    parent_rule_contexts: &[LocalAiParentRuleContextRef],
) -> Option<PolicyPreviewReadModelRow> {
    let targets = targets_from_row(row)?;
    let target = targets.primary;
    let evidence_references = evidence_references_from_row(row);
    let parent_rule_context_references = parent_rule_contexts_for_row(
        &target,
        &targets.aliases,
        &evidence_references,
        generated_at,
        &row.device_id,
        &row.platform,
        parent_rule_contexts,
    );
    let parent_rules = parent_rule_context_references
        .iter()
        .map(|reference| reference.rule.clone())
        .collect::<Vec<_>>();
    let (decision, network_evidence_mapping) = grade_mapped_network_decision(
        row,
        evaluate_policy_dry_run(PolicyDryRunEvaluationInput {
            decision_id: prefixed_id(policy::PREVIEW_DECISION_ID_PREFIX, &row.event_id),
            evaluated_at: generated_at.to_string(),
            observed_target: target.clone(),
            observed_target_aliases: targets.aliases,
            parent_rules,
            local_ai_result: None,
            evidence_references: evidence_references.clone(),
            expires_at: None,
        }),
    );
    let policy_preview_target_state = target_state_from_row(row);
    let policy_preview_target_explanation_code =
        target_explanation_code_from_row(row, policy_preview_target_state);
    let policy_preview_finding_kinds = target_finding_kinds(policy_preview_target_state);
    let policy_lifecycle = policy_lifecycle_projection_from_row(row);

    Some(PolicyPreviewReadModelRow {
        preview_id: prefixed_id(policy::PREVIEW_ID_PREFIX, &row.event_id),
        source_event_id: row.event_id.clone(),
        observed_at: row.observed_at.clone(),
        target,
        evidence_references,
        parent_rule_context_references,
        decision,
        policy_preview_save_state: None,
        policy_preview_manual_review_state: None,
        policy_preview_target_state,
        policy_preview_target_explanation_code,
        policy_preview_finding_kinds,
        policy_source_status: policy_lifecycle.policy_source_status,
        policy_source_surface: policy_lifecycle.policy_source_surface,
        policy_request_origin: policy_lifecycle.policy_request_origin,
        policy_assistant_confirmation_state: policy_lifecycle.policy_assistant_confirmation_state,
        policy_request_status: policy_lifecycle.policy_request_status,
        policy_approval_id: policy_lifecycle.policy_approval_id,
        policy_override_id: policy_lifecycle.policy_override_id,
        policy_replay_of_approval_id: policy_lifecycle.policy_replay_of_approval_id,
        policy_reviewed_by_actor_id: policy_lifecycle.policy_reviewed_by_actor_id,
        policy_reviewed_by_actor_role: policy_lifecycle.policy_reviewed_by_actor_role,
        policy_reviewed_at: policy_lifecycle.policy_reviewed_at,
        policy_audit_reference_id: policy_lifecycle.policy_audit_reference_id,
        network_evidence_mapping,
        confirmation_context: confirmation_context_projection(row),
    })
}

fn prefixed_id(prefix: &str, source_id: &str) -> String {
    let mut value = String::from(prefix);
    value.push_str(source_id);
    value
}
