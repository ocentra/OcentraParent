use ocentra_eventing::{
    contract_registry::EventContractRegistry, error::EventingError, ids::SourceComponent,
    ids::SubscriberId, ids::TargetHandler, topology::EventTopologyManifest,
    topology::EventTopologyPublisher, topology::EventTopologySubscriber,
};
use ocentra_eventing::{
    envelope::DomainEvent, envelope::EventContract, ids::AggregateKey, ids::EventType,
    ids::IdempotencyKey, ids::SchemaVersion,
};
use ocentra_parent_agent_protocol::browser::BrowserRuntimePhase;
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

use super::BrowserRuntimeInput;

pub fn browser_runtime_chain_topology_manifest() -> Result<EventTopologyManifest, EventingError> {
    let input = BrowserRuntimeInput::managed_decision_fixture();
    let mut registry = EventContractRegistry::new();
    let mut publishers = Vec::new();
    let mut subscribers = Vec::new();
    for phase in BrowserRuntimePhase::ordered_chain() {
        let payload = super::browser_runtime_event_payload_from_input(*phase, &input);
        let event_type = registry.register_event(&payload)?.event_type().clone();
        publishers.push(EventTopologyPublisher {
            event_type: event_type.clone(),
            source_component: SourceComponent::parse(
                constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE,
            )?,
        });
        subscribers.push(EventTopologySubscriber {
            event_type,
            subscriber_id: SubscriberId::parse(phase.subscriber_id())?,
            target_handler: TargetHandler::parse(phase.target_handler())?,
        });
    }
    Ok(EventTopologyManifest::from_registry(
        &registry,
        &publishers,
        &subscribers,
        &[],
        &[],
    ))
}

pub fn browser_runtime_stream_report_topology_manifest(
) -> Result<EventTopologyManifest, EventingError> {
    let request = BrowserRuntimeStreamReportTopologyRequest;
    let mut registry = EventContractRegistry::new();
    registry.register_event(&request)?;
    Ok(EventTopologyManifest::from_registry(
        &registry,
        &[EventTopologyPublisher {
            event_type: EventType::parse(
                constants::browser::EVENT_BROWSER_RUNTIME_STREAM_REPORT_REQUESTED,
            )?,
            source_component: SourceComponent::parse(
                constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE,
            )?,
        }],
        &[EventTopologySubscriber {
            event_type: EventType::parse(
                constants::browser::EVENT_BROWSER_RUNTIME_STREAM_REPORT_REQUESTED,
            )?,
            subscriber_id: SubscriberId::parse(
                constants::browser::SUBSCRIBER_BROWSER_RUNTIME_STREAM_REPORT,
            )?,
            target_handler: TargetHandler::parse(
                constants::browser::TARGET_BROWSER_RUNTIME_STREAM_REPORT,
            )?,
        }],
        &[],
        &[],
    ))
}

pub fn browser_runtime_parent_surface_status_topology_manifest(
) -> Result<EventTopologyManifest, EventingError> {
    let request = BrowserRuntimeParentSurfaceStatusTopologyRequest;
    let mut registry = EventContractRegistry::new();
    registry.register_event(&request)?;
    Ok(EventTopologyManifest::from_registry(
        &registry,
        &[EventTopologyPublisher {
            event_type: EventType::parse(
                constants::browser::EVENT_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_REQUESTED,
            )?,
            source_component: SourceComponent::parse(
                constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE,
            )?,
        }],
        &[EventTopologySubscriber {
            event_type: EventType::parse(
                constants::browser::EVENT_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_REQUESTED,
            )?,
            subscriber_id: SubscriberId::parse(
                constants::browser::SUBSCRIBER_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS,
            )?,
            target_handler: TargetHandler::parse(
                constants::browser::TARGET_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS,
            )?,
        }],
        &[],
        &[],
    ))
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct BrowserRuntimeStreamReportTopologyRequest;

impl DomainEvent for BrowserRuntimeStreamReportTopologyRequest {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::browser::EVENT_BROWSER_RUNTIME_STREAM_REPORT_REQUESTED)?,
            SchemaVersion::new(constants::browser::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(constants::browser::AGGREGATE_BROWSER_RUNTIME_PREFIX)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(constants::browser::IDEMPOTENCY_BROWSER_RUNTIME_STREAM_REPORT_PREFIX)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct BrowserRuntimeParentSurfaceStatusTopologyRequest;

impl DomainEvent for BrowserRuntimeParentSurfaceStatusTopologyRequest {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(
                constants::browser::EVENT_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_REQUESTED,
            )?,
            SchemaVersion::new(constants::browser::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(constants::browser::AGGREGATE_BROWSER_RUNTIME_PREFIX)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(
            constants::browser::IDEMPOTENCY_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_PREFIX,
        )
    }
}
