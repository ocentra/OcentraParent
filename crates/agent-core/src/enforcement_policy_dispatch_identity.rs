use ocentra_parent_agent_protocol::constants::v08_enforcement_policy_dispatch as dispatch;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::{
    EnforcementPolicyDispatchOutcomeState, EnforcementPolicyDispatchReadModelEntry,
    EnforcementPolicyDispatchRejectionReason, EnforcementPolicyDispatchSourceState,
};
use ocentra_parent_agent_protocol::policy_constants;

#[path = "enforcement_policy_dispatch_identity_references.rs"]
mod enforcement_policy_dispatch_identity_references;

pub(super) fn validate_entry_identity(
    entry: &EnforcementPolicyDispatchReadModelEntry,
) -> Result<(), EnforcementPolicyDispatchRejectionReason> {
    if entry.schema_version != policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
        || entry.intent.schema_version != policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
    {
        return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
    }
    if entry.intent.intent_id.trim().is_empty() || entry.intent.requested_at.trim().is_empty() {
        return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
    }
    if entry.intent.actor.actor_id.trim().is_empty() {
        return Err(EnforcementPolicyDispatchRejectionReason::MissingActor);
    }
    if entry.intent.device.device_id != dispatch::LOCAL_DEV_AGENT_DEVICE_ID
        || entry.intent.device.label.trim().is_empty()
        || entry
            .intent
            .device
            .child_profile_id
            .as_deref()
            .is_some_and(|profile_id| profile_id.trim().is_empty())
    {
        return Err(EnforcementPolicyDispatchRejectionReason::WrongDevice);
    }
    if !enforcement_policy_dispatch_identity_references::matching_policy_decision_references(entry)
    {
        return Err(EnforcementPolicyDispatchRejectionReason::MissingPolicyDecision);
    }
    if entry.intent.policy_version != dispatch::POLICY_VERSION_V0_8_DISPATCH {
        return Err(EnforcementPolicyDispatchRejectionReason::StalePolicyVersion);
    }
    if !enforcement_policy_dispatch_identity_references::has_dispatch_reference_prefix(
        &entry.intent.schedule_ref,
        dispatch::PREFIX_SCHEDULE,
    ) {
        return Err(EnforcementPolicyDispatchRejectionReason::MissingScheduleOrBudget);
    }
    if !enforcement_policy_dispatch_identity_references::valid_evidence_references(entry) {
        return Err(EnforcementPolicyDispatchRejectionReason::MissingEvidence);
    }
    if !enforcement_policy_dispatch_identity_references::valid_target_reference(entry) {
        return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
    }
    if entry.intent.route_ref != dispatch::LOCAL_DEV_AGENT_ROUTE_REF {
        return Err(EnforcementPolicyDispatchRejectionReason::RouteNotAuthorized);
    }
    if entry.intent.source_state != EnforcementPolicyDispatchSourceState::Ready
        && entry.matrix_row.outcome_state != EnforcementPolicyDispatchOutcomeState::Rejected
    {
        return Err(EnforcementPolicyDispatchRejectionReason::SourceNotReady);
    }
    if !enforcement_policy_dispatch_identity_references::valid_related_references(entry) {
        return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
    }

    Ok(())
}
