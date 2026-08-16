use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdAiProviderClass {
    DesktopPreferred,
    LaptopPreferred,
    ChildDesktopLocal,
    MobileDormant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdAiProviderTrustState {
    Trusted,
    Stale,
    Offline,
    Revoked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdAiProviderResourceState {
    Ready,
    Degraded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdAiWorkClass {
    HeavyScreenVision,
    LightText,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdAiRouteDecisionState {
    Selected,
    Rejected,
    Dormant,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdAiRouteRejectionReason {
    StaleProvider,
    OfflineProvider,
    RevokedProvider,
    CustodyMismatch,
    UnsupportedCapability,
    ResourceDegraded,
    MobileDormantDesktopAvailable,
    MobileFallbackDenied,
    NoProvider,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAiProviderResourcePolicy {
    pub battery_ok: bool,
    pub thermal_ok: bool,
    pub fallback_policy_allows_mobile: bool,
}

impl HouseholdAiProviderResourcePolicy {
    pub fn desktop_ready() -> Self {
        Self {
            battery_ok: true,
            thermal_ok: true,
            fallback_policy_allows_mobile: true,
        }
    }

    pub fn mobile_fallback_ready() -> Self {
        Self {
            battery_ok: true,
            thermal_ok: true,
            fallback_policy_allows_mobile: true,
        }
    }
}
