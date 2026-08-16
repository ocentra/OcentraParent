use std::fmt::Debug;

use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::test_text::{test_ok as ok, test_some as some, TestResult, TestText};
use crate::ActivityStore;

#[test]
fn activity_store_reads_latest_enforcement_audit_fields() -> TestResult {
    let store = open_in_memory_store();
    ingest_enforcement_events(
        &store,
        &[enforcement_audit_event(
            constants::enforcement::TEST_AUDIT_EVENT_ID,
            constants::enforcement::TEST_RESULT_ID,
        )],
    );
    let fields = some(
        latest_enforcement_audit_fields(&store)?,
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

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
    Ok(())
}

#[test]
fn activity_store_reads_enforcement_audit_fields_by_exact_event_id() -> TestResult {
    let store = open_in_memory_store();
    let requested_event_id = constants::enforcement::TEST_AUDIT_EVENT_ID;
    ingest_enforcement_events(
        &store,
        &[
            enforcement_audit_event(requested_event_id, constants::enforcement::TEST_RESULT_ID),
            enforcement_audit_event(
                constants::enforcement::TEST_TIMER_EVENT_ID,
                constants::enforcement::TEST_TIMER_STATE_ID,
            ),
        ],
    );

    let fields = some(
        enforcement_audit_fields_by_event_id(&store, requested_event_id)?,
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    assert_eq!(
        fields.get(constants::field::ENFORCEMENT_AUDIT_EVENT_ID),
        Some(&LogFieldValue::String(requested_event_id.to_string()))
    );
    assert_eq!(
        fields.get(constants::field::ENFORCEMENT_RESULT_ID),
        Some(&LogFieldValue::String(
            constants::enforcement::TEST_RESULT_ID.to_string()
        ))
    );
    assert_eq!(
        enforcement_audit_fields_by_event_id(&store, "missing-audit-event-id")?,
        None
    );
    Ok(())
}

#[test]
fn activity_store_replaces_exact_enforcement_audit_fields() -> TestResult {
    let store = open_in_memory_store();
    let event_id = constants::enforcement::TEST_AUDIT_EVENT_ID;
    ingest_enforcement_events(
        &store,
        &[enforcement_audit_event(
            event_id,
            constants::enforcement::TEST_RESULT_ID,
        )],
    );
    let mut replacement = enforcement_fields(event_id, constants::enforcement::TEST_TIMER_STATE_ID);
    replacement.insert(
        constants::field::EVENTS_STORED.to_string(),
        LogFieldValue::Number(1.0),
    );

    activity_store_query(
        store.replace_enforcement_audit_fields_by_event_id(event_id, &replacement),
    )?;

    assert_eq!(
        enforcement_audit_fields_by_event_id(&store, event_id)?,
        Some(replacement)
    );
    Ok(())
}

#[test]
fn activity_store_refuses_to_replace_missing_enforcement_audit_fields() {
    let store = open_in_memory_store();
    let fields = enforcement_fields(
        constants::enforcement::TEST_AUDIT_EVENT_ID,
        constants::enforcement::TEST_RESULT_ID,
    );

    assert!(store
        .replace_enforcement_audit_fields_by_event_id(
            constants::enforcement::TEST_AUDIT_EVENT_ID,
            &fields,
        )
        .is_err());
}

#[test]
fn activity_store_reads_most_recent_enforcement_audit_fields_only() -> TestResult {
    let store = open_in_memory_store();
    ingest_enforcement_events(
        &store,
        &[
            enforcement_audit_event(
                constants::enforcement::TEST_AUDIT_EVENT_ID,
                constants::enforcement::TEST_RESULT_ID,
            ),
            enforcement_audit_event(
                constants::enforcement::TEST_TIMER_EVENT_ID,
                constants::enforcement::TEST_TIMER_STATE_ID,
            ),
        ],
    );
    let fields = some(
        latest_enforcement_audit_fields(&store)?,
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

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
    Ok(())
}

#[test]
fn activity_store_uses_persisted_insert_order_for_equal_time_enforcement_audits() -> TestResult {
    let store = open_in_memory_store();
    let observed_at = constants::activity_store::TEST_FIRST_OBSERVED_AT;
    let rejected_event_id = format!(
        "{}{}",
        constants::enforcement::JOURNAL_REJECTED_ID_PREFIX,
        constants::enforcement::TEST_AUDIT_EVENT_ID,
    );
    ingest_enforcement_events(
        &store,
        &[
            enforcement_audit_event_at(
                "z-executed-audit",
                constants::enforcement::TEST_RESULT_ID,
                observed_at,
            ),
            enforcement_audit_event_at(
                &rejected_event_id,
                constants::enforcement::TEST_RESULT_ID,
                observed_at,
            ),
        ],
    );

    let fields = some(
        latest_enforcement_audit_fields(&store)?,
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    assert_eq!(
        fields.get(constants::field::ENFORCEMENT_AUDIT_EVENT_ID),
        Some(&LogFieldValue::String(rejected_event_id))
    );
    Ok(())
}

#[test]
fn activity_store_reads_latest_matching_enforcement_audit_fields() -> TestResult {
    let store = open_in_memory_store();
    ingest_enforcement_events(
        &store,
        &[
            enforcement_audit_event(
                constants::enforcement::TEST_AUDIT_EVENT_ID,
                constants::enforcement::TEST_RESULT_ID,
            ),
            enforcement_audit_event(
                constants::enforcement::TEST_TIMER_EVENT_ID,
                constants::enforcement::TEST_TIMER_STATE_ID,
            ),
        ],
    );
    let fields = some(
        latest_matching_enforcement_audit_fields(&store, |fields| {
            fields.get(constants::field::ENFORCEMENT_RESULT_ID)
                == Some(&LogFieldValue::String(
                    constants::enforcement::TEST_RESULT_ID.to_string(),
                ))
        })?,
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    assert_eq!(
        fields.get(constants::field::ENFORCEMENT_AUDIT_EVENT_ID),
        Some(&LogFieldValue::String(
            constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()
        ))
    );
    Ok(())
}

#[test]
fn activity_store_reads_bounded_recent_enforcement_audit_history_in_persisted_order() -> TestResult
{
    let store = open_in_memory_store();
    let observed_at = constants::activity_store::TEST_FIRST_OBSERVED_AT;
    ingest_enforcement_events(
        &store,
        &[
            enforcement_audit_event_at(
                constants::enforcement::TEST_AUDIT_EVENT_ID,
                constants::enforcement::TEST_RESULT_ID,
                observed_at,
            ),
            enforcement_audit_event_at(
                "executed-audit",
                constants::enforcement::TEST_TIMER_STATE_ID,
                observed_at,
            ),
            enforcement_audit_event_at(
                constants::enforcement::TEST_TIMER_EVENT_ID,
                constants::enforcement::TEST_TIMER_STATE_ID,
                constants::activity_store::TEST_SECOND_OBSERVED_AT,
            ),
        ],
    );

    let fields = recent_enforcement_audit_fields(&store, 2)?;

    assert_eq!(fields.len(), 2);
    assert_eq!(
        fields[0].get(constants::field::ENFORCEMENT_AUDIT_EVENT_ID),
        Some(&LogFieldValue::String(
            constants::enforcement::TEST_TIMER_EVENT_ID.to_string()
        ))
    );
    assert_eq!(
        fields[1].get(constants::field::ENFORCEMENT_AUDIT_EVENT_ID),
        Some(&LogFieldValue::String("executed-audit".to_string()))
    );
    Ok(())
}

#[test]
fn activity_store_returns_empty_recent_enforcement_audit_history_for_zero_limit() -> TestResult {
    let store = open_in_memory_store();
    ingest_enforcement_events(
        &store,
        &[enforcement_audit_event(
            constants::enforcement::TEST_AUDIT_EVENT_ID,
            constants::enforcement::TEST_RESULT_ID,
        )],
    );

    assert!(recent_enforcement_audit_fields(&store, 0)?.is_empty());
    Ok(())
}

#[test]
fn activity_store_returns_no_enforcement_audit_fields_when_empty() -> TestResult {
    let store = open_in_memory_store();

    assert_eq!(latest_enforcement_audit_fields(&store)?, None);
    Ok(())
}

fn open_in_memory_store() -> ActivityStore {
    activity_store_open(ActivityStore::open_in_memory())
        .expect_value(constants::error::ACTIVITY_STORE_OPENS)
}

fn ingest_enforcement_events(store: &ActivityStore, events: &[ActivityEvent]) {
    activity_store_ingest(store.ingest_events(events))
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
}

fn latest_enforcement_audit_fields(store: &ActivityStore) -> Result<Option<LogFields>, TestText> {
    activity_store_query(store.latest_enforcement_audit_fields())
}

fn enforcement_audit_fields_by_event_id(
    store: &ActivityStore,
    event_id: &str,
) -> Result<Option<LogFields>, TestText> {
    activity_store_query(store.enforcement_audit_fields_by_event_id(event_id))
}

fn latest_matching_enforcement_audit_fields(
    store: &ActivityStore,
    predicate: impl FnMut(&LogFields) -> bool,
) -> Result<Option<LogFields>, TestText> {
    activity_store_query(store.latest_matching_enforcement_audit_fields(predicate))
}

fn recent_enforcement_audit_fields(
    store: &ActivityStore,
    limit: u64,
) -> Result<Vec<LogFields>, TestText> {
    activity_store_query(store.recent_enforcement_audit_fields(limit))
}

fn activity_store_open<T, E>(result: Result<T, E>) -> Result<T, TestText>
where
    E: Debug,
{
    ok(result, constants::error::ACTIVITY_STORE_OPENS)
}

fn activity_store_ingest<T, E>(result: Result<T, E>) -> Result<T, TestText>
where
    E: Debug,
{
    ok(result, constants::error::ACTIVITY_STORE_INGESTS)
}

fn activity_store_query<T, E>(result: Result<T, E>) -> Result<T, TestText>
where
    E: Debug,
{
    ok(result, constants::error::ACTIVITY_STORE_QUERIES)
}

fn enforcement_audit_event(
    event_id: impl std::fmt::Display,
    result_id: impl std::fmt::Display,
) -> ActivityEvent {
    let event_id = TestText::from_display(event_id);
    let result_id = TestText::from_display(result_id);
    let observed_at = observed_at_for_event(event_id.to_string());
    enforcement_audit_event_at(event_id, result_id, observed_at)
}

fn enforcement_audit_event_at(
    event_id: impl std::fmt::Display,
    result_id: impl std::fmt::Display,
    observed_at: impl std::fmt::Display,
) -> ActivityEvent {
    let event_id = TestText::from_display(event_id);
    let result_id = TestText::from_display(result_id);
    let observed_at = TestText::from_display(observed_at);
    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: event_id.to_string(),
        observed_at: observed_at.to_string(),
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

fn enforcement_fields(
    event_id: impl std::fmt::Display,
    result_id: impl std::fmt::Display,
) -> LogFields {
    let event_id = TestText::from_display(event_id);
    let result_id = TestText::from_display(result_id);
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

fn observed_at_for_event(event_id: impl std::fmt::Display) -> TestText {
    let event_id = event_id.to_string();
    TestText::from_display(
        [
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ][usize::from(event_id == constants::enforcement::TEST_TIMER_EVENT_ID)],
    )
}
