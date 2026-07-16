use super::protocol_lookup;
use crate::{constants, BrowserInterventionMechanism};

impl BrowserInterventionMechanism {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::INTERVENTION_MECHANISM_CHROMIUM_CDP_FETCH,
                    Self::ChromiumCdpFetch,
                ),
                (
                    constants::browser::INTERVENTION_MECHANISM_WEBDRIVER_BIDI_NETWORK,
                    Self::WebDriverBidiNetwork,
                ),
                (
                    constants::browser::INTERVENTION_MECHANISM_MANAGED_EXTENSION,
                    Self::ManagedExtension,
                ),
                (
                    constants::browser::INTERVENTION_MECHANISM_MANAGED_BLOCK_PAGE,
                    Self::ManagedBlockPage,
                ),
                (
                    constants::browser::INTERVENTION_MECHANISM_APPROVAL_HOLD_PAGE,
                    Self::ApprovalHoldPage,
                ),
                (
                    constants::browser::INTERVENTION_MECHANISM_CHECKING_HOLD_PAGE,
                    Self::CheckingHoldPage,
                ),
                (
                    constants::browser::INTERVENTION_MECHANISM_OS_APP_CONTROL,
                    Self::OsAppControl,
                ),
                (
                    constants::browser::INTERVENTION_MECHANISM_OWNED_WEBVIEW,
                    Self::OwnedWebView,
                ),
                (
                    constants::browser::INTERVENTION_MECHANISM_MONITOR_ONLY,
                    Self::MonitorOnly,
                ),
                (constants::browser::INTERVENTION_MECHANISM_NONE, Self::None),
            ],
        )
    }
}
