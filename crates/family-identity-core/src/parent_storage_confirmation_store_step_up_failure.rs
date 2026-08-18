use crate::household_authority_runtime_composer::{
    HouseholdAuthorityParentStorageOwnerFailure, HouseholdAuthorityRuntimeFailure,
};

use super::ParentStorageConfirmationStoreError;

pub(super) fn map_parent_step_up_failure(
    failure: HouseholdAuthorityRuntimeFailure,
) -> ParentStorageConfirmationStoreError {
    match failure {
        HouseholdAuthorityRuntimeFailure::ParentStepUpUnavailable => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::ParentStepUpUnavailable,
            )
        }
        HouseholdAuthorityRuntimeFailure::ParentStepUpExpired => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::ParentStepUpExpired,
            )
        }
        HouseholdAuthorityRuntimeFailure::ParentStepUpReplayRejected => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::ParentStepUpReplayRejected,
            )
        }
        HouseholdAuthorityRuntimeFailure::ParentStepUpBindingMismatch => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::ParentStepUpBindingMismatch,
            )
        }
        HouseholdAuthorityRuntimeFailure::AccountAuthorityUnavailable
        | HouseholdAuthorityRuntimeFailure::AccountAuthorityRevoked
        | HouseholdAuthorityRuntimeFailure::AccountAuthorityStale
        | HouseholdAuthorityRuntimeFailure::AccountAuthorityGenerationMismatch
        | HouseholdAuthorityRuntimeFailure::SessionStale
        | HouseholdAuthorityRuntimeFailure::DeviceTrustUnavailable
        | HouseholdAuthorityRuntimeFailure::DeviceTrustRevoked
        | HouseholdAuthorityRuntimeFailure::DeviceTrustBindingMismatch
        | HouseholdAuthorityRuntimeFailure::DeviceTrustGenerationMismatch
        | HouseholdAuthorityRuntimeFailure::CapabilityUnavailable
        | HouseholdAuthorityRuntimeFailure::CapabilityExpired
        | HouseholdAuthorityRuntimeFailure::CapabilityRevoked
        | HouseholdAuthorityRuntimeFailure::CapabilityBindingMismatch
        | HouseholdAuthorityRuntimeFailure::ControllerLeaseUnavailable
        | HouseholdAuthorityRuntimeFailure::ControllerLeaseExpired
        | HouseholdAuthorityRuntimeFailure::ControllerLeaseRevoked
        | HouseholdAuthorityRuntimeFailure::ControllerLeaseBindingMismatch
        | HouseholdAuthorityRuntimeFailure::RuntimeFenceUnavailable
        | HouseholdAuthorityRuntimeFailure::EffectTargetMismatch
        | HouseholdAuthorityRuntimeFailure::RoleNotAuthorized
        | HouseholdAuthorityRuntimeFailure::ManualRequired
        | HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationStore(_) => {
            ParentStorageConfirmationStoreError::IntegrityRejected
        }
    }
}
