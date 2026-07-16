#[macro_use]
#[path = "../support/unit_root_basic_harness.rs"]
mod unit_root_basic_harness;
declare_agent_service_unit_root_basic_harness!();

use crate::test_text::TestText;
use ocentra_parent_agent_protocol::activity_surface::{
    ActivityAppUseReadModel, ActivityBrowserReadModel, ActivityGamesReadModel,
    ActivityNetworkReadModel, ActivityReadModelState, ActivityReportDocument,
    ActivityReportFrequency, ActivityReportSourceLabel, ActivityReportSourceReachabilityState,
    ActivityScreenReadModel,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::handle_local_command_text_for_test;
use serde::Serialize;

#[tokio::test]
async fn activity_surface_dispatcher_returns_typed_report_events() {
    for (command, frequency) in [
        (
            AgentCommandName::AgentActivityReportDailyGenerate,
            ActivityReportFrequency::Daily,
        ),
        (
            AgentCommandName::AgentActivityReportWeeklyGenerate,
            ActivityReportFrequency::Weekly,
        ),
        (
            AgentCommandName::AgentActivityReportMonthlyGenerate,
            ActivityReportFrequency::Monthly,
        ),
    ] {
        let event = send_activity_surface_command(command, activity_payload())
            .await
            .expect(constants::error::AGENT_EVENT_SERIALIZES);

        assert_eq!(event.event, AgentEventName::AgentActivityReportGenerated);
        assert_typed_surface_state(&event);
        let report = report_document_from_event(&event);
        assert_eq!(report.frequency, frequency);
        assert_eq!(report.sections.len(), 6);
        assert_eq!(report.source_states.len(), 2);
        assert_local_source_state(&report);
        assert_eq!(
            report.source_states[1].device_id,
            constants::activity_surface::FAMILY_FANOUT_SOURCE_ID
        );
        assert_eq!(
            report.source_states[1].reachability_state,
            ActivityReportSourceReachabilityState::Unreachable
        );
        assert_eq!(
            report.source_states[1].state,
            ActivityReadModelState::Unavailable
        );
        assert_eq!(
            report.source_states[1].reason.as_deref(),
            Some(constants::activity_surface::SUMMARY_FAMILY_FANOUT_UNAVAILABLE)
        );
        assert_eq!(
            report.source_states[1].source_label,
            ActivityReportSourceLabel::FamilyFanoutSourceState
        );
    }
}

#[tokio::test]
async fn activity_surface_dispatcher_returns_typed_tab_read_model_events() {
    for (command, expected_event, expected_kind) in [
        (
            AgentCommandName::AgentActivityScreenReadModelGet,
            AgentEventName::AgentActivityScreenReadModelReported,
            constants::activity_surface::READ_MODEL_SCREEN,
        ),
        (
            AgentCommandName::AgentActivityAppUseReadModelGet,
            AgentEventName::AgentActivityAppUseReadModelReported,
            constants::activity_surface::READ_MODEL_APP_USE,
        ),
        (
            AgentCommandName::AgentActivityBrowserReadModelGet,
            AgentEventName::AgentActivityBrowserReadModelReported,
            constants::activity_surface::READ_MODEL_BROWSER,
        ),
        (
            AgentCommandName::AgentActivityGamesReadModelGet,
            AgentEventName::AgentActivityGamesReadModelReported,
            constants::activity_surface::READ_MODEL_GAMES,
        ),
        (
            AgentCommandName::AgentActivityNetworkReadModelGet,
            AgentEventName::AgentActivityNetworkReadModelReported,
            constants::activity_surface::READ_MODEL_NETWORK,
        ),
    ] {
        let event = send_activity_surface_command(command, activity_payload())
            .await
            .expect(constants::error::AGENT_EVENT_SERIALIZES);

        assert_eq!(event.event, expected_event);
        assert_typed_surface_state(&event);
        assert_eq!(
            string_payload_field(&event, constants::field::ACTIVITY_READ_MODEL_KIND).as_ref(),
            expected_kind
        );
        assert_read_model_payload(&event, expected_kind);
    }
}

async fn send_activity_surface_command(
    command: AgentCommandName,
    payload: LogFields,
) -> Result<AgentEventEnvelope, TestText> {
    let body = serialize_json(&command_envelope(command, payload))?;
    handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await
}

fn command_envelope(command: AgentCommandName, payload: LogFields) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::ACTIVITY_REPORT_GENERATED.to_string(),
        sent_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            route: AgentRoute::Localhost,
        },
        command,
        payload,
    }
}

fn activity_payload() -> LogFields {
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::SCOPE_KIND.to_string(),
        LogFieldValue::String(constants::activity_surface::SCOPE_FAMILY.to_string()),
    );
    payload.insert(
        constants::field::FAMILY_ID.to_string(),
        LogFieldValue::String(constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
    );
    payload.insert(
        constants::field::REQUESTED_AT.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
    );
    payload.insert(
        constants::field::RANGE_START.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
    );
    payload.insert(
        constants::field::RANGE_END.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
    );
    payload
}

