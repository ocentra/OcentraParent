use ocentra_eventing::envelope::{EventEnvelope, EventMetadata};
use ocentra_eventing::journal::ndjson::NdjsonEventJournal;

use crate::app_game_unknown_approval_event::{
    AppGameUnknownApprovalEvent, AppGameUnknownApprovalTransition,
};
use crate::app_game_unknown_approval_projection::{
    read_unknown_approval_history, AppGameUnknownApprovalHistory,
};
use crate::app_game_unknown_approval_reducer::apply_unknown_approval_event;
use crate::app_game_unknown_approval_types::{
    AppGameUnknownApprovalError, AppGameUnknownApprovalExpiryInput,
    AppGameUnknownApprovalPersistenceState, AppGameUnknownApprovalRequest,
    AppGameUnknownApprovalRequestInput, AppGameUnknownApprovalResponseInput,
    AppGameUnknownApprovalSnapshot, AppGameUnknownApprovalWriteReceipt, AppGameUnknownCandidate,
    AppGameUnknownCandidateInput,
};
use crate::app_game_unknown_approval_validation::{
    require_text, validate_optional_refs, validate_unknown_candidate,
};

pub fn produce_app_game_unknown_candidate(
    input: AppGameUnknownCandidateInput,
) -> Result<AppGameUnknownCandidate, AppGameUnknownApprovalError> {
    let candidate = AppGameUnknownCandidate {
        candidate_id: input.candidate_id,
        subject_ref: input.subject_ref,
        device_ref: input.device_ref,
        local_user_ref: input.local_user_ref,
        kind: input.kind,
        source: input.source,
        classification: input.classification,
        observed_at_epoch_ms: input.observed_at_epoch_ms,
        evidence_refs: input.evidence_refs,
        category_candidate_ref: input.category_candidate_ref,
        child_status_refs: input.child_status_refs,
    };
    validate_unknown_candidate(&candidate)?;
    Ok(candidate)
}

pub async fn persist_app_game_unknown_approval_request(
    journal: &NdjsonEventJournal,
    metadata: EventMetadata,
    input: AppGameUnknownApprovalRequestInput,
) -> Result<AppGameUnknownApprovalWriteReceipt, AppGameUnknownApprovalError> {
    validate_request_input(&input)?;
    let request_id = input.request_id.clone();
    let event = AppGameUnknownApprovalEvent {
        transition_id: input.transition_id,
        request_id: request_id.clone(),
        occurred_at_epoch_ms: input.candidate.observed_at_epoch_ms,
        transition: AppGameUnknownApprovalTransition::RequestOpened {
            request: AppGameUnknownApprovalRequest {
                request_id: input.request_id,
                candidate: input.candidate,
                child_reason_refs: input.child_reason_refs,
                expires_at_epoch_ms: input.expires_at_epoch_ms,
            },
        },
    };
    persist_transition(journal, metadata, event, &request_id).await
}

pub async fn persist_app_game_unknown_parent_response(
    journal: &NdjsonEventJournal,
    metadata: EventMetadata,
    input: AppGameUnknownApprovalResponseInput,
) -> Result<AppGameUnknownApprovalWriteReceipt, AppGameUnknownApprovalError> {
    require_text(
        &input.transition_id,
        "app_game.unknown_approval.transition_id",
    )?;
    require_text(&input.request_id, "app_game.unknown_approval.request_id")?;
    let request_id = input.request_id.clone();
    let event = AppGameUnknownApprovalEvent {
        transition_id: input.transition_id,
        request_id: request_id.clone(),
        occurred_at_epoch_ms: input.occurred_at_epoch_ms,
        transition: AppGameUnknownApprovalTransition::ParentResponded {
            actor_ref: input.actor_ref,
            response: input.response,
            capability_state: input.capability_state,
            evidence_refs: input.evidence_refs,
            child_reason_refs: input.child_reason_refs,
            child_status_refs: input.child_status_refs,
            audit_ref: input.audit_ref,
            override_ref: input.override_ref,
            decision_expires_at_epoch_ms: input.decision_expires_at_epoch_ms,
        },
    };
    persist_transition(journal, metadata, event, &request_id).await
}

