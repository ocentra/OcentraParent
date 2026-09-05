use std::{
    fs::{create_dir_all, remove_dir, remove_file},
    path::{Path, PathBuf},
};

use ocentra_eventing::{
    expect_value::ExpectValue,
    ids::EventId,
    journal::{
        ndjson::{NdjsonEventJournal, NdjsonJournalOptions, NdjsonJournalRecord},
        policy::JournalDispatchPhase,
    },
    replay::ReplayFilter,
};
use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::{
    activity::{
        ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
        ActivitySubjectKind, ACTIVITY_SCHEMA_VERSION,
    },
    constants,
    enforcement::{
        EnforcementAuditJournalEvent, EnforcementAuditJournalProvenance, EnforcementResult,
    },
    logging::{LogFieldValue, LogFields},
    policy_constants,
    transport::{
        AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
        AgentPeerRole, AgentRoute,
    },
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::enforcement_timer_state_path::EnforcementTimerStatePath;
use crate::production_enforcement_api::{
    build_enforcement_audit_report_with_paths, EnforcementJournalPaths,
};

#[path = "enforcement_eventing_retry_production_terminal_ordering_tests.rs"]
mod terminal_ordering_tests;

#[tokio::test]
async fn production_command_retry_keeps_eventing_identity_and_command_correlation() {
    let artifacts = TestArtifacts::new("exact-retry");
    let command = command();
    let before_execution = chrono::Utc::now() - chrono::Duration::seconds(1);
    record_unrelated_activity(&artifacts, "before-command");
    let first =
        build_enforcement_audit_report_with_paths(command.clone(), artifacts.paths.clone()).await;
    let after_execution = chrono::Utc::now() + chrono::Duration::seconds(1);
    let first_result = enforcement_result(&first.payload);
    let first_completed_at = first_result
        .completed_at
        .as_deref()
        .expect_value("first execution completion timestamp");
    let parsed_completed_at = chrono::DateTime::parse_from_rfc3339(first_completed_at)
        .expect_value("parse first execution completion timestamp")
        .with_timezone(&chrono::Utc);
    assert_ne!(first_completed_at, command.sent_at);
    assert!(parsed_completed_at >= before_execution);
    assert!(parsed_completed_at <= after_execution);
    let persisted_after_first = ActivityStore::open(&artifacts.paths.store_path)
        .expect_value("open first execution activity store")
        .enforcement_audit_fields_by_event_id(constants::enforcement::TEST_AUDIT_EVENT_ID)
        .expect_value("read first execution persisted report")
        .expect_value("first execution persisted report exists");
    assert_eq!(enforcement_result(&persisted_after_first), first_result);
    record_unrelated_activity(&artifacts, "after-command");
    let second =
        build_enforcement_audit_report_with_paths(command.clone(), artifacts.paths.clone()).await;
    let journal = artifacts.journal();
    let replay = journal
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("replay production command eventing audit sidecar");

    assert_eq!(first.event, AgentEventName::AgentEnforcementAuditReported);
    assert_eq!(second.event, AgentEventName::AgentEnforcementAuditReported);
    assert_eq!(first.payload, second.payload);
    assert_eq!(enforcement_result(&second.payload), first_result);
    assert_eq!(
        first.payload.get(constants::field::EVENTS_STORED),
        Some(&LogFieldValue::Number(3.0))
    );
    assert_eq!(
        ActivityStore::open(&artifacts.paths.store_path)
            .expect_value("open retry activity store")
            .status()
            .expect_value("read current retry activity store status")
            .events_stored,
        4
    );
    assert_eq!(replay.records.len(), 2);
    assert!(replay.records.iter().all(|record| {
        record.envelope.correlation_id.as_str() == command.message_id
            && record.envelope.observed_at.as_str() == command.sent_at
    }));
    let raw = tokio::fs::read_to_string(&artifacts.eventing_path)
        .await
        .expect_value("read production eventing journal");
    let entries = raw
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(index, line)| NdjsonJournalRecord::parse(line, index + 1))
        .filter_map(|record| match record {
            Ok(record) => record.entry().map(Ok),
            Err(error) => Some(Err(error)),
        })
        .collect::<Result<Vec<_>, _>>()
        .expect_value("decode production eventing journal");
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].phase, JournalDispatchPhase::BeforeDispatch);
    assert_eq!(entries[1].phase, JournalDispatchPhase::AfterDispatch);
}

