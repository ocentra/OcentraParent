use ocentra_parent_agent_protocol::{
    LocalAiParentRuleContextRef, ParentEvidenceReference, PolicyTarget,
};

pub(crate) fn parent_rule_contexts_for_row(
    target: &PolicyTarget,
    evidence_references: &[ParentEvidenceReference],
    generated_at: &str,
    parent_rule_contexts: &[LocalAiParentRuleContextRef],
) -> Vec<LocalAiParentRuleContextRef> {
    parent_rule_contexts
        .iter()
        .filter(|context| {
            parent_rule_context_matches(context, target, evidence_references, generated_at)
        })
        .cloned()
        .collect()
}

fn parent_rule_context_matches(
    context: &LocalAiParentRuleContextRef,
    target: &PolicyTarget,
    evidence_references: &[ParentEvidenceReference],
    generated_at: &str,
) -> bool {
    context.rule.enabled
        && context_matches_target(context, target)
        && context_is_current(context, generated_at)
        && context_references_evidence(context, evidence_references)
}

fn context_matches_target(context: &LocalAiParentRuleContextRef, target: &PolicyTarget) -> bool {
    context.rule.target.target_type == target.target_type
        && context.rule.target.target_value == target.target_value
}

fn context_is_current(context: &LocalAiParentRuleContextRef, generated_at: &str) -> bool {
    match &context.expires_at {
        Some(expires_at) => expires_at.as_str() > generated_at,
        None => true,
    }
}

fn context_references_evidence(
    context: &LocalAiParentRuleContextRef,
    evidence_references: &[ParentEvidenceReference],
) -> bool {
    !context.target_evidence_refs.is_empty()
        && context.target_evidence_refs.iter().any(|target_ref| {
            evidence_references
                .iter()
                .any(|evidence| evidence.evidence_reference_id == *target_ref)
        })
}
