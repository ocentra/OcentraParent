use crate::household_authority_runtime_composer::{
    HouseholdAuthorityParentStorageOwnerFailure, HouseholdAuthorityRuntimeFailure,
};

use super::ParentStorageConfirmationStoreError;

pub(super) fn map_terminal_owner_failure(
    failure: HouseholdAuthorityRuntimeFailure,
) -> ParentStorageConfirmationStoreError {
    match failure {
        HouseholdAuthorityRuntimeFailure::RuntimeFenceUnavailable => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::RuntimeFenceUnavailable,
            )
        }
        HouseholdAuthorityRuntimeFailure::EffectTargetMismatch => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::EffectTargetMismatch,
            )
        }
        HouseholdAuthorityRuntimeFailure::RoleNotAuthorized => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::RoleNotAuthorized,
            )
        }
        HouseholdAuthorityRuntimeFailure::ManualRequired => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::ManualRequired,
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
        | HouseholdAuthorityRuntimeFailure::ParentStepUpUnavailable
        | HouseholdAuthorityRuntimeFailure::ParentStepUpExpired
        | HouseholdAuthorityRuntimeFailure::ParentStepUpReplayRejected
        | HouseholdAuthorityRuntimeFailure::ParentStepUpBindingMismatch
        | HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationStore(_) => {
            ParentStorageConfirmationStoreError::IntegrityRejected
        }
    }
}
