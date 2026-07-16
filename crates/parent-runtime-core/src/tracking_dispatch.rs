use std::time::Duration;

use ocentra_eventing::{
    bus::EventBus, envelope::DomainEvent, envelope::EventContract, envelope::EventMetadata,
    envelope::EventSource, error::EventingError, ids::AggregateKey, ids::CorrelationId,
    ids::EventCustody, ids::EventId, ids::EventType, ids::IdempotencyKey, ids::RecordedAt,
    ids::RuntimeInstanceId, ids::RuntimeRole, ids::SchemaVersion, ids::SourceComponent,
    ids::SourceService, ids::TargetHandler, request::RequestOptions, request::RequestReport,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::{
    config_update_event::{ParentTrackingConfigUpdatedEvent, TrackingConfigUpdateTargetScope},
    runtime_event::{TrackingChildCheckInRequestReceipt, TrackingChildCheckInRequestedEvent},
};
use serde::{Deserialize, Serialize};

const PARENT_RUNTIME_SCHEMA_VERSION: u16 = 1;
pub const PARENT_RUNTIME_TRACKING_DISPATCH_EVALUATED_EVENT_TYPE: &str =
    "parent-runtime.tracking-dispatch.evaluated";
const PARENT_RUNTIME_IDEMPOTENCY_SEPARATOR: &str = ":";
const PARENT_RUNTIME_TRACKING_DISPATCH_PREFIX: &str = "parent-runtime-tracking-dispatch:";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentRuntimeTarget {
    #[serde(rename = "household")]
    Household,
    #[serde(rename = "child-device")]
    ChildDevice,
    #[serde(rename = "parent-only")]
    ParentOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildRuntimeDispatchState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildAcknowledgementState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentRuntimeOriginState {
    #[serde(rename = "trusted-local-ui")]
    TrustedLocalUi,
    #[serde(rename = "untrusted")]
    Untrusted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildRuntimePublishState {
    #[serde(rename = "publish")]
    Publish,
    #[serde(rename = "do-not-publish")]
    DoNotPublish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAuditRetentionState {
    #[serde(rename = "retain")]
    Retain,
    #[serde(rename = "do-not-retain")]
    DoNotRetain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildAcknowledgementWaitState {
    #[serde(rename = "await")]
    Await,
    #[serde(rename = "do-not-await")]
    DoNotAwait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentRuntimeChangeRequest {
    pub target: ParentRuntimeTarget,
    pub origin_state: ParentRuntimeOriginState,
    pub child_runtime_dispatch_state: ChildRuntimeDispatchState,
    pub child_acknowledgement_state: ChildAcknowledgementState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentRuntimeDispatchDecision {
    pub target: ParentRuntimeTarget,
    pub child_runtime_publish_state: ChildRuntimePublishState,
    pub parent_audit_retention_state: ParentAuditRetentionState,
    pub child_acknowledgement_wait_state: ChildAcknowledgementWaitState,
}

impl ParentRuntimeDispatchDecision {
    pub async fn publish_tracking_child_check_in_request(
        &self,
        bus: &EventBus,
        event: TrackingChildCheckInRequestedEvent,
    ) -> Result<Option<RequestReport<TrackingChildCheckInRequestReceipt>>, EventingError> {
        if self.child_runtime_publish_state != ChildRuntimePublishState::Publish {
            return Ok(None);
        }

        let metadata = tracking_child_check_in_request_metadata(&event)?;
        if self.child_acknowledgement_wait_state == ChildAcknowledgementWaitState::Await {
            return Ok(Some(
                bus.publish_request(
                    event,
                    metadata,
                    RequestOptions::with_timeout(Duration::from_millis(
                        constants::tracking_runtime::TRACKING_CHILD_CHECK_IN_REQUEST_TIMEOUT_MS,
                    ))?,
                )
                .await?,
            ));
        }

        bus.publish(event, metadata).await?;
        Ok(None)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ParentRuntimeDispatchId(String);

impl ParentRuntimeDispatchId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(EventingError::EmptyValue {
                field: "parent_runtime.dispatch_id",
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ParentRuntimeDispatchId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<ParentRuntimeDispatchId> for String {
    fn from(value: ParentRuntimeDispatchId) -> Self {
        value.0
    }
}

impl std::fmt::Display for ParentRuntimeDispatchId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentRuntimeTrackingDispatchEvaluatedEvent {
    pub dispatch_id: ParentRuntimeDispatchId,
    pub source_event: ParentTrackingConfigUpdatedEvent,
    pub child_acknowledgement_state: ChildAcknowledgementState,
    pub decision: ParentRuntimeDispatchDecision,
}

impl DomainEvent for ParentRuntimeTrackingDispatchEvaluatedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(PARENT_RUNTIME_TRACKING_DISPATCH_EVALUATED_EVENT_TYPE)?,
            SchemaVersion::new(PARENT_RUNTIME_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        self.source_event.aggregate_key()
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "{}{}{}",
            PARENT_RUNTIME_TRACKING_DISPATCH_EVALUATED_EVENT_TYPE,
            PARENT_RUNTIME_IDEMPOTENCY_SEPARATOR,
            self.dispatch_id
        ))
    }
}

pub fn route_parent_runtime_change(
    request: ParentRuntimeChangeRequest,
) -> ParentRuntimeDispatchDecision {
    let dispatch_required =
        request.child_runtime_dispatch_state == ChildRuntimeDispatchState::Required;
    let trusted_origin = request.origin_state == ParentRuntimeOriginState::TrustedLocalUi;
    let child_target = matches!(
        request.target,
        ParentRuntimeTarget::Household | ParentRuntimeTarget::ChildDevice
    );
    let publish_to_child_runtime = dispatch_required && trusted_origin && child_target;

    ParentRuntimeDispatchDecision {
        target: request.target,
        child_runtime_publish_state: if publish_to_child_runtime {
            ChildRuntimePublishState::Publish
        } else {
            ChildRuntimePublishState::DoNotPublish
        },
        parent_audit_retention_state: ParentAuditRetentionState::Retain,
        child_acknowledgement_wait_state: if publish_to_child_runtime
            && request.child_acknowledgement_state == ChildAcknowledgementState::Required
        {
            ChildAcknowledgementWaitState::Await
        } else {
            ChildAcknowledgementWaitState::DoNotAwait
        },
    }
}

pub fn route_parent_tracking_config_update_event(
    event: &ParentTrackingConfigUpdatedEvent,
    child_acknowledgement_state: ChildAcknowledgementState,
) -> ParentRuntimeDispatchDecision {
    route_parent_tracking_config_update_event_from_origin(
        event,
        child_acknowledgement_state,
        ParentRuntimeOriginState::TrustedLocalUi,
    )
}

pub fn parent_runtime_tracking_dispatch_evaluated_event(
    event: &ParentTrackingConfigUpdatedEvent,
    child_acknowledgement_state: ChildAcknowledgementState,
) -> ParentRuntimeTrackingDispatchEvaluatedEvent {
    parent_runtime_tracking_dispatch_evaluated_event_from_origin(
        event,
        child_acknowledgement_state,
        ParentRuntimeOriginState::TrustedLocalUi,
    )
}

pub fn parent_runtime_tracking_dispatch_evaluated_event_from_origin(
    event: &ParentTrackingConfigUpdatedEvent,
    child_acknowledgement_state: ChildAcknowledgementState,
    origin_state: ParentRuntimeOriginState,
) -> ParentRuntimeTrackingDispatchEvaluatedEvent {
    ParentRuntimeTrackingDispatchEvaluatedEvent {
        dispatch_id: ParentRuntimeDispatchId(parent_runtime_dispatch_ref(event)),
        source_event: event.clone(),
        child_acknowledgement_state,
        decision: route_parent_tracking_config_update_event_from_origin(
            event,
            child_acknowledgement_state,
            origin_state,
        ),
    }
}

pub fn route_parent_tracking_config_update_event_from_origin(
    event: &ParentTrackingConfigUpdatedEvent,
    child_acknowledgement_state: ChildAcknowledgementState,
    origin_state: ParentRuntimeOriginState,
) -> ParentRuntimeDispatchDecision {
    route_parent_runtime_change(ParentRuntimeChangeRequest {
        target: parent_runtime_target_from_tracking_scope(&event.target.scope),
        origin_state,
        child_runtime_dispatch_state: ChildRuntimeDispatchState::Required,
        child_acknowledgement_state,
    })
}

pub fn parent_runtime_target_from_tracking_scope(
    scope: &TrackingConfigUpdateTargetScope,
) -> ParentRuntimeTarget {
    match scope {
        TrackingConfigUpdateTargetScope::Family | TrackingConfigUpdateTargetScope::DeviceGroup => {
            ParentRuntimeTarget::Household
        }
        TrackingConfigUpdateTargetScope::ChildProfile
        | TrackingConfigUpdateTargetScope::ChildDevice => ParentRuntimeTarget::ChildDevice,
    }
}

fn parent_runtime_dispatch_ref(event: &ParentTrackingConfigUpdatedEvent) -> String {
    let mut value = String::from(PARENT_RUNTIME_TRACKING_DISPATCH_PREFIX);
    value.push_str(event.source_command_id.as_str());
    value
}

fn tracking_child_check_in_request_metadata(
    event: &TrackingChildCheckInRequestedEvent,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        tracking_child_check_in_correlation_id(event.check_in_id.as_str())?,
        tracking_child_check_in_source()?,
        RecordedAt::parse(event.requested_at.as_str())?,
        Some(TargetHandler::parse(
            constants::tracking_runtime::TARGET_HANDLER_CHILD_TRACKING_CHECK_IN_REQUESTER,
        )?),
    ))
}

fn tracking_child_check_in_source() -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(constants::eventing_source::CUSTODY_LOCAL_JOURNAL)?,
        RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER)?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(constants::tracking_runtime::SOURCE_COMPONENT_PARENT_RUNTIME)?,
        RuntimeInstanceId::parse(constants::peer::PORTAL_DEV)?,
    ))
}

fn tracking_child_check_in_correlation_id(
    check_in_id: &str,
) -> Result<CorrelationId, EventingError> {
    let mut value = String::from(constants::tracking_runtime::CORRELATION_PREFIX);
    value.push_str(check_in_id);
    CorrelationId::parse(value)
}
