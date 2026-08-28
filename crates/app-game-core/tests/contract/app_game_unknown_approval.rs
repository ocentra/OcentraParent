use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ocentra_app_game_core::app_game_unknown_approval::{
    load_app_game_unknown_approval, persist_app_game_unknown_approval_expiry,
    persist_app_game_unknown_approval_request, persist_app_game_unknown_parent_response,
    produce_app_game_unknown_candidate,
};
use ocentra_app_game_core::app_game_unknown_approval_types::{
    AppGameUnknownAdapterCapabilityState, AppGameUnknownAdapterDispatchState,
    AppGameUnknownApprovalError, AppGameUnknownApprovalExpiryInput,
    AppGameUnknownApprovalRequestInput, AppGameUnknownApprovalResponseInput,
    AppGameUnknownApprovalStatus, AppGameUnknownCandidateInput, AppGameUnknownCandidateKind,
    AppGameUnknownCandidateSource, AppGameUnknownClassification, AppGameUnknownParentResponse,
};
use ocentra_eventing::envelope::{EventMetadata, EventSource};
use ocentra_eventing::ids::{
    CorrelationId, EventCustody, EventId, RecordedAt, RuntimeInstanceId, RuntimeRole,
    SourceComponent, SourceService,
};
use ocentra_eventing::journal::ndjson::NdjsonEventJournal;

static TEST_PATH_SEQUENCE: AtomicU64 = AtomicU64::new(1);

const OBSERVED_AT: u64 = 1_000;
const REQUEST_EXPIRES_AT: u64 = 5_000;
const RESPONSE_AT: u64 = 2_000;

