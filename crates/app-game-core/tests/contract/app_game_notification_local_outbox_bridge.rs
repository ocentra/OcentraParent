use std::{fs, path::PathBuf, time::SystemTime};

use ocentra_app_game_core::app_game_child_ux_outbox_store::AppGameChildUxLocalOutboxStore;
use ocentra_app_game_core::app_game_child_ux_outbox_types::AppGameChildUxOutboxPersistResult;
use ocentra_app_game_core::app_game_notification_local_outbox_bridge::{
    build_app_game_notification_local_outbox_bridge,
    parse_app_game_notification_local_outbox_jsonl,
    persist_app_game_notification_local_outbox_bridge,
    serialize_app_game_notification_local_outbox_jsonl,
};
use ocentra_app_game_core::app_game_notification_local_outbox_bridge_types::{
    AppGameNotificationLocalOutboxBridgeOptions, AppGameNotificationLocalOutboxBridgeStatus,
};
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::policy::{ParentActorReference, ParentActorRole};
use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_notification_readiness::{
    AppGameNotificationReadinessRow,
    APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_MANUAL_REQUIRED,
    APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_SUSPICIOUS_UNKNOWN,
    APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT,
    APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_UNAVAILABLE,
    APP_GAME_NOTIFICATION_READINESS_REASON_CAPABILITY_UNAVAILABLE,
    APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED,
    APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN,
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
    NotificationLocalOutboxDeliveryClaimState, NotificationLocalOutboxSeverity,
    NotificationLocalOutboxState, V3NotificationProviderChannel, V3NotificationRuleReasonCode,
};

