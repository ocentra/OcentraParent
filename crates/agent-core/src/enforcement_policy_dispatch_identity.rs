use ocentra_parent_agent_protocol::constants::v08_enforcement_policy_dispatch as dispatch;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::{
    EnforcementPolicyDispatchReadModelEntry, EnforcementPolicyDispatchRejectionReason,
    EnforcementPolicyDispatchSourceState,
};

pub(super) fn validate_entry_identity(
    entry: &EnforcementPolicyDispatchReadModelEntry,
) -> Result<(), EnforcementPolicyDispatchRejectionReason> {
    if entry.intent.actor.actor_id.is_empty() {
        return Err(EnforcementPolicyDispatchRejectionReason::MissingActor);
    }
    if entry.intent.device.device_id != dispatch::LOCAL_DEV_AGENT_DEVICE_ID {
        return Err(EnforcementPolicyDispatchRejectionReason::WrongDevice);
    }
    if !has_dispatch_reference_prefix(&entry.intent.policy_decision_id, dispatch::PREFIX_POLICY)
        || !has_dispatch_reference_prefix(
            &entry.intent.policy_decision_ref,
            dispatch::PREFIX_DECISION,
        )
    {
        return Err(EnforcementPolicyDispatchRejectionReason::MissingPolicyDecision);
    }
    if entry.intent.policy_version != dispatch::POLICY_VERSION_V0_8_DISPATCH {
        return Err(EnforcementPolicyDispatchRejectionReason::StalePolicyVersion);
    }
    if !has_dispatch_reference_prefix(&entry.intent.schedule_ref, dispatch::PREFIX_SCHEDULE) {
        return Err(EnforcementPolicyDispatchRejectionReason::MissingScheduleOrBudget);
    }
    if entry.intent.evidence_references.is_empty() {
        return Err(EnforcementPolicyDispatchRejectionReason::MissingEvidence);
    }
    if entry.intent.route_ref != dispatch::LOCAL_DEV_AGENT_ROUTE_REF {
        return Err(EnforcementPolicyDispatchRejectionReason::RouteNotAuthorized);
    }
    if entry.intent.source_state != EnforcementPolicyDispatchSourceState::Ready
        && entry.matrix_row.outcome_state !=
            ocentra_parent_agent_protocol::enforcement_policy_dispatch::EnforcementPolicyDispatchOutcomeState::Rejected
    {
        return Err(EnforcementPolicyDispatchRejectionReason::SourceNotReady);
    }

    Ok(())
}

fn has_dispatch_reference_prefix(value: &str, prefix: &str) -> bool {
    value.starts_with(prefix) && value.len() > prefix.len()
}