#[test]
fn unknown_candidate_producer_preserves_weak_game_classification_and_requires_evidence(
) -> Result<(), AppGameUnknownApprovalError> {
    let candidate = produce_app_game_unknown_candidate(candidate_input(
        AppGameUnknownCandidateKind::GameLikeExecutable,
        AppGameUnknownClassification::PossibleGame,
    ))?;
    assert_eq!(
        candidate.classification,
        AppGameUnknownClassification::PossibleGame
    );
    assert_eq!(candidate.evidence_refs, vec![String::from("evidence-1")]);

    let mut missing_evidence = candidate_input(
        AppGameUnknownCandidateKind::NewInventoryApp,
        AppGameUnknownClassification::UnknownApp,
    );
    missing_evidence.evidence_refs.clear();
    assert!(matches!(
        produce_app_game_unknown_candidate(missing_evidence),
        Err(AppGameUnknownApprovalError::InvalidField {
            field: "app_game.unknown_candidate.evidence_refs"
        })
    ));

    let incompatible = candidate_input(
        AppGameUnknownCandidateKind::NewInventoryApp,
        AppGameUnknownClassification::PossibleGame,
    );
    assert_invalid_transition(&produce_app_game_unknown_candidate(incompatible));
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn approval_is_durable_across_restart_and_exact_replay_is_idempotent(
) -> Result<(), AppGameUnknownApprovalError> {
    let path = unique_journal_path("restart-replay");
    let _cleanup = JournalCleanup(path.clone());
    let journal = NdjsonEventJournal::new(path.clone());
    let request = request_input("request-1", "open-1", candidate()?);
    let opened = persist_app_game_unknown_approval_request(
        &journal,
        metadata("event-open-1", "correlation-1")?,
        request.clone(),
    )
    .await?;
    assert_eq!(
        opened.snapshot.status,
        AppGameUnknownApprovalStatus::Pending
    );
    assert!(opened.synchronized);

    let replayed = persist_app_game_unknown_approval_request(
        &journal,
        metadata("event-open-replay", "correlation-1")?,
        request,
    )
    .await?;
    assert!(replayed.replayed);
    assert_eq!(replayed.sequence, opened.sequence);

    drop(journal);
    let restarted = NdjsonEventJournal::new(path);
    let recovered = load_app_game_unknown_approval(&restarted, "request-1").await?;
    assert_eq!(recovered.status, AppGameUnknownApprovalStatus::Pending);
    assert_eq!(recovered.request.candidate.candidate_id, "candidate-1");

    let allow_once = response_input(
        "request-1",
        "response-1",
        AppGameUnknownParentResponse::AllowOnce,
        AppGameUnknownAdapterCapabilityState::Unproven,
    );
    let allowed = persist_app_game_unknown_parent_response(
        &restarted,
        metadata("event-response-1", "correlation-1")?,
        allow_once.clone(),
    )
    .await?;
    assert_eq!(
        allowed.snapshot.status,
        AppGameUnknownApprovalStatus::AllowedOnce
    );
    assert_eq!(
        allowed.snapshot.adapter_dispatch_state,
        AppGameUnknownAdapterDispatchState::NotDispatched
    );
    assert_eq!(allowed.snapshot.audit_refs, vec![String::from("audit-1")]);
    let replayed_response = persist_app_game_unknown_parent_response(
        &restarted,
        metadata("event-response-replay", "correlation-1")?,
        allow_once,
    )
    .await?;
    assert!(replayed_response.replayed);
    assert_eq!(replayed_response.sequence, allowed.sequence);
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn unproven_block_is_manual_required_and_never_dispatches_adapter(
) -> Result<(), AppGameUnknownApprovalError> {
    let path = unique_journal_path("manual-required");
    let _cleanup = JournalCleanup(path.clone());
    let journal = NdjsonEventJournal::new(path);
    open_request(&journal, "request-2").await?;
    let blocked = persist_app_game_unknown_parent_response(
        &journal,
        metadata("event-response-2", "correlation-2")?,
        response_input(
            "request-2",
            "response-2",
            AppGameUnknownParentResponse::BlockIfSupported,
            AppGameUnknownAdapterCapabilityState::Unproven,
        ),
    )
    .await?;
    assert_eq!(
        blocked.snapshot.status,
        AppGameUnknownApprovalStatus::ManualRequired
    );
    assert_eq!(
        blocked.snapshot.adapter_dispatch_state,
        AppGameUnknownAdapterDispatchState::NotDispatched
    );
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn expiry_survives_replay_and_rejects_late_parent_response(
) -> Result<(), AppGameUnknownApprovalError> {
    let path = unique_journal_path("expiry");
    let _cleanup = JournalCleanup(path.clone());
    let journal = NdjsonEventJournal::new(path.clone());
    open_request(&journal, "request-3").await?;
    let mut at_expiry = response_input(
        "request-3",
        "response-at-expiry-3",
        AppGameUnknownParentResponse::Deny,
        AppGameUnknownAdapterCapabilityState::Unproven,
    );
    at_expiry.occurred_at_epoch_ms = REQUEST_EXPIRES_AT;
    let at_expiry_response = persist_app_game_unknown_parent_response(
        &journal,
        metadata("event-response-at-expiry-3", "correlation-3")?,
        at_expiry,
    )
    .await;
    assert_invalid_transition(&at_expiry_response);

    let expired = persist_app_game_unknown_approval_expiry(
        &journal,
        metadata("event-expiry-3", "correlation-3")?,
        AppGameUnknownApprovalExpiryInput {
            transition_id: String::from("expiry-3"),
            request_id: String::from("request-3"),
            audit_ref: String::from("audit-expiry-3"),
            occurred_at_epoch_ms: REQUEST_EXPIRES_AT,
        },
    )
    .await?;
    assert_eq!(
        expired.snapshot.status,
        AppGameUnknownApprovalStatus::Expired
    );

    let late = persist_app_game_unknown_parent_response(
        &journal,
        metadata("event-response-3", "correlation-3")?,
        response_input(
            "request-3",
            "response-3",
            AppGameUnknownParentResponse::Deny,
            AppGameUnknownAdapterCapabilityState::Unproven,
        ),
    )
    .await;
    assert_invalid_transition(&late);

    drop(journal);
    let restarted = NdjsonEventJournal::new(path);
    assert_eq!(
        load_app_game_unknown_approval(&restarted, "request-3")
            .await?
            .status,
        AppGameUnknownApprovalStatus::Expired
    );
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn ask_child_follow_up_requires_reason_refs_and_conflicting_replay_fails(
) -> Result<(), AppGameUnknownApprovalError> {
    let path = unique_journal_path("child-reason");
    let _cleanup = JournalCleanup(path.clone());
    let journal = NdjsonEventJournal::new(path);
    open_request(&journal, "request-4").await?;
    let asked = persist_app_game_unknown_parent_response(
        &journal,
        metadata("event-ask-4", "correlation-4")?,
        response_input(
            "request-4",
            "response-ask-4",
            AppGameUnknownParentResponse::AskChildWhy,
            AppGameUnknownAdapterCapabilityState::Unproven,
        ),
    )
    .await?;
    assert_eq!(
        asked.snapshot.status,
        AppGameUnknownApprovalStatus::AwaitingChildReason
    );

    let missing_reason = persist_app_game_unknown_parent_response(
        &journal,
        metadata("event-follow-up-4", "correlation-4")?,
        response_input(
            "request-4",
            "response-follow-up-4",
            AppGameUnknownParentResponse::AllowTarget,
            AppGameUnknownAdapterCapabilityState::Unproven,
        ),
    )
    .await;
    assert_invalid_transition(&missing_reason);

    let mut with_reason = response_input(
        "request-4",
        "response-follow-up-4",
        AppGameUnknownParentResponse::AllowTarget,
        AppGameUnknownAdapterCapabilityState::Unproven,
    );
    with_reason.child_reason_refs = vec![String::from("child-reason-4")];
    let allowed = persist_app_game_unknown_parent_response(
        &journal,
        metadata("event-follow-up-4b", "correlation-4")?,
        with_reason.clone(),
    )
    .await?;
    assert_eq!(
        allowed.snapshot.status,
        AppGameUnknownApprovalStatus::AllowedTarget
    );

    with_reason.response = AppGameUnknownParentResponse::Deny;
    let conflicting = persist_app_game_unknown_parent_response(
        &journal,
        metadata("event-follow-up-4c", "correlation-4")?,
        with_reason,
    )
    .await;
    assert!(matches!(
        conflicting,
        Err(AppGameUnknownApprovalError::DuplicateTransition { .. })
    ));
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn category_and_override_responses_fail_closed_without_required_refs(
) -> Result<(), AppGameUnknownApprovalError> {
    let path = unique_journal_path("response-refs");
    let _cleanup = JournalCleanup(path.clone());
    let journal = NdjsonEventJournal::new(path);
    let mut candidate = candidate()?;
    candidate.category_candidate_ref = None;
    persist_app_game_unknown_approval_request(
        &journal,
        metadata("event-open-5", "correlation-5")?,
        request_input("request-5", "open-5", candidate),
    )
    .await?;
    let allow_category = persist_app_game_unknown_parent_response(
        &journal,
        metadata("event-category-5", "correlation-5")?,
        response_input(
            "request-5",
            "response-category-5",
            AppGameUnknownParentResponse::AllowCategory,
            AppGameUnknownAdapterCapabilityState::Unproven,
        ),
    )
    .await;
    assert_invalid_transition(&allow_category);

    let override_without_ref = persist_app_game_unknown_parent_response(
        &journal,
        metadata("event-override-5", "correlation-5")?,
        response_input(
            "request-5",
            "response-override-5",
            AppGameUnknownParentResponse::Override,
            AppGameUnknownAdapterCapabilityState::Unproven,
        ),
    )
    .await;
    assert!(matches!(
        override_without_ref,
        Err(AppGameUnknownApprovalError::InvalidField {
            field: "app_game.unknown_approval.override_ref"
        })
    ));
    Ok(())
}

async fn open_request(
    journal: &NdjsonEventJournal,
    request_id: &str,
) -> Result<(), AppGameUnknownApprovalError> {
    persist_app_game_unknown_approval_request(
        journal,
        metadata(&format!("event-open-{request_id}"), request_id)?,
        request_input(request_id, &format!("open-{request_id}"), candidate()?),
    )
    .await?;
    Ok(())
}

fn candidate() -> Result<
    ocentra_app_game_core::app_game_unknown_approval_types::AppGameUnknownCandidate,
    AppGameUnknownApprovalError,
> {
    produce_app_game_unknown_candidate(candidate_input(
        AppGameUnknownCandidateKind::NewInventoryApp,
        AppGameUnknownClassification::UnknownApp,
    ))
}

fn candidate_input(
    kind: AppGameUnknownCandidateKind,
    classification: AppGameUnknownClassification,
) -> AppGameUnknownCandidateInput {
    AppGameUnknownCandidateInput {
        candidate_id: String::from("candidate-1"),
        subject_ref: String::from("subject-1"),
        device_ref: String::from("device-1"),
        local_user_ref: String::from("local-user-1"),
        kind,
        source: AppGameUnknownCandidateSource::Inventory,
        classification,
        observed_at_epoch_ms: OBSERVED_AT,
        evidence_refs: vec![String::from("evidence-1")],
        category_candidate_ref: Some(String::from("category-candidate-1")),
        child_status_refs: vec![String::from("child-status-1")],
    }
}

fn request_input(
    request_id: &str,
    transition_id: &str,
    candidate: ocentra_app_game_core::app_game_unknown_approval_types::AppGameUnknownCandidate,
) -> AppGameUnknownApprovalRequestInput {
    AppGameUnknownApprovalRequestInput {
        request_id: request_id.to_owned(),
        transition_id: transition_id.to_owned(),
        candidate,
        child_reason_refs: Vec::new(),
        expires_at_epoch_ms: REQUEST_EXPIRES_AT,
    }
}

fn response_input(
    request_id: &str,
    transition_id: &str,
    response: AppGameUnknownParentResponse,
    capability_state: AppGameUnknownAdapterCapabilityState,
) -> AppGameUnknownApprovalResponseInput {
    AppGameUnknownApprovalResponseInput {
        transition_id: transition_id.to_owned(),
        request_id: request_id.to_owned(),
        actor_ref: String::from("parent-1"),
        response,
        capability_state,
        evidence_refs: vec![String::from("evidence-1")],
        child_reason_refs: Vec::new(),
        child_status_refs: vec![String::from("child-status-1")],
        audit_ref: String::from("audit-1"),
        override_ref: None,
        occurred_at_epoch_ms: RESPONSE_AT,
        decision_expires_at_epoch_ms: (response == AppGameUnknownParentResponse::AllowOnce)
            .then_some(RESPONSE_AT + 500),
    }
}

fn metadata(
    event_id: &str,
    correlation_id: &str,
) -> Result<EventMetadata, ocentra_eventing::error::EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::parse(event_id)?,
        CorrelationId::parse(correlation_id)?,
        EventSource::new(
            EventCustody::parse("local-child")?,
            RuntimeRole::parse("child")?,
            SourceService::parse("app-game-core-contract-test")?,
            SourceComponent::parse("unknown-approval-lifecycle")?,
            RuntimeInstanceId::parse("contract-test-instance")?,
        ),
        RecordedAt::parse("2026-08-15T00:00:00Z")?,
        None,
    ))
}

fn assert_invalid_transition<T>(result: &Result<T, AppGameUnknownApprovalError>) {
    assert!(matches!(
        result,
        Err(AppGameUnknownApprovalError::InvalidTransition { .. })
    ));
}

fn unique_journal_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "ocentra-app-game-unknown-approval-{label}-{}-{}.ndjson",
        std::process::id(),
        TEST_PATH_SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ))
}

struct JournalCleanup(PathBuf);

impl Drop for JournalCleanup {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
        let mut lock_path = self.0.as_os_str().to_owned();
        lock_path.push(".lock");
        let _ = std::fs::remove_file(PathBuf::from(lock_path));
    }
}
