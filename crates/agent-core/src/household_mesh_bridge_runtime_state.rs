use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdMeshBridgeDirection {
    Export,
    Import,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdMeshBridgeEnvelopeState {
    LocalSelected,
    LanExported,
    LanReceived,
    LocalRepublished,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdMeshBridgeValidationState {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdMeshBridgeRejectionReason {
    UnselectedEvent,
    PrivateLocalEvent,
    RawScreenPayload,
    UnauthenticatedPeer,
    UnauthorizedPeer,
    DirectRemotePublish,
    PolicyAuthorityEscalation,
    MismatchedMessageRef,
    ReplayedMessage,
    StaleMessage,
    FamilyMismatch,
    WrongTargetDevice,
    UnsupportedLanMessage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdMeshBridgeCustody {
    pub selected_event_only: bool,
    pub remote_direct_publish_allowed: bool,
    pub raw_screenshot_transferred: bool,
    pub private_local_event_exported: bool,
}

impl HouseholdMeshBridgeCustody {
    pub(crate) fn selected_bridge_only() -> Self {
        Self {
            selected_event_only: true,
            remote_direct_publish_allowed: false,
            raw_screenshot_transferred: false,
            private_local_event_exported: false,
        }
    }
}

pub(crate) fn bridge_custody_label() -> &'static str {
    constants::value::LAN_PROVIDER_CUSTODY_LOCAL_NETWORK_AI_PROVIDER
}