#[tokio::test]
async fn production_command_retry_with_mismatched_identity_fails_closed_without_new_records() {
    let artifacts = TestArtifacts::new("identity-mismatch");
    let command = command();
    let first =
        build_enforcement_audit_report_with_paths(command.clone(), artifacts.paths.clone()).await;
    assert_eq!(first.event, AgentEventName::AgentEnforcementAuditReported);

    let mut mismatch = command;
    mismatch.target.route = AgentRoute::LocalNetwork;
    let rejected =
        build_enforcement_audit_report_with_paths(mismatch, artifacts.paths.clone()).await;
    assert_eq!(rejected.event, AgentEventName::AgentCommandRejected);
    assert_reason(
        &rejected.payload,
        constants::enforcement::REJECTION_RETRY_IDENTITY_MISMATCH,
    );

    let replay = artifacts
        .journal()
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("replay identity-mismatch production eventing journal");
    assert_eq!(replay.records.len(), 2);
}

#[tokio::test]
async fn production_command_retry_with_before_only_state_requires_manual_reconciliation() {
    let seed = TestArtifacts::new("before-only-seed");
    let command = command();
    let first =
        build_enforcement_audit_report_with_paths(command.clone(), seed.paths.clone()).await;
    assert_eq!(first.event, AgentEventName::AgentEnforcementAuditReported);
    let seed_replay = seed
        .journal()
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("replay production seed journal");
    let before = seed_replay
        .records
        .into_iter()
        .find(|record| {
            record
                .envelope
                .event_id
                .as_str()
                .starts_with(constants::enforcement::JOURNAL_BEFORE_ACTION_ID_PREFIX)
        })
        .expect_value("production before-dispatch journal envelope")
        .envelope;

    let retry = TestArtifacts::new("before-only-retry");
    tokio::fs::create_dir_all(
        retry
            .eventing_path
            .parent()
            .expect_value("eventing test artifact parent"),
    )
    .await
    .expect_value("create bounded eventing test artifact parent");
    retry
        .journal()
        .append_phase_idempotent_by_event_id(&before, JournalDispatchPhase::BeforeDispatch)
        .await
        .expect_value("persist production before-only crash state");

    let rejected = build_enforcement_audit_report_with_paths(command, retry.paths.clone()).await;
    assert_eq!(rejected.event, AgentEventName::AgentCommandRejected);
    assert_reason(
        &rejected.payload,
        constants::enforcement::REJECTION_RETRY_RECONCILIATION_REQUIRED,
    );
    let replay = retry
        .journal()
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("replay before-only production eventing journal");
    assert_eq!(replay.records.len(), 1);
    assert!(retry.eventing_path.exists());
    assert_eq!(raw_event_entry_count(&retry.eventing_path).await, 1);
    assert!(!retry.paths.store_path.exists());
    assert!(!retry.paths.journal_path.exists());
}

#[tokio::test]
async fn production_command_retry_with_after_event_but_missing_store_requires_reconciliation() {
    let artifacts = TestArtifacts::new("after-without-store");
    let command = command();
    let first =
        build_enforcement_audit_report_with_paths(command.clone(), artifacts.paths.clone()).await;
    assert_eq!(first.event, AgentEventName::AgentEnforcementAuditReported);
    remove_store_files(&artifacts.paths);

    let rejected =
        build_enforcement_audit_report_with_paths(command, artifacts.paths.clone()).await;

    assert_eq!(rejected.event, AgentEventName::AgentCommandRejected);
    assert_reason(
        &rejected.payload,
        constants::enforcement::REJECTION_RETRY_RECONCILIATION_REQUIRED,
    );
    let replay = artifacts
        .journal()
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("replay after-without-store production journal");
    assert_eq!(replay.records.len(), 2);
    assert_eq!(adapter_result_count(&replay.records), 1);
    assert_eq!(raw_event_entry_count(&artifacts.eventing_path).await, 2);
    assert!(!artifacts.paths.store_path.exists());
}

#[tokio::test]
async fn production_command_retry_with_incomplete_stored_report_requires_reconciliation() {
    let artifacts = TestArtifacts::new("incomplete-stored-report");
    let command = command();
    let first =
        build_enforcement_audit_report_with_paths(command.clone(), artifacts.paths.clone()).await;
    assert_eq!(first.event, AgentEventName::AgentEnforcementAuditReported);
    remove_report_field(&artifacts.paths, constants::field::DATABASE_READY);

    let rejected =
        build_enforcement_audit_report_with_paths(command, artifacts.paths.clone()).await;

    assert_eq!(rejected.event, AgentEventName::AgentCommandRejected);
    assert_reason(
        &rejected.payload,
        constants::enforcement::REJECTION_RETRY_RECONCILIATION_REQUIRED,
    );
    let replay = artifacts
        .journal()
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("replay incomplete-report production journal");
    assert_eq!(replay.records.len(), 2);
    assert_eq!(adapter_result_count(&replay.records), 1);
    assert_eq!(raw_event_entry_count(&artifacts.eventing_path).await, 2);
}

