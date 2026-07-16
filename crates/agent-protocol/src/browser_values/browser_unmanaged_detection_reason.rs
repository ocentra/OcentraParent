use super::protocol_lookup;
use crate::{constants, BrowserUnmanagedDetectionReason};

impl BrowserUnmanagedDetectionReason {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::UNMANAGED_DETECTION_REASON_SUPPORTED_BROWSER_OUTSIDE_MANAGED_SESSION,
                    Self::SupportedBrowserOutsideManagedSession,
                ),
                (
                    constants::browser::UNMANAGED_DETECTION_REASON_UNSUPPORTED_BROWSER_PROCESS,
                    Self::UnsupportedBrowserProcess,
                ),
                (
                    constants::browser::UNMANAGED_DETECTION_REASON_PORTABLE_BROWSER_PROCESS,
                    Self::PortableBrowserProcess,
                ),
                (
                    constants::browser::UNMANAGED_DETECTION_REASON_TOR_PRIVACY_BROWSER_PROCESS,
                    Self::TorPrivacyBrowserProcess,
                ),
                (
                    constants::browser::UNMANAGED_DETECTION_REASON_PACKAGED_BROWSER_PROCESS,
                    Self::PackagedBrowserProcess,
                ),
                (
                    constants::browser::UNMANAGED_DETECTION_REASON_BROWSER_LIKE_PROCESS,
                    Self::BrowserLikeProcess,
                ),
                (
                    constants::browser::UNMANAGED_DETECTION_REASON_POSSIBLE_SOCIAL_BYPASS,
                    Self::PossibleSocialBypass,
                ),
                (
                    constants::browser::UNMANAGED_DETECTION_REASON_POSSIBLE_BROWSER_GAME_BYPASS,
                    Self::PossibleBrowserGameBypass,
                ),
                (
                    constants::browser::UNMANAGED_DETECTION_REASON_POSSIBLE_CLOUD_GAMING_BYPASS,
                    Self::PossibleCloudGamingBypass,
                ),
            ],
        )
    }
}
