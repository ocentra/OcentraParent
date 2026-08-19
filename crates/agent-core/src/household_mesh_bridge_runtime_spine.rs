use ocentra_eventing::{bus::EventBus, error::EventingError};
use ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgePhase;

use crate::{
    household_mesh_bridge_runtime::HouseholdMeshBridgeInput,
    household_mesh_bridge_runtime::HouseholdMeshBridgeReport,
    household_mesh_bridge_runtime_payload::household_mesh_bridge_event_payload_from_input,
    household_mesh_bridge_runtime_source::bridge_event_metadata,
};

pub(crate) struct HouseholdMeshBridgeSpine {
    bus: EventBus,
}

impl HouseholdMeshBridgeSpine {
    pub(crate) fn without_owner_handlers() -> Self {
        Self {
            bus: EventBus::new(),
        }
    }

    pub(crate) async fn publish_input_chain(
        &self,
        input: HouseholdMeshBridgeInput,
    ) -> Result<HouseholdMeshBridgeReport, EventingError> {
        let mut reports = Vec::new();
        for phase in HouseholdMeshBridgePhase::ordered_chain() {
            reports.push(
                self.bus
                    .publish(
                        household_mesh_bridge_event_payload_from_input(*phase, &input),
                        bridge_event_metadata(*phase, &input)?,
                    )
                    .await?,
            );
        }
        Ok(HouseholdMeshBridgeReport {
            publish_reports: reports,
            stored_events: self.bus.journal().await,
            dead_letters: self.bus.dead_letters().await,
        })
    }
}
