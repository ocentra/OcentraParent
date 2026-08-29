#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../../src/activity_payload.rs"]
mod activity_payload;
#[path = "../../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../../src/activity_surface_store.rs"]
mod activity_surface_store;
#[path = "../../src/enforcement_timer_state_file.rs"]
mod enforcement_timer_state_file;
#[path = "../../src/enforcement_timer_state_path.rs"]
mod enforcement_timer_state_path;
#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/json_contract.rs"]
mod json_contract;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../support/test_text.rs"]
mod test_text;
#[path = "../../src/time.rs"]
mod time;
mod activity_api {
    pub(crate) struct ActivityEventId(pub(crate) &'static str);
    pub(crate) struct GeneratedAtText(pub(crate) String);
}
mod timer_state_fixture {
    use ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome;
    use ocentra_parent_agent_protocol::activity::policy::{
        ParentActorReference, ParentEvidenceReference, ParentEvidenceReferenceKind, PolicyAction,
        PolicyTarget, PolicyTargetType,
    };
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::enforcement::{
        EnforcementAction, EnforcementAdapterKind, EnforcementAdapterResultCode,
        EnforcementAuditEvent, EnforcementAuditEventKind, EnforcementCapabilityState,
        EnforcementCapabilityStatus, EnforcementDependencyState, EnforcementMode,
        EnforcementPermissionState, EnforcementResult, EnforcementResultStatus,
        EnforcementRollbackState, EnforcementTimerEvent, EnforcementTimerEventKind,
        ParentActionReference, ParentPlatform,
    };
    use ocentra_parent_agent_protocol::policy_constants;
    use ocentra_parent_agent_protocol::ParentActorRole;

