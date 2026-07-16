use crate::{constants, ActivityEventKind, ActivityObserver, ActivitySubjectKind};

fn protocol_lookup<T: Copy, const N: usize>(
    value: impl AsRef<str>,
    variants: [(&'static str, T); N],
) -> Option<T> {
    let value = value.as_ref();
    variants
        .into_iter()
        .find_map(|(protocol, variant)| (value == protocol).then_some(variant))
}

impl ActivityObserver {
    pub fn as_protocol_str(&self) -> &'static str {
        const PROTOCOL_STRINGS: [&str; 9] = [
            constants::activity_observer::AGENT_SERVICE,
            constants::activity_observer::WINDOWS_PROCESS,
            constants::activity_observer::WINDOWS_WINDOW,
            constants::activity_observer::WINDOWS_NETWORK,
            constants::activity_observer::MANAGED_BROWSER_BRIDGE,
            constants::activity_observer::BROWSER_EXTENSION,
            constants::activity_observer::LOCAL_AI,
            constants::activity_observer::TRACKING_ENGINE,
            constants::activity_observer::ANDROID_LOCATION,
        ];
        PROTOCOL_STRINGS[*self as usize]
    }

    pub fn from_protocol_str(value: impl AsRef<str>) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::activity_observer::AGENT_SERVICE,
                    Self::AgentService,
                ),
                (
                    constants::activity_observer::WINDOWS_PROCESS,
                    Self::WindowsProcess,
                ),
                (
                    constants::activity_observer::WINDOWS_WINDOW,
                    Self::WindowsWindow,
                ),
                (
                    constants::activity_observer::WINDOWS_NETWORK,
                    Self::WindowsNetwork,
                ),
                (
                    constants::activity_observer::MANAGED_BROWSER_BRIDGE,
                    Self::ManagedBrowserBridge,
                ),
                (
                    constants::activity_observer::BROWSER_EXTENSION,
                    Self::BrowserExtension,
                ),
                (constants::activity_observer::LOCAL_AI, Self::LocalAi),
                (
                    constants::activity_observer::TRACKING_ENGINE,
                    Self::TrackingEngine,
                ),
                (
                    constants::activity_observer::ANDROID_LOCATION,
                    Self::AndroidLocation,
                ),
            ],
        )
    }
}

