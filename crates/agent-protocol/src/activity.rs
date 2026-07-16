use serde::{Deserialize, Serialize};

#[path = "policy.rs"]
pub mod policy;

#[path = "policy_context.rs"]
pub mod policy_context;

#[path = "policy_preview.rs"]
pub mod policy_preview;

#[path = "local_ai.rs"]
pub mod local_ai;

use crate::LogFields;

pub const ACTIVITY_SCHEMA_VERSION: u16 = crate::ACTIVITY_SCHEMA_VERSION;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityObserver {
    #[serde(rename = "agent-service")]
    AgentService,
    #[serde(rename = "windows-process")]
    WindowsProcess,
    #[serde(rename = "windows-window")]
    WindowsWindow,
    #[serde(rename = "windows-network")]
    WindowsNetwork,
    #[serde(rename = "managed-browser-bridge")]
    ManagedBrowserBridge,
    #[serde(rename = "browser-extension")]
    BrowserExtension,
    #[serde(rename = "local-ai")]
    LocalAi,
    #[serde(rename = "tracking-engine")]
    TrackingEngine,
    #[serde(rename = "android-location")]
    AndroidLocation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    #[serde(rename = "activity.browser.intervention.applied")]
    BrowserInterventionApplied,
    #[serde(rename = "activity.enforcement.audit-recorded")]
    EnforcementAuditRecorded,
    #[serde(rename = "activity.device.idle-state-observed")]
    DeviceIdleStateObserved,
    #[serde(rename = "activity.screen.analysis.summarized")]
    ScreenAnalysisSummarized,
    #[serde(rename = "activity.location.observed")]
    LocationObserved,
    #[serde(rename = "activity.tracking.alert.evaluated")]
    TrackingAlertEvaluated,
    #[serde(rename = "activity.tracking.geofence-transition.evaluated")]
    TrackingGeofenceTransitionEvaluated,
    #[serde(rename = "activity.tracking.expected-place.evaluated")]
    TrackingExpectedPlaceEvaluated,
    #[serde(rename = "activity.tracking.child-check-in.responded")]
    TrackingChildCheckInResponded,
    #[serde(rename = "activity.tracking.parent-notification.requested")]
    TrackingParentNotificationRequested,
    #[serde(rename = "activity.tracking.retention.deleted")]
    TrackingRetentionDeleted,
    #[serde(rename = "activity.network.retention.deleted")]
    NetworkRetentionDeleted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    #[serde(rename = "intervention")]
    Intervention,
    #[serde(rename = "location")]
    Location,
    #[serde(rename = "tracking-rule")]
    TrackingRule,
    #[serde(rename = "check-in")]
    CheckIn,
    #[serde(rename = "retention")]
    Retention,
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
