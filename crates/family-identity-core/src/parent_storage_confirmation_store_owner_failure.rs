use crate::household_authority_runtime_composer::{
    HouseholdAuthorityParentStorageOwnerFailure, HouseholdAuthorityRuntimeFailure,
};

use super::ParentStorageConfirmationStoreError;

pub(super) fn map_capability_failure(
    failure: HouseholdAuthorityRuntimeFailure,
) -> ParentStorageConfirmationStoreError {
    match failure {
        HouseholdAuthorityRuntimeFailure::CapabilityUnavailable => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::CapabilityUnavailable,
            )
        }
        HouseholdAuthorityRuntimeFailure::CapabilityExpired => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::CapabilityExpired,
            )
        }
        HouseholdAuthorityRuntimeFailure::CapabilityRevoked => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::CapabilityRevoked,
            )
        }
        HouseholdAuthorityRuntimeFailure::CapabilityBindingMismatch => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::CapabilityBindingMismatch,
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
        | HouseholdAuthorityRuntimeFailure::ControllerLeaseUnavailable
        | HouseholdAuthorityRuntimeFailure::ControllerLeaseExpired
        | HouseholdAuthorityRuntimeFailure::ControllerLeaseRevoked
        | HouseholdAuthorityRuntimeFailure::ControllerLeaseBindingMismatch
        | HouseholdAuthorityRuntimeFailure::ParentStepUpUnavailable
        | HouseholdAuthorityRuntimeFailure::ParentStepUpExpired
        | HouseholdAuthorityRuntimeFailure::ParentStepUpReplayRejected
        | HouseholdAuthorityRuntimeFailure::ParentStepUpBindingMismatch
        | HouseholdAuthorityRuntimeFailure::RuntimeFenceUnavailable
        | HouseholdAuthorityRuntimeFailure::EffectTargetMismatch
        | HouseholdAuthorityRuntimeFailure::RoleNotAuthorized
        | HouseholdAuthorityRuntimeFailure::ManualRequired
        | HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationStore(_) => {
            ParentStorageConfirmationStoreError::IntegrityRejected
        }
    }
}

pub(super) fn map_controller_lease_failure(
    failure: HouseholdAuthorityRuntimeFailure,
) -> ParentStorageConfirmationStoreError {
    match failure {
        HouseholdAuthorityRuntimeFailure::ControllerLeaseUnavailable => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::ControllerLeaseUnavailable,
            )
        }
        HouseholdAuthorityRuntimeFailure::ControllerLeaseExpired => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::ControllerLeaseExpired,
            )
        }
        HouseholdAuthorityRuntimeFailure::ControllerLeaseRevoked => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::ControllerLeaseRevoked,
            )
        }
        HouseholdAuthorityRuntimeFailure::ControllerLeaseBindingMismatch => {
            ParentStorageConfirmationStoreError::Owner(
                HouseholdAuthorityParentStorageOwnerFailure::ControllerLeaseBindingMismatch,
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
        | HouseholdAuthorityRuntimeFailure::ParentStepUpUnavailable
        | HouseholdAuthorityRuntimeFailure::ParentStepUpExpired
        | HouseholdAuthorityRuntimeFailure::ParentStepUpReplayRejected
        | HouseholdAuthorityRuntimeFailure::ParentStepUpBindingMismatch
        | HouseholdAuthorityRuntimeFailure::RuntimeFenceUnavailable
        | HouseholdAuthorityRuntimeFailure::EffectTargetMismatch
        | HouseholdAuthorityRuntimeFailure::RoleNotAuthorized
        | HouseholdAuthorityRuntimeFailure::ManualRequired
        | HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationStore(_) => {
            ParentStorageConfirmationStoreError::IntegrityRejected
        }
    }
}
