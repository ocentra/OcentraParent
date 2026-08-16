use std::{fs, path::PathBuf, time::SystemTime};

use ocentra_app_game_core::app_game_child_ux::build_app_game_child_ux_notice;
use ocentra_app_game_core::app_game_child_ux_outbox::build_app_game_child_ux_outbox_route;
use ocentra_app_game_core::app_game_child_ux_outbox_store::AppGameChildUxLocalOutboxStore;
use ocentra_app_game_core::app_game_child_ux_outbox_types::{
    AppGameChildUxOutboxInput, AppGameChildUxOutboxPersistResult, AppGameChildUxOutboxRoute,
};
use ocentra_app_game_core::app_game_child_ux_preference_preflight::build_app_game_child_ux_preference_preflight;
use ocentra_app_game_core::app_game_child_ux_preference_preflight_types::{
    AppGameChildUxPreferencePreflightInput, AppGameChildUxPreferencePreflightStatus,
};
use ocentra_app_game_core::app_game_child_ux_provider_preflight::build_app_game_child_ux_provider_preflight;
use ocentra_app_game_core::app_game_child_ux_provider_preflight_types::{
    AppGameChildUxProviderPreflightInput, AppGameChildUxProviderPreflightRow,
    AppGameChildUxProviderPreflightStatus,
};
use ocentra_app_game_core::app_game_child_ux_provider_status::build_app_game_child_ux_provider_status_handoff;
use ocentra_app_game_core::app_game_child_ux_provider_status_types::AppGameChildUxProviderStatusInput;
use ocentra_app_game_core::app_game_child_ux_scheduler::build_app_game_child_ux_scheduler_route;
use ocentra_app_game_core::app_game_child_ux_scheduler_store::AppGameChildUxSchedulerProofStore;
use ocentra_app_game_core::app_game_child_ux_scheduler_types::{
    AppGameChildUxSchedulerInput, AppGameChildUxSchedulerPersistResult,
    AppGameChildUxSchedulerRoute,
};
use ocentra_app_game_core::app_game_child_ux_types::{
    AppGameChildReasonRef, AppGameChildStatusRef, AppGameChildUxCapabilityState,
    AppGameChildUxInput, AppGameChildUxNoticeState, AppGameChildUxRequestState,
    AppGameChildUxSubjectKind,
};
use ocentra_app_game_core::app_game_policy_evaluator_runtime::types::{
    AppGamePolicyRuntimeAdapterDispatchState, AppGamePolicyRuntimeDecision,
    AppGamePolicyRuntimeDecisionReason, AppGamePolicyRuntimeDecisionState,
};
use ocentra_app_game_core::app_game_policy_target_compiler::references::{
    AppGamePolicyAuditRef, AppGamePolicyEvidenceRef, AppGamePolicyRuleRef,
};
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::policy::{ParentActorReference, ParentActorRole};
use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::AppGameTimerParentSurfaceChildUxLocalArtifactRecord;
use ocentra_parent_agent_protocol::enforcement::ParentActionReference;
use ocentra_parent_agent_protocol::notification_provider_status_boundary::{
    V08NotificationEscalationReadiness, V08NotificationProviderDeliveryClaim,
    V08NotificationProviderStatus, V08NotificationProviderStatusProofState,
    V08NotificationQuietHoursReadiness,
};
use ocentra_parent_agent_protocol::schema_domain_mirrors::family::{
    FamilyReference, ParentDevicePlatform, ParentDeviceReference,
};
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxDeliveryClaimState, NotificationLocalOutboxRecord,
    NotificationLocalOutboxSchedulerRecord, NotificationLocalOutboxSchedulerState,
    NotificationLocalOutboxSeverity, NotificationLocalOutboxState, V3NotificationProviderChannel,
};

