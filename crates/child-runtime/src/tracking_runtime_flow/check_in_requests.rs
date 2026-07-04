use ocentra_eventing::envelope::EventMetadata;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{TrackingReasonCode, TrackingTimestamp};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingChildCheckInDeliveryState, TrackingChildCheckInRequestReceipt,
    TrackingChildCheckInRequestState, TrackingChildCheckInRequestedEvent,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

pub(super) fn tracking_child_check_in_request_receipt(
    request: &TrackingChildCheckInRequestedEvent,
    metadata: &EventMetadata,
    duplicate_request: bool,
) -> Result<TrackingChildCheckInRequestReceipt, ocentra_eventing::error::EventingError> {
    let delivery_state = if request.request_state != TrackingChildCheckInRequestState::Pending
        || request.delivery_state != TrackingChildCheckInDeliveryState::Queued
    {
        TrackingChildCheckInDeliveryState::UnsupportedDelivery
    } else if tracking_child_check_in_request_is_stale(request, metadata) {
        TrackingChildCheckInDeliveryState::Stale
    } else if duplicate_request {
        TrackingChildCheckInDeliveryState::Duplicate
    } else {
        TrackingChildCheckInDeliveryState::Requested
    };

    Ok(TrackingChildCheckInRequestReceipt {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        child_device_id: request.child_device_id.clone(),
        child_profile_id: request.child_profile_id.clone(),
        check_in_id: request.check_in_id.clone(),
        related_alert_id: request.related_alert_id.clone(),
        request_state: request.request_state.clone(),
        receipt_recorded_at: TrackingTimestamp::parse(metadata.observed_at.as_str())?,
        reason_code: tracking_child_check_in_request_reason_code(&delivery_state)?,
        delivery_state,
    })
}

fn tracking_child_check_in_request_is_stale(
    request: &TrackingChildCheckInRequestedEvent,
    metadata: &EventMetadata,
) -> bool {
    request.expires_at.as_str() <= metadata.observed_at.as_str()
}

fn tracking_child_check_in_request_reason_code(
    delivery_state: &TrackingChildCheckInDeliveryState,
) -> Result<Option<TrackingReasonCode>, ocentra_eventing::error::EventingError> {
    let value = match delivery_state {
        TrackingChildCheckInDeliveryState::Duplicate => {
            constants::tracking_runtime::REASON_DUPLICATE_CHECK_IN_REQUEST
        }
        TrackingChildCheckInDeliveryState::Stale => {
            constants::tracking_runtime::REASON_STALE_CHECK_IN_REQUEST
        }
        TrackingChildCheckInDeliveryState::UnsupportedDelivery => {
            constants::tracking_runtime::REASON_UNSUPPORTED_CHECK_IN_DELIVERY
        }
        TrackingChildCheckInDeliveryState::Queued
        | TrackingChildCheckInDeliveryState::Requested => {
            return Ok(None);
        }
    };
    Ok(Some(TrackingReasonCode::parse(value)?))
}
