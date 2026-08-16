use ocentra_app_game_core::app_game_notification_audit_history_bridge::{
    build_app_game_notification_audit_history_bridge,
    parse_app_game_notification_audit_history_jsonl,
    serialize_app_game_notification_audit_history_jsonl,
};
use ocentra_app_game_core::app_game_notification_audit_history_bridge_types::{
    AppGameNotificationAuditHistoryOptions, AppGameNotificationAuditHistoryStatus,
};
use ocentra_app_game_core::app_game_notification_local_outbox_bridge::build_app_game_notification_local_outbox_bridge;
use ocentra_app_game_core::app_game_notification_local_outbox_bridge_types::{
    AppGameNotificationLocalOutboxBridgeOptions, AppGameNotificationLocalOutboxBridgeReadModel,
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
    NotificationLocalOutboxState, V3NotificationProviderChannel,
};

#[test]
fn audit_history_bridge_maps_wp58_rows_and_preserves_source_refs(
) -> Result<(), Box<dyn std::error::Error>> {
    let model =
        build_app_game_notification_audit_history_bridge(audit_options(), source_bridge()?)?;
    assert_eq!(model.queued_local_count, 1);
    assert_eq!(model.manual_required_count, 1);
    assert_eq!(model.unavailable_count, 1);
    assert_eq!(
        model
            .entries
            .iter()
            .map(|entry| entry.status)
            .collect::<Vec<_>>(),
        vec![
            AppGameNotificationAuditHistoryStatus::QueuedLocal,
            AppGameNotificationAuditHistoryStatus::ManualRequired,
            AppGameNotificationAuditHistoryStatus::Unavailable,
        ]
    );

    let queued = &model.entries[0];
    assert_eq!(
        queued.source_outbox_state,
        Some(NotificationLocalOutboxState::QueuedLocal)
    );
    assert_eq!(queued.audit_refs, vec!["audit-60".into()]);
    assert_eq!(queued.policy_refs, vec!["policy-60".into()]);
    assert_eq!(queued.evidence_refs.len(), 1);
    assert_eq!(queued.blocked_reason_refs, Vec::new());
    assert!(!queued.provider_send_created);

    let manual = &model.entries[1];
    assert_eq!(manual.source_entry_id, None);
    assert_eq!(manual.audit_refs, vec!["audit-60".into()]);
    assert_eq!(manual.policy_refs, vec!["policy-60".into()]);
    assert_eq!(
        manual.blocked_reason_refs,
        vec![APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_MANUAL_REQUIRED.into()]
    );
    assert!(!manual.provider_send_created);
    assert!(!model.provider_delivery_runtime_claimed);
    assert!(!model.provider_receipt_ingestion_claimed);
    assert!(!model.retry_worker_runtime_claimed);
    assert!(!model.quiet_hours_timer_runtime_claimed);
    assert!(!model.production_durable_history_claimed);
    assert!(!model.parent_notification_history_ui_claimed);
    Ok(())
}

#[test]
fn audit_history_jsonl_is_deterministic_and_includes_blocked_rows(
) -> Result<(), Box<dyn std::error::Error>> {
    let model =
        build_app_game_notification_audit_history_bridge(audit_options(), source_bridge()?)?;
    let first = serialize_app_game_notification_audit_history_jsonl(&model)?;
    let second = serialize_app_game_notification_audit_history_jsonl(&model)?;
    assert_eq!(first, second);
    assert_eq!(
        parse_app_game_notification_audit_history_jsonl(&first)?,
        model.entries
    );
    assert_eq!(first.lines().count(), 3);
    Ok(())
}

#[test]
fn audit_history_bridge_rejects_tampered_refs_claims_and_identities(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut bad_refs = source_bridge()?;
    bad_refs.audit_refs = vec!["different-audit".into()];
    assert_source_rejected(bad_refs);

    let mut claimed = source_bridge()?;
    claimed.provider_delivery_runtime_claimed = true;
    assert_source_rejected(claimed);

    let mut duplicate = source_bridge()?;
    duplicate.rows[1].bridge_record_id = duplicate.rows[0].bridge_record_id.clone();
    assert_source_rejected(duplicate);
    Ok(())
}

fn assert_source_rejected(source: AppGameNotificationLocalOutboxBridgeReadModel) {
    let error = build_app_game_notification_audit_history_bridge(audit_options(), source)
        .err()
        .expect_value("tampered WP58 source must fail");
    assert_eq!(
        error,
        ocentra_eventing::error::EventingError::InvalidValue {
            field: "app_game.notification_audit_history.source_bridge",
            value: "bridge-60".to_owned(),
        }
    );
}

fn source_bridge(
) -> Result<AppGameNotificationLocalOutboxBridgeReadModel, ocentra_eventing::error::EventingError> {
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
        bridge_id: "bridge-60".to_owned(),
        generated_at: "2026-08-15T00:00:00Z".into(),
        family: FamilyReference {
            family_id: "family-60".to_owned(),
        },
        device: ParentDeviceReference {
            device_id: "device-60".into(),
            child_profile_id: Some("child-60".into()),
            label: "Child PC".to_owned(),
            platform: ParentDevicePlatform::Windows,
        },
        parent_action: ParentActionReference {
            action_reference_id: "parent-action-60".to_owned(),
            actor: ParentActorReference {
                actor_id: "parent-60".to_owned(),
                role: ParentActorRole::Parent,
            },
            policy_version: "policy-60".to_owned(),
            created_at: "2026-08-15T00:00:00Z".to_owned(),
        },
        provider_channel: V3NotificationProviderChannel::InApp,
        outbox_root_ref: "outbox-root-60".into(),
        outbox_file_ref: "outbox-file-60".into(),
        local_data_path_ref: "local-data-60".into(),
        policy_refs: vec!["policy-60".into()],
        audit_refs: vec!["audit-60".into()],
    }
}

fn audit_options() -> AppGameNotificationAuditHistoryOptions {
    AppGameNotificationAuditHistoryOptions {
        handoff_id: "audit-handoff-60".to_owned(),
        recorded_at: "2026-08-15T00:02:00Z".into(),
    }
}
