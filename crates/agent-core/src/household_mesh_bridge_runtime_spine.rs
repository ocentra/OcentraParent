use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, error::EventingError, ids::EventType,
    ids::SubscriberId, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgePhase;

use crate::{
    household_mesh_bridge_runtime::HouseholdMeshBridgeEventPayload,
    household_mesh_bridge_runtime::HouseholdMeshBridgeInput,
    household_mesh_bridge_runtime::HouseholdMeshBridgeReport,
    household_mesh_bridge_runtime_payload::household_mesh_bridge_event_payload_from_input,
    household_mesh_bridge_runtime_source::bridge_event_metadata,
};

pub(crate) struct HouseholdMeshBridgeSpine {
    bus: EventBus,
}

impl HouseholdMeshBridgeSpine {
    pub(crate) async fn with_default_handlers() -> Result<Self, EventingError> {
        let bus = EventBus::new();
        for phase in HouseholdMeshBridgePhase::ordered_chain() {
            bus.subscribe::<HouseholdMeshBridgeEventPayload, _, _>(
                EventSubscriber::new(
                    SubscriberId::parse(phase.subscriber_id())?,
                    EventType::parse(phase.event_type())?,
                    TargetHandler::parse(phase.target_handler())?,
                ),
                |_| async { Ok(()) },
            )
            .await?;
        }
        Ok(Self { bus })
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
