use std::{fs, path::PathBuf, time::SystemTime};

use ocentra_app_game_core::app_game_child_ux_scheduler_store::AppGameChildUxSchedulerProofStore;
use ocentra_app_game_core::app_game_child_ux_scheduler_types::AppGameChildUxSchedulerPersistResult;
use ocentra_app_game_core::app_game_notification_local_outbox_bridge::build_app_game_notification_local_outbox_bridge;
use ocentra_app_game_core::app_game_notification_local_outbox_bridge_types::AppGameNotificationLocalOutboxBridgeOptions;
use ocentra_app_game_core::app_game_notification_scheduler_bridge::{
    build_app_game_notification_scheduler_bridge, parse_app_game_notification_scheduler_jsonl,
    persist_app_game_notification_scheduler_bridge,
    serialize_app_game_notification_scheduler_jsonl,
};
use ocentra_app_game_core::app_game_notification_scheduler_bridge_types::{
    AppGameNotificationSchedulerBridgeOptions, AppGameNotificationSchedulerBridgeStatus,
};
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::policy::{ParentActorReference, ParentActorRole};
use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_notification_readiness::{
    AppGameNotificationReadinessRow,
    APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_MANUAL_REQUIRED,
    APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT,
    APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_UNAVAILABLE,
    APP_GAME_NOTIFICATION_READINESS_REASON_CAPABILITY_UNAVAILABLE,
    APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED,
    APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED,
    APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED,
    APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
    APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE,
};
use ocentra_parent_agent_protocol::enforcement::ParentActionReference;
use ocentra_parent_agent_protocol::schema_domain_mirrors::family::{
    FamilyReference, ParentDevicePlatform, ParentDeviceReference,
};
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxSchedulerState, V3NotificationProviderChannel,
};
use serde_json::json;

#[test]
fn scheduler_bridge_consumes_wp58_and_keeps_blocked_rows_unscheduled(
) -> Result<(), Box<dyn std::error::Error>> {
    let source = source_bridge()?;
    let model = build_app_game_notification_scheduler_bridge(scheduler_options(), source)?;
    assert_eq!(model.scheduled_count, 1);
    assert_eq!(model.manual_required_count, 1);
    assert_eq!(model.unavailable_count, 1);
    assert_eq!(
        model.rows[0].status,
        AppGameNotificationSchedulerBridgeStatus::Scheduled
    );
    assert_eq!(
        model.rows[0]
            .scheduler_record
            .as_ref()
            .expect_value("scheduled record")
            .scheduler_state,
        NotificationLocalOutboxSchedulerState::DueLocal
    );
    assert_eq!(
        model.rows[0]
            .source_outbox_record
            .as_ref()
            .expect_value("scheduled source outbox record")
            .entry_id,
        model.rows[0]
            .source_entry_id
            .clone()
            .expect_value("scheduled source entry id")
    );
    assert_eq!(
        model.rows[1].status,
        AppGameNotificationSchedulerBridgeStatus::ManualRequired
    );
    assert_eq!(
        model.rows[1].blocked_reason_refs,
        vec![APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_MANUAL_REQUIRED.into()]
    );
    assert_eq!(
        model.rows[2].status,
        AppGameNotificationSchedulerBridgeStatus::Unavailable
    );
    assert_eq!(model.rows[1].scheduler_record, None);
    assert_eq!(model.rows[2].scheduler_record, None);
    assert_eq!(model.rows[1].source_outbox_record, None);
    assert_eq!(model.rows[2].source_outbox_record, None);
    assert!(!model.provider_delivery_runtime_claimed);
    assert!(!model.provider_receipt_ingestion_claimed);
    assert!(!model.retry_worker_runtime_claimed);
    assert!(!model.quiet_hours_timer_runtime_claimed);
    assert!(!model.production_durable_outbox_storage_claimed);
    Ok(())
}

