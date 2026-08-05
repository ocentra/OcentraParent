use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants,
    logging::{LogFieldValue, LogFields},
    policy_constants,
    transport::{
        AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
        AgentPeerRole, AgentRoute,
    },
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

use super::{
    enforcement_api::{build_enforcement_audit_report_with_paths, EnforcementJournalPaths},
    test_text::{test_ok, test_some, TestResult, TestText},
};

#[tokio::test]
async fn rejected_action_is_persisted_as_a_durable_enforcement_audit() -> TestResult {
    let paths = temp_paths("rejected-action-journal");
    cleanup_paths(&paths);
    let event = build_enforcement_audit_report_with_paths(rejected_command(), paths.clone()).await;
    let fields = test_some(
        test_ok(
            ActivityStore::open(&paths.store_path),
            constants::error::ACTIVITY_STORE_OPENS,
        )?
        .latest_enforcement_audit_fields()
        .map_err(|_error| TestText::from_display(constants::error::ACTIVITY_STORE_OPENS))?,
        constants::error::JOURNAL_READS,
    )?;
    cleanup_paths(&paths);

    assert_eq!(event.event, AgentEventName::AgentCommandRejected);
    assert_eq!(
        event.payload.get(constants::field::REASON),
        Some(&LogFieldValue::String(
            constants::enforcement::REJECTION_TARGET_MISMATCH.to_string()
        ))
    );
    assert_eq!(
        fields.get(constants::field::ENFORCEMENT_AUDIT_EVENT_ID),
        Some(&LogFieldValue::String(
            constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()
        ))
    );
    assert_eq!(
        fields.get(constants::field::ENFORCEMENT_STATUS),
        Some(&LogFieldValue::String(
            constants::enforcement::RESULT_FAILED.to_string()
        ))
    );
    assert_eq!(
        fields.get(constants::field::POLICY_VERSION),
        Some(&LogFieldValue::String(
            policy_constants::TEST_POLICY_VERSION.to_string()
        ))
    );
    assert_eq!(
        fields.get(constants::field::TARGET_ID),
        Some(&LogFieldValue::String(
            constants::enforcement::TEST_CHILD_DEVICE_ID.to_string()
        ))
    );
    assert_eq!(
        fields.get(constants::field::REASON),
        Some(&LogFieldValue::String(
            constants::enforcement::REJECTION_TARGET_MISMATCH.to_string()
        ))
    );
    assert_eq!(
        fields.get(constants::field::EVIDENCE_REFERENCE_IDS),
        Some(&LogFieldValue::String(
            policy_constants::TEST_EVIDENCE_ID.to_string()
        ))
    );
    assert!(fields.get(constants::field::POLICY_TARGET_VALUE).is_none());

    Ok(())
}

#[tokio::test]
async fn rejected_audit_does_not_dedupe_a_corrected_retry_final_audit() -> TestResult {
    let paths = temp_paths("rejected-retry-final-audit");
    cleanup_paths(&paths);

    let rejected =
        build_enforcement_audit_report_with_paths(rejected_command(), paths.clone()).await;
    let completed =
        build_enforcement_audit_report_with_paths(corrected_retry_command(), paths.clone()).await;
    let store = test_ok(
        ActivityStore::open(&paths.store_path),
        constants::error::ACTIVITY_STORE_OPENS,
    )?;

    assert_eq!(rejected.event, AgentEventName::AgentCommandRejected);
    assert_eq!(
        completed.event,
        AgentEventName::AgentEnforcementAuditReported
    );
    assert!(test_ok(
        store.contains_event_id(&format!(
            "{}{}",
            constants::enforcement::JOURNAL_REJECTED_ID_PREFIX,
            constants::enforcement::TEST_AUDIT_EVENT_ID
        )),
        constants::error::JOURNAL_READS,
    )?);
    assert!(test_ok(
        store.contains_event_id(constants::enforcement::TEST_AUDIT_EVENT_ID),
        constants::error::JOURNAL_READS,
    )?);
    cleanup_paths(&paths);

    Ok(())
}

fn rejected_command() -> AgentCommandEnvelope {
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
        payload: rejected_payload(),
    }
}

fn corrected_retry_command() -> AgentCommandEnvelope {
    let mut command = rejected_command();
    command.payload.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(policy_constants::TARGET_TYPE_PROCESS.to_string()),
    );
    command.payload.insert(
        constants::field::TARGET_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_PROCESS_TARGET_ID.to_string()),
    );
    command.payload.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_PROCESS_TARGET_VALUE.to_string()),
    );
    command.payload.insert(
        constants::field::POLICY_DRY_RUN.to_string(),
        LogFieldValue::Boolean(true),
    );
    command.payload.insert(
        constants::field::PROCESS_ID.to_string(),
        LogFieldValue::Number(f64::from(u32::MAX)),
    );
    command
}

fn rejected_payload() -> LogFields {
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
            constants::field::REQUESTED_AT,
            policy_constants::TEST_EVALUATED_AT,
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
        (
            constants::field::ENFORCEMENT_INTENT_ID,
            constants::enforcement::TEST_INTENT_ID,
        ),
        (
            constants::field::POLICY_ACTION,
            policy_constants::ACTION_BLOCK,
        ),
        (
            constants::field::POLICY_TARGET_TYPE,
            policy_constants::TARGET_TYPE_DEVICE,
        ),
        (
            constants::field::TARGET_ID,
            constants::enforcement::TEST_CHILD_DEVICE_ID,
        ),
        (
            constants::field::POLICY_TARGET_VALUE,
            constants::enforcement::TEST_CHILD_DEVICE_ID,
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
    ] {
        fields.insert(key.to_string(), LogFieldValue::String(value.to_string()));
    }
    fields.insert(
        constants::field::POLICY_DRY_RUN.to_string(),
        LogFieldValue::Boolean(false),
    );
    fields
}

fn temp_paths(suffix: impl std::fmt::Display) -> EnforcementJournalPaths {
    let suffix = suffix.to_string();
    let build_path = |role: &str, extension: &str| {
        let name = format!(
            "{}{}{}{}{}{}",
            constants::journal::TEST_FILE_PREFIX,
            std::process::id(),
            constants::delimiter::HYPHEN,
            suffix,
            constants::delimiter::HYPHEN,
            role,
        );
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
    let _ = std::fs::remove_file(&paths.journal_path);
    let _ = std::fs::remove_file(&paths.key_path);
    let _ = std::fs::remove_file(&paths.store_path);
    let _ = std::fs::remove_file(&paths.timer_state_path);
    for extension in [
        constants::activity_store::WAL_FILE_EXTENSION,
        constants::activity_store::SHM_FILE_EXTENSION,
    ] {
        let mut path = paths.store_path.clone();
        path.set_extension(extension);
        let _ = std::fs::remove_file(path);
    }
}
