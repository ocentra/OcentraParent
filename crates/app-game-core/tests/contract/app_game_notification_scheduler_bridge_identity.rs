use std::fs;

use ocentra_app_game_core::app_game_child_ux_scheduler_store::AppGameChildUxSchedulerProofStore;
use ocentra_app_game_core::app_game_notification_local_outbox_bridge::build_app_game_notification_local_outbox_bridge;
use ocentra_app_game_core::app_game_notification_local_outbox_bridge_types::AppGameNotificationLocalOutboxBridgeOptions;
use ocentra_app_game_core::app_game_notification_scheduler_bridge::{
    build_app_game_notification_scheduler_bridge, parse_app_game_notification_scheduler_jsonl,
    persist_app_game_notification_scheduler_bridge,
    serialize_app_game_notification_scheduler_jsonl,
};
use ocentra_app_game_core::app_game_notification_scheduler_bridge_types::{
    AppGameNotificationSchedulerBridgeOptions, AppGameNotificationSchedulerBridgeReadModel,
};
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::policy::{ParentActorReference, ParentActorRole};
use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_notification_readiness::{
    AppGameNotificationReadinessRow, APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT,
    APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED,
    APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
};
use ocentra_parent_agent_protocol::enforcement::ParentActionReference;
use ocentra_parent_agent_protocol::schema_domain_mirrors::family::{
    FamilyReference, ParentDevicePlatform, ParentDeviceReference,
};
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxReference, NotificationLocalOutboxSchedulerEntryId,
    V3NotificationProviderChannel,
};

use super::app_game_notification_scheduler_bridge::{
    scheduler_options, source_bridge, test_directory,
};
use super::app_game_notification_scheduler_bridge_identity_support::{
    assert_duplicate_identity_error, DuplicateSchedulerRecordIdentity,
};

const INVALID_BRIDGE_MESSAGE: &str =
    "invalid eventing value for app_game.notification_scheduler.source_bridge: bridge-59";

#[test]
fn scheduler_bridge_rejects_tampered_deterministic_identities(
) -> Result<(), Box<dyn std::error::Error>> {
    for tamper in [
        tamper_bridge_record_id as fn(&mut AppGameNotificationSchedulerBridgeReadModel),
        tamper_scheduler_entry_id,
        tamper_scheduler_decision_ref,
        tamper_scheduler_artifact_ref,
    ] {
        let mut model = scheduler_bridge()?;
        tamper(&mut model);
        assert_rejected(&model);
    }
    Ok(())
}

#[test]
fn scheduler_bridge_rejects_tampered_nested_source_and_schema(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut unsafe_source = scheduler_bridge()?;
    unsafe_source.rows[0]
        .source_outbox_record
        .as_mut()
        .expect_value("scheduled source record")
        .provider_delivery_attempted = true;
    assert_rejected(&unsafe_source);

    let mut unsupported_schema = scheduler_bridge()?;
    unsupported_schema.schema_version = APP_GAME_SCHEMA_VERSION + 1;
    assert_rejected(&unsupported_schema);
    Ok(())
}

#[test]
fn scheduler_jsonl_rejects_duplicate_scheduler_and_source_identities(
) -> Result<(), Box<dyn std::error::Error>> {
    let model =
        build_app_game_notification_scheduler_bridge(scheduler_options(), source_bridge()?)?;
    let record = model.rows[0]
        .scheduler_record
        .clone()
        .expect_value("scheduled record");
    for identity in DuplicateSchedulerRecordIdentity::ALL {
        let duplicate = identity.duplicate(&record);
        let jsonl = format!(
            "{}\n{}\n",
            serde_json::to_string(&record)?,
            serde_json::to_string(&duplicate)?
        );
        let error = parse_app_game_notification_scheduler_jsonl(&jsonl)
            .err()
            .expect_value("duplicate scheduler identity must fail JSONL parsing");
        assert_duplicate_identity_error(&error.to_string(), identity, &record);
    }
    Ok(())
}