#[tokio::test]
async fn production_command_retry_with_unrelated_stored_journal_id_requires_reconciliation() {
    let artifacts = TestArtifacts::new("unrelated-stored-journal-id");
    let command = command();
    let first =
        build_enforcement_audit_report_with_paths(command.clone(), artifacts.paths.clone()).await;
    assert_eq!(first.event, AgentEventName::AgentEnforcementAuditReported);
    replace_report_field(
        &artifacts.paths,
        constants::field::ENFORCEMENT_JOURNAL_EVENT_ID,
        LogFieldValue::String(constants::enforcement::TEST_TIMER_EVENT_ID.to_string()),
    );

    let rejected =
        build_enforcement_audit_report_with_paths(command, artifacts.paths.clone()).await;

    assert_eq!(rejected.event, AgentEventName::AgentCommandRejected);
    assert_reason(
        &rejected.payload,
        constants::enforcement::REJECTION_RETRY_RECONCILIATION_REQUIRED,
    );
    let replay = artifacts
        .journal()
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("replay unrelated-journal-id production journal");
    assert_eq!(replay.records.len(), 2);
    assert_eq!(adapter_result_count(&replay.records), 1);
    assert_eq!(raw_event_entry_count(&artifacts.eventing_path).await, 2);
}

#[tokio::test]
async fn production_command_retry_rejects_incomplete_v3_without_adapter_or_new_records() {
    let seed = TestArtifacts::new("incomplete-v3-seed");
    let command = command();
    let first =
        build_enforcement_audit_report_with_paths(command.clone(), seed.paths.clone()).await;
    assert_eq!(first.event, AgentEventName::AgentEnforcementAuditReported);
    let seed_raw = tokio::fs::read_to_string(&seed.eventing_path)
        .await
        .expect_value("read complete V3 seed journal");
    let incomplete_raw = format!(
        "{}\n",
        seed_raw.lines().take(2).collect::<Vec<_>>().join("\n")
    );

    let retry = TestArtifacts::new("incomplete-v3-retry");
    tokio::fs::create_dir_all(
        retry
            .eventing_path
            .parent()
            .expect_value("incomplete V3 artifact parent"),
    )
    .await
    .expect_value("create incomplete V3 artifact parent");
    tokio::fs::write(&retry.eventing_path, incomplete_raw)
        .await
        .expect_value("persist incomplete V3 journal");
    let before_retry = retry
        .journal()
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("replay incomplete V3 journal before retry");
    assert!(before_retry.records.is_empty());
    assert_eq!(before_retry.skipped_count, 1);
    assert_eq!(before_retry.cursor.next_sequence, 1);

    let rejected = build_enforcement_audit_report_with_paths(command, retry.paths.clone()).await;

    assert_eq!(rejected.event, AgentEventName::AgentCommandRejected);
    assert_reason(
        &rejected.payload,
        constants::enforcement::REJECTION_RETRY_RECONCILIATION_REQUIRED,
    );
    let after_retry = retry
        .journal()
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("replay incomplete V3 journal after retry");
    assert!(after_retry.records.is_empty());
    assert_eq!(after_retry.skipped_count, 1);
    assert_eq!(after_retry.cursor.next_sequence, 1);
    assert_eq!(adapter_result_count(&after_retry.records), 0);
    assert_eq!(raw_event_entry_count(&retry.eventing_path).await, 1);
    assert!(!retry.paths.store_path.exists());
    assert!(!retry.paths.journal_path.exists());
}

struct TestArtifacts {
    root: PathBuf,
    paths: EnforcementJournalPaths,
    eventing_path: PathBuf,
}

impl TestArtifacts {
    fn new(label: &str) -> Self {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/test-artifacts/enforcement-eventing-retry")
            .join(format!("{label}-{}", EventId::generated().as_str()));
        let paths = paths(&root);
        let eventing_path = eventing_path(&paths);
        Self {
            root,
            paths,
            eventing_path,
        }
    }

