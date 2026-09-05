use std::{
    env,
    fs::{read_to_string, remove_file},
    path::PathBuf,
};

use ocentra_eventing::{
    expect_value::ExpectValue,
    ids::EventId,
    journal::{ndjson::NdjsonEventJournal, ndjson::NdjsonJournalOptions},
    replay::ReplayFilter,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState;
use ocentra_parent_agent_protocol::enforcement::EnforcementAuditEvent;
use ocentra_parent_agent_protocol::enforcement::EnforcementAuditJournalEvent;
use ocentra_parent_agent_protocol::enforcement::EnforcementTimerEvent;
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

use super::test_text::{optional_log_string, test_ok, test_some, TestResult, TestText};
use crate::{
    enforcement_api::{build_enforcement_audit_report_with_paths, EnforcementJournalPaths},
    enforcement_payload::EnforcementPayloadError,
    enforcement_timer_api::{
        build_enforcement_timer_report, build_enforcement_timer_report_with_paths,
    },
};

#[test]
fn timer_state_path_uses_configured_or_default_location() {
    let path = crate::enforcement_timer_state_path::enforcement_timer_state_path();
    let expected = env::var(constants::env_var::AGENT_ENFORCEMENT_TIMER_STATE_PATH)
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut default_path = env::temp_dir();
            default_path.push(constants::enforcement::TIMER_STATE_FILE_NAME);
            default_path
        });
    assert_eq!(path.as_ref(), expected.as_path());
}

#[test]
fn unsupported_capability_rejection_is_protocol_stable() {
    assert_eq!(
        EnforcementPayloadError::UnsupportedCapability.to_string(),
        constants::enforcement::REJECTION_UNSUPPORTED_CAPABILITY
    );
}

#[tokio::test]
async fn timer_default_report_rejects_non_timer_command_before_state_access() {
    let event = build_enforcement_timer_report(execute_command()).await;

    assert_eq!(event.event, AgentEventName::AgentCommandRejected);
    assert_eq!(event.target.peer_id, constants::peer::PORTAL_DEV);
    assert_eq!(
        event.payload.get(constants::field::REASON),
        Some(&LogFieldValue::String(
            constants::enforcement::REJECTION_COMMAND_PAYLOAD_INVALID.to_string()
        ))
    );
}

#[tokio::test]
async fn timer_recovery_and_parent_cancel_use_persisted_active_state() -> TestResult {
    let paths = temp_paths(constants::enforcement::TEST_TIMER_STATE_ID);
    cleanup_paths(&paths);

    let execute_event =
        build_enforcement_audit_report_with_paths(execute_command(), paths.clone()).await;
    let stored_state = read_state(&paths)?;
    let recovered_event =
        build_enforcement_timer_report_with_paths(recover_command(), paths.clone()).await;
    let recovered_state = read_state(&paths)?;
    let recovered_again_event = build_enforcement_timer_report_with_paths(
        recover_command_with_suffix("-again"),
        paths.clone(),
    )
    .await;
    let recovered_again_state = read_state(&paths)?;
    let cancel_event =
        build_enforcement_timer_report_with_paths(cancel_command(), paths.clone()).await;
    let state_after_cancel = read_to_string(&paths.timer_state_path);
    cleanup_paths(&paths);

    assert_eq!(
        execute_event.event,
        AgentEventName::AgentEnforcementAuditReported
    );
    assert_eq!(
        stored_state.timer_event.timer_event_kind.as_protocol_str(),
        constants::enforcement::TIMER_CREATED
    );
    assert_eq!(
        recovered_event.event,
        AgentEventName::AgentEnforcementTimerReported
    );
    assert_eq!(
        recovered_event
            .payload
            .get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND),
        Some(&LogFieldValue::String(
            constants::enforcement::TIMER_RESTART_RECOVERED.to_string()
        ))
    );
    assert_eq!(
        recovered_state.action.action_id,
        constants::enforcement::TEST_ACTION_ID
    );
    assert_eq!(
        recovered_again_event.event,
        AgentEventName::AgentEnforcementTimerReported
    );
    assert_eq!(
        recovered_again_event
            .payload
            .get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND),
        Some(&LogFieldValue::String(
            constants::enforcement::TIMER_RESTART_RECOVERED.to_string()
        ))
    );
    assert_eq!(
        recovered_again_state.action.action_id,
        constants::enforcement::TEST_ACTION_ID
    );
    assert_eq!(
        cancel_event.event,
        AgentEventName::AgentEnforcementTimerReported
    );
    assert_eq!(
        cancel_event
            .payload
            .get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND),
        Some(&LogFieldValue::String(
            constants::enforcement::TIMER_CANCELLED.to_string()
        ))
    );
    assert!(matches!(
        state_after_cancel,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound
    ));

    let timer = payload_timer_event(&cancel_event.payload)?;
    let recovered_audit = payload_audit_event(&recovered_event.payload)?;
    let recovered_again_audit = payload_audit_event(&recovered_again_event.payload)?;
    let audit = payload_audit_event(&cancel_event.payload)?;
    assert_eq!(timer.action_id, constants::enforcement::TEST_ACTION_ID);
    assert_eq!(
        audit.audit_event_kind.as_protocol_str(),
        constants::enforcement::AUDIT_CANCELLED
    );
    assert_eq!(
        audit
            .parent_override
            .as_ref()
            .map(|reference| reference.action_reference_id.as_str()),
        Some(constants::enforcement::TEST_PARENT_ACTION_REFERENCE_ID)
    );
    assert_eq!(recovered_audit.journal_sequence, Some("3".to_string()));
    assert_eq!(
        recovered_again_audit.journal_sequence,
        Some("4".to_string())
    );
    assert_eq!(audit.journal_sequence, Some("5".to_string()));

    Ok(())
}