#[test]
fn scheduler_store_rejects_duplicate_scheduler_and_source_identities_on_reopen(
) -> Result<(), Box<dyn std::error::Error>> {
    for identity in DuplicateSchedulerRecordIdentity::ALL {
        let directory = test_directory(identity.label());
        let model =
            build_app_game_notification_scheduler_bridge(scheduler_options(), source_bridge()?)?;
        let store = AppGameChildUxSchedulerProofStore::open(&directory)?;
        persist_app_game_notification_scheduler_bridge(&store, &model)?;
        let mut persisted = store.records()?;
        let record = persisted[0].clone();
        persisted.push(identity.duplicate(&record));
        fs::write(
            directory.join("app-game-child-ux-scheduler-proof.json"),
            serde_json::to_vec(&persisted)?,
        )?;

        let error = AppGameChildUxSchedulerProofStore::open(&directory)?
            .records()
            .err()
            .expect_value("duplicate persisted scheduler identity must fail on reopen");
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
        assert_duplicate_identity_error(&error.to_string(), identity, &record);
        fs::remove_dir_all(directory)?;
    }
    Ok(())
}

fn assert_rejected(model: &AppGameNotificationSchedulerBridgeReadModel) {
    let error = serialize_app_game_notification_scheduler_jsonl(model)
        .err()
        .expect_value("tampered scheduler bridge must fail closed");
    assert_eq!(error.classify(), serde_json::error::Category::Io);
    assert_eq!(error.to_string(), INVALID_BRIDGE_MESSAGE);
}

fn tamper_bridge_record_id(model: &mut AppGameNotificationSchedulerBridgeReadModel) {
    model.rows[0].scheduler_bridge_record_id = "bridge-59:foreign-source-row".to_owned();
}

fn tamper_scheduler_entry_id(model: &mut AppGameNotificationSchedulerBridgeReadModel) {
    model.rows[0]
        .scheduler_record
        .as_mut()
        .expect_value("scheduled record")
        .scheduler_entry_id = NotificationLocalOutboxSchedulerEntryId::from(
        "app-game-notification-scheduler:other-bridge:entry",
    );
}

fn tamper_scheduler_decision_ref(model: &mut AppGameNotificationSchedulerBridgeReadModel) {
    model.rows[0]
        .scheduler_record
        .as_mut()
        .expect_value("scheduled record")
        .scheduler_decision_ref = NotificationLocalOutboxReference::from("foreign-decision");
}

fn tamper_scheduler_artifact_ref(model: &mut AppGameNotificationSchedulerBridgeReadModel) {
    model.rows[0]
        .scheduler_record
        .as_mut()
        .expect_value("scheduled record")
        .scheduler_artifact_ref = NotificationLocalOutboxReference::from("foreign-artifact");
}

fn scheduler_bridge(
) -> Result<AppGameNotificationSchedulerBridgeReadModel, ocentra_eventing::error::EventingError> {
    let source = build_app_game_notification_local_outbox_bridge(
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
        },
        vec![ready_row()],
    )?;
    build_app_game_notification_scheduler_bridge(
        AppGameNotificationSchedulerBridgeOptions {
            bridge_id: "bridge-59".to_owned(),
            scheduler_now_at: "2026-08-15T00:01:00Z".into(),
        },
        source,
    )
}

fn ready_row() -> AppGameNotificationReadinessRow {
    let evidence = ActivityEvidenceRef {
        evidence_id: "evidence:time-limit".to_owned(),
        kind: ActivityEvidenceKind::JournalEntry,
        digest: None,
        uri: None,
    };
    AppGameNotificationReadinessRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: "time-limit".to_owned(),
        reason: APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED.to_owned(),
        readiness_state: APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT.to_owned(),
        row_count: 1,
        minimal_payload_ref: APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT.to_owned(),
        evidence_reference_ids: vec![evidence.evidence_id.clone()],
        evidence: vec![evidence],
    }
}