    fn journal(&self) -> NdjsonEventJournal {
        NdjsonEventJournal::with_options(
            self.eventing_path.clone(),
            NdjsonJournalOptions::hash_chain(),
        )
    }
}

impl Drop for TestArtifacts {
    fn drop(&mut self) {
        cleanup(&self.paths, &self.eventing_path, &self.root);
    }
}

fn assert_reason(payload: &LogFields, expected: &str) {
    assert_eq!(
        payload.get(constants::field::REASON),
        Some(&LogFieldValue::String(expected.to_string()))
    );
}

fn enforcement_result(payload: &LogFields) -> EnforcementResult {
    let serialized = match payload.get(constants::field::ENFORCEMENT_RESULT) {
        Some(LogFieldValue::String(value)) => Some(value.as_str()),
        _ => None,
    }
    .expect_value("persisted enforcement result must be a serialized value");
    serde_json::from_str(serialized).expect_value("decode persisted enforcement result")
}

fn record_unrelated_activity(artifacts: &TestArtifacts, label: &str) {
    create_dir_all(
        artifacts
            .paths
            .store_path
            .parent()
            .expect_value("retry activity store parent"),
    )
    .expect_value("create retry activity store parent");
    let event = unrelated_activity_event(label);
    crate::activity_capture_persistence::record_activity_events_to_paths(
        &artifacts.paths.journal_path,
        &artifacts.paths.key_path,
        &artifacts.paths.store_path,
        &[event],
    )
    .expect_value("persist unrelated retry activity");
}

fn unrelated_activity_event(label: &str) -> ActivityEvent {
    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: format!("unrelated-{label}-{}", EventId::generated().as_str()),
        observed_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: ActivitySource {
            device_id: constants::enforcement::TEST_CHILD_DEVICE_ID.to_string(),
            platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
            observer: ActivityObserver::AgentService,
            source_id: constants::enforcement::SOURCE_ID_AGENT_SERVICE.to_string(),
        },
        kind: ActivityEventKind::ProcessObserved,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Process,
            subject_id: format!("unrelated-{label}"),
            display_name: None,
        },
        fields: LogFields::new(),
        evidence: Vec::new(),
    }
}

fn remove_report_field(paths: &EnforcementJournalPaths, field: &str) {
    let store = ActivityStore::open(&paths.store_path).expect_value("open retry activity store");
    let fields = store
        .enforcement_audit_fields_by_event_id(constants::enforcement::TEST_AUDIT_EVENT_ID)
        .expect_value("read persisted enforcement report")
        .expect_value("persisted enforcement report exists");
    let incomplete = fields
        .into_inner()
        .into_iter()
        .filter(|(key, _)| key != field)
        .collect::<LogFields>();
    store
        .replace_enforcement_audit_fields_by_event_id(
            constants::enforcement::TEST_AUDIT_EVENT_ID,
            &incomplete,
        )
        .expect_value("replace persisted enforcement report");
}

fn replace_report_field(paths: &EnforcementJournalPaths, field: &str, value: LogFieldValue) {
    let store = ActivityStore::open(&paths.store_path).expect_value("open retry activity store");
    let mut fields = store
        .enforcement_audit_fields_by_event_id(constants::enforcement::TEST_AUDIT_EVENT_ID)
        .expect_value("read persisted enforcement report")
        .expect_value("persisted enforcement report exists");
    fields.insert(field.to_string(), value);
    store
        .replace_enforcement_audit_fields_by_event_id(
            constants::enforcement::TEST_AUDIT_EVENT_ID,
            &fields,
        )
        .expect_value("replace persisted enforcement report");
}

fn adapter_result_count(records: &[ocentra_eventing::replay::ReplayRecord]) -> usize {
    records
        .iter()
        .filter(|record| {
            record
                .envelope
                .decode::<EnforcementAuditJournalEvent>()
                .is_ok_and(|decoded| {
                    decoded.payload().provenance == EnforcementAuditJournalProvenance::AdapterResult
                })
        })
        .count()
}

async fn raw_event_entry_count(path: &Path) -> usize {
    tokio::fs::read_to_string(path)
        .await
        .expect_value("read raw production eventing journal")
        .lines()
        .enumerate()
        .filter(|(index, line)| {
            NdjsonJournalRecord::parse(line, index + 1).is_ok_and(|record| record.entry().is_some())
        })
        .count()
}

