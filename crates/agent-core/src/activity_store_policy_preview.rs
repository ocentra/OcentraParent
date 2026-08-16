use ocentra_network_evidence::{
    dns::types::NetworkEvidenceGrade,
    policy::{
        map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
        NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingError,
        NetworkEvidencePolicyMappingInput, NetworkEvidencePolicyMode,
    },
};
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::activity::policy::PolicyDecision;
use ocentra_parent_agent_protocol::activity::policy::POLICY_DRY_RUN_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::policy_context::LocalAiParentRuleContextRef;
use ocentra_parent_agent_protocol::activity::policy_preview::policy_preview_finding_kinds_csv;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyAssistantConfirmationState;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewConfirmationContext;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewFindingKind;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewNetworkEvidenceMapping;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewReadModel;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewReadModelRow;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewTargetState;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestOrigin;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicySourceStatus;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicySourceSurface;
use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED, APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::policy_constants as policy;
use rusqlite::Connection;

use crate::{
    activity_store_error::ActivityStoreError,
    activity_store_parent_rule_context::parent_rule_contexts,
    activity_store_policy_preview_parent_rules::parent_rule_contexts_for_row,
    activity_store_policy_preview_rows::{policy_preview_rows, PolicyPreviewStoreRow},
    policy_dry_run_evaluator::{evaluate_policy_dry_run, PolicyDryRunEvaluationInput},
};

use crate::activity_store_policy_preview_targets::targets_from_row;

const POLICY_PREVIEW_TARGET_STATE_RULES: &[(&[&str], PolicyPreviewTargetState)] = &[
    (
        &[
            constants::browser::CAPABILITY_STATUS_STALE,
            constants::tracking_runtime::CAPABILITY_STATUS_STALE,
            APP_GAME_CAPABILITY_STATUS_STALE,
        ],
        PolicyPreviewTargetState::Stale,
    ),
    (
        &[constants::tracking_runtime::CAPABILITY_STATUS_OFFLINE_LAST_KNOWN_ONLY],
        PolicyPreviewTargetState::Offline,
    ),
    (
        &[
            constants::browser::CAPABILITY_STATUS_UNSUPPORTED_BROWSER,
            APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
        ],
        PolicyPreviewTargetState::Unsupported,
    ),
    (
        &[
            constants::browser::CAPABILITY_STATUS_BRIDGE_MISSING,
            constants::browser::CAPABILITY_STATUS_MANAGED_PROFILE_MISSING,
            constants::browser::CAPABILITY_STATUS_PERMISSION_LIMITED,
            constants::browser::CAPABILITY_STATUS_ADAPTER_ERROR,
            constants::browser::CAPABILITY_STATUS_UNMANAGED_BROWSER,
            constants::tracking_runtime::CAPABILITY_STATUS_MANUAL_REQUIRED,
            APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED,
        ],
        PolicyPreviewTargetState::ManualRequired,
    ),
];

const POLICY_PREVIEW_TARGET_FINDING_KIND_RULES: &[(
    PolicyPreviewTargetState,
    PolicyPreviewFindingKind,
)] = &[
    (
        PolicyPreviewTargetState::Unsupported,
        PolicyPreviewFindingKind::UnsupportedTarget,
    ),
    (
        PolicyPreviewTargetState::ManualRequired,
        PolicyPreviewFindingKind::ManualRequiredTarget,
    ),
    (
        PolicyPreviewTargetState::Offline,
        PolicyPreviewFindingKind::OfflineTarget,
    ),
    (
        PolicyPreviewTargetState::Stale,
        PolicyPreviewFindingKind::StaleTarget,
    ),
];

const NETWORK_EVIDENCE_GRADE_TABLE: [NetworkEvidenceGrade; 8] = [
    NetworkEvidenceGrade::D,
    NetworkEvidenceGrade::D,
    NetworkEvidenceGrade::D,
    NetworkEvidenceGrade::D,
    NetworkEvidenceGrade::D,
    NetworkEvidenceGrade::D,
    NetworkEvidenceGrade::C,
    NetworkEvidenceGrade::B,
];

