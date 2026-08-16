#[path = "policy_preview_payload/field_pairs.rs"]
mod field_pairs;

use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyPreviewReadModel, PolicyPreviewReadModelRow,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::policy_constants as policy;

use self::field_pairs::{
    optional_bool, optional_parent_rule_context_ref_ids, optional_string_list, optional_text,
    optional_u64, policy_preview_fields_from_pairs, PolicyPreviewFieldPair,
    PolicyPreviewStringListRef, PolicyPreviewTextRef,
};

pub fn policy_preview_read_model_payload(read_model: &PolicyPreviewReadModel) -> LogFields {
    let latest = read_model.rows.first();
    let mut pairs = read_model_pairs(read_model);
    pairs.extend(row_pairs(latest));
    pairs.extend(decision_pairs(latest));
    pairs.extend(network_evidence_mapping_pairs(latest));
    policy_preview_fields_from_pairs(pairs)
}

fn read_model_pairs(read_model: &PolicyPreviewReadModel) -> Vec<PolicyPreviewFieldPair> {
    vec![
        PolicyPreviewFieldPair(
            constants::field::SCHEMA_VERSION,
            LogFieldValue::String(read_model.schema_version.clone()),
        ),
        PolicyPreviewFieldPair(
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        PolicyPreviewFieldPair(
            constants::field::CUSTODY,
            LogFieldValue::String(read_model.custody.clone()),
        ),
        PolicyPreviewFieldPair(
            constants::field::LIMIT,
            LogFieldValue::Number(read_model.limit as f64),
        ),
        PolicyPreviewFieldPair(
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned as f64),
        ),
        PolicyPreviewFieldPair(
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(read_model.capability_status.clone()),
        ),
    ]
}

fn row_pairs(row: Option<&PolicyPreviewReadModelRow>) -> Vec<PolicyPreviewFieldPair> {
    let mut pairs = row_identity_pairs(row);
    pairs.extend(row_state_pairs(row));
    pairs.extend(row_review_pairs(row));
    if let Some(context) = row.and_then(|value| value.confirmation_context.as_ref()) {
        if let Ok(serialized) = serde_json::to_string(context) {
            pairs.push(PolicyPreviewFieldPair(
                constants::field::POLICY_PREVIEW_CONFIRMATION_CONTEXT,
                LogFieldValue::String(serialized),
            ));
        }
    }
    pairs
}

fn row_identity_pairs(row: Option<&PolicyPreviewReadModelRow>) -> Vec<PolicyPreviewFieldPair> {
    vec![
        PolicyPreviewFieldPair(
            constants::field::POLICY_PREVIEW_ID,
            optional_text(row.map(|value| PolicyPreviewTextRef(value.preview_id.as_str()))),
        ),
        PolicyPreviewFieldPair(
            constants::field::LATEST_EVENT_ID,
            optional_text(row.map(|value| PolicyPreviewTextRef(value.source_event_id.as_str()))),
        ),
        PolicyPreviewFieldPair(
            constants::field::LATEST_OBSERVED_AT,
            optional_text(row.map(|value| PolicyPreviewTextRef(value.observed_at.as_str()))),
        ),
        PolicyPreviewFieldPair(
            constants::field::TARGET_ID,
            optional_text(row.map(|value| PolicyPreviewTextRef(value.target.target_id.as_str()))),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_TARGET_TYPE,
            optional_text(
                row.map(|value| PolicyPreviewTextRef(value.target.target_type.as_protocol_str())),
            ),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_TARGET_VALUE,
            optional_text(
                row.map(|value| PolicyPreviewTextRef(value.target.target_value.as_str())),
            ),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_EVIDENCE_REFERENCE_COUNT,
            optional_u64(row.map(|value| value.evidence_references.len() as u64)),
        ),
        PolicyPreviewFieldPair(
            policy::PARENT_RULE_CONTEXT_REFERENCE_COUNT_FIELD,
            optional_u64(row.map(|value| value.parent_rule_context_references.len() as u64)),
        ),
        PolicyPreviewFieldPair(
            policy::PARENT_RULE_CONTEXT_REF_IDS_FIELD,
            optional_parent_rule_context_ref_ids(row),
        ),
    ]
}

