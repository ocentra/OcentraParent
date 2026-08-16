use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReadModelState, ActivityReportCustodyLabel, ActivityReportSourceLabel,
    ActivityReportSourceReachabilityState, ActivityReportSourceState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

pub(crate) fn family_sources_from_command(
    command: &AgentCommandEnvelope,
) -> Vec<ActivityReportSourceState> {
    let Some(LogFieldValue::String(raw_sources)) = command
        .payload
        .get(constants::field::ACTIVITY_FAMILY_SOURCES)
    else {
        return Vec::new();
    };

    serde_json::from_str::<Vec<ActivityReportSourceState>>(raw_sources)
        .unwrap_or_else(|_| vec![family_source_error_record()])
}

pub(crate) fn default_family_fanout_record() -> ActivityReportSourceState {
    ActivityReportSourceState {
        device_id: constants::activity_surface::FAMILY_FANOUT_SOURCE_ID.to_string(),
        reachability_state: ActivityReportSourceReachabilityState::Unreachable,
        state: ActivityReadModelState::Unavailable,
        reason: Some(constants::activity_surface::SUMMARY_FAMILY_FANOUT_UNAVAILABLE.to_string()),
        last_updated_at: None,
        custody_label: ActivityReportCustodyLabel::ChildDeviceLocalSummary,
        source_label: ActivityReportSourceLabel::FamilyFanoutSourceState,
        raw_child_evidence_included: false,
    }
}

pub(crate) fn family_source_error_record() -> ActivityReportSourceState {
    ActivityReportSourceState {
        device_id: constants::activity_surface::FAMILY_SOURCE_ERROR_ID.to_string(),
        reachability_state: ActivityReportSourceReachabilityState::Error,
        state: ActivityReadModelState::Unavailable,
        reason: Some(constants::activity_surface::SUMMARY_FAMILY_SOURCE_ERROR.to_string()),
        last_updated_at: None,
        custody_label: ActivityReportCustodyLabel::ChildDeviceLocalSummary,
        source_label: ActivityReportSourceLabel::FamilyFanoutSourceState,
        raw_child_evidence_included: false,
    }
}
