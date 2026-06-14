use ocentra_network_evidence::{
    map_network_evidence_grade_to_policy, NetworkEvidenceGrade, NetworkEvidencePolicyAction,
    NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingError,
    NetworkEvidencePolicyMappingInput, NetworkEvidencePolicyMode,
};
use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, policy_preview_finding_kinds_csv, ActivityEvidenceKind,
    ActivityEvidenceRef, LocalAiParentRuleContextRef, LogFieldValue, LogFields,
    ParentEvidenceReference, ParentEvidenceReferenceKind, PolicyAction, PolicyDecision,
    PolicyPreviewFindingKind, PolicyPreviewNetworkEvidenceMapping, PolicyPreviewReadModel,
    PolicyPreviewReadModelRow, PolicyPreviewTargetState,
    APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED, APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM, POLICY_DRY_RUN_SCHEMA_VERSION,
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
        &row.device_id,
        &row.platform,
        parent_rule_contexts,
    );
    let parent_rules = parent_rule_context_references
        .iter()
        .map(|reference| reference.rule.clone())
        .collect::<Vec<_>>();
    let (decision, network_evidence_mapping) = grade_mapped_network_decision(
        &row,
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
    let policy_preview_target_state = policy_preview_target_state_from_row(&row);
    let policy_preview_target_explanation_code =
        policy_preview_target_explanation_code_from_row(&row, policy_preview_target_state);
    let policy_preview_finding_kinds =
        policy_preview_target_finding_kinds(policy_preview_target_state);

    Some(PolicyPreviewReadModelRow {
        preview_id: prefixed_id(policy::PREVIEW_ID_PREFIX, &row.event_id),
        source_event_id: row.event_id,
        observed_at: row.observed_at,
        target,
        evidence_references,
        parent_rule_context_references,
        decision,
        policy_preview_save_state: None,
        policy_preview_manual_review_state: None,
        policy_preview_target_state,
        policy_preview_target_explanation_code,
        policy_preview_finding_kinds,
        policy_source_status: None,
        policy_source_surface: None,
        policy_request_origin: None,
        policy_assistant_confirmation_state: None,
        policy_request_status: None,
        network_evidence_mapping,
    })
}

fn grade_mapped_network_decision(
    row: &PolicyPreviewStoreRow,
    mut decision: PolicyDecision,
) -> (PolicyDecision, Option<PolicyPreviewNetworkEvidenceMapping>) {
    let Some(evidence_grade) = network_evidence_grade(row) else {
        return (decision, None);
    };
    let Some(requested_action) = network_policy_action(decision.action) else {
        return (decision, None);
    };
    let mapping = match network_policy_mapping(evidence_grade, requested_action, &decision) {
        Ok(mapping) => mapping,
        Err(_) => {
            decision.action = PolicyAction::AskParent;
            push_unique_reason(
                &mut decision.reason_codes,
                policy::REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW.to_string(),
            );
            return (
                decision,
                Some(PolicyPreviewNetworkEvidenceMapping {
                    evidence_grade: network_evidence_grade_protocol(evidence_grade).to_string(),
                    requested_action: network_policy_action_protocol(requested_action).to_string(),
                    mapped_action: policy::ACTION_ASK_PARENT.to_string(),
                    mode: policy::NETWORK_POLICY_MAPPING_MODE_PARENT_REVIEW.to_string(),
                    adapter_action_authorized: false,
                    enforcement_command_authorized: false,
                }),
            );
        }
    };
    let mapped_action = policy_action(mapping.mapped_action);
    if mapped_action != decision.action {
        decision.action = mapped_action;
        push_unique_reason(
            &mut decision.reason_codes,
            grade_mapping_reason(mapping.mode),
        );
    }
    let preview_mapping = preview_network_evidence_mapping(&mapping);
    (decision, Some(preview_mapping))
}

fn network_evidence_grade(row: &PolicyPreviewStoreRow) -> Option<NetworkEvidenceGrade> {
    if row.kind != constants::activity_event_kind::DOMAIN_OBSERVED {
        return None;
    }
    if string_field(&row.fields, constants::field::CAPABILITY_STATUS).as_deref()
        != Some(constants::activity_capture::CAPABILITY_STATUS_AVAILABLE)
    {
        return Some(NetworkEvidenceGrade::D);
    }
    if string_field(&row.fields, constants::field::DOMAIN_ATTRIBUTION_STATUS).as_deref()
        != Some(constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_DOMAIN_OBSERVED)
    {
        return Some(NetworkEvidenceGrade::D);
    }
    if string_field(&row.fields, constants::field::PROCESS_ATTRIBUTION_STATUS).as_deref()
        == Some(constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED)
    {
        Some(NetworkEvidenceGrade::B)
    } else {
        Some(NetworkEvidenceGrade::C)
    }
}

