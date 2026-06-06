use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdMeshBridgePhase {
    LocalEventSelected,
    LanMessageExported,
    LanMessageReceived,
    LocalEventRepublished,
}

impl HouseholdMeshBridgePhase {
    pub(crate) fn ordered_chain() -> &'static [Self] {
        &[
            Self::LocalEventSelected,
            Self::LanMessageExported,
            Self::LanMessageReceived,
            Self::LocalEventRepublished,
        ]
    }

    pub(crate) fn event_type(self) -> &'static str {
        match self {
            Self::LocalEventSelected => constants::household_mesh::EVENT_BRIDGE_LOCAL_SELECTED,
            Self::LanMessageExported => constants::household_mesh::EVENT_BRIDGE_LAN_EXPORTED,
            Self::LanMessageReceived => constants::household_mesh::EVENT_BRIDGE_LAN_RECEIVED,
            Self::LocalEventRepublished => {
                constants::household_mesh::EVENT_BRIDGE_LOCAL_REPUBLISHED
            }
        }
    }

    pub(crate) fn subscriber_id(self) -> &'static str {
        match self {
            Self::LocalEventSelected => constants::household_mesh::SUBSCRIBER_BRIDGE_LOCAL_SELECTED,
            Self::LanMessageExported => constants::household_mesh::SUBSCRIBER_BRIDGE_LAN_EXPORTED,
            Self::LanMessageReceived => constants::household_mesh::SUBSCRIBER_BRIDGE_LAN_RECEIVED,
            Self::LocalEventRepublished => {
                constants::household_mesh::SUBSCRIBER_BRIDGE_LOCAL_REPUBLISHED
            }
        }
    }

    pub(crate) fn target_handler(self) -> &'static str {
        match self {
            Self::LocalEventSelected => constants::household_mesh::TARGET_BRIDGE_EXPORT_VALIDATOR,
            Self::LanMessageExported => constants::household_mesh::TARGET_BRIDGE_LAN_TRANSPORT,
            Self::LanMessageReceived => constants::household_mesh::TARGET_BRIDGE_IMPORT_VALIDATOR,
            Self::LocalEventRepublished => {
                constants::household_mesh::TARGET_LOCAL_EVENT_REPUBLISHER
            }
        }
    }
}
