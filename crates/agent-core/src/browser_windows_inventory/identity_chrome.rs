use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::constants;

use super::{BrowserWindowsExecutableIdentity, BrowserWindowsSupportKind};

pub(super) fn chrome_identity(components: &[String]) -> BrowserWindowsExecutableIdentity {
    if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_CHROME_FOR_TESTING_NORMALIZED)
    {
        return BrowserWindowsExecutableIdentity {
            browser_family: BrowserFamily::Chrome,
            browser_channel: BrowserChannel::Stable,
            product_name: constants::browser::PRODUCT_NAME_CHROME_FOR_TESTING,
            supports_managed_cdp: true,
            support_kind: BrowserWindowsSupportKind::ManagedChromium,
        };
    }
    if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_CHROMIUM_NORMALIZED)
    {
        return BrowserWindowsExecutableIdentity {
            browser_family: BrowserFamily::UnknownChromium,
            browser_channel: BrowserChannel::Unknown,
            product_name: constants::browser::PRODUCT_NAME_CHROMIUM,
            supports_managed_cdp: false,
            support_kind: BrowserWindowsSupportKind::ManualChromium,
        };
    }
    super::identity_channel::browser_identity(
        BrowserFamily::Chrome,
        super::identity_channel::browser_channel_from_components(components),
        constants::browser::PRODUCT_NAME_GOOGLE_CHROME,
        true,
        BrowserWindowsSupportKind::ManagedChromium,
    )
}
