#[macro_use]
#[path = "../support/unit_root_basic_harness.rs"]
mod unit_root_basic_harness;
declare_agent_service_unit_root_basic_harness!();

use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReadModelState, ActivityReportFrequency, ActivityReportRequest, ActivitySurfaceScope,
    ActivitySurfaceScopeKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::{
    activity_report_document_payload_for_test, build_activity_report_document_for_test,
};

#[test]
fn activity_report_payload_marks_unavailable_reports_unavailable() {
    let report = build_activity_report_document_for_test(report_request());
    let payload = activity_report_document_payload_for_test(&report);

    let state = match payload.get(constants::field::ACTIVITY_SURFACE_STATE) {
        Some(LogFieldValue::String(value)) => value.as_str(),
        _ => assert!(false, "{}", constants::error::AGENT_EVENT_SERIALIZES),
    };
    assert_eq!(state, constants::activity_surface::STATE_UNAVAILABLE);
    assert!(report
        .sections
        .iter()
        .all(|section| section.state == ActivityReadModelState::Unavailable));
}

fn report_request() -> ActivityReportRequest {
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
