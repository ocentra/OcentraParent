use std::fs::{remove_dir, remove_file};

use ocentra_eventing::{
    expect_value::ExpectValue,
    ids::EventId,
    journal::{ndjson::NdjsonEventJournal, ndjson::NdjsonJournalOptions},
    replay::ReplayFilter,
};
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

use crate::enforcement_timer_state_path::EnforcementTimerStatePath;
use crate::production_enforcement_api::{
    build_enforcement_audit_report_with_paths, EnforcementJournalPaths,
};

#[tokio::test]
async fn production_command_retry_keeps_eventing_identity_and_command_correlation() {
    let root = std::env::temp_dir().join(format!(
        "enforcement-production-eventing-retry-{}",
        EventId::generated().as_str()
    ));
    let paths = paths(&root);
    let command = command();
    let first = build_enforcement_audit_report_with_paths(command.clone(), paths.clone()).await;
    let second = build_enforcement_audit_report_with_paths(command.clone(), paths.clone()).await;
    let eventing_path = eventing_path(&paths);
    let journal =
        NdjsonEventJournal::with_options(eventing_path.clone(), NdjsonJournalOptions::hash_chain());
    let replay = journal
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("replay production command eventing audit sidecar");

    assert_eq!(first.event, AgentEventName::AgentEnforcementAuditReported);
    assert_eq!(second.event, AgentEventName::AgentEnforcementAuditReported);
    assert_eq!(replay.records.len(), 2);
    assert!(replay.records.iter().all(|record| {
        record.envelope.correlation_id.as_str() == command.message_id
            && record.envelope.observed_at.as_str() == command.sent_at
    }));

    drop(journal);
    cleanup(&paths, &eventing_path, &root);
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
