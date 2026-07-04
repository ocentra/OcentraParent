use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::constants;

use crate::browser_windows_package_inventory::{
    BrowserWindowsPackageDisplayIdentity, BrowserWindowsPackageIdentity,
    BrowserWindowsPackageSupportKind,
};

pub(crate) fn windows_browser_package_identity(
    package_identity: &BrowserWindowsPackageIdentity,
) -> Option<(
    BrowserWindowsPackageDisplayIdentity,
    BrowserWindowsPackageSupportKind,
)> {
    let mut normalized = package_identity.package_name.to_ascii_lowercase();
    if let Some(display_name) = package_identity.display_name.as_deref() {
        normalized.push_str(&display_name.to_ascii_lowercase());
    }
    if let Some(app_user_model_id) = package_identity.app_user_model_id.as_deref() {
        normalized.push_str(&app_user_model_id.to_ascii_lowercase());
    }
    package_identity_from_normalized(&normalized)
}

fn package_identity_from_normalized(
    value: &str,
) -> Option<(
    BrowserWindowsPackageDisplayIdentity,
    BrowserWindowsPackageSupportKind,
)> {
    if value.contains(constants::browser::PACKAGE_FRAGMENT_MICROSOFT_EDGE)
        || value.contains(constants::browser::PACKAGE_FRAGMENT_EDGE)
    {
        return Some((
            manual_identity(
                BrowserFamily::Edge,
                constants::browser::PRODUCT_NAME_MICROSOFT_EDGE,
            ),
            BrowserWindowsPackageSupportKind::ManualChromium,
        ));
    }
    if value.contains(constants::browser::PACKAGE_FRAGMENT_CHROME) {
        return Some((
            manual_identity(
                BrowserFamily::Chrome,
                constants::browser::PRODUCT_NAME_GOOGLE_CHROME,
            ),
            BrowserWindowsPackageSupportKind::ManualChromium,
        ));
    }
    package_chromium_fork_identity(value).or_else(|| package_unsupported_identity(value))
}

fn package_chromium_fork_identity(
    value: &str,
) -> Option<(
    BrowserWindowsPackageDisplayIdentity,
    BrowserWindowsPackageSupportKind,
)> {
    let (family, product_name) = if value.contains(constants::browser::PACKAGE_FRAGMENT_BRAVE) {
        (
            BrowserFamily::Brave,
            constants::browser::PRODUCT_NAME_BRAVE_BROWSER,
        )
    } else if value.contains(constants::browser::PACKAGE_FRAGMENT_VIVALDI) {
        (
            BrowserFamily::UnknownChromium,
            constants::browser::PRODUCT_NAME_VIVALDI_BROWSER,
        )
    } else if value.contains(constants::browser::PACKAGE_FRAGMENT_OPERA) {
        (
            BrowserFamily::Opera,
            constants::browser::PRODUCT_NAME_OPERA_BROWSER,
        )
    } else if value.contains(constants::browser::PACKAGE_FRAGMENT_CHROMIUM) {
        (
            BrowserFamily::UnknownChromium,
            constants::browser::PRODUCT_NAME_CHROMIUM,
        )
    } else {
        return None;
    };
    Some((
        manual_identity(family, product_name),
        BrowserWindowsPackageSupportKind::ManualChromium,
    ))
}

fn package_unsupported_identity(
    value: &str,
) -> Option<(
    BrowserWindowsPackageDisplayIdentity,
    BrowserWindowsPackageSupportKind,
)> {
    let (family, product_name) = if value.contains(constants::browser::PACKAGE_FRAGMENT_FIREFOX) {
        (
            BrowserFamily::Firefox,
            constants::browser::PRODUCT_NAME_MOZILLA_FIREFOX,
        )
    } else if value.contains(constants::browser::PACKAGE_FRAGMENT_TOR) {
        (
            BrowserFamily::Unknown,
            constants::browser::PRODUCT_NAME_TOR_BROWSER,
        )
    } else if value.contains(constants::browser::PACKAGE_FRAGMENT_DUCKDUCKGO) {
        (
            BrowserFamily::Unknown,
            constants::browser::PRODUCT_NAME_DUCKDUCKGO_BROWSER,
        )
    } else if value.contains(constants::browser::PACKAGE_FRAGMENT_ARC) {
        (
            BrowserFamily::UnknownChromium,
            constants::browser::PRODUCT_NAME_ARC_BROWSER,
        )
    } else {
        return None;
    };
    Some((
        manual_identity(family, product_name),
        BrowserWindowsPackageSupportKind::Unsupported,
    ))
}

fn manual_identity(
    browser_family: BrowserFamily,
    product_name: &'static str,
) -> BrowserWindowsPackageDisplayIdentity {
    BrowserWindowsPackageDisplayIdentity {
        browser_family,
        browser_channel: BrowserChannel::Stable,
        product_name,
    }
}
