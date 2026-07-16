use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenCaptureScope {
    FullScreen,
    PrimaryDisplay,
    ActiveDisplay,
    SelectedWindow,
    ActiveWindow,
    ManagedBrowserWindow,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenEvidenceCustodyState {
    LiveLocalChildAgent,
    LiveLanChildAgent,
    ChildDeviceTempQueue,
    ChildDeviceJournal,
    ChildDeviceQueryStore,
    ParentDeviceCache,
    ParentOwnedExport,
    OcentraHostedNonActivity,
    Unavailable,
}
