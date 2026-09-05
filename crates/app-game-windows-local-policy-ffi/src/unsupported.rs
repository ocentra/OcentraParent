use crate::{
    error::AppGameWindowsLocalPolicyError, observation::AppGameWindowsLocalPolicyObservation,
    Result,
};

pub(super) fn observe() -> Result<AppGameWindowsLocalPolicyObservation> {
    Err(AppGameWindowsLocalPolicyError::UnsupportedPlatform)
}