#[tokio::test]
async fn timer_eventing_projection_retains_device_source_and_route_context() -> TestResult {
    let paths = temp_paths(format!("{}-eventing", EventId::generated().as_str()));
    cleanup_paths(&paths);
    let execute_event =
        build_enforcement_audit_report_with_paths(execute_command(), paths.clone()).await;
    assert_eq!(
        execute_event.event,
        AgentEventName::AgentEnforcementAuditReported
    );
    let _ = build_enforcement_timer_report_with_paths(recover_command(), paths.clone()).await;
    let _ = build_enforcement_timer_report_with_paths(cancel_command(), paths.clone()).await;

    let eventing_path = timer_eventing_path(&paths);
    let journal =
        NdjsonEventJournal::with_options(eventing_path.clone(), NdjsonJournalOptions::hash_chain());
    let replay = journal
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("replay timer eventing audit projection");
    let events = replay
        .records
        .iter()
        .map(|record| {
            record
                .envelope
                .decode::<EnforcementAuditJournalEvent>()
                .expect_value("decode timer eventing audit projection")
                .into_payload()
        })
        .collect::<Vec<_>>();

    assert_eq!(events.len(), 4);
    assert!(events.iter().all(|event| {
        event.device_id.as_deref() == Some(constants::enforcement::TEST_CHILD_DEVICE_ID)
            && event.source_peer_id.as_deref() == Some(constants::peer::PORTAL_DEV)
            && event.target_route.as_deref() == Some("localhost")
    }));
    assert!(events.iter().any(|event| {
        event.parent_override.as_ref().is_some_and(|reference| {
            reference.action_reference_id == constants::enforcement::TEST_PARENT_ACTION_REFERENCE_ID
        })
    }));

    drop(journal);
    cleanup_paths(&paths);
    Ok(())
}

