use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, IdempotencyKey, RequestId};
use ocentra_eventing::request::{EventResponseContract, RequestEvent};

use super::runtime_event::{
    tracking_child_aggregate_key, tracking_event_contract, tracking_idempotency_key,
};
use crate::constants;
use crate::AGENT_PROTOCOL_SCHEMA_VERSION;
use crate::{
    ParentNotificationRequestedEvent, TrackingChildCheckInRecordedEvent,
    TrackingChildCheckInRequestReceipt, TrackingChildCheckInRequestedEvent,
};

impl DomainEvent for TrackingChildCheckInRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        tracking_event_contract(
            constants::tracking_runtime::TRACKING_CHILD_CHECK_IN_REQUESTED_EVENT_TYPE,
        )
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        tracking_child_aggregate_key(&self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_idempotency_key(
            constants::tracking_runtime::TRACKING_CHILD_CHECK_IN_REQUESTED_EVENT_TYPE,
            &self.check_in_id,
        )
    }
}

impl RequestEvent for TrackingChildCheckInRequestedEvent {
    type Response = TrackingChildCheckInRequestReceipt;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        RequestId::parse(self.check_in_id.as_str())
    }
}

impl DomainEvent for TrackingChildCheckInRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        tracking_event_contract(
            constants::tracking_runtime::TRACKING_CHILD_CHECK_IN_RECORDED_EVENT_TYPE,
        )
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        tracking_child_aggregate_key(&self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_idempotency_key(
            constants::tracking_runtime::TRACKING_CHILD_CHECK_IN_RECORDED_EVENT_TYPE,
            &self.check_in_id,
        )
    }
}

impl EventResponseContract for TrackingChildCheckInRequestReceipt {
    fn validate(&self) -> Result<(), EventingError> {
        (self.schema_version == AGENT_PROTOCOL_SCHEMA_VERSION)
            .then_some(())
            .ok_or(EventingError::InvalidVersion)
    }
}

impl DomainEvent for ParentNotificationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        tracking_event_contract(
            constants::tracking_runtime::PARENT_NOTIFICATION_REQUESTED_EVENT_TYPE,
        )
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        tracking_child_aggregate_key(&self.child_device_id, &self.child_profile_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_idempotency_key(
            constants::tracking_runtime::PARENT_NOTIFICATION_REQUESTED_EVENT_TYPE,
            &self.notification_id,
        )
    }
}
