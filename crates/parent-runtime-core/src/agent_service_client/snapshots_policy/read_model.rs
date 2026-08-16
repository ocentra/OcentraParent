use super::*;
use ocentra_schema::parent_ui_bridge::{
    ParentPolicyApprovalId, ParentPolicyAuditReferenceId, ParentPolicyDecisionActionId,
    ParentPolicyDecisionId, ParentPolicyOverrideId, ParentPolicyPreviewConfirmationContext,
    ParentPolicyPreviewId, ParentPolicyReasonCodes, ParentPolicyReplayApprovalId,
    ParentPolicyRuleContextRefIds, ParentPolicyRuleIds, ParentPolicyTargetId, ParentRouteEventId,
    ParentUserActorId, ParentUserLocalAiResultId,
};

pub(super) fn policy_preview_read_model_from_payload_impl(
    payload: &LogFields,
) -> Result<ParentPolicyPreviewReadModelSnapshot, String> {
    let overview = overview_fields(payload)?;
    let decision = decision_fields(payload);
    let preview = preview_state_fields(payload);
    let request = request_fields(payload);
    let review = review_fields(payload);
    let network = network_fields(payload);

    Ok(ParentPolicyPreviewReadModelSnapshot {
        schema_version: overview.schema_version,
        generated_at: overview.generated_at,
        custody: overview.custody,
        limit: overview.limit,
        returned: overview.returned,
        capability_status: overview.capability_status,
        preview_id: overview.preview_id,
        latest_event_id: overview.latest_event_id,
        latest_observed_at: overview.latest_observed_at,
        target_id: overview.target_id,
        target_type: overview.target_type,
        target_value: overview.target_value,
        evidence_reference_count: overview.evidence_reference_count,
        parent_rule_context_reference_count: overview.parent_rule_context_reference_count,
        parent_rule_context_ref_ids: overview.parent_rule_context_ref_ids,
        decision_id: decision.decision_id,
        decision_action: decision.decision_action,
        reason_codes: decision.reason_codes,
        rule_ids: decision.rule_ids,
        local_ai_result_id: decision.local_ai_result_id,
        dry_run: decision.dry_run,
        enforcement_handoff_state: decision.enforcement_handoff_state,
        policy_preview_save_state: preview.policy_preview_save_state,
        policy_preview_manual_review_state: preview.policy_preview_manual_review_state,
        policy_preview_target_state: preview.policy_preview_target_state,
        policy_preview_target_explanation_code: preview.policy_preview_target_explanation_code,
        policy_preview_finding_kinds: preview.policy_preview_finding_kinds,
        policy_source_status: request.policy_source_status,
        policy_source_surface: request.policy_source_surface,
        policy_request_origin: request.policy_request_origin,
        policy_assistant_confirmation_state: request.policy_assistant_confirmation_state,
        policy_request_status: request.policy_request_status,
        policy_approval_id: review.policy_approval_id,
        policy_override_id: review.policy_override_id,
        policy_replay_of_approval_id: review.policy_replay_of_approval_id,
        policy_reviewed_by_actor_id: review.policy_reviewed_by_actor_id,
        policy_reviewed_by_actor_role: review.policy_reviewed_by_actor_role,
        policy_reviewed_at: review.policy_reviewed_at,
        policy_audit_reference_id: review.policy_audit_reference_id,
        network_evidence_grade: network.network_evidence_grade,
        network_requested_policy_action: network.network_requested_policy_action,
        network_mapped_policy_action: network.network_mapped_policy_action,
        network_policy_mapping_mode: network.network_policy_mapping_mode,
        network_adapter_action_authorized: network.network_adapter_action_authorized,
        network_enforcement_command_authorized: network.network_enforcement_command_authorized,
        confirmation_context: optional_string_field(
            payload,
            constants::field::POLICY_PREVIEW_CONFIRMATION_CONTEXT,
        )
        .and_then(|value| {
            serde_json::from_str::<ParentPolicyPreviewConfirmationContext>(&value).ok()
        }),
    })
}

struct OverviewFields {
    schema_version: Option<String>,
    generated_at: Option<String>,
    custody: Option<String>,
    limit: Option<u64>,
    returned: u64,
    capability_status: Option<String>,
    preview_id: Option<ParentPolicyPreviewId>,
    latest_event_id: Option<ParentRouteEventId>,
    latest_observed_at: Option<String>,
    target_id: Option<ParentPolicyTargetId>,
    target_type: Option<String>,
    target_value: Option<String>,
    evidence_reference_count: Option<u64>,
    parent_rule_context_reference_count: Option<u64>,
    parent_rule_context_ref_ids: Option<ParentPolicyRuleContextRefIds>,
}

