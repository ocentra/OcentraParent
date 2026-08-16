use ocentra_parent_agent_protocol::browser::BrowserChannel;
use ocentra_parent_agent_protocol::constants;

pub(super) fn browser_channel_from_components(components: &[String]) -> BrowserChannel {
    if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_EDGE_BETA)
        || components
            .iter()
            .any(|name| name == constants::browser::PATH_SEGMENT_CHROME_BETA)
    {
        return BrowserChannel::Beta;
    }
    if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_EDGE_DEV)
        || components
            .iter()
            .any(|name| name == constants::browser::PATH_SEGMENT_CHROME_DEV)
    {
        return BrowserChannel::Dev;
    }
    if components
        .iter()
        .any(|name| name == constants::browser::PATH_SEGMENT_EDGE_SXS)
        || components
            .iter()
            .any(|name| name == constants::browser::PATH_SEGMENT_CHROME_SXS)
    {
        return BrowserChannel::Canary;
    }
    BrowserChannel::Stable
}

pub(super) fn browser_identity(
    browser_family: ocentra_parent_agent_protocol::browser::BrowserFamily,
    browser_channel: BrowserChannel,
    product_name: &'static str,
    supports_managed_cdp: bool,
    support_kind: super::BrowserWindowsSupportKind,
) -> super::BrowserWindowsExecutableIdentity {
    super::BrowserWindowsExecutableIdentity {
        browser_family,
        browser_channel,
        product_name,
        supports_managed_cdp,
        support_kind,
    }
}
