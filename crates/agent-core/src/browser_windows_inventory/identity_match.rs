use std::path::Path;

use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::constants;

use super::{BrowserWindowsExecutableIdentity, BrowserWindowsSupportKind};

pub(super) fn windows_browser_executable_identity(path: &Path) -> BrowserWindowsExecutableIdentity {
    let executable_name = super::executable_name_normalized(path);
    let components = super::normalized_component_names(path);
    match executable_name.as_str() {
        constants::browser::EXECUTABLE_MSEDGE_WINDOWS
        | constants::browser::EXECUTABLE_MSEDGE_LINUX
        | constants::browser::EXECUTABLE_MICROSOFT_EDGE_LINUX => {
            super::identity_channel::browser_identity(
                BrowserFamily::Edge,
                browser_channel_from_components(&components),
                constants::browser::PRODUCT_NAME_MICROSOFT_EDGE,
                true,
                BrowserWindowsSupportKind::ManagedChromium,
            )
        }
        constants::browser::EXECUTABLE_CHROME_WINDOWS
        | constants::browser::EXECUTABLE_CHROME_LINUX
        | constants::browser::EXECUTABLE_GOOGLE_CHROME_LINUX => {
            super::identity_chrome::chrome_identity(&components)
        }
        constants::browser::EXECUTABLE_BRAVE_WINDOWS => {
            super::identity_unsupported::manual_identity(
                BrowserFamily::Brave,
                BrowserChannel::Stable,
                constants::browser::PRODUCT_NAME_BRAVE_BROWSER,
                BrowserWindowsSupportKind::ManualChromium,
            )
        }
        constants::browser::EXECUTABLE_VIVALDI_WINDOWS => {
            super::identity_unsupported::manual_identity(
                BrowserFamily::UnknownChromium,
                BrowserChannel::Stable,
                constants::browser::PRODUCT_NAME_VIVALDI_BROWSER,
                BrowserWindowsSupportKind::ManualChromium,
            )
        }
        constants::browser::EXECUTABLE_OPERA_WINDOWS
        | constants::browser::EXECUTABLE_OPERA_GX_WINDOWS => {
            super::identity_opera::opera_identity(&components)
        }
        constants::browser::EXECUTABLE_CHROMIUM_WINDOWS => {
            super::identity_unsupported::manual_identity(
                BrowserFamily::UnknownChromium,
                BrowserChannel::Unknown,
                constants::browser::PRODUCT_NAME_CHROMIUM,
                BrowserWindowsSupportKind::ManualChromium,
            )
        }
        constants::browser::EXECUTABLE_FIREFOX_WINDOWS => {
            super::identity_firefox::firefox_identity(&components)
        }
        constants::browser::EXECUTABLE_TOR_WINDOWS => {
            super::identity_unsupported::unsupported_identity(
                BrowserFamily::Unknown,
                constants::browser::PRODUCT_NAME_TOR_BROWSER,
            )
        }
        constants::browser::EXECUTABLE_DUCKDUCKGO_WINDOWS => {
            super::identity_unsupported::unsupported_identity(
                BrowserFamily::Unknown,
                constants::browser::PRODUCT_NAME_DUCKDUCKGO_BROWSER,
            )
        }
        constants::browser::EXECUTABLE_ARC_WINDOWS => {
            super::identity_unsupported::unsupported_identity(
                BrowserFamily::UnknownChromium,
                constants::browser::PRODUCT_NAME_ARC_BROWSER,
            )
        }
        _ => super::identity_unsupported::unknown_identity(),
    }
}

fn browser_channel_from_components(components: &[String]) -> BrowserChannel {
    super::identity_channel::browser_channel_from_components(components)
}
