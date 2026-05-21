use ocentra_parent_agent_protocol::{
    policy_constants as policy, ActivityEvidenceKind, ActivityEvidenceRef,
    LocalAiParentRuleContextRef, ParentEvidenceReference, ParentEvidenceReferenceKind,
    PolicyPreviewReadModel, PolicyPreviewReadModelRow, POLICY_DRY_RUN_SCHEMA_VERSION,
};
use rusqlite::Connection;

use crate::{
    activity_store_parent_rule_context::parent_rule_contexts,
    activity_store_policy_preview_parent_rules::parent_rule_contexts_for_row,
    activity_store_policy_preview_rows::{policy_preview_rows, PolicyPreviewStoreRow},
    evaluate_policy_dry_run, ActivityStoreError, PolicyDryRunEvaluationInput,
};

use crate::activity_store_policy_preview_targets::targets_from_row;

pub(crate) fn policy_preview_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> Result<PolicyPreviewReadModel, ActivityStoreError> {
    let rows = policy_preview_rows(connection, limit)?;
    let parent_rule_contexts = parent_rule_contexts(connection)?;
    let preview_rows = rows
        .into_iter()
        .filter_map(|row| preview_row(row, generated_at, &parent_rule_contexts))
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
    row: PolicyPreviewStoreRow,
    generated_at: &str,
    parent_rule_contexts: &[LocalAiParentRuleContextRef],
) -> Option<PolicyPreviewReadModelRow> {
    let targets = targets_from_row(&row)?;
    let target = targets.primary;
    let evidence_references = evidence_references_from_row(&row);
    let parent_rule_context_references = parent_rule_contexts_for_row(
        &target,
        &targets.aliases,
        &evidence_references,
        generated_at,
        parent_rule_contexts,
    );
    let parent_rules = parent_rule_context_references
        .iter()
        .map(|reference| reference.rule.clone())
        .collect::<Vec<_>>();
    let decision = evaluate_policy_dry_run(PolicyDryRunEvaluationInput {
        decision_id: prefixed_id(policy::PREVIEW_DECISION_ID_PREFIX, &row.event_id),
        evaluated_at: generated_at.to_string(),
        observed_target: target.clone(),
        observed_target_aliases: targets.aliases,
        parent_rules,
        local_ai_result: None,
        evidence_references: evidence_references.clone(),
        expires_at: None,
    });

    Some(PolicyPreviewReadModelRow {
        preview_id: prefixed_id(policy::PREVIEW_ID_PREFIX, &row.event_id),
        source_event_id: row.event_id,
        observed_at: row.observed_at,
        target,
        evidence_references,
        parent_rule_context_references,
        decision,
    })
}

fn evidence_references_from_row(row: &PolicyPreviewStoreRow) -> Vec<ParentEvidenceReference> {
    let mut references = vec![ParentEvidenceReference {
        evidence_reference_id: row.event_id.clone(),
        kind: ParentEvidenceReferenceKind::ActivityEvent,
        observed_at: row.observed_at.clone(),
    }];

    for evidence in &row.evidence {
        if let Some(reference) = evidence_reference_from_activity(evidence, &row.observed_at) {
            push_unique_reference(&mut references, reference);
        }
    }

    references
}

fn evidence_reference_from_activity(
    evidence: &ActivityEvidenceRef,
    observed_at: &str,
) -> Option<ParentEvidenceReference> {
    let kind = match &evidence.kind {
        ActivityEvidenceKind::JournalEntry => ParentEvidenceReferenceKind::JournalEvent,
        ActivityEvidenceKind::LocalDbRow => ParentEvidenceReferenceKind::QueryStoreSummary,
        ActivityEvidenceKind::Screenshot | ActivityEvidenceKind::StorageObject => return None,
    };

    Some(ParentEvidenceReference {
        evidence_reference_id: evidence.evidence_id.clone(),
        kind,
        observed_at: observed_at.to_string(),
    })
}

fn push_unique_reference(
    references: &mut Vec<ParentEvidenceReference>,
    reference: ParentEvidenceReference,
) {
    if !references
        .iter()
        .any(|existing| existing.evidence_reference_id == reference.evidence_reference_id)
    {
        references.push(reference);
    }
}

fn prefixed_id(prefix: &str, source_id: &str) -> String {
    let mut value = String::from(prefix);
    value.push_str(source_id);
    value
}