#[test]
fn scheduler_records_round_trip_persist_reopen_and_replay_idempotently(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = test_directory("persist");
    let model =
        build_app_game_notification_scheduler_bridge(scheduler_options(), source_bridge()?)?;
    let first_jsonl = serialize_app_game_notification_scheduler_jsonl(&model)?;
    let second_jsonl = serialize_app_game_notification_scheduler_jsonl(&model)?;
    assert_eq!(first_jsonl, second_jsonl);
    assert_eq!(
        parse_app_game_notification_scheduler_jsonl(&first_jsonl)?.len(),
        1
    );

    let store = AppGameChildUxSchedulerProofStore::open(&directory)?;
    assert_eq!(
        persist_app_game_notification_scheduler_bridge(&store, &model)?,
        vec![AppGameChildUxSchedulerPersistResult::Inserted]
    );
    let reopened = AppGameChildUxSchedulerProofStore::open(&directory)?;
    assert_eq!(reopened.records()?.len(), 1);
    assert_eq!(
        persist_app_game_notification_scheduler_bridge(&reopened, &model)?,
        vec![AppGameChildUxSchedulerPersistResult::AlreadyPresent]
    );
    let mut conflicting = model;
    conflicting.rows[0]
        .scheduler_record
        .as_mut()
        .expect_value("scheduled record")
        .manual_action_required = true;
    let error = persist_app_game_notification_scheduler_bridge(&reopened, &conflicting)
        .err()
        .expect_value("conflicting scheduler identity must fail");
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    fs::remove_dir_all(directory)?;
    Ok(())
}

#[test]
fn scheduler_bridge_rejects_tampered_wp58_counts_claims_and_identities(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut bad_count = source_bridge()?;
    bad_count.linked_record_count = 9;
    assert_source_rejected(bad_count);

    let mut claimed = source_bridge()?;
    claimed.provider_delivery_runtime_claimed = true;
    assert_source_rejected(claimed);

    let mut duplicate = source_bridge()?;
    duplicate.rows[1].bridge_record_id = duplicate.rows[0].bridge_record_id.clone();
    assert_source_rejected(duplicate);
    Ok(())
}

#[test]
fn scheduler_jsonl_rejects_unsafe_claims_and_non_due_state(
) -> Result<(), Box<dyn std::error::Error>> {
    let model =
        build_app_game_notification_scheduler_bridge(scheduler_options(), source_bridge()?)?;
    let mut unsafe_model = model.clone();
    unsafe_model.rows[0]
        .scheduler_record
        .as_mut()
        .expect_value("scheduled record")
        .provider_delivery_observed = true;
    let error = serialize_app_game_notification_scheduler_jsonl(&unsafe_model)
        .err()
        .expect_value("unsafe scheduler claim must fail serialization");
    assert!(error
        .to_string()
        .contains("app_game.child_ux_scheduler.scheduler_record"));

    let mut non_due = model.rows[0]
        .scheduler_record
        .clone()
        .expect_value("scheduled record");
    non_due.scheduler_state = NotificationLocalOutboxSchedulerState::ManualRequired;
    let jsonl = serde_json::to_string(&non_due)?;
    let error = parse_app_game_notification_scheduler_jsonl(&jsonl)
        .err()
        .expect_value("non-due scheduler state must fail parsing");
    assert!(error
        .to_string()
        .contains("app_game.child_ux_scheduler.scheduler_record"));
    Ok(())
}

#[test]
fn scheduler_store_rejects_tampered_persisted_records_on_reopen(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = test_directory("tamper");
    let model =
        build_app_game_notification_scheduler_bridge(scheduler_options(), source_bridge()?)?;
    let store = AppGameChildUxSchedulerProofStore::open(&directory)?;
    persist_app_game_notification_scheduler_bridge(&store, &model)?;

    let path = directory.join("app-game-child-ux-scheduler-proof.json");
    let mut persisted: serde_json::Value = serde_json::from_slice(&fs::read(&path)?)?;
    persisted[0]["providerDeliveryObserved"] = json!(true);
    fs::write(&path, serde_json::to_vec(&persisted)?)?;

    let error = AppGameChildUxSchedulerProofStore::open(&directory)?
        .records()
        .err()
        .expect_value("tampered scheduler record must fail closed on reopen");
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert!(error
        .to_string()
        .contains("app_game.child_ux_scheduler.scheduler_record"));
    fs::remove_dir_all(directory)?;
    Ok(())
}