macro_rules! preference_preflight_input {
    ($source:expr, $scheduler:expr) => {
        AppGameChildUxPreferencePreflightInput {
            scheduler_record: $scheduler,
            source_outbox_record: $source,
            preflight_row_id: "preference-preflight-row-1".into(),
            parent_preference_requirement_ref: "parent-preference-required-1".into(),
            notification_frequency_requirement_ref: "notification-frequency-required-1".into(),
            quiet_hours_requirement_ref: "quiet-hours-required-1".into(),
        }
    };
}

#[test]
fn deliverable_child_ux_record_persists_and_reopens_without_private_payload(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = test_directory("reopen");
    let route = build_app_game_child_ux_outbox_route(outbox_input())?;
    let AppGameChildUxOutboxRoute::Queued(record) = route else {
        return Err(std::io::Error::other("deliverable notice must queue").into());
    };
    assert_eq!(record.state, NotificationLocalOutboxState::QueuedLocal);
    assert_eq!(
        record.envelope.severity,
        NotificationLocalOutboxSeverity::Attention
    );
    assert_eq!(
        record.envelope.provider_payload_preview.as_str(),
        "family-rule-new-app-approval"
    );
    assert!(record.envelope.sensitive_detail_minimized);
    assert!(!record.envelope.raw_child_evidence_included);
    assert!(!record.envelope.raw_url_or_title_included);
    assert!(!record.envelope.raw_message_text_included);
    assert!(!record.envelope.screenshot_or_report_included);
    assert!(!record.provider_delivery_attempted);

    let store = AppGameChildUxLocalOutboxStore::open(&directory)?;
    assert_eq!(
        store.persist((*record).clone())?,
        AppGameChildUxOutboxPersistResult::Inserted
    );
    let reopened = AppGameChildUxLocalOutboxStore::open(&directory)?;
    assert_eq!(reopened.records()?, vec![*record]);
    fs::remove_dir_all(directory)?;
    Ok(())
}

#[test]
fn exact_replay_is_idempotent_and_conflicting_entry_fails_closed(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = test_directory("idempotency");
    let AppGameChildUxOutboxRoute::Queued(record) =
        build_app_game_child_ux_outbox_route(outbox_input())?
    else {
        return Err(std::io::Error::other("deliverable notice must queue").into());
    };
    let store = AppGameChildUxLocalOutboxStore::open(&directory)?;
    assert_eq!(
        store.persist((*record).clone())?,
        AppGameChildUxOutboxPersistResult::Inserted
    );
    assert_eq!(
        store.persist((*record).clone())?,
        AppGameChildUxOutboxPersistResult::AlreadyPresent
    );
    let mut conflicting = *record;
    conflicting.retry_attempt_count = 1;
    let error = match store.persist(conflicting) {
        Err(error) => error,
        Ok(result) => {
            return Err(std::io::Error::other(format!(
                "conflicting entry unexpectedly persisted as {result:?}"
            ))
            .into());
        }
    };
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(store.records()?.len(), 1);
    fs::remove_dir_all(directory)?;
    Ok(())
}

#[test]
fn manual_required_child_ux_stays_blocked_out_of_the_store() {
    let mut input = outbox_input();
    input.notice.state = AppGameChildUxNoticeState::ManualRequired;
    let route = build_app_game_child_ux_outbox_route(input).expect_value("manual route");
    assert_eq!(
        route,
        AppGameChildUxOutboxRoute::Blocked {
            state: AppGameChildUxNoticeState::ManualRequired,
            blocked_reference_ids: vec!["child-reason-1".to_string(), "child-status-1".to_string(),],
        }
    );
}

#[test]
fn claimed_delivery_and_mismatched_artifact_refs_are_rejected() {
    let mut claimed = outbox_input();
    claimed.artifact.notification_delivery_claimed = true;
    assert_eq!(
        build_app_game_child_ux_outbox_route(claimed),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_outbox.artifact",
            value: "artifact-1".to_string(),
        })
    );

    let mut mismatched = outbox_input();
    mismatched.artifact.child_status_reference_ids = vec!["different-status".to_string()];
    assert_eq!(
        build_app_game_child_ux_outbox_route(mismatched),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_outbox.artifact",
            value: "artifact-1".to_string(),
        })
    );
}

