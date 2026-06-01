use serde::{Deserialize, Serialize};

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