    pub(crate) fn outcome() -> EnforcementBoundaryOutcome {
        let actor = ParentActorReference {
            actor_id: policy_constants::TEST_PARENT_ACTOR_ID.to_string(),
            role: ParentActorRole::Parent,
        };
        let evidence = ParentEvidenceReference {
            evidence_reference_id: policy_constants::TEST_EVIDENCE_ID.to_string(),
            kind: ParentEvidenceReferenceKind::ActivityEvent,
            observed_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        };
        let capability = EnforcementCapabilityStatus {
            schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            platform: ParentPlatform::Windows,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            capability_state: EnforcementCapabilityState::Supported,
            permission_state: EnforcementPermissionState::NotRequired,
            dependency_state: EnforcementDependencyState::Installed,
            supported_actions: vec![
                EnforcementMode::TerminateProcess,
                EnforcementMode::TemporaryBlock,
            ],
            degraded_reason: None,
            last_checked_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        };
        let parent_approval = ParentActionReference {
            action_reference_id: constants::enforcement::TEST_PARENT_ACTION_REFERENCE_ID
                .to_string(),
            actor: actor.clone(),
            policy_version: policy_constants::TEST_POLICY_VERSION.to_string(),
            created_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        };
        let action = EnforcementAction {
            schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            action_id: constants::enforcement::TEST_ACTION_ID.to_string(),
            intent_id: constants::enforcement::TEST_INTENT_ID.to_string(),
            policy_decision_id: policy_constants::TEST_DECISION_ID.to_string(),
            policy_action: PolicyAction::Block,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            platform: ParentPlatform::Windows,
            target: PolicyTarget {
                target_id: constants::enforcement::TEST_PROCESS_TARGET_ID.to_string(),
                target_type: PolicyTargetType::Process,
                target_value: constants::enforcement::TEST_PROCESS_TARGET_VALUE.to_string(),
            },
            mode: EnforcementMode::TerminateProcess,
            capability: capability.clone(),
            reason_codes: vec![policy_constants::TEST_REASON_PARENT_BLOCK.to_string()],
            evidence_references: vec![evidence.clone()],
            local_ai_result_id: None,
            parent_approval: Some(parent_approval),
            dry_run: false,
            requested_at: policy_constants::TEST_EVALUATED_AT.to_string(),
            expires_at: Some(policy_constants::TEST_EXPIRES_AT.to_string()),
            rollback_token: Some(constants::enforcement::TEST_ROLLBACK_TOKEN.to_string()),
        };
        let result = EnforcementResult {
            schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            result_id: constants::enforcement::TEST_RESULT_ID.to_string(),
            action_id: action.action_id.clone(),
            status: EnforcementResultStatus::ActuallyEnforced,
            adapter_result_code: EnforcementAdapterResultCode::ProcessTerminated,
            started_at: policy_constants::TEST_EVALUATED_AT.to_string(),
            completed_at: Some(policy_constants::TEST_EVALUATED_AT.to_string()),
            rollback_token: action.rollback_token.clone(),
            rollback_state: EnforcementRollbackState::Available,
            unavailable_reason: None,
            unavailable_status: None,
            failed_reason: None,
            next_check_at: None,
            capability: capability.clone(),
        };
        EnforcementBoundaryOutcome {
            action: action.clone(),
            result: result.clone(),
            audit_event: EnforcementAuditEvent {
                schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
                audit_event_id: constants::enforcement::TEST_AUDIT_EVENT_ID.to_string(),
                audit_event_kind: EnforcementAuditEventKind::Succeeded,
                action: action.clone(),
                result,
                capability,
                unavailable_status: None,
                policy_version: policy_constants::TEST_POLICY_VERSION.to_string(),
                evidence_references: vec![evidence.clone()],
                actor: Some(actor),
                parent_override: action.parent_approval.clone(),
                journal_sequence: Some(constants::enforcement::TEST_JOURNAL_SEQUENCE.to_string()),
                observed_at: policy_constants::TEST_EVALUATED_AT.to_string(),
            },
            timer_event: Some(EnforcementTimerEvent {
                schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
                timer_event_id: constants::enforcement::TEST_TIMER_EVENT_ID.to_string(),
                timer_event_kind: EnforcementTimerEventKind::RestartRecovered,
                action_id: action.action_id,
                policy_decision_id: policy_constants::TEST_DECISION_ID.to_string(),
                evidence_references: vec![evidence],
                scheduled_at: policy_constants::TEST_EVALUATED_AT.to_string(),
                effective_at: Some(policy_constants::TEST_EXPIRES_AT.to_string()),
                rollback_token: Some(constants::enforcement::TEST_ROLLBACK_TOKEN.to_string()),
                recovered_after_restart: true,
                unavailable_reason: None,
            }),
            adapter_request: None,
        }
    }
}

#[path = "../../src/activity_api/app_game_child_runtime_transport_receipt_payload.rs"]
mod app_game_child_runtime_transport_receipt_payload;
#[path = "../../src/activity_api/app_game_timer_parent_preference_setup_request.rs"]
mod app_game_timer_parent_preference_setup_request;
#[path = "../../src/activity_api/app_game_timer_parent_preference_setup_request_outbox.rs"]
mod app_game_timer_parent_preference_setup_request_outbox;
#[path = "../../src/activity_api/app_game_timer_parent_preference_setup_request_persistence.rs"]
mod app_game_timer_parent_preference_setup_request_persistence;
#[path = "../support/app_game_timer_parent_preference_setup_request_status.rs"]
mod app_game_timer_parent_preference_setup_request_status;
#[path = "app_game_timer_parent_preference_setup_request_tests.rs"]
mod app_game_timer_parent_preference_setup_request_tests;

use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

pub(crate) fn child_runtime_receipt_read_model_from_service_model(
    model: AppGameServiceReadModel,
) -> ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::AppGameChildRuntimeTransportReceiptReadModel{
    app_game_child_runtime_transport_receipt_payload::app_game_child_runtime_transport_receipt_read_model_from_service_model(model)
}

pub(crate) async fn build_timer_preference_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    app_game_timer_parent_preference_setup_request::build_activity_app_game_timer_parent_preference_setup_request_report(command).await
}

pub(crate) async fn build_timer_preference_report_for_store_path(
    command: AgentCommandEnvelope,
    store_path: app_game_timer_parent_preference_setup_request::AppGameTimerSetupStorePath,
) -> AgentEventEnvelope {
    app_game_timer_parent_preference_setup_request::build_activity_app_game_timer_parent_preference_setup_request_report_for_store_path(command, store_path).await
}

#[cfg(test)]
mod clippy_linkage {
    use super::*;
    use crate::test_invariants::{
        require_json_decode, require_log_string_field, require_ok, require_some,
        serialize_test_json,
    };
    use ocentra_parent_agent_protocol::activity::ActivityObserver;
    use ocentra_parent_agent_protocol::activity::ActivitySubjectKind;
    use ocentra_parent_agent_protocol::activity_query::{
        ActivityIngestStatus, ActivityRecentSummary,
    };
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
    use ocentra_parent_agent_protocol::transport::{
        AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
        AgentPeerRole, AgentRoute,
    };
    use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
    use std::{env, fs::remove_file};