#[test]
fn reopened_outbox_record_persists_as_due_local_scheduler_proof(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = test_directory("scheduler-reopen");
    let AppGameChildUxOutboxRoute::Queued(source) =
        build_app_game_child_ux_outbox_route(outbox_input())?
    else {
        return Err(std::io::Error::other("deliverable notice must queue").into());
    };
    let outbox_store = AppGameChildUxLocalOutboxStore::open(directory.join("outbox"))?;
    assert_eq!(
        outbox_store.persist((*source).clone())?,
        AppGameChildUxOutboxPersistResult::Inserted
    );
    let reopened_source = AppGameChildUxLocalOutboxStore::open(directory.join("outbox"))?
        .records()?
        .into_iter()
        .next()
        .ok_or_else(|| std::io::Error::other("reopened outbox record missing"))?;
    let AppGameChildUxSchedulerRoute::DueLocal(scheduled) =
        build_app_game_child_ux_scheduler_route(scheduler_input(reopened_source))?
    else {
        return Err(std::io::Error::other("queued record must schedule").into());
    };
    assert_eq!(
        scheduled.scheduler_state,
        NotificationLocalOutboxSchedulerState::DueLocal
    );
    assert_eq!(scheduled.source_entry_id.as_str(), "entry-1");
    assert_eq!(
        scheduled
            .next_attempt_at
            .as_ref()
            .map(|value| value.as_str()),
        Some("2026-08-15T00:01:00Z")
    );
    assert!(!scheduled.parent_owned_artifact_written);
    assert!(!scheduled.provider_delivery_attempted);
    assert!(!scheduled.production_durable_outbox_storage_claimed);

    let scheduler_store = AppGameChildUxSchedulerProofStore::open(directory.join("scheduler"))?;
    assert_eq!(
        scheduler_store.persist((*scheduled).clone())?,
        AppGameChildUxSchedulerPersistResult::Inserted
    );
    let persisted =
        AppGameChildUxSchedulerProofStore::open(directory.join("scheduler"))?.records()?;
    assert_eq!(persisted.len(), 1);
    assert!(persisted[0].parent_owned_artifact_written);
    assert_eq!(
        persisted[0].scheduler_entry_id,
        scheduled.scheduler_entry_id
    );
    fs::remove_dir_all(directory)?;
    Ok(())
}

#[test]
fn scheduler_exact_replay_is_idempotent_and_conflict_fails_closed(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = test_directory("scheduler-idempotency");
    let AppGameChildUxOutboxRoute::Queued(source) =
        build_app_game_child_ux_outbox_route(outbox_input())?
    else {
        return Err(std::io::Error::other("deliverable notice must queue").into());
    };
    let AppGameChildUxSchedulerRoute::DueLocal(scheduled) =
        build_app_game_child_ux_scheduler_route(scheduler_input(*source))?
    else {
        return Err(std::io::Error::other("queued record must schedule").into());
    };
    let store = AppGameChildUxSchedulerProofStore::open(&directory)?;
    assert_eq!(
        store.persist((*scheduled).clone())?,
        AppGameChildUxSchedulerPersistResult::Inserted
    );
    assert_eq!(
        store.persist((*scheduled).clone())?,
        AppGameChildUxSchedulerPersistResult::AlreadyPresent
    );
    let mut conflicting = *scheduled;
    conflicting.next_attempt_at = Some("2026-08-15T00:02:00Z".into());
    let error = match store.persist(conflicting) {
        Err(error) => error,
        Ok(result) => {
            return Err(std::io::Error::other(format!(
                "conflicting scheduler entry unexpectedly persisted as {result:?}"
            ))
            .into());
        }
    };
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(store.records()?.len(), 1);
    fs::remove_dir_all(directory)?;
    Ok(())
}

