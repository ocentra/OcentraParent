use crate::{
    constants, ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus, ActivityEventKind,
    ActivityNetworkProtocol, ActivityNetworkTcpState, ActivityObserver,
    ActivityProcessAttributionStatus, ActivitySubjectKind,
};

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
            Self::DeviceIdleStateObserved => {
                constants::activity_event_kind::DEVICE_IDLE_STATE_OBSERVED
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
            constants::activity_event_kind::DEVICE_IDLE_STATE_OBSERVED => {
                Some(Self::DeviceIdleStateObserved)
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
            _ => None,
        }
    }
}

impl ActivityCaptureCapabilityStatus {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::activity_capture::CAPABILITY_STATUS_AVAILABLE => Some(Self::Available),
            constants::activity_capture::CAPABILITY_STATUS_UNAVAILABLE => Some(Self::Unavailable),
            constants::activity_capture::CAPABILITY_STATUS_ACCESS_DENIED => {
                Some(Self::AccessDenied)
            }
            constants::activity_capture::CAPABILITY_STATUS_NO_ACTIVE_WINDOW => {
                Some(Self::NoActiveWindow)
            }
            constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS => {
                Some(Self::NoNetworkObservations)
            }
            constants::activity_capture::CAPABILITY_STATUS_ADAPTER_ERROR => {
                Some(Self::AdapterError)
            }
            _ => None,
        }
    }
}

impl ActivityNetworkProtocol {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::activity_capture::NETWORK_PROTOCOL_TCP => Some(Self::Tcp),
            constants::activity_capture::NETWORK_PROTOCOL_UDP => Some(Self::Udp),
            _ => None,
        }
    }
}

impl ActivityNetworkTcpState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::activity_capture::TCP_STATE_CLOSED => Some(Self::Closed),
            constants::activity_capture::TCP_STATE_LISTEN => Some(Self::Listen),
            constants::activity_capture::TCP_STATE_SYN_SENT => Some(Self::SynSent),
            constants::activity_capture::TCP_STATE_SYN_RECEIVED => Some(Self::SynReceived),
            constants::activity_capture::TCP_STATE_ESTABLISHED => Some(Self::Established),
            constants::activity_capture::TCP_STATE_FIN_WAIT_1 => Some(Self::FinWait1),
            constants::activity_capture::TCP_STATE_FIN_WAIT_2 => Some(Self::FinWait2),
            constants::activity_capture::TCP_STATE_CLOSE_WAIT => Some(Self::CloseWait),
            constants::activity_capture::TCP_STATE_CLOSING => Some(Self::Closing),
            constants::activity_capture::TCP_STATE_LAST_ACK => Some(Self::LastAck),
            constants::activity_capture::TCP_STATE_TIME_WAIT => Some(Self::TimeWait),
            constants::activity_capture::TCP_STATE_DELETE_TCB => Some(Self::DeleteTcb),
            constants::activity_capture::TCP_STATE_UNKNOWN => Some(Self::Unknown),
            _ => None,
        }
    }
}

impl ActivityDomainAttributionStatus {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_DOMAIN_OBSERVED => {
                Some(Self::DomainObserved)
            }
            constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_IP_ONLY => Some(Self::IpOnly),
            constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_UNAVAILABLE => {
                Some(Self::Unavailable)
            }
            _ => None,
        }
    }
}

impl ActivityProcessAttributionStatus {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED => {
                Some(Self::ProcessAttributed)
            }
            constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_UNKNOWN => {
                Some(Self::ProcessUnknown)
            }
            _ => None,
        }
    }
}
