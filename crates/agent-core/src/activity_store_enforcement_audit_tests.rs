use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind, LogFieldValue, LogFields, ACTIVITY_SCHEMA_VERSION,
};

use crate::ActivityStore;

#[test]
fn activity_store_reads_latest_enforcement_audit_fields() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(&[enforcement_audit_event(
            constants::enforcement::TEST_AUDIT_EVENT_ID,
            constants::enforcement::TEST_RESULT_ID,
        )])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let fields = store
        .latest_enforcement_audit_fields()
        .expect(constants::error::ACTIVITY_STORE_QUERIES)
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(
        fields.get(constants::field::ENFORCEMENT_RESULT_ID),
        Some(&LogFieldValue::String(
            constants::enforcement::TEST_RESULT_ID.to_string()
        ))
    );
    assert_eq!(
        fields.get(constants::field::ENFORCEMENT_AUDIT_EVENT_ID),
        Some(&LogFieldValue::String(
            constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()
        ))
    );
}

#[test]
fn activity_store_reads_most_recent_enforcement_audit_fields_only() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(&[
            enforcement_audit_event(
                constants::enforcement::TEST_AUDIT_EVENT_ID,
                constants::enforcement::TEST_RESULT_ID,
            ),
            enforcement_audit_event(
                constants::enforcement::TEST_TIMER_EVENT_ID,
                constants::enforcement::TEST_TIMER_STATE_ID,
            ),
        ])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let fields = store
        .latest_enforcement_audit_fields()
        .expect(constants::error::ACTIVITY_STORE_QUERIES)
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(
        fields.get(constants::field::ENFORCEMENT_AUDIT_EVENT_ID),
        Some(&LogFieldValue::String(
            constants::enforcement::TEST_TIMER_EVENT_ID.to_string()
        ))
    );
    assert_eq!(
        fields.get(constants::field::ENFORCEMENT_RESULT_ID),
        Some(&LogFieldValue::String(
            constants::enforcement::TEST_TIMER_STATE_ID.to_string()
        ))
    );
}

#[test]
fn activity_store_returns_no_enforcement_audit_fields_when_empty() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);

    assert_eq!(
        store
            .latest_enforcement_audit_fields()
            .expect(constants::error::ACTIVITY_STORE_QUERIES),
        None
    );
}

fn enforcement_audit_event(event_id: &str, result_id: &str) -> ActivityEvent {
    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: event_id.to_string(),
        observed_at: observed_at_for_event(event_id).to_string(),
        source: ActivitySource {
            device_id: constants::enforcement::TEST_CHILD_DEVICE_ID.to_string(),
            platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
            observer: ActivityObserver::AgentService,
            source_id: constants::enforcement::SOURCE_ID_AGENT_SERVICE.to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Intervention,
            subject_id: constants::enforcement::TEST_ACTION_ID.to_string(),
            display_name: Some(constants::enforcement::MODE_TERMINATE_PROCESS.to_string()),
        },
        fields: enforcement_fields(event_id, result_id),
        evidence: Vec::new(),
    }
}

fn enforcement_fields(event_id: &str, result_id: &str) -> LogFields {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::ENFORCEMENT_RESULT_ID.to_string(),
        LogFieldValue::String(result_id.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT_ID.to_string(),
        LogFieldValue::String(event_id.to_string()),
    );
    fields
}

fn observed_at_for_event(event_id: &str) -> &'static str {
    if event_id == constants::enforcement::TEST_TIMER_EVENT_ID {
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    } else {
        constants::activity_store::TEST_FIRST_OBSERVED_AT
    }
}
