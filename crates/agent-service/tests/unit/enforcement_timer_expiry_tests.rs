use std::fs::{read_to_string, remove_file};

use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_core::activity_store_app_game::app_game_journal_sqlite_ingest::app_game_runtime_journal_event;
use ocentra_parent_agent_core::process_capture::{process_observation_event, ProcessObservation};
use ocentra_parent_agent_core::window_capture::ForegroundWindowObservation;
use ocentra_parent_agent_core::window_capture_event::foreground_window_observation_event;
use ocentra_parent_agent_protocol::app_game::{
    AppGameRuntimeEvidenceRow, APP_GAME_CAPABILITY_STATUS_AVAILABLE, APP_GAME_CATALOG_READY,
    APP_GAME_CLASSIFICATION_KNOWN_APP, APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_OBSERVATION_MODE_PROCESS_START,
    APP_GAME_RUNTIME_RUNNING, APP_GAME_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::transport::AgentMessageTarget;
use ocentra_parent_agent_protocol::transport::AgentPeer;
use ocentra_parent_agent_protocol::transport::AgentPeerRole;
use ocentra_parent_agent_protocol::transport::AgentRoute;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

#[cfg(windows)]
use super::test_text::optional_log_string;
use super::test_text::{test_ok, test_some, TestResult, TestText};
use crate::{
    enforcement_api::{build_enforcement_audit_report_with_paths, EnforcementJournalPaths},
    enforcement_timer_api::build_enforcement_timer_report_with_paths,
};

#[tokio::test]
async fn timer_expiry_uses_persisted_time_limit_state_and_clears_it() -> TestResult {
    let paths = temp_paths(constants::enforcement::TIMER_EXPIRED);
    cleanup_paths(&paths);

    let execute_event =
        build_enforcement_audit_report_with_paths(time_limit_execute_command(), paths.clone())
            .await;
    assert_eq!(
        execute_event.event,
        AgentEventName::AgentEnforcementAuditReported
    );

    #[cfg(windows)]
    Box::pin(assert_timer_expiry_uses_persisted_state_and_clears_it(
        &paths,
    ))
    .await?;

    #[cfg(not(windows))]
    assert_timer_expiry_without_supported_adapter_reports_missing_state(&paths, &execute_event)
        .await?;

    cleanup_paths(&paths);

    Ok(())
}

#[tokio::test]
async fn app_game_timer_session_evidence_requires_matching_persisted_runtime_and_session(
) -> TestResult {
    let paths = temp_paths("app-game-session-evidence");
    cleanup_paths(&paths);
    let store = test_ok(
        ActivityStore::open(&paths.store_path),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    let runtime = app_game_timer_session_runtime_row();
    let events = app_game_timer_session_events(&runtime)?;
    test_ok(
        store.ingest_events(&events),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;
    drop(store);

    let binding = test_ok(
        crate::app_game_dispatch_evidence::validate_app_game_dispatch_evidence(
            &app_game_timer_session_payload(),
            crate::app_game_dispatch_evidence::AppGameDispatchStorePath(paths.store_path.clone()),
        )
        .await,
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;
    let store = test_ok(
        ActivityStore::open(&paths.store_path),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    let summary = test_some(
        test_ok(
            store.app_game_session_summaries(constants::activity_store::DEFAULT_RECENT_LIMIT),
            constants::error::ACTIVITY_STORE_QUERIES,
        )?
        .into_iter()
        .find(|summary| summary.primary_process_identity == runtime.process_identity),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;
    assert_eq!(binding.session_id, summary.session_id);
    assert_eq!(binding.runtime_evidence_id, runtime.runtime_evidence_id);
    assert_eq!(binding.process_identity, runtime.process_identity);
    assert_eq!(binding.process_id, runtime.process_id);
    assert_eq!(binding.process_name, runtime.process_name);
    assert_eq!(binding.classification_state, runtime.classification_state);
    assert_eq!(binding.last_observed_at, summary.last_observed_at);
    assert_eq!(binding.running_duration_ms, summary.running_duration_ms);
    assert_eq!(
        binding.foreground_duration_ms,
        summary.foreground_duration_ms
    );
    drop(store);
    test_ok(
        crate::app_game_dispatch_evidence::validate_app_game_timer_session(
            &binding,
            crate::app_game_dispatch_evidence::AppGameDispatchStorePath(paths.store_path.clone()),
        )
        .await,
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;

    let mut wrong_process = binding;
    wrong_process.process_id = wrong_process.process_id.saturating_add(1);
    assert_eq!(
        crate::app_game_dispatch_evidence::validate_app_game_timer_session(
            &wrong_process,
            crate::app_game_dispatch_evidence::AppGameDispatchStorePath(paths.store_path.clone()),
        )
        .await,
        Err(crate::app_game_dispatch_evidence::AppGameDispatchEvidenceRejection::Mismatch)
    );
    cleanup_paths(&paths);
    Ok(())
}

#[tokio::test]
async fn app_game_timer_session_evidence_rejects_unknown_runtime_identity() -> TestResult {
    let paths = temp_paths("app-game-unknown-runtime-identity");
    cleanup_paths(&paths);
    let store = test_ok(
        ActivityStore::open(&paths.store_path),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;
    let mut runtime = app_game_timer_session_runtime_row();
    runtime.classification_state = APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string();
    let events = app_game_timer_session_events(&runtime)?;
    test_ok(
        store.ingest_events(&events),
        constants::error::ACTIVITY_STORE_INGESTS,
    )?;
    drop(store);

    assert_eq!(
        crate::app_game_dispatch_evidence::validate_app_game_dispatch_evidence(
            &app_game_timer_session_payload(),
            crate::app_game_dispatch_evidence::AppGameDispatchStorePath(paths.store_path.clone()),
        )
        .await,
        Err(crate::app_game_dispatch_evidence::AppGameDispatchEvidenceRejection::Mismatch)
    );
    cleanup_paths(&paths);
    Ok(())
}

#[cfg(windows)]
async fn assert_timer_expiry_uses_persisted_state_and_clears_it(
    paths: &EnforcementJournalPaths,
) -> TestResult {
    let stored_state = read_state(paths)?;
    let expire_event =
        build_enforcement_timer_report_with_paths(expire_command(), paths.clone()).await;
    let state_after_expiry = read_to_string(&paths.timer_state_path);

    assert_eq!(
        stored_state.timer_event.timer_event_kind.as_protocol_str(),
        constants::enforcement::TIMER_CREATED
    );
    assert_eq!(
        expire_event.event,
        AgentEventName::AgentEnforcementTimerReported
    );
    assert!(matches!(
        state_after_expiry,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound
    ));
    assert_platform_expiry_payload(&expire_event.payload);

    let audit = test_ok(
        serde_json::from_str::<ocentra_parent_agent_protocol::enforcement::EnforcementAuditEvent>(
            test_some(
                optional_log_string(
                    &expire_event.payload,
                    constants::field::ENFORCEMENT_AUDIT_EVENT,
                ),
                constants::error::AGENT_EVENT_SERIALIZES,
            )?
            .as_ref(),
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    assert_eq!(
        audit.audit_event_kind.as_protocol_str(),
        expire_audit_kind().to_string()
    );

    Ok(())
}

#[cfg(not(windows))]
async fn assert_timer_expiry_without_supported_adapter_reports_missing_state(
    paths: &EnforcementJournalPaths,
    execute_event: &ocentra_parent_agent_protocol::transport::AgentEventEnvelope,
) -> TestResult {
    assert_missing_state_file(paths);
    assert_eq!(
        execute_event
            .payload
            .get(constants::field::ENFORCEMENT_STATUS),
        Some(&LogFieldValue::String(
            constants::enforcement::RESULT_UNAVAILABLE.to_string()
        ))
    );
    assert_eq!(
        execute_event
            .payload
            .get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND),
        Some(&LogFieldValue::String(
            constants::enforcement::TIMER_UNAVAILABLE.to_string()
        ))
    );
    assert_eq!(
        execute_event
            .payload
            .get(constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE),
        Some(&LogFieldValue::String(
            constants::enforcement::ADAPTER_UNSUPPORTED_PLATFORM.to_string()
        ))
    );

    let expire_event =
        build_enforcement_timer_report_with_paths(expire_command(), paths.clone()).await;

    assert_missing_state_file(paths);
    assert_eq!(
        expire_event.event,
        AgentEventName::AgentEnforcementTimerReported
    );
    assert_eq!(
        expire_event.payload.get(constants::field::AVAILABLE),
        Some(&LogFieldValue::Boolean(false))
    );
    assert_eq!(
        expire_event.payload.get(constants::field::REASON),
        Some(&LogFieldValue::String(
            constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED.to_string()
        ))
    );
    assert_eq!(
        expire_event
            .payload
            .get(constants::field::ENFORCEMENT_STATUS),
        Some(&LogFieldValue::String(
            constants::enforcement::RESULT_UNAVAILABLE.to_string()
        ))
    );
    assert_eq!(
        expire_event
            .payload
            .get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND),
        Some(&LogFieldValue::String(
            constants::enforcement::TIMER_RECOVERY_NEEDED.to_string()
        ))
    );

    Ok(())
}

#[cfg(windows)]
fn assert_platform_expiry_payload(payload: &LogFields) {
    assert_eq!(
        payload.get(constants::field::ENFORCEMENT_STATUS),
        Some(&LogFieldValue::String(
            constants::enforcement::RESULT_EXPIRED.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND),
        Some(&LogFieldValue::String(
            constants::enforcement::TIMER_EXPIRED.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE),
        Some(&LogFieldValue::String(
            constants::enforcement::ADAPTER_PROCESS_ALREADY_EXITED.to_string()
        ))
    );
}

fn time_limit_execute_command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::enforcement::TEST_ACTION_ID.to_string(),
        sent_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: portal_peer(),
        target: target(),
        command: AgentCommandName::AgentEnforcementExecute,
        payload: time_limit_execute_payload(),
    }
}

fn expire_command() -> AgentCommandEnvelope {
    let mut payload = timer_payload();
    payload.insert(
        constants::field::PROCESS_ID.to_string(),
        LogFieldValue::Number(f64::from(u32::MAX)),
    );

    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::enforcement::TEST_TIMER_EVENT_ID.to_string(),
        sent_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: portal_peer(),
        target: target(),
        command: AgentCommandName::AgentEnforcementTimerExpire,
        payload,
    }
}

fn time_limit_execute_payload() -> LogFields {
    let mut fields = timer_payload();
    fields.insert(
        constants::field::ENFORCEMENT_INTENT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_INTENT_ID.to_string()),
    );
    fields.insert(
        constants::field::POLICY_ACTION.to_string(),
        LogFieldValue::String(policy_constants::ACTION_TIME_LIMIT.to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(policy_constants::TARGET_TYPE_APP.to_string()),
    );
    fields.insert(
        constants::field::TARGET_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_PROCESS_TARGET_ID.to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_PROCESS_TARGET_VALUE.to_string()),
    );
    fields.insert(
        constants::field::POLICY_DRY_RUN.to_string(),
        LogFieldValue::Boolean(false),
    );
    fields.insert(
        constants::field::POLICY_REASON_CODES.to_string(),
        LogFieldValue::String(policy_constants::TEST_REASON_PARENT_BLOCK.to_string()),
    );
    fields.insert(
        constants::field::POLICY_RULE_IDS.to_string(),
        LogFieldValue::String(policy_constants::TEST_BLOCK_RULE_ID.to_string()),
    );
    fields.insert(
        constants::field::EVIDENCE_REFERENCE_IDS.to_string(),
        LogFieldValue::String(policy_constants::TEST_EVIDENCE_ID.to_string()),
    );
    fields.insert(
        constants::field::EXPIRES_AT.to_string(),
        LogFieldValue::String(policy_constants::TEST_EXPIRES_AT.to_string()),
    );
    fields
}

fn timer_payload() -> LogFields {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::POLICY_DECISION_ID.to_string(),
        LogFieldValue::String(policy_constants::TEST_DECISION_ID.to_string()),
    );
    fields.insert(
        constants::field::POLICY_VERSION.to_string(),
        LogFieldValue::String(policy_constants::TEST_POLICY_VERSION.to_string()),
    );
    fields.insert(
        constants::field::REQUESTED_AT.to_string(),
        LogFieldValue::String(policy_constants::TEST_EVALUATED_AT.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_ACTION_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_ACTION_ID.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_RESULT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_RESULT_ID.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_TIMER_EVENT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_TIMER_EVENT_ID.to_string()),
    );
    fields
}

fn app_game_timer_session_payload() -> LogFields {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::APP_GAME_RUNTIME_EVIDENCE_ID.to_string(),
        LogFieldValue::String("runtime-evidence-process-4242".to_string()),
    );
    fields.insert(
        constants::field::PROCESS_ID.to_string(),
        LogFieldValue::Number(4242.0),
    );
    fields.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String("ocentra-session-fixture.exe".to_string()),
    );
    fields.insert(
        constants::field::EVIDENCE_REFERENCE_IDS.to_string(),
        LogFieldValue::String("runtime-evidence-process-4242".to_string()),
    );
    fields
}

fn app_game_timer_session_events(
    runtime: &AppGameRuntimeEvidenceRow,
) -> Result<Vec<ocentra_parent_agent_protocol::activity::ActivityEvent>, TestText> {
    let observed_at = "2026-06-03T22:14:00Z";
    Ok(vec![
        process_observation_event(
            ProcessObservation {
                pid: 4242,
                name: runtime.process_name.clone(),
                executable_path: Some(std::path::PathBuf::from(
                    "C:/fixture/ocentra-session-fixture.exe",
                )),
            },
            observed_at,
            0,
        ),
        foreground_window_observation_event(
            ForegroundWindowObservation::active(
                4242,
                runtime.process_name.clone(),
                "C:/fixture/ocentra-session-fixture.exe".to_string(),
                "Ocentra session fixture".to_string(),
                "fixture-window".to_string(),
            ),
            observed_at,
        ),
        test_ok(
            app_game_runtime_journal_event(
                constants::peer::LOCAL_DEV_AGENT,
                constants::enforcement::PLATFORM_WINDOWS,
                runtime,
            ),
            constants::error::ACTIVITY_STORE_INGESTS,
        )?,
    ])
}

fn app_game_timer_session_runtime_row() -> AppGameRuntimeEvidenceRow {
    AppGameRuntimeEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        runtime_evidence_id: "runtime-evidence-process-4242".to_string(),
        observed_at: "2026-06-03T22:15:00Z".to_string(),
        process_identity: "process-4242".to_string(),
        process_id: 4242,
        parent_process_id: Some(1000),
        process_name: "ocentra-session-fixture.exe".to_string(),
        executable_path_ref: Some("path-ref-ocentra-session-fixture".to_string()),
        publisher_signature_ref: None,
        file_hash_ref: None,
        inventory_entry_id: None,
        launcher_ref: None,
        catalog_ref: Some("catalog-ref-ocentra-session-fixture".to_string()),
        started_at: Some("2026-06-03T22:14:00Z".to_string()),
        exited_at: None,
        running_duration_ms: 60000,
        runtime_state: APP_GAME_RUNTIME_RUNNING.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_PROCESS_START.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_APP.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.82,
        evidence: Vec::new(),
    }
}

fn portal_peer() -> AgentPeer {
    AgentPeer {
        peer_id: constants::peer::PORTAL_DEV.to_string(),
        role: AgentPeerRole::Portal,
    }
}

fn target() -> AgentMessageTarget {
    AgentMessageTarget {
        device_id: constants::enforcement::TEST_CHILD_DEVICE_ID.to_string(),
        platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
        route: AgentRoute::Localhost,
    }
}

#[cfg(windows)]
fn read_state(
    paths: &EnforcementJournalPaths,
) -> Result<ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState, TestText> {
    let text = test_ok(
        read_to_string(&paths.timer_state_path),
        constants::error::JOURNAL_READS,
    )?;
    test_ok(
        serde_json::from_str(&text),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}

#[cfg(not(windows))]
fn assert_missing_state_file(paths: &EnforcementJournalPaths) {
    let error_kind = read_to_string(&paths.timer_state_path)
        .as_ref()
        .err()
        .map(std::io::Error::kind);
    assert_eq!(
        error_kind,
        Some(std::io::ErrorKind::NotFound),
        "{}",
        constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED
    );
}

#[cfg(windows)]
fn expire_audit_kind() -> TestText {
    TestText::from_display(constants::enforcement::AUDIT_EXPIRED)
}

fn temp_paths(suffix: impl std::fmt::Display) -> EnforcementJournalPaths {
    let suffix = suffix.to_string();
    let build_path = |role, extension| {
        let mut name = String::from(constants::journal::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(&suffix);
        name.push(constants::delimiter::HYPHEN);
        name.push_str(role);
        let mut path = std::env::temp_dir();
        path.push(name);
        path.set_extension(extension);
        path
    };
    EnforcementJournalPaths {
        journal_path: build_path(
            constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX,
            constants::journal::FILE_EXTENSION,
        ),
        key_path: build_path(
            constants::activity_store::TEST_CAPTURE_KEY_SUFFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
        store_path: build_path(
            constants::activity_store::TEST_STORE_SUFFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
        timer_state_path: crate::enforcement_timer_state_path::EnforcementTimerStatePath(
            build_path(
                constants::enforcement::TIMER_STATE_ID_PREFIX,
                constants::activity_store::FILE_EXTENSION,
            ),
        ),
    }
}

fn cleanup_paths(paths: &EnforcementJournalPaths) {
    let _ = remove_file(&paths.journal_path);
    let _ = remove_file(&paths.key_path);
    let _ = remove_file(&paths.store_path);
    let _ = remove_file(&paths.timer_state_path);
    let mut wal_path = paths.store_path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = paths.store_path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
