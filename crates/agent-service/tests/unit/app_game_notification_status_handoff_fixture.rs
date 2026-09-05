use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

use ocentra_app_game_core::app_game_child_ux_scheduler_store::AppGameChildUxSchedulerProofStore;
use ocentra_app_game_core::app_game_notification_local_outbox_bridge::build_app_game_notification_local_outbox_bridge;
use ocentra_app_game_core::app_game_notification_local_outbox_bridge_types::AppGameNotificationLocalOutboxBridgeOptions;
use ocentra_app_game_core::app_game_notification_scheduler_bridge::{
    build_app_game_notification_scheduler_bridge, persist_app_game_notification_scheduler_bridge,
};
use ocentra_app_game_core::app_game_notification_scheduler_bridge_types::{
    AppGameNotificationSchedulerBridgeOptions, AppGameNotificationSchedulerBridgeReadModel,
};
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
    NotificationLocalOutboxSchedulerRecord, V3NotificationProviderChannel,
};

pub(super) struct NotificationStatusFixture {
    _root: TestDirectory,
    activity_db_path: PathBuf,
    scheduler: AppGameNotificationSchedulerBridgeReadModel,
}

impl NotificationStatusFixture {
    pub(super) fn persisted() -> Result<Self, Box<dyn std::error::Error>> {
        let root = TestDirectory::create()?;
        let activity_db_path = root.path().join("activity.sqlite");
        let scheduler_directory = root.path().join("activity.sqlite.app-game-notification");
        let proof_store =
            AppGameChildUxSchedulerProofStore::open(scheduler_directory.join("scheduler-proof"))?;
        let scheduler = scheduler_bridge()?;
        persist_app_game_notification_scheduler_bridge(&proof_store, &scheduler)?;
        fs::write(
            scheduler_directory.join("scheduler-bridge.json"),
            serde_json::to_vec(&scheduler)?,
        )?;
        Ok(Self {
            _root: root,
            activity_db_path,
            scheduler,
        })
    }

    pub(super) fn activity_db_path(&self) -> &Path {
        &self.activity_db_path
    }

    pub(super) fn scheduler_bridge_id(&self) -> &str {
        &self.scheduler.bridge_id
    }

    pub(super) fn scheduled_record(
        &self,
    ) -> Result<&NotificationLocalOutboxSchedulerRecord, std::io::Error> {
        self.scheduler
            .rows
            .first()
            .and_then(|row| row.scheduler_record.as_ref())
            .ok_or_else(|| invalid_fixture("scheduled row must retain its scheduler record"))
    }
}

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn create() -> Result<Self, std::io::Error> {
        let path = std::env::temp_dir().join(format!(
            "ocentra-app-game-notification-status-handoff-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        fs::create_dir_all(&path)?;
        Ok(Self(path))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
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
            bridge_id: "bridge-59-for-status".to_owned(),
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
        bridge_id: "bridge-58-for-status".to_owned(),
        generated_at: "2026-08-15T00:00:00Z".into(),
        family: FamilyReference {
            family_id: "family-65".to_owned(),
        },
        device: ParentDeviceReference {
            device_id: "device-65".into(),
            child_profile_id: Some("child-65".into()),
            label: "Child PC".to_owned(),
            platform: ParentDevicePlatform::Windows,
        },
        parent_action: ParentActionReference {
            action_reference_id: "parent-action-65".to_owned(),
            actor: ParentActorReference {
                actor_id: "parent-65".to_owned(),
                role: ParentActorRole::Parent,
            },
            policy_version: "policy-65".to_owned(),
            created_at: "2026-08-15T00:00:00Z".to_owned(),
        },
        provider_channel: V3NotificationProviderChannel::InApp,
        outbox_root_ref: "outbox-root-65".into(),
        outbox_file_ref: "outbox-file-65".into(),
        local_data_path_ref: "local-data-65".into(),
        policy_refs: vec!["policy-65".into()],
        audit_refs: vec!["audit-65".into()],
    }
}

fn invalid_fixture(message: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, message)
}
