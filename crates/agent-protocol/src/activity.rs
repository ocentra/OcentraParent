use serde::{Deserialize, Serialize};

use crate::{constants, LogFields};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityObserver {
    #[serde(rename = "agent-service")]
    AgentService,
    #[serde(rename = "windows-process")]
    WindowsProcess,
    #[serde(rename = "windows-window")]
    WindowsWindow,
    #[serde(rename = "windows-network")]
    WindowsNetwork,
    #[serde(rename = "browser-extension")]
    BrowserExtension,
    #[serde(rename = "local-ai")]
    LocalAi,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityEventKind {
    #[serde(rename = "activity.process.observed")]
    ProcessObserved,
    #[serde(rename = "activity.window.focused")]
    WindowFocused,
    #[serde(rename = "activity.domain.observed")]
    DomainObserved,
    #[serde(rename = "activity.url.observed")]
    UrlObserved,
    #[serde(rename = "activity.video.observed")]
    VideoObserved,
    #[serde(rename = "activity.device.idle-state-observed")]
    DeviceIdleStateObserved,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivitySubjectKind {
    #[serde(rename = "process")]
    Process,
    #[serde(rename = "window")]
    Window,
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "device")]
    Device,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityEvidenceKind {
    #[serde(rename = "journal-entry")]
    JournalEntry,
    #[serde(rename = "screenshot")]
    Screenshot,
    #[serde(rename = "storage-object")]
    StorageObject,
    #[serde(rename = "local-db-row")]
    LocalDbRow,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityObservationMode {
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "active-window")]
    ActiveWindow,
}

impl ActivityObservationMode {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Snapshot => constants::activity_capture::OBSERVATION_MODE_SNAPSHOT,
            Self::ActiveWindow => constants::activity_capture::OBSERVATION_MODE_ACTIVE_WINDOW,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityCaptureCapabilityStatus {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "access-denied")]
    AccessDenied,
    #[serde(rename = "no-active-window")]
    NoActiveWindow,
    #[serde(rename = "adapter-error")]
    AdapterError,
}

impl ActivityCaptureCapabilityStatus {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Available => constants::activity_capture::CAPABILITY_STATUS_AVAILABLE,
            Self::Unavailable => constants::activity_capture::CAPABILITY_STATUS_UNAVAILABLE,
            Self::AccessDenied => constants::activity_capture::CAPABILITY_STATUS_ACCESS_DENIED,
            Self::NoActiveWindow => constants::activity_capture::CAPABILITY_STATUS_NO_ACTIVE_WINDOW,
            Self::AdapterError => constants::activity_capture::CAPABILITY_STATUS_ADAPTER_ERROR,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySource {
    pub device_id: String,
    pub platform: String,
    pub observer: ActivityObserver,
    pub source_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySubject {
    pub kind: ActivitySubjectKind,
    pub subject_id: String,
    pub display_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityEvidenceRef {
    pub evidence_id: String,
    pub kind: ActivityEvidenceKind,
    pub digest: Option<String>,
    pub uri: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityEvent {
    pub schema_version: u16,
    pub event_id: String,
    pub observed_at: String,
    pub source: ActivitySource,
    pub kind: ActivityEventKind,
    pub subject: ActivitySubject,
    pub fields: LogFields,
    pub evidence: Vec<ActivityEvidenceRef>,
}