impl ActivityEventKind {
    pub fn as_protocol_str(&self) -> &'static str {
        const PROTOCOL_STRINGS: [&str; 17] = [
            constants::activity_event_kind::PROCESS_OBSERVED,
            constants::activity_event_kind::WINDOW_FOCUSED,
            constants::activity_event_kind::DOMAIN_OBSERVED,
            constants::activity_event_kind::URL_OBSERVED,
            constants::activity_event_kind::VIDEO_OBSERVED,
            constants::activity_event_kind::BROWSER_INTERVENTION_APPLIED,
            constants::activity_event_kind::ENFORCEMENT_AUDIT_RECORDED,
            constants::activity_event_kind::DEVICE_IDLE_STATE_OBSERVED,
            constants::activity_event_kind::SCREEN_ANALYSIS_SUMMARIZED,
            constants::activity_event_kind::LOCATION_OBSERVED,
            constants::activity_event_kind::TRACKING_ALERT_EVALUATED,
            constants::activity_event_kind::TRACKING_GEOFENCE_TRANSITION_EVALUATED,
            constants::activity_event_kind::TRACKING_EXPECTED_PLACE_EVALUATED,
            constants::activity_event_kind::TRACKING_CHILD_CHECK_IN_RESPONDED,
            constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED,
            constants::activity_event_kind::TRACKING_RETENTION_DELETED,
            constants::activity_event_kind::NETWORK_RETENTION_DELETED,
        ];
        PROTOCOL_STRINGS[*self as usize]
    }

    pub fn from_protocol_str(value: impl AsRef<str>) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::activity_event_kind::PROCESS_OBSERVED,
                    Self::ProcessObserved,
                ),
                (
                    constants::activity_event_kind::WINDOW_FOCUSED,
                    Self::WindowFocused,
                ),
                (
                    constants::activity_event_kind::DOMAIN_OBSERVED,
                    Self::DomainObserved,
                ),
                (
                    constants::activity_event_kind::URL_OBSERVED,
                    Self::UrlObserved,
                ),
                (
                    constants::activity_event_kind::VIDEO_OBSERVED,
                    Self::VideoObserved,
                ),
                (
                    constants::activity_event_kind::BROWSER_INTERVENTION_APPLIED,
                    Self::BrowserInterventionApplied,
                ),
                (
                    constants::activity_event_kind::ENFORCEMENT_AUDIT_RECORDED,
                    Self::EnforcementAuditRecorded,
                ),
                (
                    constants::activity_event_kind::DEVICE_IDLE_STATE_OBSERVED,
                    Self::DeviceIdleStateObserved,
                ),
                (
                    constants::activity_event_kind::SCREEN_ANALYSIS_SUMMARIZED,
                    Self::ScreenAnalysisSummarized,
                ),
                (
                    constants::activity_event_kind::LOCATION_OBSERVED,
                    Self::LocationObserved,
                ),
                (
                    constants::activity_event_kind::TRACKING_ALERT_EVALUATED,
                    Self::TrackingAlertEvaluated,
                ),
                (
                    constants::activity_event_kind::TRACKING_GEOFENCE_TRANSITION_EVALUATED,
                    Self::TrackingGeofenceTransitionEvaluated,
                ),
                (
                    constants::activity_event_kind::TRACKING_EXPECTED_PLACE_EVALUATED,
                    Self::TrackingExpectedPlaceEvaluated,
                ),
                (
                    constants::activity_event_kind::TRACKING_CHILD_CHECK_IN_RESPONDED,
                    Self::TrackingChildCheckInResponded,
                ),
                (
                    constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED,
                    Self::TrackingParentNotificationRequested,
                ),
                (
                    constants::activity_event_kind::TRACKING_RETENTION_DELETED,
                    Self::TrackingRetentionDeleted,
                ),
                (
                    constants::activity_event_kind::NETWORK_RETENTION_DELETED,
                    Self::NetworkRetentionDeleted,
                ),
            ],
        )
    }
}

impl ActivitySubjectKind {
    pub fn as_protocol_str(&self) -> &'static str {
        const PROTOCOL_STRINGS: [&str; 11] = [
            constants::activity_subject_kind::PROCESS,
            constants::activity_subject_kind::WINDOW,
            constants::activity_subject_kind::DOMAIN,
            constants::activity_subject_kind::URL,
            constants::activity_subject_kind::VIDEO,
            constants::activity_subject_kind::DEVICE,
            constants::activity_subject_kind::INTERVENTION,
            constants::activity_subject_kind::LOCATION,
            constants::activity_subject_kind::TRACKING_RULE,
            constants::activity_subject_kind::CHECK_IN,
            constants::activity_subject_kind::RETENTION,
        ];
        PROTOCOL_STRINGS[*self as usize]
    }

    pub fn from_protocol_str(value: impl AsRef<str>) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (constants::activity_subject_kind::PROCESS, Self::Process),
                (constants::activity_subject_kind::WINDOW, Self::Window),
                (constants::activity_subject_kind::DOMAIN, Self::Domain),
                (constants::activity_subject_kind::URL, Self::Url),
                (constants::activity_subject_kind::VIDEO, Self::Video),
                (constants::activity_subject_kind::DEVICE, Self::Device),
                (
                    constants::activity_subject_kind::INTERVENTION,
                    Self::Intervention,
                ),
                (constants::activity_subject_kind::LOCATION, Self::Location),
                (
                    constants::activity_subject_kind::TRACKING_RULE,
                    Self::TrackingRule,
                ),
                (constants::activity_subject_kind::CHECK_IN, Self::CheckIn),
                (constants::activity_subject_kind::RETENTION, Self::Retention),
            ],
        )
    }
}
