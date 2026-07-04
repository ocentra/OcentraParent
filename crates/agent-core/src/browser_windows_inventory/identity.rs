use std::path::Path;

use super::executable_name_normalized;
use super::normalized_component_names;
use super::BrowserWindowsExecutableIdentity;
use super::BrowserWindowsSupportKind;

#[path = "identity_channel.rs"]
mod identity_channel;
#[path = "identity_chrome.rs"]
mod identity_chrome;
#[path = "identity_firefox.rs"]
mod identity_firefox;
#[path = "identity_match.rs"]
mod identity_match;
#[path = "identity_opera.rs"]
mod identity_opera;
#[path = "identity_unsupported.rs"]
mod identity_unsupported;

pub(super) fn windows_browser_executable_identity(path: &Path) -> BrowserWindowsExecutableIdentity {
    identity_match::windows_browser_executable_identity(path)
}
