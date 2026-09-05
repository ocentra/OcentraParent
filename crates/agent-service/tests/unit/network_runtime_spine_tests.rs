use std::{
    fs::{create_dir_all, read_to_string, remove_file},
    path::{Path, PathBuf},
    time::Duration,
};

use ocentra_eventing::{error::EventingError, ids::EventId};
use ocentra_parent_agent_core::{
    activity_store::ActivityStore, network_capture::NetworkObservation,
    network_capture_event::network_observation_event,
    network_event_runtime::network_runtime_event_ids_for_source_event,
};
use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};
use ocentra_parent_agent_protocol::constants;

use crate::network_runtime_test_support::{
    lock_activity_report_env_for_test, network_runtime_journal_path_for_test,
};
use crate::test_text::{test_ok, TestResult, TestText};

#[tokio::test]
async fn network_runtime_spine_reuses_the_same_journal_path() -> TestResult {
    let path = test_ok(
        network_runtime_journal_path_for_test(),
        "network runtime test journal path must initialize",
    )?;

    test_ok(
        crate::network_runtime_delivery::initialize_network_runtime_spine(&path).await,
        "first durable spine initialization must succeed",
    )?;
    test_ok(
        crate::network_runtime_delivery::initialize_network_runtime_spine(&path).await,
        "same journal path must be idempotently reusable",
    )?;

    Ok(())
}

#[tokio::test]
async fn network_runtime_spine_rejects_a_different_journal_path() -> TestResult {
    let path = test_ok(
        network_runtime_journal_path_for_test(),
        "network runtime test journal path must initialize",
    )?;
    test_ok(
        crate::network_runtime_delivery::initialize_network_runtime_spine(&path).await,
        "durable spine must initialize before mismatch check",
    )?;

    let mut different_path = path.as_path().to_path_buf();
    different_path.set_file_name("different-network-runtime-journal.ndjson");
    let error = test_err(
        crate::network_runtime_delivery::initialize_network_runtime_spine(
            &ocentra_parent_agent_core::network_event_runtime::NetworkRuntimeJournalPath::new(
                different_path,
            ),
        )
        .await,
        "a different journal path must not reuse the process-global spine",
    )?;

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: constants::network_flow::NETWORK_RUNTIME_SPINE_FIELD,
            value: constants::network_flow::NETWORK_RUNTIME_SPINE_JOURNAL_PATH_MISMATCH.to_string(),
        }
    );

    Ok(())
}

#[tokio::test]
async fn network_runtime_composes_persisted_capture_with_exact_runtime_observation() -> TestResult {
    let _guard = lock_activity_report_env_for_test().await;
    let suffix = format!("{}-{}", std::process::id(), "wp09-composition");
    let (journal_path, key_path, store_path) = network_runtime_service_artifact_paths(&suffix)?;
    remove_network_runtime_service_artifacts(&journal_path, &key_path, &store_path);

    let observed_at = format!("2026-08-16T20:21:{:02}.000Z", std::process::id() % 60);
    let observation = persisted_capture_observation();
    let activity_event = network_observation_event(observation.clone(), &observed_at, 77);
    let source_event_id = test_ok(
        EventId::parse(activity_event.event_id.clone()),
        "persisted capture event ID must be a valid domain EventId",
    )?;
    let activity_evidence_id = activity_event.evidence[0].evidence_id.clone();

    test_ok(
        crate::activity_capture::record_activity_events_to_paths(
            &journal_path,
            &key_path,
            &store_path,
            std::slice::from_ref(&activity_event),
        ),
        "activity event must persist through the real capture boundary",
    )?;
    let store = test_ok(
        ActivityStore::open(&store_path),
        "activity SQLite store must open",
    )?;
    let read_model = test_ok(
        store.network_flow_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            &observed_at,
        ),
        "persisted network event must produce a read model",
    )?;
    assert_eq!(read_model.rows.len(), 1);
    assert_eq!(read_model.rows[0].event_id, activity_event.event_id);
    assert_eq!(read_model.rows[0].associated_pid_count, Some(3));
    assert!(read_model.rows[0]
        .evidence
        .iter()
        .any(|reference| reference.evidence_id == activity_evidence_id));
    drop(store);

    let runtime_path = test_ok(
        network_runtime_journal_path_for_test(),
        "network runtime test journal path must initialize",
    )?;
    test_ok(
        crate::network_runtime_delivery::initialize_network_runtime_spine(&runtime_path).await,
        "durable runtime spine must initialize before publication",
    )?;
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);
    let before_publish = journal_envelope_row_count(runtime_path.as_path());
    let before_publish_lines = journal_line_count(runtime_path.as_path());
    test_ok(
        crate::network_runtime_delivery::reconcile_retained_network_runtime().await,
        "restart reconciliation must preserve the persisted PID count",
    )?;
    let after_reconciliation = journal_envelope_row_count(runtime_path.as_path());
    assert_eq!(after_reconciliation - before_publish, 3);
    let captured = crate::activity_capture_network_observation::NetworkCaptureObservation {
        source_event_id: activity_event.event_id.clone(),
        observed_at: observed_at.clone(),
        observation: observation.clone(),
    };
    test_ok(
        crate::network_runtime_delivery::publish_captured_network_observations(
            std::slice::from_ref(&captured),
        )
        .await,
        "exact persisted capture observation must publish to the durable runtime",
    )?;
    let after_publish = journal_envelope_row_count(runtime_path.as_path());
    assert_eq!(after_publish, after_reconciliation);
    let after_publish_lines = journal_line_count(runtime_path.as_path());
    assert_eq!(after_publish_lines - before_publish_lines, 9);

    assert_durable_runtime_projection(
        &read_model,
        &source_event_id,
        &observation,
        &activity_event.event_id,
    )
    .await?;

    test_ok(
        crate::network_runtime_delivery::publish_captured_network_observations(&[captured]).await,
        "retrying the exact capture must remain idempotent",
    )?;
    let after_retry = journal_envelope_row_count(runtime_path.as_path());
    assert_eq!(after_retry, after_publish);
    assert_eq!(
        journal_line_count(runtime_path.as_path()),
        after_publish_lines
    );

    remove_network_runtime_service_artifacts(&journal_path, &key_path, &store_path);
    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);

    Ok(())
}