#[test]
fn scheduler_blocks_manual_state_and_rejects_unsafe_delivery_claim(
) -> Result<(), Box<dyn std::error::Error>> {
    let AppGameChildUxOutboxRoute::Queued(source) =
        build_app_game_child_ux_outbox_route(outbox_input())?
    else {
        return Err(std::io::Error::other("deliverable notice must queue").into());
    };
    let mut manual = (*source).clone();
    manual.state = NotificationLocalOutboxState::ManualRequired;
    manual.delivery_claim_state = NotificationLocalOutboxDeliveryClaimState::ManualRequired;
    manual.manual_action_required = true;
    assert_eq!(
        build_app_game_child_ux_scheduler_route(scheduler_input(manual)),
        Ok(AppGameChildUxSchedulerRoute::Blocked {
            source_entry_id: "entry-1".into(),
            source_state: NotificationLocalOutboxState::ManualRequired,
        })
    );

    let mut unsafe_source = *source;
    unsafe_source.provider_delivery_attempted = true;
    assert_eq!(
        build_app_game_child_ux_scheduler_route(scheduler_input(unsafe_source)),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_scheduler.source_record",
            value: "entry-1".to_string(),
        })
    );
    Ok(())
}

#[test]
fn persisted_due_local_row_requires_provider_adapter_credentials_and_smoke_proof(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = test_directory("provider-preflight");
    let (source, scheduler) = queued_source_and_scheduler()?;
    let scheduler_store = AppGameChildUxSchedulerProofStore::open(&directory)?;
    assert_eq!(
        scheduler_store.persist(scheduler)?,
        AppGameChildUxSchedulerPersistResult::Inserted
    );
    let persisted_scheduler = scheduler_store
        .records()?
        .into_iter()
        .next()
        .ok_or_else(|| std::io::Error::other("persisted scheduler row missing"))?;
    let row = build_app_game_child_ux_provider_preflight(provider_preflight_input(
        source,
        persisted_scheduler,
    ))?;
    assert_eq!(
        row.status,
        AppGameChildUxProviderPreflightStatus::ProviderAdapterRequired
    );
    assert_eq!(
        row.source_local_outbox_record_ref
            .as_ref()
            .map(|value| value.as_str()),
        Some("entry-1")
    );
    assert_eq!(row.adapter_requirement_refs.len(), 3);
    assert_eq!(row.manual_proof_requirements.len(), 3);
    assert_eq!(row.evidence_refs.len(), 1);
    assert!(!row.provider_delivery_runtime_claimed);
    assert!(!row.provider_credentials_claimed);
    assert!(!row.adapter_dispatch_claimed);
    fs::remove_dir_all(directory)?;
    Ok(())
}

#[test]
fn non_due_scheduler_rows_remain_manual_or_unavailable() -> Result<(), Box<dyn std::error::Error>> {
    let (mut manual_source, mut manual_scheduler) = queued_source_and_scheduler()?;
    manual_source.state = NotificationLocalOutboxState::ManualRequired;
    manual_source.delivery_claim_state = NotificationLocalOutboxDeliveryClaimState::ManualRequired;
    manual_source.manual_action_required = true;
    manual_scheduler.source_state = NotificationLocalOutboxState::ManualRequired;
    manual_scheduler.scheduler_state = NotificationLocalOutboxSchedulerState::ManualRequired;
    let manual = build_app_game_child_ux_provider_preflight(provider_preflight_input(
        manual_source,
        manual_scheduler,
    ))?;
    assert_eq!(
        manual.status,
        AppGameChildUxProviderPreflightStatus::ManualRequired
    );
    assert!(manual.source_local_outbox_record_ref.is_none());
    assert!(manual.provider_channel.is_none());

    let (mut unavailable_source, mut unavailable_scheduler) = queued_source_and_scheduler()?;
    unavailable_source.state = NotificationLocalOutboxState::DeadLettered;
    unavailable_scheduler.source_state = NotificationLocalOutboxState::DeadLettered;
    unavailable_scheduler.scheduler_state = NotificationLocalOutboxSchedulerState::DeadLetterReview;
    let unavailable = build_app_game_child_ux_provider_preflight(provider_preflight_input(
        unavailable_source,
        unavailable_scheduler,
    ))?;
    assert_eq!(
        unavailable.status,
        AppGameChildUxProviderPreflightStatus::Unavailable
    );
    assert!(unavailable.source_outbox_file_ref.is_none());
    assert_eq!(unavailable.manual_proof_requirements.len(), 3);
    Ok(())
}

