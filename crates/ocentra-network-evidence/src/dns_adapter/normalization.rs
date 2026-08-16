use super::{redirects, refs};
use super::{
    NetworkDnsAdapterArtifactRefs, NetworkDnsAdapterProofError, NetworkDnsAdapterProofInput,
    NetworkDnsAdapterRequiredArtifact, NormalizedDnsAdapterInput,
};

pub(super) fn normalize_dns_adapter_input(
    input: &NetworkDnsAdapterProofInput,
) -> Result<NormalizedDnsAdapterInput, NetworkDnsAdapterProofError> {
    Ok(NormalizedDnsAdapterInput {
        dns_adapter_plan_ref: refs::normalize_ref(&input.dns_adapter_plan_ref)
            .ok_or(NetworkDnsAdapterProofError::EmptyDnsAdapterPlanRef)?,
        policy_decision_ref: refs::normalize_ref(&input.policy_mapping.policy_decision_ref)
            .ok_or(NetworkDnsAdapterProofError::EmptyPolicyDecisionRef)?,
        parent_rule_ref: refs::normalize_ref(&input.policy_mapping.parent_rule_ref)
            .ok_or(NetworkDnsAdapterProofError::EmptyParentRuleRef)?,
        evidence_refs: refs::normalized_refs(&input.policy_mapping.evidence_refs)?,
        local_ai_result_ref: refs::normalized_local_ai_ref(
            input.policy_mapping.local_ai_result_ref.as_deref(),
        )?,
        target_domain: redirects::normalized_target_domain(&input.target_domain)?,
        redirect_target_domain: redirects::normalized_redirect_target(
            input.requested_action,
            input,
        )?,
    })
}

pub(super) fn normalize_artifact_refs(
    input: &NetworkDnsAdapterProofInput,
) -> Result<NetworkDnsAdapterArtifactRefs, NetworkDnsAdapterProofError> {
    Ok(NetworkDnsAdapterArtifactRefs {
        adapter_authorization_ref: refs::normalized_artifact_ref(
            input.adapter_authorization_ref.as_deref(),
            NetworkDnsAdapterRequiredArtifact::AdapterAuthorization,
        )?,
        adapter_capability_proof_ref: refs::normalized_artifact_ref(
            input.adapter_capability_proof_ref.as_deref(),
            NetworkDnsAdapterRequiredArtifact::CapabilityProof,
        )?,
        apply_artifact_ref: refs::normalized_artifact_ref(
            input.apply_artifact_ref.as_deref(),
            NetworkDnsAdapterRequiredArtifact::ApplyArtifact,
        )?,
        result_artifact_ref: refs::normalized_artifact_ref(
            input.result_artifact_ref.as_deref(),
            NetworkDnsAdapterRequiredArtifact::ResultArtifact,
        )?,
        rollback_artifact_ref: refs::normalized_artifact_ref(
            input.rollback_artifact_ref.as_deref(),
            NetworkDnsAdapterRequiredArtifact::RollbackArtifact,
        )?,
        audit_event_ref: refs::normalized_artifact_ref(
            input.audit_event_ref.as_deref(),
            NetworkDnsAdapterRequiredArtifact::AuditEvent,
        )?,
    })
}