const NETWORK_EVIDENCE_GRADE_PROTOCOL_RULES: &[(NetworkEvidenceGrade, &str)] = &[
    (NetworkEvidenceGrade::A, policy::NETWORK_EVIDENCE_GRADE_A),
    (NetworkEvidenceGrade::B, policy::NETWORK_EVIDENCE_GRADE_B),
    (NetworkEvidenceGrade::C, policy::NETWORK_EVIDENCE_GRADE_C),
    (NetworkEvidenceGrade::D, policy::NETWORK_EVIDENCE_GRADE_D),
];

const NETWORK_POLICY_ACTION_RULES: &[(NetworkEvidencePolicyAction, PolicyAction)] = &[
    (NetworkEvidencePolicyAction::None, PolicyAction::Unknown),
    (
        NetworkEvidencePolicyAction::AskParent,
        PolicyAction::AskParent,
    ),
    (NetworkEvidencePolicyAction::WarnChild, PolicyAction::Warn),
    (NetworkEvidencePolicyAction::Monitor, PolicyAction::Unknown),
    (NetworkEvidencePolicyAction::Limit, PolicyAction::TimeLimit),
    (NetworkEvidencePolicyAction::Block, PolicyAction::Block),
];

const NETWORK_POLICY_ACTION_PROTOCOL_RULES: &[(NetworkEvidencePolicyAction, &str)] = &[
    (
        NetworkEvidencePolicyAction::AskParent,
        policy::ACTION_ASK_PARENT,
    ),
    (NetworkEvidencePolicyAction::WarnChild, policy::ACTION_WARN),
    (
        NetworkEvidencePolicyAction::Monitor,
        policy::NETWORK_POLICY_ACTION_MONITOR,
    ),
    (
        NetworkEvidencePolicyAction::Limit,
        policy::ACTION_TIME_LIMIT,
    ),
    (NetworkEvidencePolicyAction::Block, policy::ACTION_BLOCK),
    (
        NetworkEvidencePolicyAction::None,
        policy::NETWORK_POLICY_ACTION_NONE,
    ),
];

const POLICY_TO_NETWORK_ACTION_RULES: &[(PolicyAction, NetworkEvidencePolicyAction)] = &[
    (PolicyAction::Warn, NetworkEvidencePolicyAction::WarnChild),
    (PolicyAction::Block, NetworkEvidencePolicyAction::Block),
    (PolicyAction::TimeLimit, NetworkEvidencePolicyAction::Limit),
    (
        PolicyAction::AskParent,
        NetworkEvidencePolicyAction::AskParent,
    ),
];

const NETWORK_POLICY_MODE_PROTOCOL_RULES: &[(NetworkEvidencePolicyMode, &str)] = &[
    (
        NetworkEvidencePolicyMode::ObserveOnly,
        policy::NETWORK_POLICY_MAPPING_MODE_OBSERVE_ONLY,
    ),
    (
        NetworkEvidencePolicyMode::DryRun,
        policy::NETWORK_POLICY_MAPPING_MODE_DRY_RUN,
    ),
    (
        NetworkEvidencePolicyMode::ParentReview,
        policy::NETWORK_POLICY_MAPPING_MODE_PARENT_REVIEW,
    ),
];

const POLICY_PREVIEW_FINDING_KIND_CSV_RULES: &[(
    PolicyPreviewTargetState,
    PolicyPreviewFindingKind,
)] = POLICY_PREVIEW_TARGET_FINDING_KIND_RULES;

const POLICY_MAPPING_REASON_RULES: &[(NetworkEvidencePolicyMode, &str)] = &[
    (
        NetworkEvidencePolicyMode::ParentReview,
        policy::REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW,
    ),
    (
        NetworkEvidencePolicyMode::ObserveOnly,
        policy::REASON_NETWORK_EVIDENCE_GRADE_OBSERVE_ONLY,
    ),
    (
        NetworkEvidencePolicyMode::DryRun,
        policy::REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW,
    ),
];

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
    let policy_lifecycle = policy_lifecycle_projection_from_row(&row);

    Some(PolicyPreviewReadModelRow {
        preview_id: prefixed_id(policy::PREVIEW_ID_PREFIX, &row.event_id),
        source_event_id: row.event_id,
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
        confirmation_context: confirmation_context_projection(&row),
    })
}

