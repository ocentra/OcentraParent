//! Windows local-policy evidence sampler for App/Game proof surfaces.

pub mod error;
pub mod observation;
mod output;

#[cfg(not(windows))]
mod unsupported;
#[cfg(windows)]
mod windows;

pub const APP_GAME_WINDOWS_LOCAL_POLICY_OUTPUT_MAX_BYTES: usize = 4 * 1024;
pub const APP_GAME_WINDOWS_LOCAL_POLICY_MAX_COLLECTION_COUNT: u64 = 16;
pub const APP_GAME_WINDOWS_LOCAL_POLICY_MAX_RULE_COUNT: u64 = 100_000;

pub type Result<T> = core::result::Result<T, error::AppGameWindowsLocalPolicyError>;

pub fn parse_local_policy_output(
    output: &[u8],
) -> Result<observation::AppGameWindowsLocalPolicyObservation> {
    output::parse(output)
}

#[cfg(windows)]
pub fn observe_local_policy() -> Result<observation::AppGameWindowsLocalPolicyObservation> {
    windows::observe()
}

#[cfg(not(windows))]
pub fn observe_local_policy() -> Result<observation::AppGameWindowsLocalPolicyObservation> {
    unsupported::observe()
}
