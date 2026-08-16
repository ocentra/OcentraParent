use ocentra_app_game_core::app_game_notification_local_outbox_bridge::build_app_game_notification_local_outbox_bridge;
use ocentra_app_game_core::app_game_notification_local_outbox_bridge_types::AppGameNotificationLocalOutboxBridgeOptions;
use ocentra_app_game_core::app_game_notification_provider_preflight_bridge::build_app_game_notification_provider_preflight_bridge;
use ocentra_app_game_core::app_game_notification_provider_preflight_bridge_types::AppGameNotificationProviderPreflightBridgeOptions;
use ocentra_app_game_core::app_game_notification_scheduler_bridge::{
    build_app_game_notification_scheduler_bridge, persist_app_game_notification_scheduler_bridge,
};
use ocentra_app_game_core::app_game_notification_scheduler_bridge_types::{
    AppGameNotificationSchedulerBridgeOptions, AppGameNotificationSchedulerBridgeReadModel,
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
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::V3NotificationProviderChannel;

use ocentra_app_game_core::app_game_child_ux_provider_preflight_types::AppGameChildUxProviderPreflightStatus;

#[test]
fn provider_preflight_bridge_consumes_wp59_and_preserves_blocked_rows(
) -> Result<(), Box<dyn std::error::Error>> {
    let (directory, store, source) = persisted_scheduler_bridge("rows")?;
    let model =
        build_app_game_notification_provider_preflight_bridge(&store, preflight_options(), source)?;
    assert_eq!(model.provider_adapter_required_count, 1);
    assert_eq!(model.manual_required_count, 1);
    assert_eq!(model.unavailable_count, 1);
    assert_eq!(
        model.rows[0].status,
        AppGameChildUxProviderPreflightStatus::ProviderAdapterRequired
    );
    assert_eq!(
        model.rows[1].status,
        AppGameChildUxProviderPreflightStatus::ManualRequired
    );
    assert_eq!(
        model.rows[1].blocked_reason_refs,
        vec![APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_MANUAL_REQUIRED.into()]
    );
    assert_eq!(
        model.rows[2].status,
        AppGameChildUxProviderPreflightStatus::Unavailable
    );
    assert_eq!(model.rows[1].preflight_row, None);
    assert_eq!(model.rows[2].preflight_row, None);
    assert!(!model.provider_delivery_runtime_claimed);
    assert!(!model.provider_receipt_ingestion_claimed);
    assert!(!model.provider_credentials_claimed);
    assert!(!model.retry_worker_runtime_claimed);
    assert!(!model.quiet_hours_timer_runtime_claimed);
    assert!(!model.production_durable_outbox_storage_claimed);
    assert!(!model.cloud_routing_claimed);
    assert!(!model.parent_notification_ui_claimed);
    assert!(!model.child_delivery_claimed);
    assert!(!model.adapter_dispatch_claimed);
    assert!(!model.platform_enforcement_claimed);
    fs::remove_dir_all(directory)?;
    Ok(())
}

#[test]
fn provider_preflight_bridge_generates_deterministic_requirements_and_preserves_refs(
) -> Result<(), Box<dyn std::error::Error>> {
    let (directory, store, source) = persisted_scheduler_bridge("requirements")?;
    let first = build_app_game_notification_provider_preflight_bridge(
        &store,
        preflight_options(),
        source.clone(),
    )?;
    let second =
        build_app_game_notification_provider_preflight_bridge(&store, preflight_options(), source)?;
    assert_eq!(first, second);
    let row = first.rows[0]
        .preflight_row
        .as_ref()
        .expect_value("scheduled provider preflight row");
    assert_eq!(row.evidence_refs.len(), 1);
    assert_eq!(row.policy_refs, vec!["policy-61".into()]);
    assert_eq!(row.audit_refs, vec!["audit-61".into()]);
    assert_eq!(row.adapter_requirement_refs.len(), 3);
    assert_eq!(row.manual_proof_requirements, row.adapter_requirement_refs);
    assert!(row.adapter_requirement_refs.iter().all(|reference| {
        reference.as_str().contains("bridge-61")
            && reference
                .as_str()
                .contains(row.source_scheduler_entry_id.as_str())
    }));
    assert!(!row.provider_delivery_runtime_claimed);
    assert!(!row.provider_receipt_ingestion_claimed);
    assert!(!row.provider_credentials_claimed);
    assert!(!row.adapter_dispatch_claimed);
    fs::remove_dir_all(directory)?;
    Ok(())
}

#[test]
fn provider_preflight_bridge_rejects_tampered_wp59_counts_claims_and_sources(
) -> Result<(), Box<dyn std::error::Error>> {
    let (directory, store, source) = persisted_scheduler_bridge("tamper")?;
    let mut bad_count = source.clone();
    bad_count.scheduled_count = 9;
    assert_source_rejected(&store, bad_count);

    let mut claimed = source.clone();
    claimed.provider_delivery_runtime_claimed = true;
    assert_source_rejected(&store, claimed);

    let mut duplicate = source.clone();
    duplicate.rows[1].scheduler_bridge_record_id =
        duplicate.rows[0].scheduler_bridge_record_id.clone();
    assert_source_rejected(&store, duplicate);

    let mut duplicate_scheduled_identity = source.clone();
    let mut duplicate_row = duplicate_scheduled_identity.rows[0].clone();
    duplicate_row.scheduler_bridge_record_id = "different-bridge-record".to_owned();
    duplicate_scheduled_identity.rows.push(duplicate_row);
    duplicate_scheduled_identity.scheduled_count = 2;
    assert_source_rejected(&store, duplicate_scheduled_identity);

    let mut mismatched = source;
    mismatched.rows[0]
        .source_outbox_record
        .as_mut()
        .expect_value("scheduled source record")
        .entry_id = "different-entry".into();
    assert_source_rejected(&store, mismatched);
    fs::remove_dir_all(directory)?;
    Ok(())
}

#[test]
fn provider_preflight_bridge_rejects_unpersisted_scheduler_rows(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = test_directory("unpersisted");
    let store = AppGameChildUxSchedulerProofStore::open(&directory)?;
    assert_source_rejected(&store, scheduler_bridge()?);
    fs::remove_dir_all(directory)?;
    Ok(())
}

fn assert_source_rejected(
    store: &AppGameChildUxSchedulerProofStore,
    source: AppGameNotificationSchedulerBridgeReadModel,
) {
    let error =
        build_app_game_notification_provider_preflight_bridge(store, preflight_options(), source)
            .err()
            .expect_value("tampered WP59 source must fail");
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert!(error
        .to_string()
        .contains("app_game.notification_provider_preflight.source_bridge"));
}

fn persisted_scheduler_bridge(
    label: &str,
) -> Result<
    (
        PathBuf,
        AppGameChildUxSchedulerProofStore,
        AppGameNotificationSchedulerBridgeReadModel,
    ),
    Box<dyn std::error::Error>,
> {
    let directory = test_directory(label);
    let store = AppGameChildUxSchedulerProofStore::open(&directory)?;
    let source = scheduler_bridge()?;
    persist_app_game_notification_scheduler_bridge(&store, &source)?;
    Ok((directory, store, source))
}

fn scheduler_bridge(
) -> Result<AppGameNotificationSchedulerBridgeReadModel, ocentra_eventing::error::EventingError> {
    let source = build_app_game_notification_local_outbox_bridge(
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
    )?;
    build_app_game_notification_scheduler_bridge(
        AppGameNotificationSchedulerBridgeOptions {
            bridge_id: "bridge-59-for-61".to_owned(),
            scheduler_now_at: "2026-08-15T00:01:00Z".into(),
        },
        source,
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
        bridge_id: "bridge-58-for-61".to_owned(),
        generated_at: "2026-08-15T00:00:00Z".into(),
        family: FamilyReference {
            family_id: "family-61".to_owned(),
        },
        device: ParentDeviceReference {
            device_id: "device-61".into(),
            child_profile_id: Some("child-61".into()),
            label: "Child PC".to_owned(),
            platform: ParentDevicePlatform::Windows,
        },
        parent_action: ParentActionReference {
            action_reference_id: "parent-action-61".to_owned(),
            actor: ParentActorReference {
                actor_id: "parent-61".to_owned(),
                role: ParentActorRole::Parent,
            },
            policy_version: "policy-61".to_owned(),
            created_at: "2026-08-15T00:00:00Z".to_owned(),
        },
        provider_channel: V3NotificationProviderChannel::InApp,
        outbox_root_ref: "outbox-root-61".into(),
        outbox_file_ref: "outbox-file-61".into(),
        local_data_path_ref: "local-data-61".into(),
        policy_refs: vec!["policy-61".into()],
        audit_refs: vec!["audit-61".into()],
    }
}

fn preflight_options() -> AppGameNotificationProviderPreflightBridgeOptions {
    AppGameNotificationProviderPreflightBridgeOptions {
        bridge_id: "bridge-61".to_owned(),
        generated_at: "2026-08-15T00:02:00Z".into(),
    }
}

fn test_directory(label: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "ocentra-app-game-notification-provider-preflight-{label}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    path
}
use std::{fs, path::PathBuf, time::SystemTime};

use ocentra_app_game_core::app_game_child_ux_scheduler_store::AppGameChildUxSchedulerProofStore;