fn network_policy_mapping(
    evidence_grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
    decision: &PolicyDecision,
) -> Result<NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingError> {
    let parent_rule_ref = decision.rule_ids.first().cloned().unwrap_or_default();
    let evidence_refs = decision
        .evidence_references
        .iter()
        .map(|reference| reference.evidence_reference_id.clone())
        .collect::<Vec<_>>();
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: decision.decision_id.clone(),
        parent_rule_ref,
        evidence_refs,
        local_ai_result_ref: decision.local_ai_result_id.clone(),
        evidence_grade,
        requested_action,
        adapter_capability_proof_ref: None,
    })
}

fn preview_network_evidence_mapping(
    mapping: &NetworkEvidencePolicyMapping,
) -> PolicyPreviewNetworkEvidenceMapping {
    PolicyPreviewNetworkEvidenceMapping {
        evidence_grade: network_evidence_grade_protocol(mapping.evidence_grade).to_string(),
        requested_action: network_policy_action_protocol(mapping.requested_action).to_string(),
        mapped_action: network_policy_action_protocol(mapping.mapped_action).to_string(),
        mode: network_policy_mode_protocol(mapping.mode).to_string(),
        adapter_action_authorized: mapping.adapter_action_authorized,
        enforcement_command_authorized: mapping.enforcement_command_authorized,
    }
}

fn network_evidence_grade_protocol(grade: NetworkEvidenceGrade) -> &'static str {
    match grade {
        NetworkEvidenceGrade::A => policy::NETWORK_EVIDENCE_GRADE_A,
        NetworkEvidenceGrade::B => policy::NETWORK_EVIDENCE_GRADE_B,
        NetworkEvidenceGrade::C => policy::NETWORK_EVIDENCE_GRADE_C,
        NetworkEvidenceGrade::D => policy::NETWORK_EVIDENCE_GRADE_D,
    }
}

fn network_policy_action_protocol(action: NetworkEvidencePolicyAction) -> &'static str {
    match action {
        NetworkEvidencePolicyAction::None => policy::NETWORK_POLICY_ACTION_NONE,
        NetworkEvidencePolicyAction::AskParent => policy::ACTION_ASK_PARENT,
        NetworkEvidencePolicyAction::WarnChild => policy::ACTION_WARN,
        NetworkEvidencePolicyAction::Monitor => policy::NETWORK_POLICY_ACTION_MONITOR,
        NetworkEvidencePolicyAction::Limit => policy::ACTION_TIME_LIMIT,
        NetworkEvidencePolicyAction::Block => policy::ACTION_BLOCK,
    }
}

fn network_policy_mode_protocol(mode: NetworkEvidencePolicyMode) -> &'static str {
    match mode {
        NetworkEvidencePolicyMode::ObserveOnly => policy::NETWORK_POLICY_MAPPING_MODE_OBSERVE_ONLY,
        NetworkEvidencePolicyMode::DryRun => policy::NETWORK_POLICY_MAPPING_MODE_DRY_RUN,
        NetworkEvidencePolicyMode::ParentReview => {
            policy::NETWORK_POLICY_MAPPING_MODE_PARENT_REVIEW
        }
    }
}

fn network_policy_action(action: PolicyAction) -> Option<NetworkEvidencePolicyAction> {
    match action {
        PolicyAction::Warn => Some(NetworkEvidencePolicyAction::WarnChild),
        PolicyAction::Block => Some(NetworkEvidencePolicyAction::Block),
        PolicyAction::TimeLimit => Some(NetworkEvidencePolicyAction::Limit),
        PolicyAction::AskParent => Some(NetworkEvidencePolicyAction::AskParent),
        PolicyAction::Allow | PolicyAction::Unknown => None,
    }
}