fn row_state_pairs(row: Option<&PolicyPreviewReadModelRow>) -> Vec<PolicyPreviewFieldPair> {
    let mut pairs = preview_state_pairs(row);
    pairs.extend(request_state_pairs(row));
    pairs
}

fn preview_state_pairs(row: Option<&PolicyPreviewReadModelRow>) -> Vec<PolicyPreviewFieldPair> {
    vec![
        PolicyPreviewFieldPair(
            constants::field::POLICY_PREVIEW_SAVE_STATE,
            optional_text(row.and_then(|value| {
                value
                    .policy_preview_save_state
                    .map(|state| PolicyPreviewTextRef(state.as_protocol_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_PREVIEW_MANUAL_REVIEW_STATE,
            optional_text(row.and_then(|value| {
                value
                    .policy_preview_manual_review_state
                    .map(|state| PolicyPreviewTextRef(state.as_protocol_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_PREVIEW_TARGET_STATE,
            optional_text(row.and_then(|value| {
                value
                    .policy_preview_target_state
                    .map(|state| PolicyPreviewTextRef(state.as_protocol_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_PREVIEW_TARGET_EXPLANATION_CODE,
            optional_text(row.and_then(|value| {
                value
                    .policy_preview_target_explanation_code
                    .as_ref()
                    .map(|text| PolicyPreviewTextRef(text.as_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_PREVIEW_FINDING_KINDS,
            optional_text(row.and_then(|value| {
                value
                    .policy_preview_finding_kinds
                    .as_ref()
                    .map(|text| PolicyPreviewTextRef(text.as_str()))
            })),
        ),
    ]
}

fn request_state_pairs(row: Option<&PolicyPreviewReadModelRow>) -> Vec<PolicyPreviewFieldPair> {
    vec![
        PolicyPreviewFieldPair(
            constants::field::POLICY_SOURCE_STATUS,
            optional_text(row.and_then(|value| {
                value
                    .policy_source_status
                    .map(|state| PolicyPreviewTextRef(state.as_protocol_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_SOURCE_SURFACE,
            optional_text(row.and_then(|value| {
                value
                    .policy_source_surface
                    .map(|state| PolicyPreviewTextRef(state.as_protocol_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_REQUEST_ORIGIN,
            optional_text(row.and_then(|value| {
                value
                    .policy_request_origin
                    .map(|state| PolicyPreviewTextRef(state.as_protocol_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_ASSISTANT_CONFIRMATION_STATE,
            optional_text(row.and_then(|value| {
                value
                    .policy_assistant_confirmation_state
                    .map(|state| PolicyPreviewTextRef(state.as_protocol_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_REQUEST_STATUS,
            optional_text(row.and_then(|value| {
                value
                    .policy_request_status
                    .map(|state| PolicyPreviewTextRef(state.as_protocol_str()))
            })),
        ),
    ]
}

fn row_review_pairs(row: Option<&PolicyPreviewReadModelRow>) -> Vec<PolicyPreviewFieldPair> {
    vec![
        PolicyPreviewFieldPair(
            constants::field::POLICY_APPROVAL_ID,
            optional_text(row.and_then(|value| {
                value
                    .policy_approval_id
                    .as_ref()
                    .map(|text| PolicyPreviewTextRef(text.as_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_OVERRIDE_ID,
            optional_text(row.and_then(|value| {
                value
                    .policy_override_id
                    .as_ref()
                    .map(|text| PolicyPreviewTextRef(text.as_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_REPLAY_OF_APPROVAL_ID,
            optional_text(row.and_then(|value| {
                value
                    .policy_replay_of_approval_id
                    .as_ref()
                    .map(|text| PolicyPreviewTextRef(text.as_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_REVIEWED_BY_ACTOR_ID,
            optional_text(row.and_then(|value| {
                value
                    .policy_reviewed_by_actor_id
                    .as_ref()
                    .map(|text| PolicyPreviewTextRef(text.as_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_REVIEWED_BY_ACTOR_ROLE,
            optional_text(row.and_then(|value| {
                value
                    .policy_reviewed_by_actor_role
                    .as_ref()
                    .map(|text| PolicyPreviewTextRef(text.as_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_REVIEWED_AT,
            optional_text(row.and_then(|value| {
                value
                    .policy_reviewed_at
                    .as_ref()
                    .map(|text| PolicyPreviewTextRef(text.as_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_AUDIT_REFERENCE_ID,
            optional_text(row.and_then(|value| {
                value
                    .policy_audit_reference_id
                    .as_ref()
                    .map(|text| PolicyPreviewTextRef(text.as_str()))
            })),
        ),
    ]
}

fn decision_pairs(row: Option<&PolicyPreviewReadModelRow>) -> Vec<PolicyPreviewFieldPair> {
    vec![
        PolicyPreviewFieldPair(
            constants::field::POLICY_DECISION_ID,
            optional_text(
                row.map(|value| PolicyPreviewTextRef(value.decision.decision_id.as_str())),
            ),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_ACTION,
            optional_text(
                row.map(|value| PolicyPreviewTextRef(value.decision.action.as_protocol_str())),
            ),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_REASON_CODES,
            optional_string_list(
                row.map(|value| PolicyPreviewStringListRef(value.decision.reason_codes.as_slice())),
            ),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_RULE_IDS,
            optional_string_list(
                row.map(|value| PolicyPreviewStringListRef(value.decision.rule_ids.as_slice())),
            ),
        ),
        PolicyPreviewFieldPair(
            constants::field::LOCAL_AI_RESULT_ID,
            optional_text(row.and_then(|value| {
                value
                    .decision
                    .local_ai_result_id
                    .as_ref()
                    .map(|text| PolicyPreviewTextRef(text.as_str()))
            })),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_DRY_RUN,
            optional_bool(row.map(|value| value.decision.dry_run)),
        ),
        PolicyPreviewFieldPair(
            constants::field::POLICY_HANDOFF_STATE,
            optional_text(row.map(|value| {
                PolicyPreviewTextRef(value.decision.enforcement_handoff_state.as_protocol_str())
            })),
        ),
    ]
}

fn network_evidence_mapping_pairs(
    row: Option<&PolicyPreviewReadModelRow>,
) -> Vec<PolicyPreviewFieldPair> {
    let mapping = row.and_then(|value| value.network_evidence_mapping.as_ref());
    vec![
        PolicyPreviewFieldPair(
            constants::field::NETWORK_EVIDENCE_GRADE,
            optional_text(mapping.map(|value| PolicyPreviewTextRef(value.evidence_grade.as_str()))),
        ),
        PolicyPreviewFieldPair(
            constants::field::NETWORK_REQUESTED_POLICY_ACTION,
            optional_text(
                mapping.map(|value| PolicyPreviewTextRef(value.requested_action.as_str())),
            ),
        ),
        PolicyPreviewFieldPair(
            constants::field::NETWORK_MAPPED_POLICY_ACTION,
            optional_text(mapping.map(|value| PolicyPreviewTextRef(value.mapped_action.as_str()))),
        ),
        PolicyPreviewFieldPair(
            constants::field::NETWORK_POLICY_MAPPING_MODE,
            optional_text(mapping.map(|value| PolicyPreviewTextRef(value.mode.as_str()))),
        ),
        PolicyPreviewFieldPair(
            constants::field::NETWORK_ADAPTER_ACTION_AUTHORIZED,
            optional_bool(mapping.map(|value| value.adapter_action_authorized)),
        ),
        PolicyPreviewFieldPair(
            constants::field::NETWORK_ENFORCEMENT_COMMAND_AUTHORIZED,
            optional_bool(mapping.map(|value| value.enforcement_command_authorized)),
        ),
    ]
}