#[test]
fn provider_preflight_rejects_unpersisted_mismatched_and_claimed_rows(
) -> Result<(), Box<dyn std::error::Error>> {
    let (source, mut scheduler) = queued_source_and_scheduler()?;
    scheduler.parent_owned_artifact_written = false;
    assert_eq!(
        build_app_game_child_ux_provider_preflight(provider_preflight_input(
            source.clone(),
            scheduler,
        )),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_provider_preflight.source",
            value: "scheduler-entry-1".to_string(),
        })
    );

    let (_, mut mismatched) = queued_source_and_scheduler()?;
    mismatched.source_entry_id = "different-entry".into();
    assert_eq!(
        build_app_game_child_ux_provider_preflight(provider_preflight_input(
            source.clone(),
            mismatched,
        )),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_provider_preflight.source",
            value: "scheduler-entry-1".to_string(),
        })
    );

    let (_, mut claimed) = queued_source_and_scheduler()?;
    claimed.provider_delivery_attempted = true;
    assert_eq!(
        build_app_game_child_ux_provider_preflight(provider_preflight_input(source, claimed)),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_provider_preflight.source",
            value: "scheduler-entry-1".to_string(),
        })
    );
    Ok(())
}

#[test]
fn provider_required_preflight_maps_to_unclaimed_manual_v08_status(
) -> Result<(), Box<dyn std::error::Error>> {
    let preflight = provider_required_preflight()?;
    let row = build_app_game_child_ux_provider_status_handoff(provider_status_input(preflight))?;
    let entry = &row.provider_status_boundary_entry;
    assert_eq!(
        row.source_preflight_status,
        AppGameChildUxProviderPreflightStatus::ProviderAdapterRequired
    );
    assert_eq!(
        entry.provider_status,
        V08NotificationProviderStatus::ManualRequired
    );
    assert_eq!(
        entry.status_proof_state,
        V08NotificationProviderStatusProofState::ManualActionRequired
    );
    assert_eq!(
        entry.quiet_hours_readiness,
        V08NotificationQuietHoursReadiness::ManualRequired
    );
    assert_eq!(
        entry.escalation_readiness,
        V08NotificationEscalationReadiness::ManualRequired
    );
    assert_eq!(
        entry.delivery_claim_state,
        V08NotificationProviderDeliveryClaim::NotObserved
    );
    assert_eq!(entry.readiness_refs.len(), 3);
    assert_eq!(entry.manual_proof_requirements.len(), 3);
    assert!(entry.provider_receipt_refs.is_empty());
    assert!(!entry.provider_delivery_implemented);
    assert!(!row.provider_delivery_runtime_claimed);
    Ok(())
}

#[test]
fn unavailable_preflight_maps_to_unavailable_v08_status() -> Result<(), Box<dyn std::error::Error>>
{
    let (mut source, mut scheduler) = queued_source_and_scheduler()?;
    source.state = NotificationLocalOutboxState::DeadLettered;
    scheduler.source_state = NotificationLocalOutboxState::DeadLettered;
    scheduler.scheduler_state = NotificationLocalOutboxSchedulerState::DeadLetterReview;
    let preflight =
        build_app_game_child_ux_provider_preflight(provider_preflight_input(source, scheduler))?;
    let row = build_app_game_child_ux_provider_status_handoff(provider_status_input(preflight))?;
    let entry = &row.provider_status_boundary_entry;
    assert_eq!(
        entry.provider_status,
        V08NotificationProviderStatus::Unavailable
    );
    assert_eq!(
        entry.status_proof_state,
        V08NotificationProviderStatusProofState::ProviderUnavailableContract
    );
    assert_eq!(
        entry.delivery_claim_state,
        V08NotificationProviderDeliveryClaim::NotImplemented
    );
    assert!(row.source_local_outbox_record_ref.is_none());
    assert!(row.source_provider_channel.is_none());
    Ok(())
}

