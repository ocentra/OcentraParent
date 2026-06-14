use crate::{constants, ActivityEventKind, ActivityObserver, ActivitySubjectKind};

impl ActivityObserver {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::AgentService => constants::activity_observer::AGENT_SERVICE,
            Self::WindowsProcess => constants::activity_observer::WINDOWS_PROCESS,
            Self::WindowsWindow => constants::activity_observer::WINDOWS_WINDOW,
            Self::WindowsNetwork => constants::activity_observer::WINDOWS_NETWORK,
            Self::ManagedBrowserBridge => constants::activity_observer::MANAGED_BROWSER_BRIDGE,
            Self::BrowserExtension => constants::activity_observer::BROWSER_EXTENSION,
            Self::LocalAi => constants::activity_observer::LOCAL_AI,
            Self::TrackingEngine => constants::activity_observer::TRACKING_ENGINE,
            Self::AndroidLocation => constants::activity_observer::ANDROID_LOCATION,
        }
    }

    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::activity_observer::AGENT_SERVICE => Some(Self::AgentService),
            constants::activity_observer::WINDOWS_PROCESS => Some(Self::WindowsProcess),
            constants::activity_observer::WINDOWS_WINDOW => Some(Self::WindowsWindow),
            constants::activity_observer::WINDOWS_NETWORK => Some(Self::WindowsNetwork),
            constants::activity_observer::MANAGED_BROWSER_BRIDGE => {
                Some(Self::ManagedBrowserBridge)
            }
            constants::activity_observer::BROWSER_EXTENSION => Some(Self::BrowserExtension),
            constants::activity_observer::LOCAL_AI => Some(Self::LocalAi),
            constants::activity_observer::TRACKING_ENGINE => Some(Self::TrackingEngine),
            constants::activity_observer::ANDROID_LOCATION => Some(Self::AndroidLocation),
            _ => None,
        }
    }
}

