use std::fmt::Display;
use std::fs::remove_file;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::journal::ActivityJournalRotationPolicy;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::{
    activity_store::ActivityStore,
    journal::ActivityJournal,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
    test_text::{TestResult, TestText},
};

#[test]
fn activity_store_ingests_journal_replay_into_sqlite() -> TestResult {
    let journal_path = temp_path(
        constants::activity_store::TEST_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let store_path = temp_path(
        constants::activity_store::TEST_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_paths(&journal_path, &store_path);
    let key = test_key();
    let mut journal = ActivityJournal::open_with_policy(
        journal_path.to_path_buf(),
        key.clone(),
        ActivityJournalRotationPolicy {
            max_segment_bytes: constants::journal::TEST_ROTATION_BYTES,
        },
    )
    .map_err(|error| {
        TestText::from_display(format!("{}: {error:?}", constants::error::JOURNAL_OPENS))
    })?;
    journal
        .append(&activity_event(
            constants::event_id::HEALTH_REPORTED,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
        ))
        .map_err(|error| {
            TestText::from_display(format!("{}: {error:?}", constants::error::JOURNAL_APPENDS))
        })?;
    journal
        .append(&activity_event(
            constants::event_id::LOG_SNAPSHOT_REPORTED,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ))
        .map_err(|error| {
            TestText::from_display(format!("{}: {error:?}", constants::error::JOURNAL_APPENDS))
        })?;
    let reader = ActivityJournal::open(journal_path.to_path_buf(), key).map_err(|error| {
        TestText::from_display(format!("{}: {error:?}", constants::error::JOURNAL_OPENS))
    })?;
    let store = ActivityStore::open(&store_path).map_err(|error| {
        TestText::from_display(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_OPENS
        ))
    })?;

    let status = store.ingest_journal(&reader).map_err(|error| {
        TestText::from_display(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_INGESTS
        ))
    })?;
    let summary = store
        .recent_summary(constants::activity_store::DEFAULT_RECENT_LIMIT)
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_QUERIES
            ))
        })?;
    cleanup_paths(&journal_path, &store_path);

    assert_eq!(status.events_ingested, 2);
    assert_eq!(status.events_stored, 2);
    assert_eq!(status.duplicate_events, 0);
    assert_eq!(
        summary.last_event_id,
        Some(constants::event_id::LOG_SNAPSHOT_REPORTED.to_string())
    );
    assert_eq!(
        summary.most_recent_kind,
        Some(ActivityEventKind::ProcessObserved)
    );
    assert_eq!(summary.returned, 2);
    Ok(())
}

#[test]
fn activity_store_reports_duplicate_ingest_without_double_counting() -> TestResult {
    let store = ActivityStore::open_in_memory().map_err(|error| {
        TestText::from_display(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_OPENS
        ))
    })?;
    let event = activity_event(
        constants::event_id::HEALTH_REPORTED,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    );

    let first = store
        .ingest_events(std::slice::from_ref(&event))
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_INGESTS
            ))
        })?;
    let second = store
        .ingest_events(std::slice::from_ref(&event))
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_INGESTS
            ))
        })?;

    assert_eq!(first.events_ingested, 1);
    assert_eq!(first.events_stored, 1);
    assert_eq!(second.events_ingested, 0);
    assert_eq!(second.duplicate_events, 1);
    assert_eq!(second.events_stored, 1);
    Ok(())
}