#[test]
fn provider_status_rejects_claimed_preflight_and_missing_preference_refs(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut claimed = provider_required_preflight()?;
    claimed.provider_delivery_runtime_claimed = true;
    assert_eq!(
        build_app_game_child_ux_provider_status_handoff(provider_status_input(claimed)),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_provider_status.source",
            value: "provider-preflight-row-1".to_string(),
        })
    );

    let mut missing_context = provider_status_input(provider_required_preflight()?);
    missing_context.preference_refs.clear();
    assert_eq!(
        build_app_game_child_ux_provider_status_handoff(missing_context),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_provider_status.context",
            value: "provider-status-handoff-row-1".to_string(),
        })
    );
    Ok(())
}

#[test]
fn preference_preflight_routes_due_manual_and_unavailable_without_runtime_claims(
) -> Result<(), Box<dyn std::error::Error>> {
    let (source, scheduler) = queued_source_and_scheduler()?;
    let row = build_app_game_child_ux_preference_preflight(preference_preflight_input!(
        source, scheduler
    ))?;
    assert_eq!(
        row.status,
        AppGameChildUxPreferencePreflightStatus::ParentPreferenceRequired
    );
    assert_eq!(
        row.source_local_outbox_record_ref
            .as_ref()
            .map(|reference| reference.as_str()),
        Some("entry-1")
    );
    assert_eq!(row.parent_preference_requirement_refs.len(), 1);
    assert_eq!(row.notification_frequency_requirement_refs.len(), 1);
    assert_eq!(row.quiet_hours_requirement_refs.len(), 1);
    assert_eq!(row.manual_proof_requirements.len(), 3);
    assert!(!row.parent_preference_mutation_runtime_claimed);
    assert!(!row.parent_frequency_control_ui_claimed);
    assert!(!row.quiet_hours_timer_runtime_claimed);
    assert!(!row.provider_delivery_runtime_claimed);
    assert!(!row.child_delivery_claimed);

    let (source, mut scheduler) = queued_source_and_scheduler()?;
    scheduler.scheduler_state = NotificationLocalOutboxSchedulerState::ManualRequired;
    let manual = build_app_game_child_ux_preference_preflight(preference_preflight_input!(
        source, scheduler
    ))?;
    assert_eq!(
        manual.status,
        AppGameChildUxPreferencePreflightStatus::ManualRequired
    );
    assert!(manual.source_local_outbox_record_ref.is_none());
    assert!(manual.parent_preference_requirement_refs.is_empty());
    assert_eq!(manual.manual_proof_requirements.len(), 3);

    let (mut source, mut scheduler) = queued_source_and_scheduler()?;
    source.state = NotificationLocalOutboxState::DeadLettered;
    scheduler.source_state = NotificationLocalOutboxState::DeadLettered;
    scheduler.scheduler_state = NotificationLocalOutboxSchedulerState::DeadLetterReview;
    let unavailable = build_app_game_child_ux_preference_preflight(preference_preflight_input!(
        source, scheduler
    ))?;
    assert_eq!(
        unavailable.status,
        AppGameChildUxPreferencePreflightStatus::Unavailable
    );
    assert_eq!(
        unavailable.provider_channel,
        Some(V3NotificationProviderChannel::InApp)
    );
    assert!(unavailable.quiet_hours_requirement_refs.is_empty());
    Ok(())
}

