use super::{
    constants, ActivityEventKind, ActivityIngestStatus, ActivityObserver, ActivityRecentSummary,
    ActivitySubjectKind, ACTIVITY_QUERY_SCHEMA_VERSION,
};
use crate::activity_query::ActivityRecentQuery;
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn activity_ingest_status_serializes_to_typescript_contract_shape() {
    let status = ActivityIngestStatus {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        database_ready: true,
        events_ingested: 2,
        events_stored: 2,
        duplicate_events: 0,
        last_event_id: Some(constants::event_id::HEALTH_REPORTED.to_string()),
    };

    let serialized =
        serde_json::to_value(status).expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(serialized[constants::field::SCHEMA_VERSION], 1);
    assert_eq!(serialized[constants::field::DATABASE_READY], true);
    assert_eq!(serialized[constants::field::EVENTS_INGESTED], 2);
    assert_eq!(
        serialized[constants::field::LAST_EVENT_ID],
        constants::event_id::HEALTH_REPORTED
    );
}

#[test]
fn activity_recent_summary_serializes_to_typescript_contract_shape() {
    let query = ActivityRecentQuery {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
    };
    let summary = ActivityRecentSummary {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        limit: query.limit,
        returned: 1,
        first_observed_at: Some(constants::event_id::HEALTH_REPORTED.to_string()),
        last_observed_at: Some(constants::event_id::HEALTH_REPORTED.to_string()),
        last_event_id: Some(constants::event_id::HEALTH_REPORTED.to_string()),
        most_recent_kind: Some(ActivityEventKind::ProcessObserved),
        most_recent_observer: Some(ActivityObserver::WindowsProcess),
        most_recent_subject_kind: Some(ActivitySubjectKind::Process),
        most_recent_subject_id: Some(constants::peer::LOCAL_DEV_AGENT.to_string()),
        most_recent_subject_name: None,
    };

    let serialized =
        serde_json::to_value(summary).expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(query.limit, constants::activity_store::DEFAULT_RECENT_LIMIT);
    assert_eq!(serialized[constants::field::RETURNED], 1);
    assert_eq!(
        serialized[constants::field::MOST_RECENT_KIND],
        constants::activity_event_kind::PROCESS_OBSERVED
    );
    assert_eq!(
        serialized[constants::field::MOST_RECENT_SUBJECT_KIND],
        constants::activity_subject_kind::PROCESS
    );
}

#[test]
fn activity_protocol_string_conversions_are_stable() {
    assert_eq!(
        ActivityEventKind::ProcessObserved.as_protocol_str(),
        constants::activity_event_kind::PROCESS_OBSERVED
    );
    assert_eq!(
        ActivityEventKind::from_protocol_str(constants::activity_event_kind::PROCESS_OBSERVED),
        Some(ActivityEventKind::ProcessObserved)
    );
    assert_eq!(
        ActivityObserver::from_protocol_str(constants::activity_observer::WINDOWS_PROCESS),
        Some(ActivityObserver::WindowsProcess)
    );
    assert_eq!(
        ActivitySubjectKind::from_protocol_str(constants::activity_subject_kind::PROCESS),
        Some(ActivitySubjectKind::Process)
    );
}
