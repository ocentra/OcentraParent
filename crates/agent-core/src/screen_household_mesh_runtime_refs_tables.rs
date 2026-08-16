use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenHouseholdMeshPhase, ScreenMeshChildValidationState, ScreenMeshClaimState,
    ScreenMeshLeaseState, ScreenMeshPolicyState, ScreenMeshProviderResultState,
};

struct ScreenPhaseStateRow {
    phase: ScreenHouseholdMeshPhase,
    previous_phase_ref: Option<&'static str>,
    claim_state: ScreenMeshClaimState,
    lease_state: ScreenMeshLeaseState,
    provider_result_state: ScreenMeshProviderResultState,
    child_validation_state: ScreenMeshChildValidationState,
    policy_state: ScreenMeshPolicyState,
}

const SCREEN_PHASE_ROWS: [ScreenPhaseStateRow; 8] = [
    ScreenPhaseStateRow {
        phase: ScreenHouseholdMeshPhase::WorkQueued,
        previous_phase_ref: None,
        claim_state: ScreenMeshClaimState::NotRequested,
        lease_state: ScreenMeshLeaseState::NotCreated,
        provider_result_state: ScreenMeshProviderResultState::NotReturned,
        child_validation_state: ScreenMeshChildValidationState::NotReady,
        policy_state: ScreenMeshPolicyState::NotReady,
    },
    ScreenPhaseStateRow {
        phase: ScreenHouseholdMeshPhase::OfferPublished,
        previous_phase_ref: Some(constants::screen_flow::SCREEN_MESH_WORK_EVENT_REF),
        claim_state: ScreenMeshClaimState::NotRequested,
        lease_state: ScreenMeshLeaseState::NotCreated,
        provider_result_state: ScreenMeshProviderResultState::NotReturned,
        child_validation_state: ScreenMeshChildValidationState::NotReady,
        policy_state: ScreenMeshPolicyState::NotReady,
    },
    ScreenPhaseStateRow {
        phase: ScreenHouseholdMeshPhase::ClaimRequested,
        previous_phase_ref: Some(constants::screen_flow::SCREEN_MESH_OFFER_EVENT_REF),
        claim_state: ScreenMeshClaimState::Requested,
        lease_state: ScreenMeshLeaseState::NotCreated,
        provider_result_state: ScreenMeshProviderResultState::NotReturned,
        child_validation_state: ScreenMeshChildValidationState::NotReady,
        policy_state: ScreenMeshPolicyState::NotReady,
    },
    ScreenPhaseStateRow {
        phase: ScreenHouseholdMeshPhase::ClaimGranted,
        previous_phase_ref: Some(constants::screen_flow::SCREEN_MESH_CLAIM_REQUEST_EVENT_REF),
        claim_state: ScreenMeshClaimState::Granted,
        lease_state: ScreenMeshLeaseState::NotCreated,
        provider_result_state: ScreenMeshProviderResultState::NotReturned,
        child_validation_state: ScreenMeshChildValidationState::NotReady,
        policy_state: ScreenMeshPolicyState::NotReady,
    },
    ScreenPhaseStateRow {
        phase: ScreenHouseholdMeshPhase::LeaseCreated,
        previous_phase_ref: Some(constants::screen_flow::SCREEN_MESH_CLAIM_GRANT_EVENT_REF),
        claim_state: ScreenMeshClaimState::Granted,
        lease_state: ScreenMeshLeaseState::Active,
        provider_result_state: ScreenMeshProviderResultState::NotReturned,
        child_validation_state: ScreenMeshChildValidationState::NotReady,
        policy_state: ScreenMeshPolicyState::NotReady,
    },
    ScreenPhaseStateRow {
        phase: ScreenHouseholdMeshPhase::ProviderResultReturned,
        previous_phase_ref: Some(constants::screen_flow::SCREEN_MESH_LEASE_EVENT_REF),
        claim_state: ScreenMeshClaimState::Granted,
        lease_state: ScreenMeshLeaseState::Active,
        provider_result_state: ScreenMeshProviderResultState::Returned,
        child_validation_state: ScreenMeshChildValidationState::Requested,
        policy_state: ScreenMeshPolicyState::NotReady,
    },
    ScreenPhaseStateRow {
        phase: ScreenHouseholdMeshPhase::ChildResultAccepted,
        previous_phase_ref: Some(constants::screen_flow::SCREEN_MESH_PROVIDER_RESULT_EVENT_REF),
        claim_state: ScreenMeshClaimState::Granted,
        lease_state: ScreenMeshLeaseState::Active,
        provider_result_state: ScreenMeshProviderResultState::Returned,
        child_validation_state: ScreenMeshChildValidationState::Accepted,
        policy_state: ScreenMeshPolicyState::NotReady,
    },
    ScreenPhaseStateRow {
        phase: ScreenHouseholdMeshPhase::PolicyRequested,
        previous_phase_ref: Some(constants::screen_flow::SCREEN_MESH_CHILD_ACCEPTED_EVENT_REF),
        claim_state: ScreenMeshClaimState::Granted,
        lease_state: ScreenMeshLeaseState::Active,
        provider_result_state: ScreenMeshProviderResultState::Returned,
        child_validation_state: ScreenMeshChildValidationState::Accepted,
        policy_state: ScreenMeshPolicyState::Ready,
    },
];

pub(crate) fn mesh_aggregate_key(queue_job_id: &str) -> String {
    let mut value = String::from(constants::screen_flow::AGGREGATE_SCREEN_QUEUE_PREFIX);
    value.push_str(queue_job_id);
    value
}

pub(crate) fn previous_mesh_phase_ref(phase: ScreenHouseholdMeshPhase) -> Option<String> {
    SCREEN_PHASE_ROWS
        .iter()
        .find_map(|row| (row.phase == phase).then(|| row.previous_phase_ref.map(str::to_string)))
        .flatten()
}

pub(crate) fn claim_state(phase: ScreenHouseholdMeshPhase) -> ScreenMeshClaimState {
    screen_phase_row(phase)
        .map(|row| row.claim_state)
        .unwrap_or(ScreenMeshClaimState::NotRequested)
}

pub(crate) fn lease_state(phase: ScreenHouseholdMeshPhase) -> ScreenMeshLeaseState {
    screen_phase_row(phase)
        .map(|row| row.lease_state)
        .unwrap_or(ScreenMeshLeaseState::NotCreated)
}

pub(crate) fn provider_result_state(
    phase: ScreenHouseholdMeshPhase,
) -> ScreenMeshProviderResultState {
    screen_phase_row(phase)
        .map(|row| row.provider_result_state)
        .unwrap_or(ScreenMeshProviderResultState::NotReturned)
}

pub(crate) fn child_validation_state(
    phase: ScreenHouseholdMeshPhase,
) -> ScreenMeshChildValidationState {
    screen_phase_row(phase)
        .map(|row| row.child_validation_state)
        .unwrap_or(ScreenMeshChildValidationState::NotReady)
}

pub(crate) fn policy_state(phase: ScreenHouseholdMeshPhase) -> ScreenMeshPolicyState {
    screen_phase_row(phase)
        .map(|row| row.policy_state)
        .unwrap_or(ScreenMeshPolicyState::NotReady)
}

fn screen_phase_row(phase: ScreenHouseholdMeshPhase) -> Option<&'static ScreenPhaseStateRow> {
    SCREEN_PHASE_ROWS.iter().find(|row| row.phase == phase)
}