fn remove_store_files(paths: &EnforcementJournalPaths) {
    let _ = remove_file(&paths.store_path);
    let _ = remove_file(
        paths
            .store_path
            .with_extension(constants::activity_store::WAL_FILE_EXTENSION),
    );
    let _ = remove_file(
        paths
            .store_path
            .with_extension(constants::activity_store::SHM_FILE_EXTENSION),
    );
}

fn command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::enforcement::TEST_ACTION_ID.to_string(),
        sent_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::enforcement::TEST_CHILD_DEVICE_ID.to_string(),
            platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentEnforcementExecute,
        payload: payload(),
    }
}

fn payload() -> LogFields {
    let mut fields = LogFields::new();
    for (key, value) in [
        (
            constants::field::POLICY_DECISION_ID,
            policy_constants::TEST_DECISION_ID,
        ),
        (
            constants::field::POLICY_VERSION,
            policy_constants::TEST_POLICY_VERSION,
        ),
        (
            constants::field::POLICY_ACTION,
            policy_constants::ACTION_BLOCK,
        ),
        (
            constants::field::POLICY_TARGET_TYPE,
            policy_constants::TARGET_TYPE_PROCESS,
        ),
        (
            constants::field::TARGET_ID,
            constants::enforcement::TEST_PROCESS_TARGET_ID,
        ),
        (
            constants::field::POLICY_TARGET_VALUE,
            constants::enforcement::TEST_PROCESS_TARGET_VALUE,
        ),
        (
            constants::field::POLICY_REASON_CODES,
            policy_constants::TEST_REASON_PARENT_BLOCK,
        ),
        (
            constants::field::POLICY_RULE_IDS,
            policy_constants::TEST_BLOCK_RULE_ID,
        ),
        (
            constants::field::EVIDENCE_REFERENCE_IDS,
            policy_constants::TEST_EVIDENCE_ID,
        ),
        (
            constants::field::REQUESTED_AT,
            policy_constants::TEST_EVALUATED_AT,
        ),
        (
            constants::field::EXPIRES_AT,
            policy_constants::TEST_EXPIRES_AT,
        ),
        (
            constants::field::ENFORCEMENT_ACTION_ID,
            constants::enforcement::TEST_ACTION_ID,
        ),
        (
            constants::field::ENFORCEMENT_RESULT_ID,
            constants::enforcement::TEST_RESULT_ID,
        ),
        (
            constants::field::ENFORCEMENT_AUDIT_EVENT_ID,
            constants::enforcement::TEST_AUDIT_EVENT_ID,
        ),
        (
            constants::field::ENFORCEMENT_TIMER_EVENT_ID,
            constants::enforcement::TEST_TIMER_EVENT_ID,
        ),
    ] {
        fields.insert(key.to_string(), LogFieldValue::String(value.to_string()));
    }
    fields.insert(
        constants::field::POLICY_DRY_RUN.to_string(),
        LogFieldValue::Boolean(true),
    );
    fields.insert(
        constants::field::PROCESS_ID.to_string(),
        LogFieldValue::Number(f64::from(u32::MAX)),
    );
    fields
}

fn paths(root: &std::path::Path) -> EnforcementJournalPaths {
    let nested = root.join("nested");
    EnforcementJournalPaths {
        journal_path: nested.join("activity.journal"),
        key_path: nested.join("activity.key"),
        store_path: nested.join("activity.db"),
        timer_state_path: EnforcementTimerStatePath(nested.join("timer-state.json")),
    }
}

fn eventing_path(paths: &EnforcementJournalPaths) -> std::path::PathBuf {
    let mut path = paths.journal_path.clone();
    path.set_extension(constants::enforcement::EVENTING_JOURNAL_EXTENSION);
    path
}

fn cleanup(
    paths: &EnforcementJournalPaths,
    eventing_path: &std::path::Path,
    root: &std::path::Path,
) {
    let _ = remove_file(&paths.journal_path);
    let _ = remove_file(&paths.key_path);
    let _ = remove_file(&paths.store_path);
    let _ = remove_file(&paths.timer_state_path.0);
    let _ = remove_file(eventing_path);
    let _ = remove_file(eventing_path.with_extension(format!(
        "{}.append.lock",
        constants::enforcement::EVENTING_JOURNAL_EXTENSION
    )));
    let _ = remove_file(
        paths
            .store_path
            .with_extension(constants::activity_store::WAL_FILE_EXTENSION),
    );
    let _ = remove_file(
        paths
            .store_path
            .with_extension(constants::activity_store::SHM_FILE_EXTENSION),
    );
    let _ = remove_dir(root.join("nested"));
    let _ = remove_dir(root);
}