fn confirmation_context_projection(
    row: &PolicyPreviewStoreRow,
) -> Option<PolicyPreviewConfirmationContext> {
    let request_id = string_field(
        &row.fields,
        constants::policy_control::request::FIELD_REQUEST_ID,
    );
    let submission_key = string_field(
        &row.fields,
        constants::policy_control::request::FIELD_SUBMISSION_KEY,
    );
    let household_id = string_field(
        &row.fields,
        constants::policy_control::request::FIELD_HOUSEHOLD_ID,
    )
    .or_else(|| {
        string_field(
            &row.fields,
            constants::policy_control::source::FIELD_HOUSEHOLD_ID,
        )
    });
    let policy_version = number_field(
        &row.fields,
        constants::policy_control::request::FIELD_POLICY_VERSION,
    )
    .or_else(|| {
        number_field(
            &row.fields,
            constants::policy_control::source::FIELD_POLICY_VERSION,
        )
    });
    let audit_reference_ids = string_field(
        &row.fields,
        constants::policy_control::request::FIELD_AUDIT_REFERENCE_IDS,
    )
    .or_else(|| {
        string_field(
            &row.fields,
            constants::policy_control::source::FIELD_AUDIT_REFERENCE_IDS,
        )
    });
    let actor_role = string_field(
        &row.fields,
        constants::policy_control::request::FIELD_ACTOR_ROLE,
    )
    .or_else(|| {
        string_field(
            &row.fields,
            constants::policy_control::source::FIELD_ACTOR_ROLE,
        )
    });
    let actor_state = string_field(
        &row.fields,
        constants::policy_control::request::FIELD_ACTOR_STATE,
    )
    .or_else(|| {
        string_field(
            &row.fields,
            constants::policy_control::source::FIELD_ACTOR_STATE,
        )
    });

    let context = PolicyPreviewConfirmationContext {
        request_id,
        submission_key,
        household_id,
        child_profile_id: string_field(
            &row.fields,
            constants::policy_control::source::FIELD_CHILD_PROFILE_ID,
        ),
        device_id: string_field(
            &row.fields,
            constants::policy_control::source::FIELD_DEVICE_ID,
        ),
        source_document_id: string_field(
            &row.fields,
            constants::policy_control::source::FIELD_DOCUMENT_ID,
        ),
        policy_version,
        target_reference_id: string_field(
            &row.fields,
            constants::policy_control::source::FIELD_TARGET_REFERENCE_ID,
        ),
        rule_id: string_field(
            &row.fields,
            constants::policy_control::source::FIELD_RULE_ID,
        ),
        requested_at: string_field(&row.fields, constants::field::REQUESTED_AT).or_else(|| {
            string_field(
                &row.fields,
                constants::policy_control::request::FIELD_TIMESTAMP,
            )
        }),
        expires_at: string_field(&row.fields, constants::field::EXPIRES_AT),
        assistant_preview_id: string_field(
            &row.fields,
            constants::policy_control::request::FIELD_ASSISTANT_PREVIEW_ID,
        ),
        audit_reference_ids,
        actor_id: string_field(
            &row.fields,
            constants::policy_control::source::FIELD_ACTOR_ID,
        ),
        actor_role,
        actor_state,
        confirmation_audit_reference_id: string_field(
            &row.fields,
            constants::field::POLICY_AUDIT_REFERENCE_ID,
        )
        .or_else(|| {
            string_field(
                &row.fields,
                constants::policy_control::source::FIELD_AUDIT_REFERENCE_ID,
            )
        }),
    };

    context_has_any_value(&context).then_some(context)
}

fn context_has_any_value(context: &PolicyPreviewConfirmationContext) -> bool {
    context.request_id.is_some()
        || context.submission_key.is_some()
        || context.household_id.is_some()
        || context.child_profile_id.is_some()
        || context.device_id.is_some()
        || context.source_document_id.is_some()
        || context.policy_version.is_some()
        || context.target_reference_id.is_some()
        || context.rule_id.is_some()
        || context.requested_at.is_some()
        || context.expires_at.is_some()
        || context.assistant_preview_id.is_some()
        || context.audit_reference_ids.is_some()
        || context.actor_id.is_some()
        || context.actor_role.is_some()
        || context.actor_state.is_some()
        || context.confirmation_audit_reference_id.is_some()
}