#[tokio::test]
async fn startup_reconciliation_rejects_malformed_persisted_row_without_journal_mutation(
) -> TestResult {
    let _guard = lock_activity_report_env_for_test().await;
    let suffix = format!("{}-{}", std::process::id(), "wp09-reconciliation");
    let (journal_path, key_path, store_path) = network_runtime_service_artifact_paths(&suffix)?;
    remove_network_runtime_service_artifacts(&journal_path, &key_path, &store_path);

    let observed_at = format!("2026-08-16T20:22:{:02}.000Z", std::process::id() % 60);
    let observation = NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        destination_ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string()),
        destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
        tcp_state: Some(ActivityNetworkTcpState::Established),
        pid: Some(4242),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        associated_pid_count: 1,
    };
    let activity_event = network_observation_event(observation, &observed_at, 78);
    test_ok(
        crate::activity_capture::record_activity_events_to_paths(
            &journal_path,
            &key_path,
            &store_path,
            std::slice::from_ref(&activity_event),
        ),
        "valid activity event must persist before corruption is introduced",
    )?;

    let store = test_ok(
        ActivityStore::open(&store_path),
        "activity SQLite store must open",
    )?;
    let updated = test_ok(
        store.connection_for_test().execute(
            "UPDATE activity_events SET observed_at = ?1 WHERE event_id = ?2",
            ["not-rfc3339", activity_event.event_id.as_str()],
        ),
        "persisted canonical timestamp must be mutated for the restart regression",
    )?;
    assert_eq!(updated, 1);
    drop(store);

    let runtime_path = test_ok(
        network_runtime_journal_path_for_test(),
        "network runtime test journal path must initialize",
    )?;
    test_ok(
        crate::network_runtime_delivery::initialize_network_runtime_spine(&runtime_path).await,
        "durable runtime spine must initialize before restart reconciliation",
    )?;
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);
    let before_lines = journal_line_count(runtime_path.as_path());
    let before_envelopes = journal_envelope_row_count(runtime_path.as_path());

    let error = test_err(
        crate::network_runtime_delivery::reconcile_retained_network_runtime().await,
        "malformed persisted timestamp must fail startup reconciliation",
    )?;
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: constants::field::OBSERVED_AT,
            value: "not-rfc3339: premature end of input".to_string(),
        }
    );
    assert_eq!(journal_line_count(runtime_path.as_path()), before_lines);
    assert_eq!(
        journal_envelope_row_count(runtime_path.as_path()),
        before_envelopes
    );

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    remove_network_runtime_service_artifacts(&journal_path, &key_path, &store_path);

    Ok(())
}

#[tokio::test]
async fn startup_reconciliation_bounds_missing_rows_and_skips_complete_chains() -> TestResult {
    let _guard = lock_activity_report_env_for_test().await;
    let suffix = format!("{}-{}", std::process::id(), "wp09-bounded-reconciliation");
    let (journal_path, key_path, store_path) = network_runtime_service_artifact_paths(&suffix)?;
    remove_network_runtime_service_artifacts(&journal_path, &key_path, &store_path);

    let observed_at = format!("2026-08-16T20:23:{:02}.000Z", std::process::id() % 60);
    let activity_events = (0..40)
        .map(|index| {
            network_observation_event(
                persisted_capture_observation(),
                &observed_at,
                10_000 + index,
            )
        })
        .collect::<Vec<_>>();
    test_ok(
        crate::activity_capture::record_activity_events_to_paths(
            &journal_path,
            &key_path,
            &store_path,
            &activity_events,
        ),
        "retained network observations must persist before reconciliation",
    )?;

    let runtime_path = test_ok(
        network_runtime_journal_path_for_test(),
        "network runtime test journal path must initialize",
    )?;
    test_ok(
        crate::network_runtime_delivery::initialize_network_runtime_spine(&runtime_path).await,
        "durable runtime spine must initialize before bounded reconciliation",
    )?;
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);
    let initial_envelopes = journal_envelope_row_count(runtime_path.as_path());

    run_reconciliation_with_timeout().await?;
    let first_batch_envelopes = journal_envelope_row_count(runtime_path.as_path());
    assert_eq!(first_batch_envelopes - initial_envelopes, 32 * 3);

    run_reconciliation_with_timeout().await?;
    let complete_envelopes = journal_envelope_row_count(runtime_path.as_path());
    assert_eq!(complete_envelopes - first_batch_envelopes, 8 * 3);
    let complete_lines = journal_line_count(runtime_path.as_path());

    run_reconciliation_with_timeout().await?;
    assert_eq!(
        journal_envelope_row_count(runtime_path.as_path()),
        complete_envelopes
    );
    assert_eq!(journal_line_count(runtime_path.as_path()), complete_lines);

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    remove_network_runtime_service_artifacts(&journal_path, &key_path, &store_path);

    Ok(())
}

