use super::protocol_lookup;
use crate::{constants, BrowserUnmanagedProcessKind};

impl BrowserUnmanagedProcessKind {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::UNMANAGED_PROCESS_KIND_SUPPORTED_BROWSER,
                    Self::SupportedBrowser,
                ),
                (
                    constants::browser::UNMANAGED_PROCESS_KIND_UNSUPPORTED_BROWSER,
                    Self::UnsupportedBrowser,
                ),
                (
                    constants::browser::UNMANAGED_PROCESS_KIND_PORTABLE_BROWSER,
                    Self::PortableBrowser,
                ),
                (
                    constants::browser::UNMANAGED_PROCESS_KIND_TOR_PRIVACY_BROWSER,
                    Self::TorPrivacyBrowser,
                ),
                (
                    constants::browser::UNMANAGED_PROCESS_KIND_PACKAGED_BROWSER,
                    Self::PackagedBrowser,
                ),
                (
                    constants::browser::UNMANAGED_PROCESS_KIND_EMBEDDED_BROWSER_LIKE,
                    Self::EmbeddedBrowserLike,
                ),
                (
                    constants::browser::UNMANAGED_PROCESS_KIND_UNKNOWN_BROWSER_LIKE,
                    Self::UnknownBrowserLike,
                ),
                (
                    constants::browser::UNMANAGED_PROCESS_KIND_POSSIBLE_SOCIAL_BYPASS,
                    Self::PossibleSocialBypass,
                ),
                (
                    constants::browser::UNMANAGED_PROCESS_KIND_POSSIBLE_BROWSER_GAME_BYPASS,
                    Self::PossibleBrowserGameBypass,
                ),
                (
                    constants::browser::UNMANAGED_PROCESS_KIND_POSSIBLE_CLOUD_GAMING_BYPASS,
                    Self::PossibleCloudGamingBypass,
                ),
            ],
        )
    }
}
