use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::PolicyDecision;
use ocentra_parent_agent_protocol::enforcement::EnforcementIntent;

use super::EnforcementBoundaryRejection;

pub(super) fn validate_intent_decision(
    intent: &EnforcementIntent,
    decision: &PolicyDecision,
) -> Result<(), EnforcementBoundaryRejection> {
    if intent.policy_decision_id != decision.decision_id {
        return Err(EnforcementBoundaryRejection::PolicyDecisionIdMismatch);
    }
    if intent.requested_action != decision.action {
        return Err(EnforcementBoundaryRejection::PolicyActionMismatch);
    }
    if intent.evidence_references.is_empty() || decision.evidence_references.is_empty() {
        return Err(EnforcementBoundaryRejection::MissingPolicyEvidenceReference);
    }
    if !intent
        .evidence_references
        .iter()
        .all(|intent_ref| evidence_ref_is_in_decision(intent_ref, &decision.evidence_references))
    {
        return Err(EnforcementBoundaryRejection::MissingPolicyEvidenceReference);
    }

    Ok(())
}

fn evidence_ref_is_in_decision(
    intent_ref: &ParentEvidenceReference,
    decision_refs: &[ParentEvidenceReference],
) -> bool {
    decision_refs
        .iter()
        .any(|decision_ref| decision_ref.evidence_reference_id == intent_ref.evidence_reference_id)
}