async fn run_reconciliation_with_timeout() -> TestResult {
    let reconciliation = test_ok(
        tokio::time::timeout(
            Duration::from_secs(10),
            crate::network_runtime_delivery::reconcile_retained_network_runtime(),
        )
        .await,
        "retained network reconciliation must finish within its startup budget",
    )?;
    test_ok(
        reconciliation,
        "retained network reconciliation must publish its bounded batch",
    )
}

fn network_runtime_service_artifact_paths(
    suffix: &str,
) -> Result<(PathBuf, PathBuf, PathBuf), TestText> {
    let artifact_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join("test-artifacts")
        .join("network-runtime-service");
    test_ok(
        create_dir_all(&artifact_dir),
        "network runtime service artifact directory must exist",
    )?;
    let stem = artifact_dir.join(format!("ocentra-{suffix}"));
    Ok((
        stem.with_extension("ndjson"),
        stem.with_extension("key"),
        stem.with_extension("db"),
    ))
}

fn persisted_capture_observation() -> NetworkObservation {
    NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        destination_ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string()),
        destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
        tcp_state: Some(ActivityNetworkTcpState::Established),
        pid: Some(4242),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        associated_pid_count: 3,
    }
}

fn remove_network_runtime_service_artifacts(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
) {
    for path in [journal_path, key_path, store_path] {
        let _ = remove_file(path);
    }

    let mut wal_path = store_path.to_path_buf();
    wal_path.set_extension("db-wal");
    let _ = remove_file(wal_path);

    let mut shm_path = store_path.to_path_buf();
    shm_path.set_extension("db-shm");
    let _ = remove_file(shm_path);

    let mut append_lock_path = journal_path.to_path_buf();
    if let Some(file_name) = journal_path.file_name() {
        let mut lock_name = file_name.to_os_string();
        lock_name.push(".append.lock");
        append_lock_path.set_file_name(lock_name);
        let _ = remove_file(append_lock_path);
    }
}

async fn assert_durable_runtime_projection(
    read_model: &ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel,
    source_event_id: &EventId,
    observation: &NetworkObservation,
    source_evidence_ref: &str,
) -> TestResult {
    let report = test_ok(
        crate::network_runtime_delivery::durable_network_runtime_projection(read_model).await,
        "durable runtime projection must replay",
    )?;
    let expected_ids = test_ok(
        network_runtime_event_ids_for_source_event(source_event_id, observation),
        "expected runtime IDs must derive from the valid source ID",
    )?;
    let actual_ids = report
        .stored_events
        .iter()
        .map(|event| event.event_id.clone())
        .collect::<Vec<_>>();
    assert_eq!(actual_ids, expected_ids);
    assert_eq!(
        report
            .stored_events
            .iter()
            .map(|event| event.contract.event_type.as_str())
            .collect::<Vec<_>>(),
        vec![
            constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED,
            constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED,
            constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED,
        ]
    );
    for event in &report.stored_events {
        let payload = test_ok(
            event.decode::<ocentra_parent_agent_core::network_event_runtime::NetworkRuntimeEventPayload>(),
            "owned runtime payload must decode",
        )?;
        assert_eq!(payload.payload().evidence_ref, source_evidence_ref);
        assert_eq!(
            payload.payload().associated_pid_count,
            observation.associated_pid_count
        );
        assert_ne!(
            event.contract.event_type.as_str(),
            constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
        );
    }

    Ok(())
}

fn test_err<T, E>(result: Result<T, E>, context: &str) -> Result<E, TestText> {
    match result {
        Ok(_) => Err(TestText::from_display(format!(
            "{context}: operation unexpectedly succeeded"
        ))),
        Err(error) => Ok(error),
    }
}

fn journal_envelope_row_count(path: &Path) -> usize {
    read_to_string(path)
        .map(|text| {
            text.lines()
                .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
                .filter(|record| record.get("envelope").is_some())
                .count()
        })
        .unwrap_or_default()
}

fn journal_line_count(path: &Path) -> usize {
    read_to_string(path)
        .map(|text| text.lines().filter(|line| !line.trim().is_empty()).count())
        .unwrap_or_default()
}
