#[path = "../src/enforcement_api/enforcement_pre_action_journal/eventing_journal.rs"]
mod eventing_journal;

use std::{
    ffi::OsString,
    fs::{remove_dir, remove_file},
};

use ocentra_eventing::{
    expect_value::ExpectValue,
    ids::{CorrelationId, EventId},
    journal::{
        ndjson::NdjsonEventJournal, ndjson::NdjsonJournalOptions, policy::JournalDispatchPhase,
    },
    replay::ReplayFilter,
};
use ocentra_parent_agent_protocol::{
    activity::policy::{PolicyAction, PolicyTargetType},
    constants::{enforcement, peer},
    enforcement::{
        EnforcementAdapterKind, EnforcementAdapterResultCode, EnforcementAuditEventKind,
        EnforcementAuditJournalEvent, EnforcementCapabilityState, EnforcementResultStatus,
        EnforcementRollbackState, ParentPlatform,
    },
    policy_constants as policy,
};

use ocentra_parent_agent_service::enforcement_audit_history::{
    read_enforcement_audit_history, EnforcementAuditHistoryKind, EnforcementAuditHistoryPath,
    EnforcementAuditHistoryRow,
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
    let first = eventing_journal::append_enforcement_audit_journal_event_phase(
        journal_path.clone(),
        event.clone(),
        CorrelationId::parse(enforcement::TEST_AUDIT_EVENT_ID.to_string())
            .expect_value("typed correlation id"),
        JournalDispatchPhase::AfterDispatch,
    )
    .await
    .expect_value("first typed enforcement audit append");
    let second = eventing_journal::append_enforcement_audit_journal_event_phase(
        journal_path,
        event.clone(),
        CorrelationId::parse(enforcement::TEST_AUDIT_EVENT_ID.to_string())
            .expect_value("typed correlation id"),
        JournalDispatchPhase::AfterDispatch,
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

#[tokio::test]
async fn bare_relative_eventing_path_does_not_require_empty_parent_directory() {
    let file_name = format!(
        "enforcement-eventing-bare-{}.eventing",
        EventId::generated().as_str()
    );
    let eventing_path = std::path::PathBuf::from(&file_name);
    let journal_path = eventing_journal::EnforcementEventingJournalPath {
        path: eventing_path.clone(),
    };
    let result = eventing_journal::append_enforcement_audit_journal_event_phase(
        journal_path,
        journal_event(),
        CorrelationId::parse(enforcement::TEST_AUDIT_EVENT_ID.to_string())
            .expect_value("typed correlation id"),
        JournalDispatchPhase::AfterDispatch,
    )
    .await
    .expect_value("bare relative eventing audit append");

    assert_eq!(result.sequence, 1);
    cleanup(&eventing_path);
}

#[tokio::test]
async fn projection_history_orders_enforcement_transition_matrix_and_deduplicates_replay() {
    let root = std::env::temp_dir().join(format!(
        "enforcement-eventing-history-{}",
        EventId::generated().as_str()
    ));
    let eventing_path = root.join("audit.eventing");
    let journal_path = eventing_journal::EnforcementEventingJournalPath {
        path: eventing_path.clone(),
    };
    let matrix = transition_matrix();
    append_transition_matrix(&journal_path, &matrix).await;

    let rows = read_enforcement_audit_history(EnforcementAuditHistoryPath(eventing_path.clone()))
        .await
        .expect_value("projection-only enforcement history");
    assert_transition_history(&rows);
    cleanup(&eventing_path);
    let _ = remove_dir(&root);
}

fn transition_matrix() -> [EnforcementAuditJournalEvent; 7] {
    [
        matrix_event(
            "rejected",
            EnforcementAuditEventKind::Failed,
            EnforcementResultStatus::Failed,
            EnforcementRollbackState::NotRequired,
        ),
        matrix_event(
            "accepted",
            EnforcementAuditEventKind::Attempted,
            EnforcementResultStatus::WouldEnforce,
            EnforcementRollbackState::NotRequired,
        ),
        matrix_event(
            "no-op",
            EnforcementAuditEventKind::Succeeded,
            EnforcementResultStatus::NoOp,
            EnforcementRollbackState::NotRequired,
        ),
        matrix_event(
            "unavailable",
            EnforcementAuditEventKind::Unavailable,
            EnforcementResultStatus::Unavailable,
            EnforcementRollbackState::Unavailable,
        ),
        matrix_event(
            "expired",
            EnforcementAuditEventKind::Expired,
            EnforcementResultStatus::Expired,
            EnforcementRollbackState::Available,
        ),
        matrix_event(
            "rollback",
            EnforcementAuditEventKind::RollbackCompleted,
            EnforcementResultStatus::RolledBack,
            EnforcementRollbackState::Completed,
        ),
        matrix_event(
            "cancelled",
            EnforcementAuditEventKind::Cancelled,
            EnforcementResultStatus::Superseded,
            EnforcementRollbackState::Completed,
        ),
    ]
}

async fn append_transition_matrix(
    journal_path: &eventing_journal::EnforcementEventingJournalPath,
    matrix: &[EnforcementAuditJournalEvent],
) {
    for event in matrix {
        eventing_journal::append_enforcement_audit_journal_event_phase(
            journal_path.clone(),
            event.clone(),
            CorrelationId::parse(format!("correlation:{}", event.audit_event_id))
                .expect_value("matrix correlation id"),
            JournalDispatchPhase::AfterDispatch,
        )
        .await
        .expect_value("matrix append");
    }
    eventing_journal::append_enforcement_audit_journal_event_phase(
        journal_path.clone(),
        matrix[2].clone(),
        CorrelationId::parse(format!("correlation:{}", matrix[2].audit_event_id))
            .expect_value("duplicate correlation id"),
        JournalDispatchPhase::AfterDispatch,
    )
    .await
    .expect_value("duplicate matrix append");
}

fn assert_transition_history(rows: &[EnforcementAuditHistoryRow]) {
    assert_eq!(rows.len(), 7);
    assert_eq!(
        rows.iter().map(|row| row.sequence).collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5, 6, 7]
    );
    assert_eq!(
        rows.iter().map(|row| row.kind).collect::<Vec<_>>(),
        vec![
            EnforcementAuditHistoryKind::RejectedIntent,
            EnforcementAuditHistoryKind::AcceptedIntent,
            EnforcementAuditHistoryKind::AdapterResult,
            EnforcementAuditHistoryKind::AdapterResult,
            EnforcementAuditHistoryKind::TimerExpired,
            EnforcementAuditHistoryKind::TimerRollback,
            EnforcementAuditHistoryKind::TimerCancelled,
        ]
    );
    for row in rows {
        assert_eq!(
            row.event
                .actor
                .as_ref()
                .map(|actor| actor.actor_id.as_str()),
            Some("parent:actor-1")
        );
        assert_eq!(row.event.target_id, "process:owned-1");
        assert_eq!(row.event.policy_decision_id, "policy:decision-1");
        assert_eq!(
            row.event.evidence_references[0].evidence_reference_id,
            "evidence:redacted-1"
        );
        assert_eq!(row.event.target_route.as_deref(), Some("child:route-1"));
        assert_eq!(
            row.event.reason.as_deref(),
            Some("redacted-transition-reason")
        );
    }
}

fn matrix_event(
    suffix: &str,
    audit_event_kind: EnforcementAuditEventKind,
    result_status: EnforcementResultStatus,
    rollback_state: EnforcementRollbackState,
) -> EnforcementAuditJournalEvent {
    let mut event = journal_event();
    event.audit_event_id = if suffix == "rejected" {
        format!("{}{suffix}", enforcement::JOURNAL_REJECTED_ID_PREFIX)
    } else {
        format!("audit:{suffix}")
    };
    event.action_id = format!("action:{suffix}");
    event.intent_id = format!("intent:{suffix}");
    event.result_id = format!("result:{suffix}");
    event.policy_decision_id = "policy:decision-1".to_string();
    event.target_id = "process:owned-1".to_string();
    event.audit_event_kind = audit_event_kind;
    event.result_status = result_status;
    event.rollback_state = rollback_state;
    event.actor = Some(
        ocentra_parent_agent_protocol::activity::policy::ParentActorReference {
            actor_id: "parent:actor-1".to_string(),
            role: ocentra_parent_agent_protocol::activity::policy::ParentActorRole::Parent,
        },
    );
    event.evidence_references = vec![
        ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference {
            evidence_reference_id: "evidence:redacted-1".to_string(),
            kind: ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind::JournalEvent,
            observed_at: "2026-08-10T00:00:00Z".to_string(),
        },
    ];
    event.target_route = Some("child:route-1".to_string());
    event.reason = Some("redacted-transition-reason".to_string());
    event
}

fn journal_event() -> EnforcementAuditJournalEvent {
    EnforcementAuditJournalEvent {
        audit_event_id: enforcement::TEST_AUDIT_EVENT_ID.to_string(),
        action_id: enforcement::TEST_ACTION_ID.to_string(),
        intent_id: enforcement::TEST_INTENT_ID.to_string(),
        result_id: enforcement::TEST_RESULT_ID.to_string(),
        policy_decision_id: policy::TEST_DECISION_ID.to_string(),
        policy_version: policy::TEST_POLICY_VERSION.to_string(),
        policy_action: PolicyAction::Block,
        target_id: enforcement::TEST_PROCESS_TARGET_ID.to_string(),
        target_type: PolicyTargetType::Process,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        platform: ParentPlatform::Windows,
        audit_event_kind: EnforcementAuditEventKind::Attempted,
        result_status: EnforcementResultStatus::WouldEnforce,
        adapter_result_code: EnforcementAdapterResultCode::NoOp,
        capability_state: EnforcementCapabilityState::Supported,
        evidence_references: Vec::new(),
        actor: None,
        parent_override: None,
        unavailable_status: None,
        rollback_state: EnforcementRollbackState::NotRequired,
        dry_run: true,
        reason_codes: Vec::new(),
        reason: None,
        requested_at: policy::TEST_EVALUATED_AT.to_string(),
        started_at: None,
        completed_at: None,
        journal_sequence: None,
        device_id: Some(enforcement::TEST_CHILD_DEVICE_ID.to_string()),
        source_peer_id: Some(peer::PORTAL_DEV.to_string()),
        target_route: Some("local-network".to_string()),
        observed_at: "2026-08-05T00:00:00Z".to_string(),
    }
}

fn cleanup(eventing_path: &std::path::Path) {
    let _ = remove_file(eventing_path);
    let mut lock_path = OsString::from(eventing_path.as_os_str());
    lock_path.push(".append.lock");
    let _ = remove_file(std::path::PathBuf::from(lock_path));
}
