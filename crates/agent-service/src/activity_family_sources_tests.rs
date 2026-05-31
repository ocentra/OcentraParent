use ocentra_parent_agent_protocol::{
    constants, ActivityReadModelState, ActivityReportFrequency, ActivityReportRequest,
    ActivityReportSourceReachabilityState, ActivityReportSourceState, ActivitySurfaceScope,
    ActivitySurfaceScopeKind, AgentCommandEnvelope, AgentCommandName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue, LogFields,
    ACTIVITY_SURFACE_SCHEMA_VERSION, AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{
    activity_family_sources::family_sources_from_command, activity_surface_report::report_document,
    fields::fields_from_pairs,
};

#[test]
fn activity_family_sources_parse_reachable_offline_stale_and_error_records_from_command_payload() {
    let sources = family_sources_from_command(&command_with_sources(vec![
        source_record(
            constants::activity_surface::DEFAULT_DEVICE_ID,
            ActivityReportSourceReachabilityState::Reachable,
            ActivityReadModelState::Ready,
        ),
        source_record(
            constants::activity_surface::FAMILY_SOURCE_OFFLINE_ID,
            ActivityReportSourceReachabilityState::Offline,
            ActivityReadModelState::Offline,
        ),
        source_record(
            constants::activity_surface::FAMILY_SOURCE_STALE_ID,
            ActivityReportSourceReachabilityState::Unreachable,
            ActivityReadModelState::Stale,
        ),
        source_record(
            constants::activity_surface::FAMILY_SOURCE_ERROR_ID,
            ActivityReportSourceReachabilityState::Error,
            ActivityReadModelState::Unavailable,
        ),
    ]));

    assert_eq!(sources.len(), 4);
    assert_eq!(
        sources[0].reachability_state,
        ActivityReportSourceReachabilityState::Reachable
    );
    assert_eq!(
        sources[1].reachability_state,
        ActivityReportSourceReachabilityState::Offline
    );
    assert_eq!(
        sources[2].reachability_state,
        ActivityReportSourceReachabilityState::Unreachable
    );
    assert_eq!(sources[2].state, ActivityReadModelState::Stale);
    assert_eq!(
        sources[3].reachability_state,
        ActivityReportSourceReachabilityState::Error
    );
}

#[test]
fn activity_family_sources_return_error_record_for_invalid_source_payload() {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::ACTIVITY_FAMILY_SOURCES.to_string(),
        LogFieldValue::String(constants::activity_surface::SUMMARY_STORE_UNAVAILABLE.to_string()),
    );

    let sources = family_sources_from_command(&command(fields));

    assert_eq!(sources.len(), 1);
    assert_eq!(
        sources[0].reachability_state,
        ActivityReportSourceReachabilityState::Error
    );
    assert_eq!(sources[0].state, ActivityReadModelState::Unavailable);
}

#[tokio::test]
async fn activity_family_report_preserves_reachable_offline_stale_and_error_source_records() {
    let snapshot = crate::activity_surface_store::ActivitySurfaceStoreSnapshot {
        device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
        recent_returned: 1,
        last_event_id: Some(constants::event_id::HEALTH_REPORTED.to_string()),
        last_observed_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
        browser_returned: 0,
        network_returned: 0,
        games_returned: 0,
        screen_returned: 0,
    };
    let report = report_document(
        family_report_request(),
        Some(snapshot),
        vec![
            source_record(
                constants::activity_surface::FAMILY_SOURCE_OFFLINE_ID,
                ActivityReportSourceReachabilityState::Offline,
                ActivityReadModelState::Offline,
            ),
            source_record(
                constants::activity_surface::FAMILY_SOURCE_STALE_ID,
                ActivityReportSourceReachabilityState::Unreachable,
                ActivityReadModelState::Stale,
            ),
            source_record(
                constants::activity_surface::FAMILY_SOURCE_ERROR_ID,
                ActivityReportSourceReachabilityState::Error,
                ActivityReadModelState::Unavailable,
            ),
        ],
    );

    assert_eq!(report.source_states.len(), 4);
    assert_eq!(
        report.source_states[0].reachability_state,
        ActivityReportSourceReachabilityState::Reachable
    );
    assert_eq!(
        report.source_states[1].reachability_state,
        ActivityReportSourceReachabilityState::Offline
    );
    assert_eq!(
        report.source_states[2].reachability_state,
        ActivityReportSourceReachabilityState::Unreachable
    );
    assert_eq!(report.source_states[2].state, ActivityReadModelState::Stale);
    assert_eq!(
        report.source_states[3].reachability_state,
        ActivityReportSourceReachabilityState::Error
    );
    assert_eq!(report.sections[2].state, ActivityReadModelState::Ready);
}

fn command_with_sources(sources: Vec<ActivityReportSourceState>) -> AgentCommandEnvelope {
    let encoded_sources =
        serde_json::to_string(&sources).expect(constants::error::AGENT_EVENT_SERIALIZES);
    command(fields_from_pairs(vec![(
        constants::field::ACTIVITY_FAMILY_SOURCES,
        LogFieldValue::String(encoded_sources),
    )]))
}

fn command(payload: LogFields) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::parent_assistant::DEFAULT_MESSAGE_ID.to_string(),
        sent_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityReportDailyGenerate,
        payload,
    }
}

fn family_report_request() -> ActivityReportRequest {
    ActivityReportRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        frequency: ActivityReportFrequency::Daily,
        scope: ActivitySurfaceScope {
            scope_kind: ActivitySurfaceScopeKind::Family,
            family_id: Some(constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
            device_id: None,
        },
        requested_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        range_start: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        range_end: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
    }
}

fn source_record(
    device_id: &str,
    reachability_state: ActivityReportSourceReachabilityState,
    state: ActivityReadModelState,
) -> ActivityReportSourceState {
    ActivityReportSourceState {
        device_id: device_id.to_string(),
        reachability_state,
        state,
        reason: Some(constants::activity_surface::SUMMARY_FAMILY_LOCAL_SOURCE.to_string()),
        last_updated_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
    }
}