fn policy_preview_target_state_from_row(
    row: &PolicyPreviewStoreRow,
) -> Option<PolicyPreviewTargetState> {
    let capability_status = string_field(&row.fields, constants::field::CAPABILITY_STATUS);
    let capability_status = capability_status.as_deref();

    if capability_status == Some(constants::browser::CAPABILITY_STATUS_STALE)
        || capability_status == Some(constants::tracking_runtime::CAPABILITY_STATUS_STALE)
        || capability_status == Some(APP_GAME_CAPABILITY_STATUS_STALE)
    {
        Some(PolicyPreviewTargetState::Stale)
    } else if capability_status
        == Some(constants::tracking_runtime::CAPABILITY_STATUS_OFFLINE_LAST_KNOWN_ONLY)
    {
        Some(PolicyPreviewTargetState::Offline)
    } else if capability_status == Some(constants::browser::CAPABILITY_STATUS_UNSUPPORTED_BROWSER)
        || capability_status == Some(APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM)
    {
        Some(PolicyPreviewTargetState::Unsupported)
    } else if capability_status == Some(constants::browser::CAPABILITY_STATUS_BRIDGE_MISSING)
        || capability_status == Some(constants::browser::CAPABILITY_STATUS_MANAGED_PROFILE_MISSING)
        || capability_status == Some(constants::browser::CAPABILITY_STATUS_PERMISSION_LIMITED)
        || capability_status == Some(constants::browser::CAPABILITY_STATUS_ADAPTER_ERROR)
        || capability_status == Some(constants::browser::CAPABILITY_STATUS_UNMANAGED_BROWSER)
        || capability_status == Some(constants::tracking_runtime::CAPABILITY_STATUS_MANUAL_REQUIRED)
        || capability_status == Some(APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED)
    {
        Some(PolicyPreviewTargetState::ManualRequired)
    } else {
        None
    }
}

fn policy_preview_target_explanation_code_from_row(
    row: &PolicyPreviewStoreRow,
    target_state: Option<PolicyPreviewTargetState>,
) -> Option<String> {
    target_state.and_then(|_| {
        string_field(&row.fields, constants::field::DEGRADED_REASON)
            .or_else(|| string_field(&row.fields, constants::field::CAPABILITY_STATUS))
    })
}

fn policy_preview_target_finding_kinds(
    target_state: Option<PolicyPreviewTargetState>,
) -> Option<String> {
    match target_state {
        Some(PolicyPreviewTargetState::Unsupported) => {
            policy_preview_finding_kinds_csv(&[PolicyPreviewFindingKind::UnsupportedTarget])
        }
        Some(PolicyPreviewTargetState::ManualRequired) => {
            policy_preview_finding_kinds_csv(&[PolicyPreviewFindingKind::ManualRequiredTarget])
        }
        Some(PolicyPreviewTargetState::Offline) => {
            policy_preview_finding_kinds_csv(&[PolicyPreviewFindingKind::OfflineTarget])
        }
        Some(PolicyPreviewTargetState::Stale) => {
            policy_preview_finding_kinds_csv(&[PolicyPreviewFindingKind::StaleTarget])
        }
        Some(PolicyPreviewTargetState::Supported) | None => None,
    }
}

fn policy_action(action: NetworkEvidencePolicyAction) -> PolicyAction {
    match action {
        NetworkEvidencePolicyAction::AskParent => PolicyAction::AskParent,
        NetworkEvidencePolicyAction::WarnChild => PolicyAction::Warn,
        NetworkEvidencePolicyAction::Limit => PolicyAction::TimeLimit,
        NetworkEvidencePolicyAction::Block => PolicyAction::Block,
        NetworkEvidencePolicyAction::Monitor | NetworkEvidencePolicyAction::None => {
            PolicyAction::Unknown
        }
    }
}

fn grade_mapping_reason(mode: NetworkEvidencePolicyMode) -> String {
    match mode {
        NetworkEvidencePolicyMode::ParentReview => {
            policy::REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW.to_string()
        }
        NetworkEvidencePolicyMode::ObserveOnly => {
            policy::REASON_NETWORK_EVIDENCE_GRADE_OBSERVE_ONLY.to_string()
        }
        NetworkEvidencePolicyMode::DryRun => {
            policy::REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW.to_string()
        }
    }
}

fn push_unique_reason(reason_codes: &mut Vec<String>, reason_code: String) {
    if !reason_codes.iter().any(|existing| existing == &reason_code) {
        reason_codes.push(reason_code);
    }
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

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}
