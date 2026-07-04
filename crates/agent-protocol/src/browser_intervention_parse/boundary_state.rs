use super::protocol_lookup;
use crate::{constants, BrowserBoundaryState};

impl BrowserBoundaryState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::INTERVENTION_BOUNDARY_MANAGED_SESSION,
                    Self::ManagedSession,
                ),
                (
                    constants::browser::INTERVENTION_BOUNDARY_UNMANAGED_BROWSER_PROCESS,
                    Self::UnmanagedBrowserProcess,
                ),
                (
                    constants::browser::INTERVENTION_BOUNDARY_BROWSER_LIKE_PROCESS,
                    Self::BrowserLikeProcess,
                ),
                (
                    constants::browser::INTERVENTION_BOUNDARY_UNSUPPORTED,
                    Self::Unsupported,
                ),
                (
                    constants::browser::INTERVENTION_BOUNDARY_UNKNOWN,
                    Self::Unknown,
                ),
            ],
        )
    }
}
