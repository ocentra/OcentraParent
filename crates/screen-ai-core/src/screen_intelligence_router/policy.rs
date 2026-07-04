use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenIntelligenceSourceKind {
    ManagedBrowser,
    NativeApp,
    NativeGame,
    Launcher,
    UnknownProcess,
    NetworkOrSessionSummary,
    ScreenAdjacentEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenIntelligenceRouteKind {
    NoScreenNeeded,
    ManagedBrowserStructuredExtraction,
    ScreenCaptureActiveWindow,
    ScreenCaptureSelectedWindow,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenIntelligencePolicySensitivity {
    Ordinary,
    Private,
    CredentialRisk,
    ProtectedSurface,
}
