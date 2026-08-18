use crate::household_authority_runtime_composer::HouseholdAuthorityRuntimeFailure;

use super::ParentStorageConfirmationStoreError as StoreError;

pub(super) fn map_device_failure(failure: HouseholdAuthorityRuntimeFailure) -> StoreError {
    match failure {
        HouseholdAuthorityRuntimeFailure::DeviceTrustUnavailable => {
            StoreError::DeviceTrustUnavailable
        }
        HouseholdAuthorityRuntimeFailure::DeviceTrustRevoked
        | HouseholdAuthorityRuntimeFailure::DeviceTrustBindingMismatch
        | HouseholdAuthorityRuntimeFailure::DeviceTrustGenerationMismatch => {
            StoreError::DeviceTrustNotCurrent
        }
        HouseholdAuthorityRuntimeFailure::AccountAuthorityUnavailable
        | HouseholdAuthorityRuntimeFailure::AccountAuthorityRevoked
        | HouseholdAuthorityRuntimeFailure::AccountAuthorityStale
        | HouseholdAuthorityRuntimeFailure::AccountAuthorityGenerationMismatch
        | HouseholdAuthorityRuntimeFailure::SessionStale => StoreError::AccountAuthorityNotCurrent,
        failure @ (HouseholdAuthorityRuntimeFailure::CapabilityUnavailable
        | HouseholdAuthorityRuntimeFailure::CapabilityExpired
        | HouseholdAuthorityRuntimeFailure::CapabilityRevoked
        | HouseholdAuthorityRuntimeFailure::CapabilityBindingMismatch) => {
            super::owner_failure::map_capability_failure(failure)
        }
        failure @ (HouseholdAuthorityRuntimeFailure::ControllerLeaseUnavailable
        | HouseholdAuthorityRuntimeFailure::ControllerLeaseExpired
        | HouseholdAuthorityRuntimeFailure::ControllerLeaseRevoked
        | HouseholdAuthorityRuntimeFailure::ControllerLeaseBindingMismatch) => {
            super::owner_failure::map_controller_lease_failure(failure)
        }
        failure @ (HouseholdAuthorityRuntimeFailure::ParentStepUpUnavailable
        | HouseholdAuthorityRuntimeFailure::ParentStepUpExpired
        | HouseholdAuthorityRuntimeFailure::ParentStepUpReplayRejected
        | HouseholdAuthorityRuntimeFailure::ParentStepUpBindingMismatch) => {
            super::step_up_failure::map_parent_step_up_failure(failure)
        }
        failure @ (HouseholdAuthorityRuntimeFailure::RuntimeFenceUnavailable
        | HouseholdAuthorityRuntimeFailure::EffectTargetMismatch
        | HouseholdAuthorityRuntimeFailure::RoleNotAuthorized
        | HouseholdAuthorityRuntimeFailure::ManualRequired) => {
            super::terminal_failure::map_terminal_owner_failure(failure)
        }
        HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationStore(failure) => failure,
    }
}
