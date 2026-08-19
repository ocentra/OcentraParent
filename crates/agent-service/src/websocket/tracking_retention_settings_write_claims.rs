use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingExecutionClaimState;
use ocentra_parent_runtime_core::tracking_config_update_flow::ParentTrackingConfigUpdateEventFlowReport;

use super::{TrackingRetentionSettingsWriteFlowReport, TrackingWriteRequestParseState};

pub(super) fn tracking_parent_runtime_flow_outcome(
    result: Result<ParentTrackingConfigUpdateEventFlowReport, EventingError>,
) -> (
    Option<ParentTrackingConfigUpdateEventFlowReport>,
    Option<EventingError>,
) {
    match result {
        Ok(report) => (Some(report), None),
        Err(error) => (None, Some(error)),
    }
}

pub(super) fn tracking_write_flow_claim_state(
    parse_state: TrackingWriteRequestParseState,
    flow_report: &TrackingRetentionSettingsWriteFlowReport,
) -> TrackingExecutionClaimState {
    let flow_reached_terminal_result =
        flow_report.change_approved.is_some() || flow_report.change_rejected.is_some();
    if parse_state == TrackingWriteRequestParseState::Accepted
        && flow_report.parent_runtime_flow_error.is_none()
        && flow_report.parent_command_validated.is_some()
        && flow_reached_terminal_result
        && flow_report.audit_entry_committed.is_some()
        && flow_report.portal_read_model_updated.is_some()
    {
        return TrackingExecutionClaimState::Claimed;
    }
    TrackingExecutionClaimState::Unclaimed
}