    #[tokio::test]
    async fn public_wrapper_and_outbox_helper_are_linked() {
        let encoded = serialize_test_json(&serde_json::json!({
            "app_game_pref_request": true
        }));
        let decoded: serde_json::Value =
            require_json_decode(&encoded, "app_game_timer_parent_pref_request linkage json");
        assert!(require_some(
            decoded
                .get("app_game_pref_request")
                .and_then(|value| value.as_bool()),
            "app_game_timer_parent_pref_request linkage bool",
        ));
        let json_text = crate::json_contract::serialize_json_string(&serde_json::json!({
            "app_game_pref_request": true
        }));
        let field = LogFieldValue::String(json_text.0);
        let text = require_log_string_field(
            Some(&field),
            "app_game_timer_parent_pref_request linkage field",
        );
        let _: serde_json::Value = require_json_decode(
            text,
            "app_game_timer_parent_pref_request linkage field json",
        );
        let _: () = require_ok(
            Ok::<(), std::io::Error>(()),
            "app_game_timer_parent_pref_request linkage ok",
        );
        link_shared_pref_request_helpers();

        let mut store_name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        store_name.push_str(&std::process::id().to_string());
        store_name.push(constants::delimiter::HYPHEN);
        store_name.push_str("app-game-pref-request-clippy");

        let mut store_path = std::env::temp_dir();
        store_path.push(store_name);
        store_path.set_extension(constants::activity_store::FILE_EXTENSION);
        cleanup_path(&store_path);
        let previous_store_path = env::var(constants::env_var::ACTIVITY_DB_PATH).ok();
        env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

        let event = app_game_timer_parent_preference_setup_request::build_activity_app_game_timer_parent_preference_setup_request_report(
            command_envelope(),
        )
        .await;
        link_activity_surface_store_helpers(&store_path).await;
        let _ =
            app_game_child_runtime_transport_receipt_payload::build_activity_app_game_child_runtime_transport_receipt_report(
                command_envelope(),
            )
            .await;
        assert_eq!(
            event.event,
            AgentEventName::AgentActivityAppGameTimerParentPreferenceSetupRequested
        );

        match previous_store_path {
            Some(value) => env::set_var(constants::env_var::ACTIVITY_DB_PATH, value),
            None => env::remove_var(constants::env_var::ACTIVITY_DB_PATH),
        }
        cleanup_path(&store_path);
    }

    fn link_shared_pref_request_helpers() {
        let _ = crate::activity_store_path::activity_journal_path();
        let _ = crate::activity_store_path::activity_journal_key_path();
        let _ = crate::event_builder::portal_peer();
        let value = crate::json_contract::serialize_json_value(serde_json::json!({
            "app_game_pref_request": true
        }));
        assert_eq!(value["app_game_pref_request"], true);
        let ingest = crate::activity_payload::ingest_status_payload(&ActivityIngestStatus {
            schema_version: 1,
            database_ready: true,
            events_ingested: 1,
            events_stored: 1,
            duplicate_events: 0,
            last_event_id: Some("event-1".to_string()),
        });
        assert_eq!(
            ingest.get(constants::field::DATABASE_READY),
            Some(&LogFieldValue::Boolean(true))
        );
        let recent = crate::activity_payload::recent_summary_payload(&ActivityRecentSummary {
            schema_version: 1,
            limit: 1,
            returned: 1,
            first_observed_at: Some("2026-06-29T00:00:00Z".to_string()),
            last_observed_at: Some("2026-06-29T00:00:00Z".to_string()),
            last_event_id: Some("event-1".to_string()),
            most_recent_kind: None,
            most_recent_observer: Some(ActivityObserver::AgentService),
            most_recent_subject_kind: Some(ActivitySubjectKind::Device),
            most_recent_subject_id: Some("device-1".to_string()),
            most_recent_subject_name: Some("device".to_string()),
        });
        assert_eq!(
            recent.get(constants::field::RETURNED),
            Some(&LogFieldValue::Number(1.0))
        );
        assert_eq!(
            crate::activity_payload::activity_store_error_payload().get(constants::field::REASON),
            Some(&LogFieldValue::String(
                constants::value::ACTIVITY_STORE_UNAVAILABLE.to_string()
            ))
        );
        let _: String = crate::time::timestamp_after_epoch_seconds(1, 0);
        let _: String = crate::time::timestamp_after_epoch_seconds(1, 1);
        let timer_state_path = crate::enforcement_timer_state_path::enforcement_timer_state_path();
        if let Some(timer_state_dir) = timer_state_path.parent_dir() {
            let _ = timer_state_dir.create_all();
        }
    }