impl ActivityEventKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ProcessObserved => constants::activity_event_kind::PROCESS_OBSERVED,
            Self::WindowFocused => constants::activity_event_kind::WINDOW_FOCUSED,
            Self::DomainObserved => constants::activity_event_kind::DOMAIN_OBSERVED,
            Self::UrlObserved => constants::activity_event_kind::URL_OBSERVED,
            Self::VideoObserved => constants::activity_event_kind::VIDEO_OBSERVED,
            Self::BrowserInterventionApplied => {
                constants::activity_event_kind::BROWSER_INTERVENTION_APPLIED
            }
            Self::EnforcementAuditRecorded => {
                constants::activity_event_kind::ENFORCEMENT_AUDIT_RECORDED
            }
            Self::DeviceIdleStateObserved => {
                constants::activity_event_kind::DEVICE_IDLE_STATE_OBSERVED
            }
            Self::ScreenAnalysisSummarized => {
                constants::activity_event_kind::SCREEN_ANALYSIS_SUMMARIZED
            }
            Self::LocationObserved => constants::activity_event_kind::LOCATION_OBSERVED,
            Self::TrackingAlertEvaluated => {
                constants::activity_event_kind::TRACKING_ALERT_EVALUATED
            }
            Self::TrackingGeofenceTransitionEvaluated => {
                constants::activity_event_kind::TRACKING_GEOFENCE_TRANSITION_EVALUATED
            }
            Self::TrackingExpectedPlaceEvaluated => {
                constants::activity_event_kind::TRACKING_EXPECTED_PLACE_EVALUATED
            }
            Self::TrackingChildCheckInResponded => {
                constants::activity_event_kind::TRACKING_CHILD_CHECK_IN_RESPONDED
            }
            Self::TrackingParentNotificationRequested => {
                constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED
            }
            Self::TrackingRetentionDeleted => {
                constants::activity_event_kind::TRACKING_RETENTION_DELETED
            }
            Self::NetworkRetentionDeleted => {
                constants::activity_event_kind::NETWORK_RETENTION_DELETED
            }
        }
    }

    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::activity_event_kind::PROCESS_OBSERVED => Some(Self::ProcessObserved),
            constants::activity_event_kind::WINDOW_FOCUSED => Some(Self::WindowFocused),
            constants::activity_event_kind::DOMAIN_OBSERVED => Some(Self::DomainObserved),
            constants::activity_event_kind::URL_OBSERVED => Some(Self::UrlObserved),
            constants::activity_event_kind::VIDEO_OBSERVED => Some(Self::VideoObserved),
            constants::activity_event_kind::BROWSER_INTERVENTION_APPLIED => {
                Some(Self::BrowserInterventionApplied)
            }
            constants::activity_event_kind::ENFORCEMENT_AUDIT_RECORDED => {
                Some(Self::EnforcementAuditRecorded)
            }
            constants::activity_event_kind::DEVICE_IDLE_STATE_OBSERVED => {
                Some(Self::DeviceIdleStateObserved)
            }
            constants::activity_event_kind::SCREEN_ANALYSIS_SUMMARIZED => {
                Some(Self::ScreenAnalysisSummarized)
            }
            constants::activity_event_kind::LOCATION_OBSERVED => Some(Self::LocationObserved),
            constants::activity_event_kind::TRACKING_ALERT_EVALUATED => {
                Some(Self::TrackingAlertEvaluated)
            }
            constants::activity_event_kind::TRACKING_GEOFENCE_TRANSITION_EVALUATED => {
                Some(Self::TrackingGeofenceTransitionEvaluated)
            }
            constants::activity_event_kind::TRACKING_EXPECTED_PLACE_EVALUATED => {
                Some(Self::TrackingExpectedPlaceEvaluated)
            }
            constants::activity_event_kind::TRACKING_CHILD_CHECK_IN_RESPONDED => {
                Some(Self::TrackingChildCheckInResponded)
            }
            constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED => {
                Some(Self::TrackingParentNotificationRequested)
            }
            constants::activity_event_kind::TRACKING_RETENTION_DELETED => {
                Some(Self::TrackingRetentionDeleted)
            }
            constants::activity_event_kind::NETWORK_RETENTION_DELETED => {
                Some(Self::NetworkRetentionDeleted)
            }
            _ => None,
        }
    }
}

impl ActivitySubjectKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Process => constants::activity_subject_kind::PROCESS,
            Self::Window => constants::activity_subject_kind::WINDOW,
            Self::Domain => constants::activity_subject_kind::DOMAIN,
            Self::Url => constants::activity_subject_kind::URL,
            Self::Video => constants::activity_subject_kind::VIDEO,
            Self::Device => constants::activity_subject_kind::DEVICE,
            Self::Intervention => constants::activity_subject_kind::INTERVENTION,
            Self::Location => constants::activity_subject_kind::LOCATION,
            Self::TrackingRule => constants::activity_subject_kind::TRACKING_RULE,
            Self::CheckIn => constants::activity_subject_kind::CHECK_IN,
            Self::Retention => constants::activity_subject_kind::RETENTION,
        }
    }

    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::activity_subject_kind::PROCESS => Some(Self::Process),
            constants::activity_subject_kind::WINDOW => Some(Self::Window),
            constants::activity_subject_kind::DOMAIN => Some(Self::Domain),
            constants::activity_subject_kind::URL => Some(Self::Url),
            constants::activity_subject_kind::VIDEO => Some(Self::Video),
            constants::activity_subject_kind::DEVICE => Some(Self::Device),
            constants::activity_subject_kind::INTERVENTION => Some(Self::Intervention),
            constants::activity_subject_kind::LOCATION => Some(Self::Location),
            constants::activity_subject_kind::TRACKING_RULE => Some(Self::TrackingRule),
            constants::activity_subject_kind::CHECK_IN => Some(Self::CheckIn),
            constants::activity_subject_kind::RETENTION => Some(Self::Retention),
            _ => None,
        }
    }
}
