use serde::{Deserialize, Serialize};

use super::{LanPairingOptionalText, LanPairingText};

impl From<String> for LanPairingText {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for LanPairingText {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<Option<String>> for LanPairingOptionalText {
    fn from(value: Option<String>) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingProductionDiscoveryState {
    Discovered,
    Pending,
    Paired,
    Rejected,
    Expired,
    Revoked,
    Stale,
    Offline,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanAiProviderRoutingState {
    AuthorizedResult,
    Busy,
    Degraded,
    Unavailable,
    UnsupportedCapability,
}
