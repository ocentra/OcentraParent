use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};

use super::{BrowserWindowsExecutableIdentity, BrowserWindowsSupportKind};

pub(super) fn manual_identity(
    browser_family: BrowserFamily,
    browser_channel: BrowserChannel,
    product_name: &'static str,
    support_kind: BrowserWindowsSupportKind,
) -> BrowserWindowsExecutableIdentity {
    BrowserWindowsExecutableIdentity {
        browser_family,
        browser_channel,
        product_name,
        supports_managed_cdp: matches!(support_kind, BrowserWindowsSupportKind::ManagedChromium),
        support_kind,
    }
}

pub(super) fn unsupported_identity(
    browser_family: BrowserFamily,
    product_name: &'static str,
) -> BrowserWindowsExecutableIdentity {
    manual_identity(
        browser_family,
        BrowserChannel::Unknown,
        product_name,
        BrowserWindowsSupportKind::Unsupported,
    )
}

pub(super) fn unknown_identity() -> BrowserWindowsExecutableIdentity {
    BrowserWindowsExecutableIdentity {
        browser_family: BrowserFamily::Unknown,
        browser_channel: BrowserChannel::Unknown,
        product_name: constants::browser::FAMILY_UNKNOWN,
        supports_managed_cdp: false,
        support_kind: BrowserWindowsSupportKind::Unknown,
    }
}

use ocentra_parent_agent_protocol::constants;
