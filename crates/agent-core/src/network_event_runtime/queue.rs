#[cfg(test)]
use std::time::Duration;

use ocentra_eventing::{
    bus::reports::QueueDrainReport, bus::subscriber::EventSubscriber, bus::EventBus,
    error::EventingError, ids::EventType, ids::SubscriberId, ids::TargetHandler,
    queue::policy::EventQueuePolicy,
};

#[cfg(test)]
use ocentra_eventing::clock::ManualEventClock;

use ocentra_parent_agent_protocol::{constants, NetworkRuntimePhase};

use crate::NetworkObservation;

use super::{network_event_metadata, NetworkRuntimeEventPayload};

#[derive(Clone, Debug)]
#[cfg(test)]
pub struct NetworkRuntimeQueueDrainReport {
    pub queued_publish_report: ocentra_eventing::bus::reports::PublishReport,
    pub drain_report: ocentra_eventing::bus::reports::QueueDrainReport,
    pub stored_events: Vec<ocentra_eventing::envelope::StoredEventEnvelope>,
    pub dead_letters: Vec<ocentra_eventing::bus::reports::DeadLetter>,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeQueueOverflowReport {
    #[cfg(test)]
    pub first_publish_report: ocentra_eventing::bus::reports::PublishReport,
    #[cfg(test)]
    pub overflow_publish_report: ocentra_eventing::bus::reports::PublishReport,
    pub stored_events: Vec<ocentra_eventing::envelope::StoredEventEnvelope>,
    pub dead_letters: Vec<ocentra_eventing::bus::reports::DeadLetter>,
}

#[derive(Clone, Debug)]
#[cfg(test)]
pub struct NetworkRuntimeQueueTtlReport {
    pub queued_publish_report: ocentra_eventing::bus::reports::PublishReport,
    pub drain_report: ocentra_eventing::bus::reports::QueueDrainReport,
    pub stored_events: Vec<ocentra_eventing::envelope::StoredEventEnvelope>,
    pub dead_letters: Vec<ocentra_eventing::bus::reports::DeadLetter>,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeQueueIdempotencyReport {
    #[cfg(test)]
    pub first_publish_report: ocentra_eventing::bus::reports::PublishReport,
    pub queued_duplicate_error: EventingError,
    #[cfg(test)]
    pub drain_report: ocentra_eventing::bus::reports::QueueDrainReport,
    pub completed_duplicate_error: EventingError,
    pub stored_events: Vec<ocentra_eventing::envelope::StoredEventEnvelope>,
    #[cfg(test)]
    pub dead_letters: Vec<ocentra_eventing::bus::reports::DeadLetter>,
}

#[cfg(test)]
pub async fn queue_network_runtime_flow_until_subscriber(
    observation: NetworkObservation,
    observed_at: &str,
) -> Result<NetworkRuntimeQueueDrainReport, EventingError> {
    let bus = EventBus::with_queue_policy(
        EventQueuePolicy::no_subscriber_queue(1)?.with_idempotency_registry(),
    );
    let queued_publish_report = publish_flow_observation(&bus, &observation, observed_at).await?;

    let drain_report = subscribe_flow_observer(&bus).await?;
    Ok(NetworkRuntimeQueueDrainReport {
        queued_publish_report,
        drain_report,
        stored_events: bus.journal().await,
        dead_letters: bus.dead_letters().await,
    })
}

pub async fn queue_network_runtime_flow_overflow_dead_letters(
    first_observation: NetworkObservation,
    first_observed_at: &str,
    overflow_observation: NetworkObservation,
    overflow_observed_at: &str,
) -> Result<NetworkRuntimeQueueOverflowReport, EventingError> {
    let bus = EventBus::with_queue_policy(
        EventQueuePolicy::no_subscriber_queue(1)?.with_idempotency_registry(),
    );
    let first_publish_report =
        publish_flow_observation(&bus, &first_observation, first_observed_at).await?;
    let overflow_publish_report =
        publish_flow_observation(&bus, &overflow_observation, overflow_observed_at).await?;
    #[cfg(not(test))]
    let _ = (&first_publish_report, &overflow_publish_report);

    Ok(NetworkRuntimeQueueOverflowReport {
        #[cfg(test)]
        first_publish_report,
        #[cfg(test)]
        overflow_publish_report,
        stored_events: bus.journal().await,
        dead_letters: bus.dead_letters().await,
    })
}

#[cfg(test)]
pub async fn queue_network_runtime_flow_expires_before_drain(
    observation: NetworkObservation,
    observed_at: &str,
    ttl: Duration,
    elapsed: Duration,
) -> Result<NetworkRuntimeQueueTtlReport, EventingError> {
    let clock = ManualEventClock::new();
    let policy = EventQueuePolicy::no_subscriber_queue(2)?
        .with_ttl(ttl)?
        .with_idempotency_registry();
    let bus = EventBus::with_queue_policy_and_clock(policy, clock.shared());
    let queued_publish_report = publish_flow_observation(&bus, &observation, observed_at).await?;

    clock.advance(elapsed);
    let drain_report = subscribe_flow_observer(&bus).await?;
    Ok(NetworkRuntimeQueueTtlReport {
        queued_publish_report,
        drain_report,
        stored_events: bus.journal().await,
        dead_letters: bus.dead_letters().await,
    })
}

pub async fn queue_network_runtime_flow_rejects_duplicate_idempotency(
    observation: NetworkObservation,
    observed_at: &str,
) -> Result<NetworkRuntimeQueueIdempotencyReport, EventingError> {
    let bus = EventBus::with_queue_policy(
        EventQueuePolicy::no_subscriber_queue(2)?.with_idempotency_registry(),
    );
    let first_publish_report = publish_flow_observation(&bus, &observation, observed_at).await?;
    let queued_duplicate_error =
        duplicate_publish_error(publish_flow_observation(&bus, &observation, observed_at).await)?;

    let drain_report = subscribe_flow_observer(&bus).await?;
    #[cfg(not(test))]
    let _ = (&first_publish_report, &drain_report);

    let completed_duplicate_error =
        duplicate_publish_error(publish_flow_observation(&bus, &observation, observed_at).await)?;
    Ok(NetworkRuntimeQueueIdempotencyReport {
        #[cfg(test)]
        first_publish_report,
        queued_duplicate_error,
        #[cfg(test)]
        drain_report,
        completed_duplicate_error,
        stored_events: bus.journal().await,
        #[cfg(test)]
        dead_letters: bus.dead_letters().await,
    })
}

async fn publish_flow_observation(
    bus: &EventBus,
    observation: &NetworkObservation,
    observed_at: &str,
) -> Result<ocentra_eventing::bus::reports::PublishReport, EventingError> {
    let phase = NetworkRuntimePhase::FlowObserved;
    let payload =
        super::network_runtime_event_payload_from_observation(phase, observation, observed_at);
    let metadata = network_event_metadata(phase, observation, observed_at, phase.target_handler())?;
    bus.publish(payload, metadata).await
}

async fn subscribe_flow_observer(bus: &EventBus) -> Result<QueueDrainReport, EventingError> {
    let phase = NetworkRuntimePhase::FlowObserved;
    bus.subscribe::<NetworkRuntimeEventPayload, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(phase.subscriber_id())?,
            EventType::parse(constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED)?,
            TargetHandler::parse(phase.target_handler())?,
        ),
        |_| async { Ok(()) },
    )
    .await
    .map(|report| report.drain_report)
}

fn duplicate_publish_error(
    result: Result<ocentra_eventing::bus::reports::PublishReport, EventingError>,
) -> Result<EventingError, EventingError> {
    match result {
        Ok(_) => Err(EventingError::InvalidQueuePolicy {
            reason: constants::network_flow::ERROR_NETWORK_RUNTIME_QUEUE_IDEMPOTENCY_REJECTS
                .to_string(),
        }),
        Err(error) => Ok(error),
    }
}
