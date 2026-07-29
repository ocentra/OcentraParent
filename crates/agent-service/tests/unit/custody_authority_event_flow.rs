use std::fs;

use ocentra_child_runtime::{
    child_runtime_tombstone_event_flow::ChildRuntimeTombstoneEventFlow,
    runtime_gate_tombstone::{
        ChildRuntimeTombstoneMilestone, ChildRuntimeTombstonePublicationOutcome,
    },
};
use ocentra_eventing::{
    envelope::{EventMetadata, EventSource},
    ids::{
        CorrelationId, EventCustody, RuntimeInstanceId, RuntimeRole, SourceComponent, SourceService,
    },
    journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions},
};
use ocentra_family_identity_core::{
    family_identity::{
        ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
        HouseholdMembershipState, HouseholdRole, SessionFreshnessState,
    },
    household_authority::{
        HouseholdAuthorityAction, HouseholdAuthorityInput, HouseholdAuthorizationFailureReason,
        ParentStepUpAssertionSnapshot,
    },
    parent_presence::{
        ParentPresenceChallenge, ParentPresenceVerificationFailureReason,
        ParentPresenceVerificationInput, ParentPresenceVerificationPort,
    },
};
use ocentra_parent_agent_service::{
    custody_authority_event_flow::{
        publish_authorized_custody_delete, AuthorizedCustodyDeleteCommand,
        AuthorizedCustodyDeleteError,
    },
    custody_tombstone_runtime::{
        acknowledge_consumed_tombstone_action, recover_pending_tombstone_actions,
        TombstoneDeletionRef, TombstoneJournalPath, TombstoneStoreDirectory,
    },
};
use ocentra_storage_custody_core::{
    retention_delete_tombstone_store::RetentionDeleteTombstoneStore,
    storage_custody::{
        ParentExportState, RemoteSyncState, RetentionWindowState, StorageCustodyAggregateId,
        StorageCustodyDecisionId, StorageCustodyInput, StorageCustodyLocation,
    },
};

#[tokio::test]
async fn authorized_parent_delete_persists_before_journal_and_terminal_replay_does_not_resurrect(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = temp_directory("authorized")?;
    let journal_path = directory.join("custody.ndjson");
    let flow = flow(&directory, &journal_path)?;
    let command = command("authorized", expired_input())?;
    let mut parent_presence = parent_presence(&directory, &command)?;

    let outcome =
        match publish_authorized_custody_delete(&flow, &mut parent_presence, command.clone()).await
        {
            Ok(outcome) => outcome,
            Err(error) => return Err(format!("authorized delete failed: {error:?}").into()),
        };
    let ChildRuntimeTombstonePublicationOutcome::Journaled(report) = outcome else {
        return Err("authorized delete must journal".into());
    };
    assert_eq!(report.correlation_id.as_str(), "custody-authorized");
    assert_eq!(
        report.milestones,
        vec![
            ChildRuntimeTombstoneMilestone::DurableOutboxWritten,
            ChildRuntimeTombstoneMilestone::JournalAppendConfirmed,
        ]
    );
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    assert_eq!(store.records()?.len(), 1);
    assert!(store.records()?[0].terminal_pending);

    let recovery = recover_pending_tombstone_actions(
        TombstoneStoreDirectory::from(directory.clone()),
        TombstoneJournalPath::from(journal_path.clone()),
    )
    .await;
    assert_eq!(recovery.recovered_count, 1);
    assert!(RetentionDeleteTombstoneStore::open(&directory)?.records()?[0].terminal_pending);

    let mut command = command;
    command.metadata = metadata("authorized-retry")?;
    let repeated = publish_authorized_custody_delete(&flow, &mut parent_presence, command).await;
    assert_eq!(
        repeated,
        Err(AuthorizedCustodyDeleteError::ParentPresenceRejected(
            ParentPresenceVerificationFailureReason::ReplayRejected
        ))
    );
    acknowledge_consumed_tombstone_action(
        TombstoneStoreDirectory::from(directory.clone()),
        TombstoneDeletionRef::from("storage-custody-delete:decision-authorized".to_owned()),
    )
    .await?;
    let records = RetentionDeleteTombstoneStore::open(&directory)?.records()?;
    assert_eq!(records.len(), 1);
    assert!(!records[0].terminal_pending);
    let _ = fs::remove_dir_all(&directory);
    Ok(())
}

