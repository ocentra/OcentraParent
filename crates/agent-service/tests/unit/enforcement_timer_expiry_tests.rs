use std::fs::{read_to_string, remove_file};

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

use super::test_text::TestResult;
#[cfg(windows)]
use super::test_text::{optional_log_string, test_ok, test_some, TestText};
use crate::{
    enforcement_api::EnforcementJournalPaths,
    enforcement_tests::build_trusted_enforcement_audit_report,
    enforcement_timer_api::build_enforcement_timer_report_with_paths,
};

#[tokio::test]
async fn timer_expiry_uses_persisted_time_limit_state_and_clears_it() -> TestResult {
    let paths = temp_paths(constants::enforcement::TIMER_EXPIRED);
    cleanup_paths(&paths);

    let execute_event =
        build_trusted_enforcement_audit_report(time_limit_execute_command(), paths.clone()).await;
    assert_eq!(
        execute_event.event,
        AgentEventName::AgentEnforcementAuditReported
    );

    #[cfg(windows)]
    assert_timer_expiry_uses_persisted_state_and_clears_it(&paths).await?;

    #[cfg(not(windows))]
    assert_timer_expiry_without_supported_adapter_reports_missing_state(&paths, &execute_event)
        .await?;

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