fn assert_typed_surface_state(event: &AgentEventEnvelope) {
    let decoded = decoded_surface_state(event);
    assert!(matches!(
        decoded,
        ActivityReadModelState::Ready
            | ActivityReadModelState::Empty
            | ActivityReadModelState::Unavailable
            | ActivityReadModelState::Offline
            | ActivityReadModelState::Stale
            | ActivityReadModelState::PermissionRequired
            | ActivityReadModelState::ScaffoldOnly
    ));
}

fn decoded_surface_state(event: &AgentEventEnvelope) -> ActivityReadModelState {
    let encoded = string_payload_field(event, constants::field::ACTIVITY_SURFACE_STATE);
    serde_json::from_str(encoded.as_ref()).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn report_document_from_event(event: &AgentEventEnvelope) -> ActivityReportDocument {
    let encoded = string_payload_field(event, constants::field::ACTIVITY_REPORT_DOCUMENT);
    serde_json::from_str(encoded.as_ref()).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn assert_local_source_state(report: &ActivityReportDocument) {
    let source = &report.source_states[0];

    assert_eq!(
        source.device_id,
        constants::activity_surface::DEFAULT_DEVICE_ID
    );
    assert_eq!(
        source.source_label,
        ActivityReportSourceLabel::ActivityQueryStoreSummary
    );
    assert_eq!(
        source.reachability_state,
        ActivityReportSourceReachabilityState::Reachable
    );
    assert_eq!(
        source.reason.as_deref(),
        Some(constants::activity_surface::SUMMARY_FAMILY_LOCAL_SOURCE)
    );
    assert!(matches!(
        source.state,
        ActivityReadModelState::Ready | ActivityReadModelState::Empty
    ));
}

fn assert_read_model_payload(event: &AgentEventEnvelope, expected_kind: impl std::fmt::Display) {
    let expected_kind = expected_kind.to_string();
    let expected_state = decoded_surface_state(event);
    let read_model = string_payload_field(event, constants::field::ACTIVITY_READ_MODEL);
    match expected_kind {
        constants::activity_surface::READ_MODEL_SCREEN => {
            let decoded: ActivityScreenReadModel = serde_json::from_str(read_model.as_ref())
                .expect(constants::error::AGENT_EVENT_SERIALIZES);
            assert_eq!(decoded.state, expected_state);
            assert!(decoded.rows.len() <= constants::activity_store::DEFAULT_RECENT_LIMIT as usize);
        }
        constants::activity_surface::READ_MODEL_APP_USE => {
            let decoded: ActivityAppUseReadModel = serde_json::from_str(read_model.as_ref())
                .expect(constants::error::AGENT_EVENT_SERIALIZES);
            assert_eq!(decoded.state, expected_state);
            assert!(decoded.rows.len() <= constants::activity_store::DEFAULT_RECENT_LIMIT as usize);
        }
        constants::activity_surface::READ_MODEL_BROWSER => {
            let decoded: ActivityBrowserReadModel = serde_json::from_str(read_model.as_ref())
                .expect(constants::error::AGENT_EVENT_SERIALIZES);
            assert_eq!(decoded.state, expected_state);
            assert!(decoded.rows.len() <= constants::activity_store::DEFAULT_RECENT_LIMIT as usize);
        }
        constants::activity_surface::READ_MODEL_GAMES => {
            let decoded: ActivityGamesReadModel = serde_json::from_str(read_model.as_ref())
                .expect(constants::error::AGENT_EVENT_SERIALIZES);
            assert_eq!(decoded.state, expected_state);
            assert!(decoded.rows.len() <= constants::activity_store::DEFAULT_RECENT_LIMIT as usize);
        }
        constants::activity_surface::READ_MODEL_NETWORK => {
            let decoded: ActivityNetworkReadModel = serde_json::from_str(read_model.as_ref())
                .expect(constants::error::AGENT_EVENT_SERIALIZES);
            assert_eq!(decoded.state, expected_state);
            assert!(decoded.rows.len() <= constants::activity_store::DEFAULT_RECENT_LIMIT as usize);
        }
        _ => assert!(false, "{}", constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn string_payload_field(event: &AgentEventEnvelope, field: impl std::fmt::Display) -> TestText {
    let field = field.to_string();
    match event.payload.get(field.as_str()) {
        Some(LogFieldValue::String(value)) => TestText::from_display(value.as_str()),
        _ => assert!(false, "{}", constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn serialize_json<T>(value: &T) -> Result<TestText, TestText>
where
    T: Serialize + ?Sized,
{
    serde_json::to_string(value)
        .map(TestText::from_display)
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error}",
                constants::error::AGENT_EVENT_SERIALIZES
            ))
        })
}
