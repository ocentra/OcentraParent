use crate::activity::ActivityEventKind;

pub const PROCESS_OBSERVED: &str = "activity.process.observed";
pub const WINDOW_FOCUSED: &str = "activity.window.focused";
pub const DOMAIN_OBSERVED: &str = "activity.domain.observed";
pub const URL_OBSERVED: &str = "activity.url.observed";
pub const VIDEO_OBSERVED: &str = "activity.video.observed";
pub const BROWSER_INTERVENTION_APPLIED: &str = "activity.browser.intervention.applied";
pub const ENFORCEMENT_AUDIT_RECORDED: &str = "activity.enforcement.audit-recorded";
pub const DEVICE_IDLE_STATE_OBSERVED: &str = "activity.device.idle-state-observed";
pub const SCREEN_ANALYSIS_SUMMARIZED: &str = "activity.screen.analysis.summarized";
pub const LOCATION_OBSERVED: &str = "activity.location.observed";
pub const TRACKING_ALERT_EVALUATED: &str = "activity.tracking.alert.evaluated";
pub const TRACKING_GEOFENCE_TRANSITION_EVALUATED: &str =
    "activity.tracking.geofence-transition.evaluated";
pub const TRACKING_EXPECTED_PLACE_EVALUATED: &str = "activity.tracking.expected-place.evaluated";
pub const TRACKING_CHILD_CHECK_IN_RESPONDED: &str = "activity.tracking.child-check-in.responded";
pub const TRACKING_PARENT_NOTIFICATION_REQUESTED: &str =
    "activity.tracking.parent-notification.requested";
pub const TRACKING_RETENTION_DELETED: &str = "activity.tracking.retention.deleted";
pub const NETWORK_RETENTION_DELETED: &str = "activity.network.retention.deleted";

pub const ALL: [(&str, ActivityEventKind); 17] = [
    ("ProcessObserved", ActivityEventKind::ProcessObserved),
    ("WindowFocused", ActivityEventKind::WindowFocused),
    ("DomainObserved", ActivityEventKind::DomainObserved),
    ("UrlObserved", ActivityEventKind::UrlObserved),
    ("VideoObserved", ActivityEventKind::VideoObserved),
    (
        "BrowserInterventionApplied",
        ActivityEventKind::BrowserInterventionApplied,
    ),
    (
        "EnforcementAuditRecorded",
        ActivityEventKind::EnforcementAuditRecorded,
    ),
    (
        "DeviceIdleStateObserved",
        ActivityEventKind::DeviceIdleStateObserved,
    ),
    (
        "ScreenAnalysisSummarized",
        ActivityEventKind::ScreenAnalysisSummarized,
    ),
    ("LocationObserved", ActivityEventKind::LocationObserved),
    (
        "TrackingAlertEvaluated",
        ActivityEventKind::TrackingAlertEvaluated,
    ),
    (
        "TrackingGeofenceTransitionEvaluated",
        ActivityEventKind::TrackingGeofenceTransitionEvaluated,
    ),
    (
        "TrackingExpectedPlaceEvaluated",
        ActivityEventKind::TrackingExpectedPlaceEvaluated,
    ),
    (
        "TrackingChildCheckInResponded",
        ActivityEventKind::TrackingChildCheckInResponded,
    ),
    (
        "TrackingParentNotificationRequested",
        ActivityEventKind::TrackingParentNotificationRequested,
    ),
    (
        "TrackingRetentionDeleted",
        ActivityEventKind::TrackingRetentionDeleted,
    ),
    (
        "NetworkRetentionDeleted",
        ActivityEventKind::NetworkRetentionDeleted,
    ),
];