#[test]
fn preference_preflight_rejects_unpersisted_mismatched_claimed_and_duplicate_requirements(
) -> Result<(), Box<dyn std::error::Error>> {
    let (source, mut scheduler) = queued_source_and_scheduler()?;
    scheduler.parent_owned_artifact_written = false;
    assert_eq!(
        build_app_game_child_ux_preference_preflight(preference_preflight_input!(
            source, scheduler
        )),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_preference_preflight.source",
            value: "scheduler-entry-1".to_string(),
        })
    );

    let (source, mut scheduler) = queued_source_and_scheduler()?;
    scheduler.source_entry_id = "different-entry".into();
    assert_eq!(
        build_app_game_child_ux_preference_preflight(preference_preflight_input!(
            source, scheduler
        )),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_preference_preflight.source",
            value: "scheduler-entry-1".to_string(),
        })
    );

    let (source, mut scheduler) = queued_source_and_scheduler()?;
    scheduler.parent_notification_ui_claimed = true;
    assert_eq!(
        build_app_game_child_ux_preference_preflight(preference_preflight_input!(
            source, scheduler
        )),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_preference_preflight.source",
            value: "scheduler-entry-1".to_string(),
        })
    );

    let (source, scheduler) = queued_source_and_scheduler()?;
    let mut duplicate = preference_preflight_input!(source, scheduler);
    duplicate.quiet_hours_requirement_ref = duplicate.parent_preference_requirement_ref.clone();
    assert_eq!(
        build_app_game_child_ux_preference_preflight(duplicate),
        Err(ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.child_ux_preference_preflight.requirements",
            value: "preference-preflight-row-1".to_string(),
        })
    );
    Ok(())
}

fn provider_required_preflight(
) -> Result<AppGameChildUxProviderPreflightRow, Box<dyn std::error::Error>> {
    let (source, scheduler) = queued_source_and_scheduler()?;
    Ok(build_app_game_child_ux_provider_preflight(
        provider_preflight_input(source, scheduler),
    )?)
}

fn provider_status_input(
    preflight_row: AppGameChildUxProviderPreflightRow,
) -> AppGameChildUxProviderStatusInput {
    AppGameChildUxProviderStatusInput {
        preflight_row,
        handoff_row_id: "provider-status-handoff-row-1".into(),
        status_entry_id: "provider-status-entry-1".into(),
        notification_intent_ref: "notification-intent-1".into(),
        notification_status_ref: "notification-status-1".into(),
        provider_attempt_ref: "provider-attempt-not-started-1".into(),
        preference_refs: vec!["provider-preference-manual-review-1".into()],
        last_checked_at: "2026-08-15T00:02:00Z".to_string(),
    }
}

fn queued_source_and_scheduler() -> Result<
    (
        NotificationLocalOutboxRecord,
        NotificationLocalOutboxSchedulerRecord,
    ),
    Box<dyn std::error::Error>,
> {
    let AppGameChildUxOutboxRoute::Queued(source) =
        build_app_game_child_ux_outbox_route(outbox_input())?
    else {
        return Err(std::io::Error::other("deliverable notice must queue").into());
    };
    let AppGameChildUxSchedulerRoute::DueLocal(scheduler) =
        build_app_game_child_ux_scheduler_route(scheduler_input((*source).clone()))?
    else {
        return Err(std::io::Error::other("queued record must schedule").into());
    };
    let mut scheduler = *scheduler;
    scheduler.parent_owned_artifact_written = true;
    Ok((*source, scheduler))
}

fn provider_preflight_input(
    source_outbox_record: NotificationLocalOutboxRecord,
    scheduler_record: NotificationLocalOutboxSchedulerRecord,
) -> AppGameChildUxProviderPreflightInput {
    AppGameChildUxProviderPreflightInput {
        scheduler_record,
        source_outbox_record,
        preflight_row_id: "provider-preflight-row-1".into(),
        adapter_requirement_ref: "provider-adapter-required-1".into(),
        credential_requirement_ref: "provider-credentials-required-1".into(),
        smoke_proof_requirement_ref: "provider-smoke-proof-required-1".into(),
    }
}

