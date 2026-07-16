use crate::event_flow_scaffold;
use ocentra_eventing::{envelope::EventMetadata, request::RequestCompletionReport};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::TrackingCheckInId;
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingChildCheckInRequestReceipt, TrackingChildCheckInRequestedEvent,
};

use super::state::TrackingRuntimeEventState;

impl TrackingRuntimeEventState {
    pub(super) fn record_parent_requested_check_in(
        &self,
        event: TrackingChildCheckInRequestedEvent,
        metadata: EventMetadata,
    ) {
        event_flow_scaffold::record_optional_event(
            &self.parent_requested_check_in,
            event,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
        event_flow_scaffold::record_optional_event(
            &self.parent_requested_check_in_metadata,
            metadata,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
    }

    pub(super) fn record_parent_requested_check_in_receipt(
        &self,
        receipt: TrackingChildCheckInRequestReceipt,
        completion: RequestCompletionReport,
    ) {
        event_flow_scaffold::record_optional_event(
            &self.parent_requested_check_in_receipt,
            receipt,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
        event_flow_scaffold::record_optional_event(
            &self.parent_requested_check_in_completion,
            completion,
            constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED,
        );
    }

    pub(super) fn mark_parent_requested_check_in_seen(&self, check_in_id: TrackingCheckInId) {
        crate::event_flow_scaffold::lock_recover(&self.seen_parent_requested_check_in_ids)
            .insert(check_in_id);
    }

    pub(super) fn has_seen_parent_requested_check_in(
        &self,
        check_in_id: &TrackingCheckInId,
    ) -> bool {
        self.seen_parent_requested_check_in_ids
            .lock()
            .map(|seen| seen.contains(check_in_id))
            .unwrap_or(false)
    }

    pub(super) fn parent_requested_check_in(&self) -> Option<TrackingChildCheckInRequestedEvent> {
        event_flow_scaffold::optional_event(&self.parent_requested_check_in)
    }

    pub(super) fn parent_requested_check_in_metadata(&self) -> Option<EventMetadata> {
        event_flow_scaffold::optional_event(&self.parent_requested_check_in_metadata)
    }

    pub(super) fn parent_requested_check_in_receipt(
        &self,
    ) -> Option<TrackingChildCheckInRequestReceipt> {
        event_flow_scaffold::optional_event(&self.parent_requested_check_in_receipt)
    }

    pub(super) fn parent_requested_check_in_completion(&self) -> Option<RequestCompletionReport> {
        event_flow_scaffold::optional_event(&self.parent_requested_check_in_completion)
    }
}