struct PolicyLifecycleProjection {
    policy_source_status: Option<PolicySourceStatus>,
    policy_source_surface: Option<PolicySourceSurface>,
    policy_request_origin: Option<PolicyRequestOrigin>,
    policy_assistant_confirmation_state: Option<PolicyAssistantConfirmationState>,
    policy_request_status: Option<PolicyRequestStatus>,
    policy_approval_id: Option<String>,
    policy_override_id: Option<String>,
    policy_replay_of_approval_id: Option<String>,
    policy_reviewed_by_actor_id: Option<String>,
    policy_reviewed_by_actor_role: Option<String>,
    policy_reviewed_at: Option<String>,
    policy_audit_reference_id: Option<String>,
}

fn policy_lifecycle_projection_from_row(row: &PolicyPreviewStoreRow) -> PolicyLifecycleProjection {
    PolicyLifecycleProjection {
        policy_source_status: protocol_field(
            &row.fields,
            constants::field::POLICY_SOURCE_STATUS,
            |value| PolicySourceStatus::from_protocol_str(value),
        ),
        policy_source_surface: protocol_field(
            &row.fields,
            constants::field::POLICY_SOURCE_SURFACE,
            |value| PolicySourceSurface::from_protocol_str(value),
        ),
        policy_request_origin: protocol_field(
            &row.fields,
            constants::field::POLICY_REQUEST_ORIGIN,
            |value| PolicyRequestOrigin::from_protocol_str(value),
        ),
        policy_assistant_confirmation_state: protocol_field(
            &row.fields,
            constants::field::POLICY_ASSISTANT_CONFIRMATION_STATE,
            |value| PolicyAssistantConfirmationState::from_protocol_str(value),
        ),
        policy_request_status: protocol_field(
            &row.fields,
            constants::field::POLICY_REQUEST_STATUS,
            |value| PolicyRequestStatus::from_protocol_str(value),
        ),
        policy_approval_id: string_field(&row.fields, constants::field::POLICY_APPROVAL_ID),
        policy_override_id: string_field(&row.fields, constants::field::POLICY_OVERRIDE_ID),
        policy_replay_of_approval_id: string_field(
            &row.fields,
            constants::field::POLICY_REPLAY_OF_APPROVAL_ID,
        ),
        policy_reviewed_by_actor_id: string_field(
            &row.fields,
            constants::field::POLICY_REVIEWED_BY_ACTOR_ID,
        ),
        policy_reviewed_by_actor_role: string_field(
            &row.fields,
            constants::field::POLICY_REVIEWED_BY_ACTOR_ROLE,
        ),
        policy_reviewed_at: string_field(&row.fields, constants::field::POLICY_REVIEWED_AT),
        policy_audit_reference_id: string_field(
            &row.fields,
            constants::field::POLICY_AUDIT_REFERENCE_ID,
        ),
    }
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
    (row.kind == constants::activity_event_kind::DOMAIN_OBSERVED).then_some(())?;
    let capability_status_available =
        string_field(&row.fields, constants::field::CAPABILITY_STATUS).as_deref()
            == Some(constants::activity_capture::CAPABILITY_STATUS_AVAILABLE);
    let domain_observed = string_field(&row.fields, constants::field::DOMAIN_ATTRIBUTION_STATUS)
        .as_deref()
        == Some(constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_DOMAIN_OBSERVED);
    let process_attributed =
        string_field(&row.fields, constants::field::PROCESS_ATTRIBUTION_STATUS).as_deref()
            == Some(constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED);
    Some(
        NETWORK_EVIDENCE_GRADE_TABLE[((capability_status_available as usize) << 2)
            | ((domain_observed as usize) << 1)
            | (process_attributed as usize)],
    )
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
    NETWORK_EVIDENCE_GRADE_PROTOCOL_RULES
        .iter()
        .find_map(|(candidate, protocol)| (*candidate == grade).then_some(*protocol))
        .unwrap_or(policy::NETWORK_EVIDENCE_GRADE_D)
}

