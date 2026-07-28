use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::enforcement_api::build_enforcement_audit_report_with_paths;
use crate::enforcement_tests::{
    cleanup_paths, command, journal_event_ids, persist_authenticated_parent_delivery, temp_paths,
    trusted_delivery_directory,
};

#[tokio::test]
async fn raw_enforcement_envelope_without_persisted_delivery_rejects_before_journal_or_adapter() {
    let paths = temp_paths(constants::enforcement::TEST_ACTION_ID);
    cleanup_paths(&paths);

    let event = build_enforcement_audit_report_with_paths(command(false), paths.clone()).await;

    assert_rejected_before_journal(&event, &paths);
    cleanup_paths(&paths);
}

#[tokio::test]
async fn malformed_candidate_does_not_consume_persisted_delivery_before_rejection() {
    let paths = temp_paths(constants::enforcement::TEST_RESULT_ID);
    cleanup_paths(&paths);
    let valid_command = command(false);
    persist_authenticated_parent_delivery(&valid_command, &trusted_delivery_directory(&paths));
    let mut malformed_command = valid_command.clone();
    malformed_command.payload = malformed_command
        .payload
        .into_inner()
        .into_iter()
        .filter(|(field, _)| field != constants::field::POLICY_VERSION)
        .collect();

    let rejected =
        build_enforcement_audit_report_with_paths(malformed_command, paths.clone()).await;
    assert_rejected_before_journal(&rejected, &paths);

    let accepted = build_enforcement_audit_report_with_paths(valid_command, paths.clone()).await;
    assert_eq!(
        accepted.event,
        AgentEventName::AgentEnforcementAuditReported
    );
    assert_eq!(journal_event_ids(&paths).len(), 2);
    cleanup_paths(&paths);
}

#[tokio::test]
async fn persisted_delivery_is_consumed_once_and_replay_does_not_append_a_second_journal() {
    let paths = temp_paths(constants::enforcement::TEST_AUDIT_EVENT_ID);
    cleanup_paths(&paths);
    let delivery = command(false);
    persist_authenticated_parent_delivery(&delivery, &trusted_delivery_directory(&paths));

    let accepted = build_enforcement_audit_report_with_paths(delivery.clone(), paths.clone()).await;
    assert_eq!(
        accepted.event,
        AgentEventName::AgentEnforcementAuditReported
    );
    let first_journal_count = journal_event_ids(&paths).len();

    let replay = build_enforcement_audit_report_with_paths(delivery, paths.clone()).await;
    assert_eq!(replay.event, AgentEventName::AgentCommandRejected);
    assert_eq!(journal_event_ids(&paths).len(), first_journal_count);
    cleanup_paths(&paths);
}

#[tokio::test]
async fn persisted_delivery_binding_rejects_process_and_evidence_mismatch_before_journal() {
    for (suffix, field, value) in [
        (
            constants::enforcement::TEST_PROCESS_TARGET_ID,
            constants::field::PROCESS_ID,
            LogFieldValue::Number(1.0),
        ),
        (
            constants::enforcement::TEST_PROCESS_TARGET_VALUE,
            constants::field::EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(constants::enforcement::TEST_RESULT_ID.to_string()),
        ),
    ] {
        let paths = temp_paths(suffix);
        cleanup_paths(&paths);
        let delivery = command(false);
        persist_authenticated_parent_delivery(&delivery, &trusted_delivery_directory(&paths));
        let mut mismatch = delivery;
        mismatch.payload.insert(field.to_string(), value);

        let event = build_enforcement_audit_report_with_paths(mismatch, paths.clone()).await;
        assert_rejected_before_journal(&event, &paths);
        cleanup_paths(&paths);
    }
}

fn assert_rejected_before_journal(
    event: &ocentra_parent_agent_protocol::transport::AgentEventEnvelope,
    paths: &crate::enforcement_api::EnforcementJournalPaths,
) {
    assert_eq!(event.event, AgentEventName::AgentCommandRejected);
    assert!(!paths.journal_path.exists());
}
