use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::constants;

use super::{BrowserWindowsExecutableIdentity, BrowserWindowsSupportKind};

pub(super) fn opera_identity(components: &[String]) -> BrowserWindowsExecutableIdentity {
    let product_name = if components.iter().any(|name| {
        name == constants::browser::PATH_SEGMENT_OPERA_GX_STABLE
            .to_ascii_lowercase()
            .as_str()
    }) {
        constants::browser::PRODUCT_NAME_OPERA_GX_BROWSER
    } else {
        constants::browser::PRODUCT_NAME_OPERA_BROWSER
    };
    super::identity_channel::browser_identity(
        BrowserFamily::Opera,
        BrowserChannel::Stable,
        product_name,
        false,
        BrowserWindowsSupportKind::ManualChromium,
    )
}