struct DecisionFields {
    decision_id: Option<ParentPolicyDecisionId>,
    decision_action: Option<ParentPolicyDecisionActionId>,
    reason_codes: Option<ParentPolicyReasonCodes>,
    rule_ids: Option<ParentPolicyRuleIds>,
    local_ai_result_id: Option<ParentUserLocalAiResultId>,
    dry_run: Option<bool>,
    enforcement_handoff_state: Option<String>,
}

struct PreviewStateFields {
    policy_preview_save_state: Option<String>,
    policy_preview_manual_review_state: Option<String>,
    policy_preview_target_state: Option<String>,
    policy_preview_target_explanation_code: Option<String>,
    policy_preview_finding_kinds: Option<String>,
}

struct RequestFields {
    policy_source_status: Option<String>,
    policy_source_surface: Option<String>,
    policy_request_origin: Option<String>,
    policy_assistant_confirmation_state: Option<String>,
    policy_request_status: Option<String>,
}

struct ReviewFields {
    policy_approval_id: Option<ParentPolicyApprovalId>,
    policy_override_id: Option<ParentPolicyOverrideId>,
    policy_replay_of_approval_id: Option<ParentPolicyReplayApprovalId>,
    policy_reviewed_by_actor_id: Option<ParentUserActorId>,
    policy_reviewed_by_actor_role: Option<String>,
    policy_reviewed_at: Option<String>,
    policy_audit_reference_id: Option<ParentPolicyAuditReferenceId>,
}

struct NetworkFields {
    network_evidence_grade: Option<String>,
    network_requested_policy_action: Option<String>,
    network_mapped_policy_action: Option<String>,
    network_policy_mapping_mode: Option<String>,
    network_adapter_action_authorized: Option<bool>,
    network_enforcement_command_authorized: Option<bool>,
}

fn overview_fields(payload: &LogFields) -> Result<OverviewFields, String> {
    Ok(OverviewFields {
        schema_version: optional_string_field(payload, constants::field::SCHEMA_VERSION),
        generated_at: optional_string_field(payload, constants::field::GENERATED_AT),
        custody: optional_string_field(payload, constants::field::CUSTODY),
        limit: optional_u64_field(payload, constants::field::LIMIT),
        returned: required_u64_field_with_context(
            payload,
            constants::field::RETURNED,
            "agent-service policy preview payload",
        )?,
        capability_status: optional_string_field(payload, constants::field::CAPABILITY_STATUS),
        preview_id: optional_string_field(payload, constants::field::POLICY_PREVIEW_ID)
            .and_then(ParentPolicyPreviewId::parse),
        latest_event_id: optional_string_field(payload, constants::field::LATEST_EVENT_ID)
            .and_then(ParentRouteEventId::parse),
        latest_observed_at: optional_string_field(payload, constants::field::LATEST_OBSERVED_AT),
        target_id: optional_string_field(payload, constants::field::TARGET_ID)
            .and_then(ParentPolicyTargetId::parse),
        target_type: optional_string_field(payload, constants::field::POLICY_TARGET_TYPE),
        target_value: optional_string_field(payload, constants::field::POLICY_TARGET_VALUE),
        evidence_reference_count: optional_u64_field(
            payload,
            constants::field::POLICY_EVIDENCE_REFERENCE_COUNT,
        ),
        parent_rule_context_reference_count: optional_u64_field(
            payload,
            policy::PARENT_RULE_CONTEXT_REFERENCE_COUNT_FIELD,
        ),
        parent_rule_context_ref_ids: optional_string_field(
            payload,
            policy::PARENT_RULE_CONTEXT_REF_IDS_FIELD,
        )
        .and_then(ParentPolicyRuleContextRefIds::parse),
    })
}

fn decision_fields(payload: &LogFields) -> DecisionFields {
    DecisionFields {
        decision_id: optional_string_field(payload, constants::field::POLICY_DECISION_ID)
            .and_then(ParentPolicyDecisionId::parse),
        decision_action: optional_string_field(payload, constants::field::POLICY_ACTION)
            .and_then(ParentPolicyDecisionActionId::parse),
        reason_codes: optional_string_field(payload, constants::field::POLICY_REASON_CODES)
            .and_then(ParentPolicyReasonCodes::parse),
        rule_ids: optional_string_field(payload, constants::field::POLICY_RULE_IDS)
            .and_then(ParentPolicyRuleIds::parse),
        local_ai_result_id: optional_string_field(payload, constants::field::LOCAL_AI_RESULT_ID)
            .and_then(ParentUserLocalAiResultId::parse),
        dry_run: optional_bool_field(payload, constants::field::POLICY_DRY_RUN),
        enforcement_handoff_state: optional_string_field(
            payload,
            constants::field::POLICY_HANDOFF_STATE,
        ),
    }
}

