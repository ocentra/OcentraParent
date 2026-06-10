use ocentra_parent_agent_protocol::constants;

use crate::screen_household_mesh_runtime_phase::ScreenHouseholdMeshPhase;

pub(crate) fn mesh_aggregate_key(queue_job_id: &str) -> String {
    let mut value = String::from(constants::screen_flow::AGGREGATE_SCREEN_QUEUE_PREFIX);
    value.push_str(queue_job_id);
    value
}

pub(crate) fn previous_mesh_phase_ref(phase: ScreenHouseholdMeshPhase) -> Option<String> {
    let value = match phase {
        ScreenHouseholdMeshPhase::WorkQueued => return None,
        ScreenHouseholdMeshPhase::OfferPublished => {
            constants::screen_flow::SCREEN_MESH_WORK_EVENT_REF
        }
        ScreenHouseholdMeshPhase::ClaimRequested => {
            constants::screen_flow::SCREEN_MESH_OFFER_EVENT_REF
        }
        ScreenHouseholdMeshPhase::ClaimGranted => {
            constants::screen_flow::SCREEN_MESH_CLAIM_REQUEST_EVENT_REF
        }
        ScreenHouseholdMeshPhase::LeaseCreated => {
            constants::screen_flow::SCREEN_MESH_CLAIM_GRANT_EVENT_REF
        }
        ScreenHouseholdMeshPhase::ProviderResultReturned => {
            constants::screen_flow::SCREEN_MESH_LEASE_EVENT_REF
        }
        ScreenHouseholdMeshPhase::ChildResultAccepted => {
            constants::screen_flow::SCREEN_MESH_PROVIDER_RESULT_EVENT_REF
        }
        ScreenHouseholdMeshPhase::PolicyRequested => {
            constants::screen_flow::SCREEN_MESH_CHILD_ACCEPTED_EVENT_REF
        }
    };
    Some(value.to_string())
}

pub(crate) fn claim_state(phase: ScreenHouseholdMeshPhase) -> crate::ScreenMeshClaimState {
    match phase {
        ScreenHouseholdMeshPhase::WorkQueued | ScreenHouseholdMeshPhase::OfferPublished => {
            crate::ScreenMeshClaimState::NotRequested
        }
        ScreenHouseholdMeshPhase::ClaimRequested => crate::ScreenMeshClaimState::Requested,
        ScreenHouseholdMeshPhase::ClaimGranted
        | ScreenHouseholdMeshPhase::LeaseCreated
        | ScreenHouseholdMeshPhase::ProviderResultReturned
        | ScreenHouseholdMeshPhase::ChildResultAccepted
        | ScreenHouseholdMeshPhase::PolicyRequested => crate::ScreenMeshClaimState::Granted,
    }
}

pub(crate) fn lease_state(phase: ScreenHouseholdMeshPhase) -> crate::ScreenMeshLeaseState {
    match phase {
        ScreenHouseholdMeshPhase::LeaseCreated
        | ScreenHouseholdMeshPhase::ProviderResultReturned
        | ScreenHouseholdMeshPhase::ChildResultAccepted
        | ScreenHouseholdMeshPhase::PolicyRequested => crate::ScreenMeshLeaseState::Active,
        _ => crate::ScreenMeshLeaseState::NotCreated,
    }
}

pub(crate) fn provider_result_state(
    phase: ScreenHouseholdMeshPhase,
) -> crate::ScreenMeshProviderResultState {
    match phase {
        ScreenHouseholdMeshPhase::ProviderResultReturned
        | ScreenHouseholdMeshPhase::ChildResultAccepted
        | ScreenHouseholdMeshPhase::PolicyRequested => {
            crate::ScreenMeshProviderResultState::Returned
        }
        _ => crate::ScreenMeshProviderResultState::NotReturned,
    }
}

pub(crate) fn child_validation_state(
    phase: ScreenHouseholdMeshPhase,
) -> crate::ScreenMeshChildValidationState {
    match phase {
        ScreenHouseholdMeshPhase::ChildResultAccepted
        | ScreenHouseholdMeshPhase::PolicyRequested => {
            crate::ScreenMeshChildValidationState::Accepted
        }
        ScreenHouseholdMeshPhase::ProviderResultReturned => {
            crate::ScreenMeshChildValidationState::Requested
        }
        _ => crate::ScreenMeshChildValidationState::NotReady,
    }
}

pub(crate) fn policy_state(phase: ScreenHouseholdMeshPhase) -> crate::ScreenMeshPolicyState {
    match phase {
        ScreenHouseholdMeshPhase::PolicyRequested => crate::ScreenMeshPolicyState::Ready,
        _ => crate::ScreenMeshPolicyState::NotReady,
    }
}
