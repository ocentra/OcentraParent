use ocentra_parent_agent_core::policy_dry_run_evaluator::PolicyDryRunEvaluationInput;
use ocentra_parent_agent_protocol::activity::local_ai::{
    LocalAiGraphReference, LocalAiGraphReferenceKind, LocalAiMemoryReference,
    LocalAiMemoryReferenceKind, LocalAiSafetyResult, LocalAiUnknownState,
};
use ocentra_parent_agent_protocol::activity::policy::ParentActorReference;
use ocentra_parent_agent_protocol::activity::policy::ParentActorRole;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::activity::policy::PolicyRule;
use ocentra_parent_agent_protocol::activity::policy::PolicyTarget;
use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiCapabilityFlag;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiDegradedState;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiModelLoadState;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiResourceClass;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::{
    LocalAiAdapterBoundary, LocalAiExecutionState, LocalAiProviderPrivacyMode,
    LocalAiProviderSource,
};
use ocentra_parent_agent_protocol::policy_constants as policy;
use std::fmt::Display;

use crate::test_text::TestText;

pub(crate) fn input_with_rules(
    parent_rules: Vec<PolicyRule>,
    local_ai_result: Option<LocalAiSafetyResult>,
    evidence_references: Vec<ParentEvidenceReference>,
) -> PolicyDryRunEvaluationInput {
    PolicyDryRunEvaluationInput {
        decision_id: policy::TEST_DECISION_ID.to_string(),
        evaluated_at: policy::TEST_EVALUATED_AT.to_string(),
        observed_target: target(),
        observed_target_aliases: Vec::new(),
        parent_rules,
        local_ai_result,
        evidence_references,
        expires_at: Some(policy::TEST_EXPIRES_AT.to_string()),
    }
}

pub(crate) fn rule(
    rule_id: impl Display,
    action: PolicyAction,
    reason_code: impl Display,
    priority: i64,
) -> PolicyRule {
    let rule_id = TestText::from_display(rule_id);
    let reason_code = TestText::from_display(reason_code);
    PolicyRule {
        rule_id: rule_id.to_string(),
        target: target(),
        action,
        schedule_id: None,
        priority,
        reason_code: reason_code.to_string(),
        created_by: ParentActorReference {
            actor_id: policy::TEST_PARENT_ACTOR_ID.to_string(),
            role: ParentActorRole::Parent,
        },
        enabled: true,
        effective_from: None,
        effective_until: None,
    }
}

pub(crate) fn evidence() -> ParentEvidenceReference {
    ParentEvidenceReference {
        evidence_reference_id: policy::TEST_EVIDENCE_ID.to_string(),
        kind: ParentEvidenceReferenceKind::ActivityEvent,
        observed_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}

pub(crate) fn local_ai_result(
    action: PolicyAction,
    unknown_state: LocalAiUnknownState,
    reason_code: impl Display,
) -> LocalAiSafetyResult {
    let reason_code = TestText::from_display(reason_code);
    LocalAiSafetyResult {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        result_id: policy::TEST_AI_RESULT_ID.to_string(),
        request_id: policy::TEST_AI_REQUEST_ID.to_string(),
        action,
        confidence: 0.61,
        unknown_state,
        degraded_state: LocalAiDegradedState::None,
        reason_codes: vec![reason_code.to_string()],
        explanation_reference: None,
        evidence_references: vec![evidence()],
        parent_rule_references: Vec::new(),
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

fn target() -> PolicyTarget {
    PolicyTarget {
        target_id: policy::TEST_TARGET_ID.to_string(),
        target_type: PolicyTargetType::Domain,
        target_value: policy::TEST_TARGET_VALUE.to_string(),
    }
}