fn preview_state_fields(payload: &LogFields) -> PreviewStateFields {
    PreviewStateFields {
        policy_preview_save_state: optional_string_field(
            payload,
            constants::field::POLICY_PREVIEW_SAVE_STATE,
        ),
        policy_preview_manual_review_state: optional_string_field(
            payload,
            constants::field::POLICY_PREVIEW_MANUAL_REVIEW_STATE,
        ),
        policy_preview_target_state: optional_string_field(
            payload,
            constants::field::POLICY_PREVIEW_TARGET_STATE,
        ),
        policy_preview_target_explanation_code: optional_string_field(
            payload,
            constants::field::POLICY_PREVIEW_TARGET_EXPLANATION_CODE,
        ),
        policy_preview_finding_kinds: optional_string_field(
            payload,
            constants::field::POLICY_PREVIEW_FINDING_KINDS,
        ),
    }
}

fn request_fields(payload: &LogFields) -> RequestFields {
    RequestFields {
        policy_source_status: optional_string_field(
            payload,
            constants::field::POLICY_SOURCE_STATUS,
        ),
        policy_source_surface: optional_string_field(
            payload,
            constants::field::POLICY_SOURCE_SURFACE,
        ),
        policy_request_origin: optional_string_field(
            payload,
            constants::field::POLICY_REQUEST_ORIGIN,
        ),
        policy_assistant_confirmation_state: optional_string_field(
            payload,
            constants::field::POLICY_ASSISTANT_CONFIRMATION_STATE,
        ),
        policy_request_status: optional_string_field(
            payload,
            constants::field::POLICY_REQUEST_STATUS,
        ),
    }
}

fn review_fields(payload: &LogFields) -> ReviewFields {
    ReviewFields {
        policy_approval_id: optional_string_field(payload, constants::field::POLICY_APPROVAL_ID)
            .and_then(ParentPolicyApprovalId::parse),
        policy_override_id: optional_string_field(payload, constants::field::POLICY_OVERRIDE_ID)
            .and_then(ParentPolicyOverrideId::parse),
        policy_replay_of_approval_id: optional_string_field(
            payload,
            constants::field::POLICY_REPLAY_OF_APPROVAL_ID,
        )
        .and_then(ParentPolicyReplayApprovalId::parse),
        policy_reviewed_by_actor_id: optional_string_field(
            payload,
            constants::field::POLICY_REVIEWED_BY_ACTOR_ID,
        )
        .and_then(ParentUserActorId::parse),
        policy_reviewed_by_actor_role: optional_string_field(
            payload,
            constants::field::POLICY_REVIEWED_BY_ACTOR_ROLE,
        ),
        policy_reviewed_at: optional_string_field(payload, constants::field::POLICY_REVIEWED_AT),
        policy_audit_reference_id: optional_string_field(
            payload,
            constants::field::POLICY_AUDIT_REFERENCE_ID,
        )
        .and_then(ParentPolicyAuditReferenceId::parse),
    }
}

fn network_fields(payload: &LogFields) -> NetworkFields {
    NetworkFields {
        network_evidence_grade: optional_string_field(
            payload,
            constants::field::NETWORK_EVIDENCE_GRADE,
        ),
        network_requested_policy_action: optional_string_field(
            payload,
            constants::field::NETWORK_REQUESTED_POLICY_ACTION,
        ),
        network_mapped_policy_action: optional_string_field(
            payload,
            constants::field::NETWORK_MAPPED_POLICY_ACTION,
        ),
        network_policy_mapping_mode: optional_string_field(
            payload,
            constants::field::NETWORK_POLICY_MAPPING_MODE,
        ),
        network_adapter_action_authorized: optional_bool_field(
            payload,
            constants::field::NETWORK_ADAPTER_ACTION_AUTHORIZED,
        ),
        network_enforcement_command_authorized: optional_bool_field(
            payload,
            constants::field::NETWORK_ENFORCEMENT_COMMAND_AUTHORIZED,
        ),
    }
}
