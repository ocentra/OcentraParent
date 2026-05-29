use ocentra_parent_agent_protocol::{
    constants, ActivityReadModelState, ActivityReportFrequency, ActivityReportRequest,
    ActivitySurfaceScope, ActivitySurfaceScopeKind, LogFieldValue, ACTIVITY_SURFACE_SCHEMA_VERSION,
};

use crate::{
    activity_surface_payload::activity_report_document_payload,
    activity_surface_report::report_document,
};

#[test]
fn activity_report_payload_marks_unavailable_reports_unavailable() {
    let report = report_document(report_request(), None, Vec::new());
    let payload = activity_report_document_payload(&report);

    match payload.get(constants::field::ACTIVITY_SURFACE_STATE) {
        Some(LogFieldValue::String(state)) => {
            assert_eq!(state, constants::activity_surface::STATE_UNAVAILABLE);
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
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