    #[tokio::test]
    async fn enforcement_timer_state_helpers_are_linked() {
        let timer_state_path = crate::enforcement_timer_state_path::enforcement_timer_state_path();
        let outcome = crate::timer_state_fixture::outcome();
        let stored = crate::enforcement_timer_state_file::store_active_timer_state_for_outcome_with_app_game_session(
            &outcome,
            &timer_state_path,
            "2026-06-29T00:00:00Z",
            None,
        )
        .await;
        assert!(matches!(stored, Ok(Some(_))));
        assert!(matches!(
            crate::enforcement_timer_state_file::read_active_timer_state(&timer_state_path).await,
            Ok(Some(_))
        ));
        assert!(
            crate::enforcement_timer_state_file::remove_active_timer_state(&timer_state_path)
                .await
                .is_ok()
        );
    }

    async fn link_activity_surface_store_helpers(store_path: &std::path::Path) {
        if let Some(snapshot) = activity_surface_store::local_store_snapshot().await {
            touch_activity_surface_snapshot(&snapshot);
        }
        if let Some(snapshot) = activity_surface_store::local_store_snapshot_from_path(
            activity_surface_store::ActivityStorePath(store_path.to_path_buf()),
        )
        .await
        {
            touch_activity_surface_snapshot(&snapshot);
        }
        let _ = activity_surface_store::load_browser_model().await;
        let _ = activity_surface_store::load_browser_model_from_path(
            activity_surface_store::ActivityStorePath(store_path.to_path_buf()),
        )
        .await;
        let _ = activity_surface_store::load_network_model().await;
        let _ = activity_surface_store::load_network_model_from_path(
            activity_surface_store::ActivityStorePath(store_path.to_path_buf()),
        )
        .await;
        let _ = activity_surface_store::load_app_game_model().await;
        let _ = activity_surface_store::load_app_game_model_from_path(
            activity_surface_store::ActivityStorePath(store_path.to_path_buf()),
        )
        .await;
        let _ = activity_surface_store::load_screen_summary().await;
        let _ = activity_surface_store::load_screen_summary_from_path(
            activity_surface_store::ActivityStorePath(store_path.to_path_buf()),
        )
        .await;
    }

    fn touch_activity_surface_snapshot(
        snapshot: &activity_surface_store::ActivitySurfaceStoreSnapshot,
    ) {
        let _ = (
            snapshot.device_id.0.as_str(),
            snapshot.last_event_id.as_deref(),
            snapshot.last_observed_at.as_deref(),
            snapshot.recent_returned,
            snapshot.browser_returned,
            snapshot.network_returned,
            snapshot.games_returned,
            snapshot.screen_returned,
        );
    }

    fn command_envelope() -> AgentCommandEnvelope {
        let event_id = crate::activity_api::ActivityEventId("cmd-app-game-pref-linkage");
        let generated_at = crate::activity_api::GeneratedAtText("2026-06-29T00:00:00Z".to_string());
        AgentCommandEnvelope {
            schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
            message_id: event_id.0.to_string(),
            sent_at: generated_at.0,
            source: AgentPeer {
                peer_id: constants::peer::PORTAL_DEV.to_string(),
                role: AgentPeerRole::Portal,
            },
            target: AgentMessageTarget {
                device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                platform: "windows".to_string(),
                route: AgentRoute::Localhost,
            },
            command: AgentCommandName::AgentActivityAppGameTimerParentPreferenceSetupRequest,
            payload: LogFields::new(),
        }
    }

    fn cleanup_path(path: &std::path::Path) {
        let _ = remove_file(path);
        let mut wal_path = path.to_path_buf();
        wal_path.set_file_name(format!(
            "{}.{}",
            path.file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or_default(),
            constants::activity_store::WAL_FILE_EXTENSION
        ));
        let _ = remove_file(wal_path);
        let mut shm_path = path.to_path_buf();
        shm_path.set_file_name(format!(
            "{}.{}",
            path.file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or_default(),
            constants::activity_store::SHM_FILE_EXTENSION
        ));
        let _ = remove_file(shm_path);
    }
}
