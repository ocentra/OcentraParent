use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};

pub(super) fn browser_identity_matches(
    browser: &str,
    family: BrowserFamily,
    channel: BrowserChannel,
) -> bool {
    let normalized = browser.to_ascii_lowercase();
    let family_matches = match family {
        BrowserFamily::Edge => normalized.contains("edge"),
        BrowserFamily::Chrome => normalized.contains("chrome"),
        BrowserFamily::Brave => normalized.contains("brave"),
        BrowserFamily::Opera => normalized.contains("opera"),
        BrowserFamily::Firefox | BrowserFamily::Unknown | BrowserFamily::UnknownChromium => false,
    };
    let channel_matches = match channel {
        BrowserChannel::Stable => {
            !normalized.contains("beta")
                && !normalized.contains("dev")
                && !normalized.contains("canary")
        }
        BrowserChannel::Beta => normalized.contains("beta"),
        BrowserChannel::Dev => normalized.contains("dev"),
        BrowserChannel::Canary => normalized.contains("canary"),
        BrowserChannel::Unknown => false,
    };
    family_matches && channel_matches
}