#[tokio::test]
async fn journal_failure_keeps_authorized_action_pending_until_restart_replay(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = temp_directory("pending-retry")?;
    let journal_path = directory.join("custody.ndjson");
    let journal =
        NdjsonEventJournal::with_options(&journal_path, NdjsonJournalOptions::hash_chain());
    journal.inject_next_sync_failure_for_debug();
    let flow = ChildRuntimeTombstoneEventFlow::new(
        journal,
        RetentionDeleteTombstoneStore::open(&directory)?,
    );
    let command = command("pending-retry", expired_input())?;
    let mut parent_presence = parent_presence(&directory, &command)?;

    let outcome =
        match publish_authorized_custody_delete(&flow, &mut parent_presence, command).await {
            Ok(outcome) => outcome,
            Err(error) => return Err(format!("pending retry delete failed: {error:?}").into()),
        };
    let ChildRuntimeTombstonePublicationOutcome::PendingJournalRetry(report) = outcome else {
        return Err("failed journal must remain pending".into());
    };
    assert_eq!(report.correlation_id.as_str(), "custody-pending-retry");
    assert_eq!(
        report.milestones,
        vec![
            ChildRuntimeTombstoneMilestone::DurableOutboxWritten,
            ChildRuntimeTombstoneMilestone::JournalAppendPendingRetry,
        ]
    );
    assert!(RetentionDeleteTombstoneStore::open(&directory)?.records()?[0].terminal_pending);

    let recovery = recover_pending_tombstone_actions(
        TombstoneStoreDirectory::from(directory.clone()),
        TombstoneJournalPath::from(journal_path),
    )
    .await;
    assert_eq!(recovery.recovered_count, 1);
    assert_eq!(recovery.failed_count, 0);
    assert!(RetentionDeleteTombstoneStore::open(&directory)?.records()?[0].terminal_pending);
    let _ = fs::remove_dir_all(&directory);
    Ok(())
}

#[tokio::test]
async fn untrusted_parent_or_non_delete_input_is_rejected_before_an_outbox_intent_exists(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = temp_directory("rejected")?;
    let journal_path = directory.join("custody.ndjson");
    let flow = flow(&directory, &journal_path)?;
    let mut unauthorized = command("unauthorized", expired_input())?;
    let mut unauthorized_parent_presence = parent_presence(&directory, &unauthorized)?;
    unauthorized.authority.actor_role = HouseholdRole::CoParentGuardian;
    let rejection =
        publish_authorized_custody_delete(&flow, &mut unauthorized_parent_presence, unauthorized)
            .await;
    assert_eq!(
        rejection,
        Err(AuthorizedCustodyDeleteError::HouseholdAuthorityRejected(
            HouseholdAuthorizationFailureReason::RoleNotAuthorized
        ))
    );
    assert!(RetentionDeleteTombstoneStore::open(&directory)?
        .records()?
        .is_empty());

    let mut malformed = command("malformed-step-up", expired_input())?;
    let mut malformed_parent_presence = parent_presence(&directory, &malformed)?;
    malformed.parent_presence.assertion.nonce = "different-nonce".to_owned();
    let malformed_rejection =
        publish_authorized_custody_delete(&flow, &mut malformed_parent_presence, malformed).await;
    assert_eq!(
        malformed_rejection,
        Err(AuthorizedCustodyDeleteError::ParentPresenceRejected(
            ParentPresenceVerificationFailureReason::NonceMismatch
        ))
    );
    assert!(RetentionDeleteTombstoneStore::open(&directory)?
        .records()?
        .is_empty());

    let non_delete_command = command("active", active_input())?;
    let mut non_delete_parent_presence = parent_presence(&directory, &non_delete_command)?;
    let non_delete = publish_authorized_custody_delete(
        &flow,
        &mut non_delete_parent_presence,
        non_delete_command.clone(),
    )
    .await;
    assert_eq!(
        non_delete,
        Err(AuthorizedCustodyDeleteError::CustodyActionIsNotDelete)
    );
    assert!(RetentionDeleteTombstoneStore::open(&directory)?
        .records()?
        .is_empty());

    // The rejected active-window request must not burn the one-time challenge:
    // the same parent authority can correct the retention input and submit the
    // intended deletion without redoing step-up.
    let mut corrected = command("active", expired_input())?;
    corrected.parent_presence = non_delete_command.parent_presence;
    let corrected_outcome =
        publish_authorized_custody_delete(&flow, &mut non_delete_parent_presence, corrected).await;
    assert!(matches!(
        corrected_outcome,
        Ok(ChildRuntimeTombstonePublicationOutcome::Journaled(_))
    ));
    let _ = fs::remove_dir_all(&directory);
    Ok(())
}

