use crate::device_trust_lifecycle::DeviceTrustLifecycleError;

use super::DeviceTrustRuntimeFenceError;

impl From<DeviceTrustLifecycleError> for DeviceTrustRuntimeFenceError {
    fn from(error: DeviceTrustLifecycleError) -> Self {
        match error {
            DeviceTrustLifecycleError::RevokedDevice | DeviceTrustLifecycleError::InvalidState => {
                Self::DeviceTrustRevoked
            }
            DeviceTrustLifecycleError::ParentReauthorizationRequired
            | DeviceTrustLifecycleError::InvalidGeneration => Self::GenerationMismatch,
            DeviceTrustLifecycleError::InvalidIdentity
            | DeviceTrustLifecycleError::InvalidSignerKey => Self::InvalidTarget,
            DeviceTrustLifecycleError::RegistrationMissing
            | DeviceTrustLifecycleError::SignerRegistrationMissing => Self::DeviceTrustUnavailable,
            DeviceTrustLifecycleError::Unavailable
            | DeviceTrustLifecycleError::DuplicateRegistration
            | DeviceTrustLifecycleError::DuplicateSignerRegistration
            | DeviceTrustLifecycleError::SignerRegistrationConflict => Self::Unavailable,
        }
    }
}
