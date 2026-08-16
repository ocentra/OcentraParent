use super::*;

pub(crate) enum LanRouteQuery {
    NotRequired,
    Available(Box<LanAgentServiceSnapshot>),
    Unavailable(String),
}

impl LanRouteQuery {
    pub(crate) fn read_model(&self) -> Option<&LanBrowserAddDeviceReadModel> {
        match self {
            Self::Available(snapshot) => Some(&snapshot.read_model),
            Self::NotRequired | Self::Unavailable(_) => None,
        }
    }

    pub(crate) fn event(&self) -> Option<&ParentRouteEventSnapshot> {
        match self {
            Self::Available(snapshot) => Some(&snapshot.event),
            Self::NotRequired | Self::Unavailable(_) => None,
        }
    }

    pub(crate) fn events(&self) -> &[ParentRouteEventSnapshot] {
        match self {
            Self::Available(snapshot) => snapshot.events.as_slice(),
            Self::NotRequired | Self::Unavailable(_) => &[],
        }
    }

    pub(crate) fn discovery_event(&self) -> Option<&ParentRouteEventSnapshot> {
        self.event()
            .filter(|event| event.event.as_deref() == Some(LAN_DISCOVERY_REPORTED_EVENT))
    }
}