#[test]
fn activity_store_ingests_tracking_mvp_events_into_sqlite() -> TestResult {
    let store = ActivityStore::open_in_memory().map_err(|error| {
        TestText::from_display(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_OPENS
        ))
    })?;
    let location = tracking_activity_event(
        constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID,
        constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
        ActivityEventKind::LocationObserved,
        ActivityObserver::AndroidLocation,
        ActivitySubjectKind::Location,
    );
    let geofence = tracking_activity_event(
        constants::activity_store::TEST_TRACKING_GEOFENCE_EVENT_ID,
        constants::activity_store::TEST_TRACKING_GEOFENCE_OBSERVED_AT,
        ActivityEventKind::TrackingGeofenceTransitionEvaluated,
        ActivityObserver::TrackingEngine,
        ActivitySubjectKind::TrackingRule,
    );
    let expected_place = tracking_activity_event(
        constants::activity_store::TEST_TRACKING_EXPECTED_PLACE_EVENT_ID,
        constants::activity_store::TEST_TRACKING_EXPECTED_PLACE_OBSERVED_AT,
        ActivityEventKind::TrackingExpectedPlaceEvaluated,
        ActivityObserver::TrackingEngine,
        ActivitySubjectKind::TrackingRule,
    );
    let check_in = tracking_activity_event(
        constants::activity_store::TEST_TRACKING_CHECK_IN_EVENT_ID,
        constants::activity_store::TEST_TRACKING_CHECK_IN_OBSERVED_AT,
        ActivityEventKind::TrackingChildCheckInResponded,
        ActivityObserver::TrackingEngine,
        ActivitySubjectKind::CheckIn,
    );
    let retention_delete = tracking_activity_event(
        constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID,
        constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
        ActivityEventKind::TrackingRetentionDeleted,
        ActivityObserver::TrackingEngine,
        ActivitySubjectKind::Retention,
    );

    let status = store
        .ingest_events(&[
            location,
            geofence,
            expected_place,
            check_in,
            retention_delete,
        ])
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_INGESTS
            ))
        })?;
    let summary = store
        .recent_summary(constants::activity_store::DEFAULT_RECENT_LIMIT)
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_QUERIES
            ))
        })?;

    assert_eq!(status.events_ingested, 5);
    assert_eq!(status.events_stored, 5);
    assert_eq!(summary.returned, 5);
    assert_eq!(
        summary.most_recent_kind,
        Some(ActivityEventKind::TrackingRetentionDeleted)
    );
    assert_eq!(
        summary.most_recent_observer,
        Some(ActivityObserver::TrackingEngine)
    );
    assert_eq!(
        summary.most_recent_subject_kind,
        Some(ActivitySubjectKind::Retention)
    );
    Ok(())
}

fn activity_event(event_id: impl Display, observed_at: impl Display) -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::PID.to_string(),
        LogFieldValue::Number(std::process::id() as f64),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: event_id.to_string(),
        observed_at: observed_at.to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::WindowsProcess,
            source_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        },
        kind: ActivityEventKind::ProcessObserved,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Process,
            subject_id: constants::activity_store::TEST_PROCESS_SUBJECT_ID.to_string(),
            display_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        },
        fields,
        evidence: Vec::new(),
    }
}

fn tracking_activity_event(
    event_id: impl Display,
    observed_at: impl Display,
    kind: ActivityEventKind,
    observer: ActivityObserver,
    subject_kind: ActivitySubjectKind,
) -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(
            constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT.to_string(),
        ),
    );
    fields.insert(
        constants::field::EVIDENCE_REFERENCE_IDS.to_string(),
        LogFieldValue::String(
            constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string(),
        ),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: event_id.to_string(),
        observed_at: observed_at.to_string(),
        source: ActivitySource {
            device_id: constants::activity_store::TEST_REMOTE_DEVICE_ID.to_string(),
            platform: constants::activity_store::TEST_TRACKING_PLATFORM_ANDROID.to_string(),
            observer,
            source_id: constants::activity_store::TEST_TRACKING_SOURCE_ID.to_string(),
        },
        kind,
        subject: ActivitySubject {
            kind: subject_kind,
            subject_id: constants::activity_store::TEST_TRACKING_SUBJECT_ID.to_string(),
            display_name: Some(constants::activity_store::TEST_TRACKING_SUBJECT_NAME.to_string()),
        },
        fields,
        evidence: Vec::new(),
    }
}

fn temp_path(suffix: impl Display, extension: impl Display) -> TestText {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&unique_temp_token().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&suffix.to_string());

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(extension.to_string());
    TestText::from_display(path.display())
}

fn unique_temp_token() -> TestText {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| TestText::from_display(duration.as_nanos()))
        .unwrap_or_else(|_| TestText::from_display(constants::value::EMPTY))
}

fn cleanup_paths(journal_path: &Path, store_path: &Path) {
    let _ = remove_file(journal_path);
    let _ = remove_file(store_path);
    let mut store_wal_path = store_path.to_path_buf();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path.to_path_buf();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
    for index in 1..=3 {
        let mut rotated_path = journal_path.to_path_buf();
        let mut extension = index.to_string();
        extension.push(constants::delimiter::DOT);
        extension.push_str(constants::journal::FILE_EXTENSION);
        rotated_path.set_extension(extension);
        let _ = remove_file(rotated_path);
    }
}

fn test_key() -> JournalKey {
    JournalKey::from_bytes([9; JOURNAL_KEY_BYTES])
}