fn assert_source_rejected(
    source: ocentra_app_game_core::app_game_notification_local_outbox_bridge_types::AppGameNotificationLocalOutboxBridgeReadModel,
) {
    let error = build_app_game_notification_scheduler_bridge(scheduler_options(), source)
        .err()
        .expect_value("tampered WP58 source must fail");
    assert_eq!(
        error,
        ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.notification_scheduler.source_bridge",
            value: "bridge-58".to_owned(),
        }
    );
}

fn source_bridge() -> Result<
    ocentra_app_game_core::app_game_notification_local_outbox_bridge_types::AppGameNotificationLocalOutboxBridgeReadModel,
    ocentra_eventing::error::EventingError,
>{
    build_app_game_notification_local_outbox_bridge(
        outbox_options(),
        vec![
            readiness_row(
                "time-limit",
                APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED,
                APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
                APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT,
                true,
            ),
            readiness_row(
                "manual",
                APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED,
                APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED,
                APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_MANUAL_REQUIRED,
                false,
            ),
            readiness_row(
                "unavailable",
                APP_GAME_NOTIFICATION_READINESS_REASON_CAPABILITY_UNAVAILABLE,
                APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE,
                APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_UNAVAILABLE,
                false,
            ),
        ],
    )
}

fn readiness_row(
    row_id: &str,
    reason: &str,
    readiness_state: &str,
    minimal_payload_ref: &str,
    with_evidence: bool,
) -> AppGameNotificationReadinessRow {
    let evidence = with_evidence.then(|| ActivityEvidenceRef {
        evidence_id: format!("evidence:{row_id}"),
        kind: ActivityEvidenceKind::JournalEntry,
        digest: None,
        uri: None,
    });
    AppGameNotificationReadinessRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        reason: reason.to_owned(),
        readiness_state: readiness_state.to_owned(),
        row_count: u64::from(with_evidence),
        minimal_payload_ref: minimal_payload_ref.to_owned(),
        evidence_reference_ids: evidence
            .iter()
            .map(|reference| reference.evidence_id.clone())
            .collect(),
        evidence: evidence.into_iter().collect(),
    }
}

fn outbox_options() -> AppGameNotificationLocalOutboxBridgeOptions {
    AppGameNotificationLocalOutboxBridgeOptions {
        bridge_id: "bridge-58".to_owned(),
        generated_at: "2026-08-15T00:00:00Z".into(),
        family: FamilyReference {
            family_id: "family-59".to_owned(),
        },
        device: ParentDeviceReference {
            device_id: "device-59".into(),
            child_profile_id: Some("child-59".into()),
            label: "Child PC".to_owned(),
            platform: ParentDevicePlatform::Windows,
        },
        parent_action: ParentActionReference {
            action_reference_id: "parent-action-59".to_owned(),
            actor: ParentActorReference {
                actor_id: "parent-59".to_owned(),
                role: ParentActorRole::Parent,
            },
            policy_version: "policy-59".to_owned(),
            created_at: "2026-08-15T00:00:00Z".to_owned(),
        },
        provider_channel: V3NotificationProviderChannel::InApp,
        outbox_root_ref: "outbox-root-59".into(),
        outbox_file_ref: "outbox-file-59".into(),
        local_data_path_ref: "local-data-59".into(),
        policy_refs: vec!["policy-59".into()],
        audit_refs: vec!["audit-59".into()],
    }
}

fn scheduler_options() -> AppGameNotificationSchedulerBridgeOptions {
    AppGameNotificationSchedulerBridgeOptions {
        bridge_id: "bridge-59".to_owned(),
        scheduler_now_at: "2026-08-15T00:01:00Z".into(),
    }
}

fn test_directory(label: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "ocentra-app-game-notification-scheduler-{label}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    path
}
