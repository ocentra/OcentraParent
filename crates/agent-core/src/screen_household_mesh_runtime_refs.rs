use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenHouseholdMeshPhase, ScreenMeshChildValidationState, ScreenMeshClaimState,
    ScreenMeshLeaseState, ScreenMeshPolicyState, ScreenMeshProviderResultState,
};

#[path = "screen_household_mesh_runtime_refs_tables.rs"]
mod screen_household_mesh_runtime_refs_tables;

pub(crate) fn mesh_aggregate_key(queue_job_id: &str) -> String {
    screen_household_mesh_runtime_refs_tables::mesh_aggregate_key(queue_job_id)
}

pub(crate) fn previous_mesh_phase_ref(phase: ScreenHouseholdMeshPhase) -> Option<String> {
    screen_household_mesh_runtime_refs_tables::previous_mesh_phase_ref(phase)
}

pub(crate) fn claim_state(phase: ScreenHouseholdMeshPhase) -> ScreenMeshClaimState {
    screen_household_mesh_runtime_refs_tables::claim_state(phase)
}

pub(crate) fn lease_state(phase: ScreenHouseholdMeshPhase) -> ScreenMeshLeaseState {
    screen_household_mesh_runtime_refs_tables::lease_state(phase)
}

pub(crate) fn provider_result_state(
    phase: ScreenHouseholdMeshPhase,
) -> ScreenMeshProviderResultState {
    screen_household_mesh_runtime_refs_tables::provider_result_state(phase)
}

pub(crate) fn child_validation_state(
    phase: ScreenHouseholdMeshPhase,
) -> ScreenMeshChildValidationState {
    screen_household_mesh_runtime_refs_tables::child_validation_state(phase)
}

pub(crate) fn policy_state(phase: ScreenHouseholdMeshPhase) -> ScreenMeshPolicyState {
    screen_household_mesh_runtime_refs_tables::policy_state(phase)
}