#[tokio::test]
async fn timer_recovery_reports_unavailable_when_active_state_is_missing() {
    let paths = temp_paths(constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED);
    cleanup_paths(&paths);
    let event = build_enforcement_timer_report_with_paths(recover_command(), paths.clone()).await;
    cleanup_paths(&paths);

    assert_eq!(event.event, AgentEventName::AgentEnforcementTimerReported);
    assert_eq!(
        event.payload.get(constants::field::AVAILABLE),
        Some(&LogFieldValue::Boolean(false))
    );
    assert_eq!(
        event.payload.get(constants::field::REASON),
        Some(&LogFieldValue::String(
            constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND),
        Some(&LogFieldValue::String(
            constants::enforcement::TIMER_RECOVERY_NEEDED.to_string()
        ))
    );
}

#[tokio::test]
async fn timer_recovery_reports_recovery_needed_for_invalid_persisted_state() -> TestResult {
    let paths = temp_paths("invalid-persisted-state");
    cleanup_paths(&paths);

    let execute_event =
        build_enforcement_audit_report_with_paths(execute_command(), paths.clone()).await;
    assert_eq!(
        execute_event.event,
        AgentEventName::AgentEnforcementAuditReported
    );

    let mut state = read_state(&paths)?;
    state.result.action_id.push_str("-other");
    let serialized = test_ok(
        serde_json::to_string(&state),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    test_ok(
        std::fs::write(&paths.timer_state_path, serialized),
        constants::error::JOURNAL_APPENDS,
    )?;

    let inconsistent_event =
        build_enforcement_timer_report_with_paths(recover_command(), paths.clone()).await;
    assert_eq!(
        inconsistent_event.event,
        AgentEventName::AgentEnforcementTimerReported
    );
    assert_eq!(
        inconsistent_event.payload.get(constants::field::AVAILABLE),
        Some(&LogFieldValue::Boolean(false))
    );
    assert_eq!(
        inconsistent_event.payload.get(constants::field::REASON),
        Some(&LogFieldValue::String(
            constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED.to_string()
        ))
    );
    assert_eq!(
        inconsistent_event
            .payload
            .get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND),
        Some(&LogFieldValue::String(
            constants::enforcement::TIMER_RECOVERY_NEEDED.to_string()
        ))
    );

    test_ok(
        std::fs::write(&paths.timer_state_path, "{"),
        constants::error::JOURNAL_APPENDS,
    )?;
    let malformed_event =
        build_enforcement_timer_report_with_paths(recover_command(), paths.clone()).await;
    assert_eq!(
        malformed_event.event,
        AgentEventName::AgentEnforcementTimerReported
    );
    assert_eq!(
        malformed_event.payload.get(constants::field::AVAILABLE),
        Some(&LogFieldValue::Boolean(false))
    );
    assert_eq!(
        malformed_event
            .payload
            .get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND),
        Some(&LogFieldValue::String(
            constants::enforcement::TIMER_RECOVERY_NEEDED.to_string()
        ))
    );

    cleanup_paths(&paths);
    Ok(())
}

#[tokio::test]
async fn timer_recovery_reports_recovery_needed_for_corrupt_persisted_clock() -> TestResult {
    let paths = temp_paths("invalid-persisted-clock");
    cleanup_paths(&paths);

    let execute_event =
        build_enforcement_audit_report_with_paths(execute_command(), paths.clone()).await;
    assert_eq!(
        execute_event.event,
        AgentEventName::AgentEnforcementAuditReported
    );

    let mut state = read_state(&paths)?;
    state.stored_at = "not-a-timestamp".to_string();
    let serialized = test_ok(
        serde_json::to_string(&state),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    test_ok(
        std::fs::write(&paths.timer_state_path, serialized),
        constants::error::JOURNAL_APPENDS,
    )?;

    let recovery_event =
        build_enforcement_timer_report_with_paths(recover_command(), paths.clone()).await;
    cleanup_paths(&paths);

    assert_eq!(
        recovery_event.event,
        AgentEventName::AgentEnforcementTimerReported
    );
    assert_eq!(
        recovery_event.payload.get(constants::field::AVAILABLE),
        Some(&LogFieldValue::Boolean(false))
    );
    assert_eq!(
        recovery_event.payload.get(constants::field::REASON),
        Some(&LogFieldValue::String(
            constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED.to_string()
        ))
    );
    assert_eq!(
        recovery_event
            .payload
            .get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND),
        Some(&LogFieldValue::String(
            constants::enforcement::TIMER_RECOVERY_NEEDED.to_string()
        ))
    );

    Ok(())
}

fn execute_command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::enforcement::TEST_ACTION_ID.to_string(),
        sent_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: portal_peer(),
        target: target(),
        command: AgentCommandName::AgentEnforcementExecute,
        payload: execute_payload(),
    }
}

fn recover_command() -> AgentCommandEnvelope {
    recover_command_with_suffix("")
}

