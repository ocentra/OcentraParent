use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::PolicyTarget;
use ocentra_parent_agent_protocol::activity::policy_context::LocalAiParentRuleContextRef;

pub(crate) fn parent_rule_contexts_for_row(
    target: &PolicyTarget,
    target_aliases: &[PolicyTarget],
    evidence_references: &[ParentEvidenceReference],
    generated_at: &str,
    device_id: &str,
    platform: &str,
    parent_rule_contexts: &[LocalAiParentRuleContextRef],
) -> Vec<LocalAiParentRuleContextRef> {
    parent_rule_contexts
        .iter()
        .filter(|context| {
            parent_rule_context_matches(
                context,
                target,
                target_aliases,
                evidence_references,
                generated_at,
                device_id,
                platform,
            )
        })
        .cloned()
        .collect()
}

fn parent_rule_context_matches(
    context: &LocalAiParentRuleContextRef,
    target: &PolicyTarget,
    target_aliases: &[PolicyTarget],
    evidence_references: &[ParentEvidenceReference],
    generated_at: &str,
    device_id: &str,
    platform: &str,
) -> bool {
    context.rule.enabled
        && context_scope_matches_row(context, device_id, platform)
        && context_matches_target_or_alias(context, target, target_aliases)
        && context_is_current(context, generated_at)
        && context_rule_has_supported_schedule(context)
        && context_rule_is_effective_at(context, generated_at)
        && context_references_evidence(context, evidence_references)
}

fn context_scope_matches_row(
    context: &LocalAiParentRuleContextRef,
    device_id: &str,
    platform: &str,
) -> bool {
    !context.family.family_id.is_empty()
        && !context.child_profile.child_profile_id.is_empty()
        && context.device.device_id == device_id
        && context.device.platform == platform
        && context.device.child_profile_id.as_deref()
            == Some(context.child_profile.child_profile_id.as_str())
}

fn context_matches_target_or_alias(
    context: &LocalAiParentRuleContextRef,
    target: &PolicyTarget,
    target_aliases: &[PolicyTarget],
) -> bool {
    context_matches_target(context, target)
        || target_aliases
            .iter()
            .any(|target_alias| context_matches_target(context, target_alias))
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

fn context_rule_has_supported_schedule(context: &LocalAiParentRuleContextRef) -> bool {
    context.rule.schedule_id.is_none()
}

fn context_rule_is_effective_at(context: &LocalAiParentRuleContextRef, generated_at: &str) -> bool {
    if let Some(effective_from) = &context.rule.effective_from {
        if effective_from.as_str() > generated_at {
            return false;
        }
    }

    if let Some(effective_until) = &context.rule.effective_until {
        if effective_until.as_str() <= generated_at {
            return false;
        }
    }

    true
}

fn context_references_evidence(
    context: &LocalAiParentRuleContextRef,
    evidence_references: &[ParentEvidenceReference],
) -> bool {
    !context.target_evidence_refs.is_empty()
        && context.target_evidence_refs.iter().all(|target_ref| {
            evidence_references
                .iter()
                .any(|evidence| evidence.evidence_reference_id == *target_ref)
        })
}