fn network_policy_action_protocol(action: NetworkEvidencePolicyAction) -> &'static str {
    NETWORK_POLICY_ACTION_PROTOCOL_RULES
        .iter()
        .find_map(|(candidate, protocol)| (*candidate == action).then_some(*protocol))
        .unwrap_or(policy::NETWORK_POLICY_ACTION_NONE)
}

fn network_policy_mode_protocol(mode: NetworkEvidencePolicyMode) -> &'static str {
    NETWORK_POLICY_MODE_PROTOCOL_RULES
        .iter()
        .find_map(|(candidate, protocol)| (*candidate == mode).then_some(*protocol))
        .unwrap_or(policy::NETWORK_POLICY_MAPPING_MODE_PARENT_REVIEW)
}

fn network_policy_action(action: PolicyAction) -> Option<NetworkEvidencePolicyAction> {
    POLICY_TO_NETWORK_ACTION_RULES
        .iter()
        .find_map(|(candidate, mapped)| (*candidate == action).then_some(*mapped))
}

fn policy_preview_target_state_from_row(
    row: &PolicyPreviewStoreRow,
) -> Option<PolicyPreviewTargetState> {
    let capability_status = string_field(&row.fields, constants::field::CAPABILITY_STATUS)?;
    POLICY_PREVIEW_TARGET_STATE_RULES
        .iter()
        .find_map(|(statuses, state)| {
            statuses
                .contains(&capability_status.as_str())
                .then_some(*state)
        })
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
    target_state.and_then(|target_state| {
        POLICY_PREVIEW_FINDING_KIND_CSV_RULES
            .iter()
            .find_map(|(candidate, kind)| (*candidate == target_state).then_some(*kind))
            .and_then(|kind| policy_preview_finding_kinds_csv(&[kind]))
    })
}

fn policy_action(action: NetworkEvidencePolicyAction) -> PolicyAction {
    NETWORK_POLICY_ACTION_RULES
        .iter()
        .find_map(|(candidate, mapped)| (*candidate == action).then_some(*mapped))
        .unwrap_or(PolicyAction::Unknown)
}

fn grade_mapping_reason(mode: NetworkEvidencePolicyMode) -> String {
    POLICY_MAPPING_REASON_RULES
        .iter()
        .find_map(|(candidate, reason)| (*candidate == mode).then_some(*reason))
        .unwrap_or(policy::REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW)
        .to_string()
}

fn push_unique_reason(reason_codes: &mut Vec<String>, reason_code: String) {
    (!reason_codes.iter().any(|existing| existing == &reason_code))
        .then(|| reason_codes.push(reason_code));
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
    let kind = evidence_reference_kind(&evidence.kind)?;

    Some(ParentEvidenceReference {
        evidence_reference_id: evidence.evidence_id.clone(),
        kind,
        observed_at: observed_at.to_string(),
    })
}

fn evidence_reference_kind(kind: &ActivityEvidenceKind) -> Option<ParentEvidenceReferenceKind> {
    const RULES: &[(ActivityEvidenceKind, ParentEvidenceReferenceKind)] = &[
        (
            ActivityEvidenceKind::JournalEntry,
            ParentEvidenceReferenceKind::JournalEvent,
        ),
        (
            ActivityEvidenceKind::LocalDbRow,
            ParentEvidenceReferenceKind::QueryStoreSummary,
        ),
    ];

    RULES
        .iter()
        .find_map(|(candidate, mapped)| (*candidate == *kind).then_some(*mapped))
}

fn push_unique_reference(
    references: &mut Vec<ParentEvidenceReference>,
    reference: ParentEvidenceReference,
) {
    (!references
        .iter()
        .any(|existing| existing.evidence_reference_id == reference.evidence_reference_id))
    .then(|| references.push(reference));
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

fn number_field(fields: &LogFields, key: &str) -> Option<u64> {
    match fields.get(key) {
        Some(LogFieldValue::Number(value)) if value.is_finite() && *value >= 0.0 => {
            (*value as u64 == *value).then_some(*value as u64)
        }
        Some(LogFieldValue::String(value)) => value.parse::<u64>().ok(),
        _ => None,
    }
}

fn protocol_field<T>(
    fields: &LogFields,
    key: &str,
    parse: impl for<'a> Fn(&'a str) -> Option<T>,
) -> Option<T> {
    string_field(fields, key).and_then(|value| parse(&value))
}
