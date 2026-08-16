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
#[path = "../../src/time.rs"]
mod time;
mod activity_api {
    #[derive(Clone, Copy)]
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

mod activity_store_error_event {
    use super::activity_api::ActivityEventId;
    use ocentra_parent_agent_protocol::logging::LogLevel;
    use ocentra_parent_agent_protocol::transport::{
        AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
    };

    pub(crate) fn activity_store_error_event(
        command: AgentCommandEnvelope,
        event_id_suffix: ActivityEventId,
        event: AgentEventName,
    ) -> AgentEventEnvelope {
        let ActivityEventId(event_id_suffix) = event_id_suffix;
        crate::event_builder::build_event(
            event_id_suffix,
            &command.message_id,
            command.source,
            event,
            LogLevel::Error,
            crate::activity_payload::activity_store_error_payload(),
            None,
        )
    }
}
#[path = "../../src/activity_api/app_game_timer_parent_surface_action_results.rs"]
mod app_game_timer_parent_surface_action_results;
#[path = "../support/app_game_timer_parent_surface_payload.rs"]
mod app_game_timer_parent_surface_payload;

#[cfg(test)]
mod clippy_linkage {
    use crate::app_game_timer_parent_surface_payload;
    use crate::test_invariants::{
        require_json_decode, require_log_string_field, require_ok, require_some,
        serialize_test_json,
    };
    use ocentra_parent_agent_protocol::activity::ActivityObserver;
    use ocentra_parent_agent_protocol::activity::ActivitySubjectKind;
    use ocentra_parent_agent_protocol::activity_query::{
        ActivityIngestStatus, ActivityRecentSummary,
    };
    use ocentra_parent_agent_protocol::logging::LogFieldValue;
    use ocentra_parent_agent_protocol::transport::{
        AgentCommandEnvelope, AgentEventName, AgentMessageTarget, AgentPeer, AgentPeerRole,
        AgentRoute,
    };

    async fn activity_surface_store_helpers_are_linked() {
        let _ = crate::activity_store_path::activity_db_path();
        let _ = crate::activity_store_path::activity_journal_path();
        let _ = crate::activity_store_path::activity_journal_key_path();
        let _ = crate::activity_payload::ingest_status_payload(&ActivityIngestStatus {
            schema_version: 1,
            database_ready: true,
            events_ingested: 1,
            events_stored: 1,
            duplicate_events: 0,
            last_event_id: Some("event-1".to_string()),
        });
        let _ = crate::activity_payload::recent_summary_payload(&ActivityRecentSummary {
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
        let _ = crate::activity_payload::activity_store_error_payload();
        if let Some(snapshot) = crate::activity_surface_store::local_store_snapshot().await {
            let _ = (
                snapshot.device_id.0.as_str(),
                snapshot.recent_returned,
                snapshot.last_event_id.as_deref(),
                snapshot.last_observed_at.as_deref(),
                snapshot.browser_returned,
                snapshot.network_returned,
                snapshot.games_returned,
                snapshot.screen_returned,
            );
        }
        let store_path = std::path::PathBuf::from(
            "C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/tmp/app-game-surface-clippy.db",
        );
        let _ = crate::activity_surface_store::local_store_snapshot_from_path(
            crate::activity_surface_store::ActivityStorePath(store_path.clone()),
        )
        .await;
        let _ = crate::activity_surface_store::load_browser_model().await;
        let _ = crate::activity_surface_store::load_browser_model_from_path(
            crate::activity_surface_store::ActivityStorePath(store_path.clone()),
        )
        .await;
        let _ = crate::activity_surface_store::load_network_model().await;
        let _ = crate::activity_surface_store::load_network_model_from_path(
            crate::activity_surface_store::ActivityStorePath(store_path.clone()),
        )
        .await;
        let _ = crate::activity_surface_store::load_app_game_model().await;
        let _ = crate::activity_surface_store::load_app_game_model_from_path(
            crate::activity_surface_store::ActivityStorePath(store_path.clone()),
        )
        .await;
        let _ = crate::activity_surface_store::load_screen_summary().await;
        let _ = crate::activity_surface_store::load_screen_summary_from_path(
            crate::activity_surface_store::ActivityStorePath(store_path),
        )
        .await;
        let timer_state_path = crate::enforcement_timer_state_path::enforcement_timer_state_path();
        let outcome = crate::timer_state_fixture::outcome();
        assert!(matches!(
            crate::enforcement_timer_state_file::store_active_timer_state_for_outcome_with_app_game_session(
                &outcome,
                &timer_state_path,
                "2026-06-29T00:00:00Z",
                None,
            )
            .await,
            Ok(Some(_))
        ));
        let _ =
            crate::enforcement_timer_state_file::read_active_timer_state(&timer_state_path).await;
        let _ =
            crate::enforcement_timer_state_file::remove_active_timer_state(&timer_state_path).await;
        let _ = crate::event_builder::portal_peer();
        let _ = crate::json_contract::serialize_json_value(serde_json::json!({
            "app_game_timer_parent_surface": true
        }));
        let _: String = crate::time::timestamp_after_epoch_seconds(1, 0);
        let _: String = crate::time::timestamp_after_epoch_seconds(1, 1);
        if let Some(timer_state_dir) = timer_state_path.parent_dir() {
            let _ = timer_state_dir.create_all();
        }
    }

    #[tokio::test]
    async fn test_invariants_are_linked() {
        let generated_at = crate::activity_api::GeneratedAtText("2026-06-29T00:00:00Z".to_string());
        let encoded = serialize_test_json(&serde_json::json!({
            "app_game_timer_parent_surface": true
        }));
        let decoded: serde_json::Value =
            require_json_decode(&encoded, "app_game_timer_parent_surface linkage json");
        assert!(require_some(
            decoded
                .get("app_game_timer_parent_surface")
                .and_then(|value| value.as_bool()),
            "app_game_timer_parent_surface linkage bool",
        ));

        let json_text = crate::json_contract::serialize_json_string(&serde_json::json!({
            "app_game_timer_parent_surface": true
        }));
        let field = LogFieldValue::String(json_text.0);
        let text =
            require_log_string_field(Some(&field), "app_game_timer_parent_surface linkage field");
        let _: serde_json::Value =
            require_json_decode(text, "app_game_timer_parent_surface linkage field json");
        let _: () = require_ok(
            Ok::<(), std::io::Error>(()),
            "app_game_timer_parent_surface linkage ok",
        );
        activity_surface_store_helpers_are_linked().await;

        let event = app_game_timer_parent_surface_payload::build_activity_app_game_timer_parent_surface_report(
            AgentCommandEnvelope {
                schema_version: ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION,
                message_id: "cmd-app-game-surface-clippy".to_string(),
                sent_at: generated_at.0,
                source: AgentPeer {
                    peer_id: ocentra_parent_agent_protocol::constants::peer::PORTAL_DEV
                        .to_string(),
                    role: AgentPeerRole::Portal,
                },
                target: AgentMessageTarget {
                    device_id:
                        ocentra_parent_agent_protocol::constants::peer::LOCAL_DEV_AGENT.to_string(),
                    platform: "windows".to_string(),
                    route: AgentRoute::Localhost,
                },
                command:
                    ocentra_parent_agent_protocol::transport::AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet,
                payload: ocentra_parent_agent_protocol::logging::LogFields::new(),
            },
        )
        .await;
        assert_eq!(
            event.event,
            AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported
        );
    }
}
