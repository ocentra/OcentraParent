/* generated from crates/schema/src/activity_event_kind_ts.rs */

export const ActivityEventKindLiteral = {
  ProcessObserved: 'activity.process.observed',
  WindowFocused: 'activity.window.focused',
  DomainObserved: 'activity.domain.observed',
  UrlObserved: 'activity.url.observed',
  VideoObserved: 'activity.video.observed',
  BrowserInterventionApplied: 'activity.browser.intervention.applied',
  EnforcementAuditRecorded: 'activity.enforcement.audit-recorded',
  DeviceIdleStateObserved: 'activity.device.idle-state-observed',
  ScreenAnalysisSummarized: 'activity.screen.analysis.summarized',
  LocationObserved: 'activity.location.observed',
  TrackingAlertEvaluated: 'activity.tracking.alert.evaluated',
  TrackingGeofenceTransitionEvaluated: 'activity.tracking.geofence-transition.evaluated',
  TrackingExpectedPlaceEvaluated: 'activity.tracking.expected-place.evaluated',
  TrackingChildCheckInResponded: 'activity.tracking.child-check-in.responded',
  TrackingParentNotificationRequested: 'activity.tracking.parent-notification.requested',
  TrackingRetentionDeleted: 'activity.tracking.retention.deleted',
  NetworkRetentionDeleted: 'activity.network.retention.deleted',
} as const;
