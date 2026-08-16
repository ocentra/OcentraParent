use super::protocol_lookup;
use crate::{constants, BrowserInterventionCapabilityState};

impl BrowserInterventionCapabilityState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::INTERVENTION_CAPABILITY_READY,
                    Self::Ready,
                ),
                (
                    constants::browser::INTERVENTION_CAPABILITY_NEEDS_MANAGED_SESSION,
                    Self::NeedsManagedSession,
                ),
                (
                    constants::browser::INTERVENTION_CAPABILITY_NEEDS_MANAGED_EXTENSION,
                    Self::NeedsManagedExtension,
                ),
                (
                    constants::browser::INTERVENTION_CAPABILITY_NEEDS_OS_APP_CONTROL,
                    Self::NeedsOsAppControl,
                ),
                (
                    constants::browser::INTERVENTION_CAPABILITY_UNSUPPORTED_BROWSER,
                    Self::UnsupportedBrowser,
                ),
                (
                    constants::browser::INTERVENTION_CAPABILITY_DISABLED_BY_PARENT,
                    Self::DisabledByParent,
                ),
                (
                    constants::browser::INTERVENTION_CAPABILITY_ADAPTER_ERROR,
                    Self::AdapterError,
                ),
            ],
        )
    }
}
