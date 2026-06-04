use ocentra_eventing::{
    DispatchMode, EventBus, EventQueuePolicy, EventSubscriber, EventType, EventingError,
    SubscriberId, TargetHandler,
};

use ocentra_parent_agent_protocol::constants;

use crate::{network_event_runtime_phase::NetworkRuntimePhase, NetworkObservation};

use super::{network_event_metadata, NetworkRuntimeEventPayload};

#[derive(Clone, Debug)]
pub struct NetworkRuntimeQueueDrainReport {
    pub queued_publish_report: ocentra_eventing::PublishReport,
    pub drain_report: ocentra_eventing::QueueDrainReport,
    pub stored_events: Vec<ocentra_eventing::StoredEventEnvelope>,
    pub dead_letters: Vec<ocentra_eventing::DeadLetter>,
}

pub async fn queue_network_runtime_flow_until_subscriber(
    observation: NetworkObservation,
    observed_at: &str,
) -> Result<NetworkRuntimeQueueDrainReport, EventingError> {
    let bus = EventBus::with_queue_policy(
        EventQueuePolicy::no_subscriber_queue(1)?.with_idempotency_registry(),
    );
    let phase = NetworkRuntimePhase::FlowObserved;
    let payload = NetworkRuntimeEventPayload::from_observation(phase, &observation, observed_at);
    let metadata =
        network_event_metadata(phase, &observation, observed_at, phase.target_handler())?;
    let queued_publish_report = bus.publish(payload, metadata).await?;

    bus.subscribe::<NetworkRuntimeEventPayload, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(phase.subscriber_id())?,
            EventType::parse(constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED)?,
            TargetHandler::parse(phase.target_handler())?,
        ),
        |_| async { Ok(()) },
    )
    .await?;

    let drain_report = bus.drain_queued(DispatchMode::Sequential).await?;
    Ok(NetworkRuntimeQueueDrainReport {
        queued_publish_report,
        drain_report,
        stored_events: bus.journal().await,
        dead_letters: bus.dead_letters().await,
    })
}
