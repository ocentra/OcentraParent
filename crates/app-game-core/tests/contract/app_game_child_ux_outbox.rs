use std::{fs, path::PathBuf, time::SystemTime};

use ocentra_app_game_core::app_game_child_ux::build_app_game_child_ux_notice;
use ocentra_app_game_core::app_game_child_ux_outbox::build_app_game_child_ux_outbox_route;
use ocentra_app_game_core::app_game_child_ux_outbox_store::AppGameChildUxLocalOutboxStore;
use ocentra_app_game_core::app_game_child_ux_outbox_types::{
    AppGameChildUxOutboxInput, AppGameChildUxOutboxPersistResult, AppGameChildUxOutboxRoute,
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
use ocentra_parent_agent_protocol::schema_domain_mirrors::family::{
    FamilyReference, ParentDevicePlatform, ParentDeviceReference,
};
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxSeverity, NotificationLocalOutboxState, V3NotificationProviderChannel,
};

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