fn recover_command_with_suffix(suffix: &str) -> AgentCommandEnvelope {
    let mut payload = timer_payload().into_inner();
    payload.remove(constants::field::REQUESTED_AT);
    payload.insert(
        constants::field::ENFORCEMENT_RESULT_ID.to_string(),
        LogFieldValue::String(format!(
            "{}{}",
            constants::enforcement::TEST_RESULT_ID,
            suffix
        )),
    );
    payload.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT_ID.to_string(),
        LogFieldValue::String(format!(
            "{}-recover{}",
            constants::enforcement::TEST_AUDIT_EVENT_ID,
            suffix
        )),
    );
    payload.insert(
        constants::field::ENFORCEMENT_TIMER_EVENT_ID.to_string(),
        LogFieldValue::String(format!(
            "{}{}",
            constants::enforcement::TEST_TIMER_EVENT_ID,
            suffix
        )),
    );
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: format!("{}{}", constants::enforcement::TEST_TIMER_EVENT_ID, suffix),
        sent_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: portal_peer(),
        target: target(),
        command: AgentCommandName::AgentEnforcementTimerRecover,
        payload: payload.into(),
    }
}

fn cancel_command() -> AgentCommandEnvelope {
    let mut payload = timer_payload();
    payload.insert(
        constants::field::PARENT_ACTION_REFERENCE_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_PARENT_ACTION_REFERENCE_ID.to_string()),
    );
    payload.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT_ID.to_string(),
        LogFieldValue::String(format!(
            "{}-cancel",
            constants::enforcement::TEST_AUDIT_EVENT_ID
        )),
    );
    payload.insert(
        constants::field::PARENT_ACTOR_ID.to_string(),
        LogFieldValue::String(policy_constants::TEST_PARENT_ACTOR_ID.to_string()),
    );
    payload.insert(
        constants::field::PARENT_ACTOR_ROLE.to_string(),
        LogFieldValue::String(policy_constants::ACTOR_ROLE_PARENT.to_string()),
    );
    payload.insert(
        constants::field::PARENT_ACTION_CREATED_AT.to_string(),
        LogFieldValue::String(policy_constants::TEST_EVALUATED_AT.to_string()),
    );

    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::enforcement::TEST_PARENT_ACTION_REFERENCE_ID.to_string(),
        sent_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: portal_peer(),
        target: target(),
        command: AgentCommandName::AgentEnforcementOverrideCancel,
        payload,
    }
}

fn execute_payload() -> LogFields {
    let mut fields = timer_payload();
    fields.insert(
        constants::field::ENFORCEMENT_INTENT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_INTENT_ID.to_string()),
    );
    fields.insert(
        constants::field::POLICY_ACTION.to_string(),
        LogFieldValue::String(policy_constants::ACTION_ASK_PARENT.to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(policy_constants::TARGET_TYPE_DEVICE.to_string()),
    );
    fields.insert(
        constants::field::TARGET_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_CHILD_DEVICE_ID.to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_CHILD_DEVICE_ID.to_string()),
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

fn read_state(paths: &EnforcementJournalPaths) -> Result<EnforcementActiveTimerState, TestText> {
    let text = test_ok(
        read_to_string(&paths.timer_state_path),
        constants::error::JOURNAL_READS,
    )?;
    test_ok(
        serde_json::from_str(&text),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}

fn payload_timer_event(payload: &LogFields) -> Result<EnforcementTimerEvent, TestText> {
    let text = test_some(
        optional_log_string(payload, constants::field::ENFORCEMENT_TIMER_EVENT),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    test_ok(
        serde_json::from_str(text.as_ref()),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}

fn payload_audit_event(payload: &LogFields) -> Result<EnforcementAuditEvent, TestText> {
    let text = test_some(
        optional_log_string(payload, constants::field::ENFORCEMENT_AUDIT_EVENT),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    test_ok(
        serde_json::from_str(text.as_ref()),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
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

fn timer_eventing_path(paths: &EnforcementJournalPaths) -> PathBuf {
    let mut path = paths.journal_path.clone();
    path.set_extension(constants::enforcement::EVENTING_JOURNAL_EXTENSION);
    path
}

fn cleanup_paths(paths: &EnforcementJournalPaths) {
    let _ = remove_file(&paths.journal_path);
    let _ = remove_file(&paths.key_path);
    let _ = remove_file(&paths.store_path);
    let _ = remove_file(&paths.timer_state_path);
    let eventing_path = timer_eventing_path(paths);
    let _ = remove_file(&eventing_path);
    let _ = remove_file(eventing_path.with_extension(format!(
        "{}.append.lock",
        constants::enforcement::EVENTING_JOURNAL_EXTENSION
    )));
    let mut wal_path = paths.store_path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = paths.store_path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