pub async fn persist_app_game_unknown_approval_expiry(
    journal: &NdjsonEventJournal,
    metadata: EventMetadata,
    input: AppGameUnknownApprovalExpiryInput,
) -> Result<AppGameUnknownApprovalWriteReceipt, AppGameUnknownApprovalError> {
    require_text(
        &input.transition_id,
        "app_game.unknown_approval.transition_id",
    )?;
    require_text(&input.request_id, "app_game.unknown_approval.request_id")?;
    let request_id = input.request_id.clone();
    let event = AppGameUnknownApprovalEvent {
        transition_id: input.transition_id,
        request_id: request_id.clone(),
        occurred_at_epoch_ms: input.occurred_at_epoch_ms,
        transition: AppGameUnknownApprovalTransition::RequestExpired {
            audit_ref: input.audit_ref,
        },
    };
    persist_transition(journal, metadata, event, &request_id).await
}

pub async fn load_app_game_unknown_approval(
    journal: &NdjsonEventJournal,
    request_id: &str,
) -> Result<AppGameUnknownApprovalSnapshot, AppGameUnknownApprovalError> {
    require_text(request_id, "app_game.unknown_approval.request_id")?;
    read_unknown_approval_history(journal, request_id)
        .await?
        .snapshot
        .ok_or_else(|| AppGameUnknownApprovalError::RequestNotFound {
            request_id: request_id.to_owned(),
        })
}

async fn persist_transition(
    journal: &NdjsonEventJournal,
    metadata: EventMetadata,
    event: AppGameUnknownApprovalEvent,
    request_id: &str,
) -> Result<AppGameUnknownApprovalWriteReceipt, AppGameUnknownApprovalError> {
    let history = read_unknown_approval_history(journal, request_id).await?;
    if let Some(receipt) = replay_receipt_if_present(&history, &event)? {
        return Ok(receipt);
    }
    if history.snapshot.is_none()
        && !matches!(
            &event.transition,
            AppGameUnknownApprovalTransition::RequestOpened { .. }
        )
    {
        return Err(AppGameUnknownApprovalError::RequestNotFound {
            request_id: request_id.to_owned(),
        });
    }
    let mut snapshot = apply_unknown_approval_event(history.snapshot.as_ref(), &event)?;
    let stored = EventEnvelope::from_event(event, metadata)?.store()?;
    let append = journal.append_idempotent(&stored).await?;
    snapshot.persistence_state = AppGameUnknownApprovalPersistenceState::Replayable;
    Ok(AppGameUnknownApprovalWriteReceipt {
        sequence: append.sequence,
        replayed: false,
        synchronized: append.is_synchronized(),
        snapshot,
    })
}

fn replay_receipt_if_present(
    history: &AppGameUnknownApprovalHistory,
    event: &AppGameUnknownApprovalEvent,
) -> Result<Option<AppGameUnknownApprovalWriteReceipt>, AppGameUnknownApprovalError> {
    let Some((sequence, existing)) = history
        .events
        .iter()
        .find(|(_sequence, existing)| existing.transition_id == event.transition_id)
    else {
        return Ok(None);
    };
    if existing != event {
        return Err(AppGameUnknownApprovalError::DuplicateTransition {
            transition_id: event.transition_id.clone(),
        });
    }
    let snapshot =
        history
            .snapshot
            .clone()
            .ok_or_else(|| AppGameUnknownApprovalError::RequestNotFound {
                request_id: event.request_id.clone(),
            })?;
    Ok(Some(AppGameUnknownApprovalWriteReceipt {
        sequence: *sequence,
        replayed: true,
        synchronized: false,
        snapshot,
    }))
}

fn validate_request_input(
    input: &AppGameUnknownApprovalRequestInput,
) -> Result<(), AppGameUnknownApprovalError> {
    require_text(&input.request_id, "app_game.unknown_approval.request_id")?;
    require_text(
        &input.transition_id,
        "app_game.unknown_approval.transition_id",
    )?;
    validate_optional_refs(
        &input.child_reason_refs,
        "app_game.unknown_approval.child_reason_refs",
    )?;
    validate_unknown_candidate(&input.candidate)?;
    if input.expires_at_epoch_ms <= input.candidate.observed_at_epoch_ms {
        return Err(AppGameUnknownApprovalError::InvalidTransition {
            reason: "request expiry must be after candidate observation",
        });
    }
    Ok(())
}
