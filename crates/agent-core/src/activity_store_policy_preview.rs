use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, ActivityEvidenceKind, ActivityEvidenceRef,
    LogFieldValue, LogFields, ParentEvidenceReference, ParentEvidenceReferenceKind,
    PolicyPreviewReadModel, PolicyPreviewReadModelRow, PolicyTarget, PolicyTargetType,
    POLICY_DRY_RUN_SCHEMA_VERSION,
};
use rusqlite::Connection;

use crate::{
    activity_store_policy_preview_rows::{policy_preview_rows, PolicyPreviewStoreRow},
    evaluate_policy_dry_run, ActivityStoreError, PolicyDryRunEvaluationInput,
};

pub(crate) fn policy_preview_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> Result<PolicyPreviewReadModel, ActivityStoreError> {
    let rows = policy_preview_rows(connection, limit)?;
    let preview_rows = rows
        .into_iter()
        .filter_map(|row| preview_row(row, generated_at))
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
) -> Option<PolicyPreviewReadModelRow> {
    let target = target_from_row(&row)?;
    let evidence_references = evidence_references_from_row(&row);
    let decision = evaluate_policy_dry_run(PolicyDryRunEvaluationInput {
        decision_id: prefixed_id(policy::PREVIEW_DECISION_ID_PREFIX, &row.event_id),
        evaluated_at: generated_at.to_string(),
        observed_target: target.clone(),
        parent_rules: Vec::new(),
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
        decision,
    })
}

fn target_from_row(row: &PolicyPreviewStoreRow) -> Option<PolicyTarget> {
    let (target_type, target_value) =
        target_type_and_value(row.subject_kind.as_str(), &row.fields, row)?;
    Some(PolicyTarget {
        target_id: row.subject_id.clone(),
        target_type,
        target_value,
    })
}

fn target_type_and_value(
    subject_kind: &str,
    fields: &LogFields,
    row: &PolicyPreviewStoreRow,
) -> Option<(PolicyTargetType, String)> {
    match subject_kind {
        constants::activity_subject_kind::PROCESS => Some((
            PolicyTargetType::Process,
            field_or_subject_value(fields, constants::field::PROCESS_NAME, row),
        )),
        constants::activity_subject_kind::WINDOW => Some((
            PolicyTargetType::Window,
            field_or_subject_value(fields, constants::field::WINDOW_TITLE, row),
        )),
        constants::activity_subject_kind::DOMAIN => Some((
            PolicyTargetType::Domain,
            domain_or_subject_value(fields, row),
        )),
        constants::activity_subject_kind::URL => url_target(fields, row),
        constants::activity_subject_kind::VIDEO => {
            Some((PolicyTargetType::Video, subject_value(row)))
        }
        constants::activity_subject_kind::DEVICE => {
            Some((PolicyTargetType::Device, subject_value(row)))
        }
        _ => None,
    }
}

fn url_target(
    fields: &LogFields,
    row: &PolicyPreviewStoreRow,
) -> Option<(PolicyTargetType, String)> {
    if let Some(domain) = string_field(fields, constants::field::DOMAIN) {
        return Some((PolicyTargetType::Domain, domain));
    }

    Some((
        PolicyTargetType::Site,
        string_field(fields, constants::field::URL).unwrap_or_else(|| subject_value(row)),
    ))
}

fn domain_or_subject_value(fields: &LogFields, row: &PolicyPreviewStoreRow) -> String {
    string_field(fields, constants::field::DESTINATION_DOMAIN)
        .or_else(|| string_field(fields, constants::field::DOMAIN))
        .unwrap_or_else(|| subject_value(row))
}

fn field_or_subject_value(fields: &LogFields, key: &str, row: &PolicyPreviewStoreRow) -> String {
    string_field(fields, key).unwrap_or_else(|| subject_value(row))
}

fn subject_value(row: &PolicyPreviewStoreRow) -> String {
    row.subject_display_name
        .clone()
        .unwrap_or_else(|| row.subject_id.clone())
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

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}

fn prefixed_id(prefix: &str, source_id: &str) -> String {
    let mut value = String::from(prefix);
    value.push_str(source_id);
    value
}