#[test]
fn eligible_rows_link_while_manual_and_unavailable_rows_stay_out_of_outbox(
) -> Result<(), Box<dyn std::error::Error>> {
    let model = build_app_game_notification_local_outbox_bridge(
        options(),
        vec![
            readiness_row(
                "time-limit",
                APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED,
                APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
                APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT,
                true,
            ),
            readiness_row(
                "suspicious",
                APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN,
                APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
                APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_SUSPICIOUS_UNKNOWN,
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
    )?;

    assert_eq!(model.linked_record_count, 2);
    assert_eq!(model.manual_required_count, 1);
    assert_eq!(model.unavailable_count, 1);
    assert_eq!(model.policy_refs, vec!["policy-58".into()]);
    assert_eq!(model.audit_refs, vec!["audit-58".into()]);
    assert!(!model.provider_delivery_runtime_claimed);
    assert!(!model.provider_receipt_ingestion_claimed);
    assert!(!model.scheduler_runtime_claimed);
    assert!(!model.cloud_routing_claimed);
    assert!(!model.parent_notification_ui_claimed);
    assert!(!model.child_delivery_claimed);
    assert!(!model.adapter_dispatch_claimed);
    assert_eq!(
        model.rows[2].status,
        AppGameNotificationLocalOutboxBridgeStatus::ManualRequired
    );
    assert!(model.rows[2].outbox_record.is_none());
    assert_eq!(
        model.rows[2].blocked_reason_refs,
        vec![APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_MANUAL_REQUIRED.into()]
    );
    assert_eq!(
        model.rows[3].status,
        AppGameNotificationLocalOutboxBridgeStatus::Unavailable
    );
    assert!(model.rows[3].outbox_record.is_none());

    let time_limit = model.rows[0]
        .outbox_record
        .as_ref()
        .expect_value("time limit record");
    assert_eq!(time_limit.state, NotificationLocalOutboxState::QueuedLocal);
    assert_eq!(
        time_limit.envelope.severity,
        NotificationLocalOutboxSeverity::Urgent
    );
    assert_eq!(
        time_limit.envelope.reason_code,
        V3NotificationRuleReasonCode::PolicyViolation
    );
    assert_eq!(
        time_limit.delivery_claim_state,
        NotificationLocalOutboxDeliveryClaimState::LocalOutboxOnly
    );
    assert!(!time_limit.provider_delivery_attempted);
    let suspicious = model.rows[1]
        .outbox_record
        .as_ref()
        .expect_value("suspicious record");
    assert_eq!(
        suspicious.envelope.reason_code,
        V3NotificationRuleReasonCode::SuspiciousUnknown
    );
    Ok(())
}

#[test]
fn linked_records_persist_reopen_and_replay_idempotently() -> Result<(), Box<dyn std::error::Error>>
{
    let directory = test_directory("persist");
    let model = build_app_game_notification_local_outbox_bridge(
        options(),
        vec![readiness_row(
            "time-limit",
            APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED,
            APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
            APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT,
            true,
        )],
    )?;
    let store = AppGameChildUxLocalOutboxStore::open(&directory)?;
    assert_eq!(
        persist_app_game_notification_local_outbox_bridge(&store, &model)?,
        vec![AppGameChildUxOutboxPersistResult::Inserted]
    );
    let reopened = AppGameChildUxLocalOutboxStore::open(&directory)?;
    assert_eq!(reopened.records()?.len(), 1);
    assert_eq!(
        persist_app_game_notification_local_outbox_bridge(&reopened, &model)?,
        vec![AppGameChildUxOutboxPersistResult::AlreadyPresent]
    );
    let mut conflicting = model;
    conflicting.rows[0]
        .outbox_record
        .as_mut()
        .expect_value("linked record")
        .retry_attempt_count = 1;
    let error = match persist_app_game_notification_local_outbox_bridge(&reopened, &conflicting) {
        Err(error) => error,
        Ok(result) => {
            return Err(std::io::Error::other(format!(
                "conflicting notification entry unexpectedly persisted as {result:?}"
            ))
            .into());
        }
    };
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(reopened.records()?.len(), 1);
    fs::remove_dir_all(directory)?;
    Ok(())
}

#[test]
fn deterministic_jsonl_round_trips_only_linked_records() -> Result<(), Box<dyn std::error::Error>> {
    let model = build_app_game_notification_local_outbox_bridge(
        options(),
        vec![
            readiness_row(
                "suspicious",
                APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN,
                APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
                APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_SUSPICIOUS_UNKNOWN,
                true,
            ),
            readiness_row(
                "manual",
                APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED,
                APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED,
                APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_MANUAL_REQUIRED,
                false,
            ),
        ],
    )?;
    let first = serialize_app_game_notification_local_outbox_jsonl(&model)?;
    let second = serialize_app_game_notification_local_outbox_jsonl(&model)?;
    assert_eq!(first, second);
    assert!(first.ends_with('\n'));
    let parsed = parse_app_game_notification_local_outbox_jsonl(&first)?;
    assert_eq!(parsed.len(), 1);
    assert_eq!(
        parsed[0],
        model.rows[0].outbox_record.clone().expect_value("linked")
    );
    let parse_error = parse_app_game_notification_local_outbox_jsonl("{not-json}\n")
        .err()
        .expect_value("invalid JSONL must fail");
    assert_eq!(parse_error.classify(), serde_json::error::Category::Syntax);
    Ok(())
}

#[test]
fn ready_row_without_evidence_fails_closed() {
    let error = build_app_game_notification_local_outbox_bridge(
        options(),
        vec![readiness_row(
            "time-limit",
            APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED,
            APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
            APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT,
            false,
        )],
    )
    .err()
    .expect_value("ready row without evidence must fail");
    assert_eq!(
        error,
        ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.notification_local_outbox.source",
            value: "time-limit".to_owned(),
        }
    );
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

fn options() -> AppGameNotificationLocalOutboxBridgeOptions {
    AppGameNotificationLocalOutboxBridgeOptions {
        bridge_id: "bridge-58".to_owned(),
        generated_at: "2026-08-15T00:00:00Z".into(),
        family: FamilyReference {
            family_id: "family-58".to_owned(),
        },
        device: ParentDeviceReference {
            device_id: "device-58".into(),
            child_profile_id: Some("child-58".into()),
            label: "Child PC".to_owned(),
            platform: ParentDevicePlatform::Windows,
        },
        parent_action: ParentActionReference {
            action_reference_id: "parent-action-58".to_owned(),
            actor: ParentActorReference {
                actor_id: "parent-58".to_owned(),
                role: ParentActorRole::Parent,
            },
            policy_version: "policy-58".to_owned(),
            created_at: "2026-08-15T00:00:00Z".to_owned(),
        },
        provider_channel: V3NotificationProviderChannel::InApp,
        outbox_root_ref: "outbox-root-58".into(),
        outbox_file_ref: "outbox-file-58".into(),
        local_data_path_ref: "local-data-58".into(),
        policy_refs: vec!["policy-58".into()],
        audit_refs: vec!["audit-58".into()],
    }
}

fn test_directory(label: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "ocentra-app-game-notification-local-outbox-{label}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    path
}
