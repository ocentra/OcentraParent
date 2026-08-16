use crate::activity::local_ai::{
    LocalAiGraphReference, LocalAiGraphReferenceKind, LocalAiMemoryReference,
    LocalAiMemoryReferenceKind, LocalAiSafetyResult, LocalAiUnknownState,
};
use crate::activity::policy::{
    ParentActorReference, ParentActorRole, ParentEvidenceReference, ParentEvidenceReferenceKind,
    PolicyAction, PolicyDecision, PolicyDecisionHandoffState, PolicyRule, PolicyTarget,
    PolicyTargetType,
};
use crate::constants;
use crate::policy_constants as policy;
use crate::POLICY_DRY_RUN_SCHEMA_VERSION;
use crate::{
    LocalAiAdapterBoundary, LocalAiCapabilityFlag, LocalAiDegradedState, LocalAiExecutionState,
    LocalAiModelLoadState, LocalAiProviderPrivacyMode, LocalAiProviderSource, LocalAiResourceClass,
    LocalModelRuntimeStatus,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn policy_rule_serializes_parent_authored_shape() {
    let rule = rule(PolicyAction::AskParent);
    let serialized = serde_json::to_value(rule).expect_value("policy rule serializes");

    assert_eq!(serialized["ruleId"], policy::TEST_ASK_PARENT_RULE_ID);
    assert_eq!(serialized["action"], policy::ACTION_ASK_PARENT);
    assert_eq!(
        serialized["target"]["targetType"],
        policy::TARGET_TYPE_DOMAIN
    );
    assert_eq!(serialized["createdBy"]["role"], policy::ACTOR_ROLE_PARENT);
}

#[test]
fn policy_decision_serializes_dry_run_disabled_handoff_shape() {
    let decision = PolicyDecision {
        schema_version: POLICY_DRY_RUN_SCHEMA_VERSION.to_string(),
        decision_id: policy::TEST_DECISION_ID.to_string(),
        action: PolicyAction::Block,
        reason_codes: vec![policy::TEST_REASON_PARENT_BLOCK.to_string()],
        evidence_references: vec![evidence()],
        rule_ids: vec![policy::TEST_BLOCK_RULE_ID.to_string()],
        local_ai_result_id: Some(policy::TEST_AI_RESULT_ID.to_string()),
        dry_run: true,
        enforcement_handoff_state: PolicyDecisionHandoffState::Disabled,
        expires_at: None,
    };
    let serialized = serde_json::to_value(decision).expect_value("policy decision serializes");

    assert_eq!(
        serialized["schemaVersion"],
        policy::CONTRACT_SCHEMA_VERSION_V0_6
    );
    assert_eq!(serialized["action"], policy::ACTION_BLOCK);
    assert_eq!(serialized["dryRun"], true);
    assert_eq!(
        serialized["enforcementHandoffState"],
        policy::HANDOFF_DISABLED
    );
    assert_eq!(serialized["localAiResultId"], policy::TEST_AI_RESULT_ID);
}

#[test]
fn local_ai_safety_result_serializes_policy_signal_shape() {
    let result = local_ai_result(PolicyAction::AskParent, LocalAiUnknownState::LowConfidence);
    let serialized = serde_json::to_value(result).expect_value("local AI safety result serializes");

    assert_eq!(
        serialized["schemaVersion"],
        policy::CONTRACT_SCHEMA_VERSION_V0_6
    );
    assert_eq!(serialized["action"], policy::ACTION_ASK_PARENT);
    assert_eq!(serialized["unknownState"], policy::UNKNOWN_LOW_CONFIDENCE);
    assert_eq!(
        serialized["modelRuntime"]["capabilityFlags"][0],
        constants::local_ai_runtime::CAPABILITY_SAFETY_DECISION
    );
    assert_eq!(
        serialized["memoryReferences"][0]["kind"],
        policy::MEMORY_KIND_RECENT_ACTIVITY
    );
    assert_eq!(
        serialized["graphReferences"][0]["kind"],
        policy::GRAPH_KIND_ENTITY
    );
}

fn rule(action: PolicyAction) -> PolicyRule {
    PolicyRule {
        rule_id: policy::TEST_ASK_PARENT_RULE_ID.to_string(),
        target: target(),
        action,
        schedule_id: None,
        priority: 10,
        reason_code: policy::TEST_REASON_PARENT_ASK.to_string(),
        created_by: ParentActorReference {
            actor_id: policy::TEST_PARENT_ACTOR_ID.to_string(),
            role: ParentActorRole::Parent,
        },
        enabled: true,
        effective_from: None,
        effective_until: None,
    }
}

fn target() -> PolicyTarget {
    PolicyTarget {
        target_id: policy::TEST_TARGET_ID.to_string(),
        target_type: PolicyTargetType::Domain,
        target_value: policy::TEST_TARGET_VALUE.to_string(),
    }
}

fn evidence() -> ParentEvidenceReference {
    ParentEvidenceReference {
        evidence_reference_id: policy::TEST_EVIDENCE_ID.to_string(),
        kind: ParentEvidenceReferenceKind::ActivityEvent,
        observed_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

fn local_ai_result(
    action: PolicyAction,
    unknown_state: LocalAiUnknownState,
) -> LocalAiSafetyResult {
    LocalAiSafetyResult {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        result_id: policy::TEST_AI_RESULT_ID.to_string(),
        request_id: policy::TEST_AI_REQUEST_ID.to_string(),
        action,
        confidence: 0.61,
        unknown_state,
        degraded_state: LocalAiDegradedState::None,
        reason_codes: vec![policy::TEST_REASON_AI_ALLOW.to_string()],
        explanation_reference: None,
        evidence_references: vec![evidence()],
        parent_rule_references: vec![policy::TEST_ALLOW_RULE_ID.to_string()],
        memory_references: vec![LocalAiMemoryReference {
            memory_reference_id: policy::TEST_MEMORY_REFERENCE_ID.to_string(),
            kind: LocalAiMemoryReferenceKind::RecentActivity,
            source_evidence_references: vec![evidence()],
            source_policy_version: Some(policy::TEST_POLICY_VERSION.to_string()),
            generated_at: policy::TEST_EVALUATED_AT.to_string(),
            confidence: 0.82,
            derived_index_version: policy::TEST_DERIVED_INDEX_VERSION.to_string(),
        }],
        graph_references: vec![LocalAiGraphReference {
            graph_reference_id: policy::TEST_GRAPH_REFERENCE_ID.to_string(),
            kind: LocalAiGraphReferenceKind::GraphEntity,
            source_evidence_references: vec![evidence()],
            source_policy_version: Some(policy::TEST_POLICY_VERSION.to_string()),
            generated_at: policy::TEST_EVALUATED_AT.to_string(),
            confidence: 0.78,
            derived_index_version: policy::TEST_DERIVED_INDEX_VERSION.to_string(),
        }],
        model_runtime: LocalModelRuntimeStatus {
            runtime_reference_id: policy::TEST_RUNTIME_REFERENCE_ID.to_string(),
            provider_id: policy::TEST_PROVIDER_ID.to_string(),
            model_id: policy::TEST_MODEL_ID.to_string(),
            model_reference: policy::TEST_MODEL_REFERENCE.to_string(),
            privacy_mode: LocalAiProviderPrivacyMode::LocalOnly,
            adapter_boundary: LocalAiAdapterBoundary::LocalAdapterReady,
            execution_state: LocalAiExecutionState::DryRunReady,
            provider_source: LocalAiProviderSource::LocalModelCache,
            load_state: LocalAiModelLoadState::Loaded,
            capability_flags: vec![LocalAiCapabilityFlag::SafetyDecision],
            resource_class: LocalAiResourceClass::Cpu,
            degraded_state: LocalAiDegradedState::None,
            last_checked_at: policy::TEST_EVALUATED_AT.to_string(),
            unavailable_reason: None,
        },
        prompt_version: policy::TEST_PROMPT_VERSION.to_string(),
        expires_at: None,
    }
}
