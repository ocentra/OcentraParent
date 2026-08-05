#[path = "../src/enforcement_api/enforcement_pre_action_journal/eventing_journal.rs"]
mod eventing_journal;

use std::{
    ffi::OsString,
    fs::{remove_dir, remove_file},
};

use ocentra_eventing::{
    expect_value::ExpectValue,
    ids::{CorrelationId, EventId},
    journal::{ndjson::NdjsonEventJournal, ndjson::NdjsonJournalOptions},
    replay::ReplayFilter,
};
use ocentra_parent_agent_protocol::{
    constants::enforcement,
    enforcement::{
        EnforcementAdapterResultCode, EnforcementAuditEventKind, EnforcementAuditJournalEvent,
        EnforcementCapabilityState, EnforcementResultStatus,
    },
};

#[tokio::test]
async fn typed_enforcement_audit_append_is_idempotent_and_replays_projection_only() {
    let root = std::env::temp_dir().join(format!(
        "enforcement-eventing-journal-{}",
        EventId::generated().as_str()
    ));
    let activity_journal_path = root.join("nested").join("activity.activity");
    let event = journal_event();
    let mut eventing_path = activity_journal_path.clone();
    eventing_path.set_extension(enforcement::EVENTING_JOURNAL_EXTENSION);
    let journal_path = eventing_journal::EnforcementEventingJournalPath {
        path: eventing_path.clone(),
    };
    assert!(!root.join("nested").exists());
    let first = eventing_journal::append_enforcement_audit_journal_event(
        journal_path.clone(),
        event.clone(),
        CorrelationId::parse(enforcement::TEST_AUDIT_EVENT_ID.to_string())
            .expect_value("typed correlation id"),
    )
    .await
    .expect_value("first typed enforcement audit append");
    let second = eventing_journal::append_enforcement_audit_journal_event(
        journal_path,
        event.clone(),
        CorrelationId::parse(enforcement::TEST_AUDIT_EVENT_ID.to_string())
            .expect_value("typed correlation id"),
    )
    .await
    .expect_value("idempotent typed enforcement audit append");
    let journal =
        NdjsonEventJournal::with_options(eventing_path.clone(), NdjsonJournalOptions::hash_chain());
    let replay = journal
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("projection-only enforcement audit replay");

    assert_eq!(first.sequence, second.sequence);
    assert_eq!(replay.records.len(), 1);
    assert_eq!(
        replay.records[0].envelope.contract.event_type.as_str(),
        enforcement::EVENT_AUDIT_JOURNAL_RECORDED
    );
    let decoded = replay.records[0]
        .envelope
        .decode::<EnforcementAuditJournalEvent>()
        .expect_value("typed enforcement audit replay decode");
    drop(journal);
    cleanup(&eventing_path);
    remove_dir(root.join("nested")).expect_value("nested test directory removed");
    remove_dir(&root).expect_value("test directory removed");

    assert_eq!(decoded.payload, event);
}

fn journal_event() -> EnforcementAuditJournalEvent {
    EnforcementAuditJournalEvent {
        audit_event_id: enforcement::TEST_AUDIT_EVENT_ID.to_string(),
        action_id: enforcement::TEST_ACTION_ID.to_string(),
        result_id: enforcement::TEST_RESULT_ID.to_string(),
        audit_event_kind: EnforcementAuditEventKind::Attempted,
        result_status: EnforcementResultStatus::WouldEnforce,
        adapter_result_code: EnforcementAdapterResultCode::NoOp,
        capability_state: EnforcementCapabilityState::Supported,
        observed_at: "2026-08-05T00:00:00Z".to_string(),
    }
}

fn cleanup(eventing_path: &std::path::Path) {
    let _ = remove_file(eventing_path);
    let mut lock_path = OsString::from(eventing_path.as_os_str());
    lock_path.push(".append.lock");
    let _ = remove_file(std::path::PathBuf::from(lock_path));
}