fn flow(
    directory: &std::path::Path,
    journal_path: &std::path::Path,
) -> Result<ChildRuntimeTombstoneEventFlow, Box<dyn std::error::Error>> {
    Ok(ChildRuntimeTombstoneEventFlow::new(
        NdjsonEventJournal::with_options(journal_path, NdjsonJournalOptions::hash_chain()),
        RetentionDeleteTombstoneStore::open(directory)?,
    ))
}

fn command(
    id: &str,
    custody_input: StorageCustodyInput,
) -> Result<AuthorizedCustodyDeleteCommand, Box<dyn std::error::Error>> {
    Ok(AuthorizedCustodyDeleteCommand {
        authority: HouseholdAuthorityInput {
            actor_role: HouseholdRole::ParentOwner,
            same_family: true,
            actor_account_state: ActorAccountState::Active,
            membership_state: HouseholdMembershipState::Active,
            child_profile_binding_state: ChildProfileBindingState::Bound,
            device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
            device_trust_state: DeviceTrustState::Trusted,
            session_freshness_state: SessionFreshnessState::Fresh,
            capability_granted: true,
            controller_lease_state: None,
            action: HouseholdAuthorityAction::ExportDeleteData,
        },
        parent_presence: ParentPresenceVerificationInput {
            correlation_id: CorrelationId::parse(format!("custody-{id}"))?,
            challenge_ref: format!("challenge-{id}"),
            assertion: ParentStepUpAssertionSnapshot {
                family_id: "family-main".to_owned(),
                parent_account_id: "parent-account-1".to_owned(),
                action_device_id: "device-parent-1".to_owned(),
                action_device_child_profile_id: None,
                target_child_profile_id: Some("child-1".to_owned()),
                action: HouseholdAuthorityAction::ExportDeleteData,
                nonce: format!("nonce-{id}"),
                expires_at: "2026-06-13T16:01:00.000Z".to_owned(),
            },
        },
        aggregate_id: StorageCustodyAggregateId::parse(format!("family-{id}"))?,
        decision_id: StorageCustodyDecisionId::parse(format!("decision-{id}"))?,
        custody_input,
        metadata: metadata(id)?,
    })
}

fn parent_presence(
    directory: &std::path::Path,
    command: &AuthorizedCustodyDeleteCommand,
) -> Result<ParentPresenceVerificationPort, Box<dyn std::error::Error>> {
    let mut port = ParentPresenceVerificationPort::open_unsealed_test_custody_at(
        directory.join(format!("presence-{}.sqlite", command.decision_id.as_str())),
        "2026-06-13T15:58:00.000Z",
    )
    .map_err(|error| format!("parent-presence test custody unavailable: {error:?}"))?;
    port.issue_challenge(ParentPresenceChallenge {
        challenge_ref: command.parent_presence.challenge_ref.clone(),
        nonce_ref: command.parent_presence.assertion.nonce.clone(),
        family_id: command.parent_presence.assertion.family_id.clone(),
        parent_account_id: command.parent_presence.assertion.parent_account_id.clone(),
        privileged_action: command.parent_presence.assertion.action,
        action_device_id: command.parent_presence.assertion.action_device_id.clone(),
        action_device_child_profile_id: command
            .parent_presence
            .assertion
            .action_device_child_profile_id
            .clone(),
        target_child_profile_id: command
            .parent_presence
            .assertion
            .target_child_profile_id
            .clone(),
        expires_at: command.parent_presence.assertion.expires_at.clone(),
    })
    .map_err(|error| format!("parent-presence challenge issuance failed: {error:?}"))?;
    Ok(port)
}

fn metadata(id: &str) -> Result<EventMetadata, Box<dyn std::error::Error>> {
    Ok(EventMetadata::new(
        CorrelationId::parse(format!("custody-{id}"))?,
        EventSource::new(
            EventCustody::parse("local-journal")?,
            RuntimeRole::parse("controller")?,
            SourceService::parse("agent-service")?,
            SourceComponent::parse("custody-authority-event-flow")?,
            RuntimeInstanceId::parse("service-test")?,
        ),
    ))
}

fn expired_input() -> StorageCustodyInput {
    StorageCustodyInput {
        location: StorageCustodyLocation::ParentDeviceLocal,
        retention_window_state: RetentionWindowState::Expired,
        parent_export_state: ParentExportState::NotRequested,
        remote_sync_state: RemoteSyncState::Disabled,
    }
}

fn active_input() -> StorageCustodyInput {
    StorageCustodyInput {
        retention_window_state: RetentionWindowState::Active,
        ..expired_input()
    }
}

fn temp_directory(name: &str) -> Result<std::path::PathBuf, std::io::Error> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-agent-service-custody-authority-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory)?;
    Ok(directory)
}
