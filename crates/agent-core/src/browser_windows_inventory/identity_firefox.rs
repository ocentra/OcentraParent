use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::constants;

use super::{BrowserWindowsExecutableIdentity, BrowserWindowsSupportKind};

pub(super) fn firefox_identity(components: &[String]) -> BrowserWindowsExecutableIdentity {
    if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_TOR_BROWSER_NORMALIZED)
    {
        return super::identity_unsupported::unsupported_identity(
            BrowserFamily::Unknown,
            constants::browser::PRODUCT_NAME_TOR_BROWSER,
        );
    }
    let product_name = if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_FIREFOX_DEVELOPER_NORMALIZED)
    {
        constants::browser::PRODUCT_NAME_FIREFOX_DEVELOPER_EDITION
    } else {
        constants::browser::PRODUCT_NAME_MOZILLA_FIREFOX
    };
    super::identity_unsupported::manual_identity(
        BrowserFamily::Firefox,
        BrowserChannel::Unknown,
        product_name,
        BrowserWindowsSupportKind::Unsupported,
    )
}