fn scheduler_input(source_record: NotificationLocalOutboxRecord) -> AppGameChildUxSchedulerInput {
    AppGameChildUxSchedulerInput {
        source_record,
        scheduler_entry_id: "scheduler-entry-1".into(),
        scheduler_decision_ref: "scheduler-decision-1".into(),
        scheduler_artifact_ref: "scheduler-artifact-1".into(),
        scheduler_now_at: "2026-08-15T00:01:00Z".into(),
        scheduler_payload_preview: "family-rule-new-app-approval".into(),
    }
}

fn outbox_input() -> AppGameChildUxOutboxInput {
    let notice = build_app_game_child_ux_notice(AppGameChildUxInput {
        subject_kind: AppGameChildUxSubjectKind::App,
        runtime_decision: runtime_decision(),
        request_state: AppGameChildUxRequestState::ApprovalNeeded,
        capability_state: AppGameChildUxCapabilityState::Available,
        policy_rule_ref: AppGamePolicyRuleRef::parse("rule-1").expect_value("rule ref"),
        evidence_refs: vec![AppGamePolicyEvidenceRef::parse("evidence-1").expect_value("evidence")],
        child_reason_refs: vec![
            AppGameChildReasonRef::parse("child-reason-1").expect_value("reason")
        ],
        child_status_refs: vec![
            AppGameChildStatusRef::parse("child-status-1").expect_value("status")
        ],
        adapter_action_ref: None,
    })
    .expect_value("child UX notice");
    AppGameChildUxOutboxInput {
        artifact: artifact(),
        notice,
        entry_id: "entry-1".into(),
        alert_ref: "alert-1".into(),
        family: FamilyReference {
            family_id: "family-1".to_string(),
        },
        device: ParentDeviceReference {
            device_id: "device-1".into(),
            child_profile_id: Some("child-1".into()),
            label: "child-device".to_string(),
            platform: ParentDevicePlatform::Windows,
        },
        parent_action: ParentActionReference {
            action_reference_id: "action-1".to_string(),
            actor: ParentActorReference {
                actor_id: "parent-1".to_string(),
                role: ParentActorRole::Parent,
            },
            policy_version: "policy-v1".to_string(),
            created_at: "2026-08-15T00:00:00Z".to_string(),
        },
        provider_channel: V3NotificationProviderChannel::InApp,
        observed_at: "2026-08-15T00:00:00Z".into(),
        audit_refs: vec!["audit-1".into()],
        outbox_file_ref: "outbox-file-1".into(),
        local_data_path_ref: "local-data-1".into(),
    }
}

fn artifact() -> AppGameTimerParentSurfaceChildUxLocalArtifactRecord {
    AppGameTimerParentSurfaceChildUxLocalArtifactRecord {
        schema_version: 1,
        artifact_reference_id: "artifact-1".to_string(),
        source_result_id: "result-1".to_string(),
        target_domain: "native-app".to_string(),
        child_reason_reference_ids: vec!["child-reason-1".to_string()],
        child_status_reference_ids: vec!["child-status-1".to_string()],
        child_delivery_claimed: false,
        notification_delivery_claimed: false,
        adapter_dispatch_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
    }
}

fn runtime_decision() -> AppGamePolicyRuntimeDecision {
    AppGamePolicyRuntimeDecision {
        state: AppGamePolicyRuntimeDecisionState::AskParent,
        reason: AppGamePolicyRuntimeDecisionReason::WithinBudget,
        consumed_seconds: 0,
        effective_budget_seconds: 300,
        remaining_seconds: 300,
        counted_session_refs: Vec::new(),
        excluded_session_refs: Vec::new(),
        timer_ref: None,
        bonus_approval_ref: None,
        audit_ref: AppGamePolicyAuditRef::parse("audit-runtime-1").expect_value("audit ref"),
        adapter_dispatch_state: AppGamePolicyRuntimeAdapterDispatchState::NotDispatched,
    }
}

fn test_directory(label: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/app-game-child-ux-outbox-tests")
        .join(format!("{label}-{}-{unique}", std::process::id()))
}
