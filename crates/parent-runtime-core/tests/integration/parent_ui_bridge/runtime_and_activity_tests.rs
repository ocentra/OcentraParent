use std::time::Duration;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use serde_json::json;

use super::tests_support::{
    require_ok, start_local_server_with_capture_responses, with_agent_addr,
};
use super::{
    dispatch_parent_ui_action, load_parent_route_snapshot, ParentRouteId, ParentUiAction,
    ParentUiActionKind,
};

use super::common::events::responses::screen_settings_response_event;
use super::common::events::responses::*;
use super::common::events::samples::*;
use super::common::events::tracking::*;
use super::common::helpers::*;

#[test]
fn screen_settings_actions_attach_runtime_service_response_snapshot() {
    let (address, capture) = start_local_server_with_capture_responses(vec![
        screen_settings_response_event(
            PayloadText("screen-settings-request-9".to_string()),
            PayloadText("get".to_string()),
            AgentEventName::AgentScreenSettingsReported,
            PayloadText("accepted".to_string()),
            None,
        ),
        screen_settings_response_event(
            PayloadText("screen-settings-request-10".to_string()),
            PayloadText("replace".to_string()),
            AgentEventName::AgentScreenSettingsReplaceRejected,
            PayloadText("rejected".to_string()),
            Some(PayloadText("stale-revision".to_string())),
        ),
    ]);
    let get_action = ParentUiAction {
        action: ParentUiActionKind::ScreenSettingsGetRequested,
        route: ParentRouteId::SettingsRules,
        command: None,
        payload: json!({
            "screenSettingsRequest": "{\"schemaVersion\":1,\"requestId\":\"screen-settings-request-9\",\"kind\":\"get\"}",
            "screenSettingsUpdateKind": "get"
        }),
        context: None,
    };
    let replace_action = ParentUiAction {
        action: ParentUiActionKind::ScreenSettingsReplaceRequested,
        route: ParentRouteId::SettingsRules,
        command: None,
        payload: json!({
            "screenSettingsRequest": "{\"schemaVersion\":1,\"requestId\":\"screen-settings-request-10\",\"kind\":\"replace\",\"baseSettingVersion\":7,\"setting\":null}",
            "screenSettingsUpdateKind": "replace"
        }),
        context: None,
    };
    let get_result = with_agent_addr(&address, || dispatch_parent_ui_action(&get_action));
    let replace_result = with_agent_addr(&address, || dispatch_parent_ui_action(&replace_action));
    let get_request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured screen settings get command arrives",
    );
    let replace_request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured screen settings replace command arrives",
    );

    assert_eq!(
        get_request.command["command"],
        json!("agent.screen-settings.get")
    );
    assert_eq!(
        replace_request.command["command"],
        json!("agent.screen-settings.replace")
    );

    let get_snapshot = require_some(
        get_result.snapshot,
        TestContext("screen settings get returns snapshot"),
    );
    assert_json_field_eq(
        get_snapshot.screen_settings_service_response.as_ref(),
        TestLabel("requestId"),
        TestValue("screen-settings-request-9"),
    );
    assert_json_field_eq(
        get_snapshot.screen_settings_service_response.as_ref(),
        TestLabel("status"),
        TestValue("accepted"),
    );

    let replace_snapshot = require_some(
        replace_result.snapshot,
        TestContext("screen settings replace returns snapshot"),
    );
    assert_json_field_eq(
        replace_snapshot.screen_settings_service_response.as_ref(),
        TestLabel("requestId"),
        TestValue("screen-settings-request-10"),
    );
    assert_json_field_eq(
        replace_snapshot.screen_settings_service_response.as_ref(),
        TestLabel("status"),
        TestValue("rejected"),
    );
    assert_json_field_eq(
        replace_snapshot.screen_settings_service_response.as_ref(),
        TestLabel("rejectionReason"),
        TestValue("stale-revision"),
    );
}

#[test]
fn policy_preview_real_conflict_reasons_are_projected_before_manual_review() {
    for (field, finding_kind) in [
        (
            constants::field::POLICY_PREVIEW_FINDING_KINDS,
            "overlapping-schedule",
        ),
        (
            constants::field::POLICY_PREVIEW_FINDING_KINDS,
            "ambiguous-local-time",
        ),
        (
            constants::field::POLICY_PREVIEW_TARGET_EXPLANATION_CODE,
            "schedule-timezone-boundary",
        ),
        (
            constants::field::POLICY_PREVIEW_TARGET_EXPLANATION_CODE,
            "nonexistent-local-time",
        ),
        (
            constants::field::POLICY_PREVIEW_TARGET_EXPLANATION_CODE,
            "clock-skew",
        ),
    ] {
        let card = policy_preview_attention_card(&[
            (constants::field::POLICY_PREVIEW_SAVE_STATE, "blocked"),
            (
                constants::field::POLICY_PREVIEW_MANUAL_REVIEW_STATE,
                "required",
            ),
            (field, finding_kind),
        ]);
        assert_eq!(
            card,
            json!({
                "title": "Parent attention",
                "summary": "Conflict requires parent review before this preview can be saved.",
                "details": [
                    { "label": "Attention type", "value": "Conflict" },
                    { "label": "Conflict evidence", "value": finding_kind },
                    { "label": "Save state", "value": "Blocked" }
                ]
            })
        );
    }
}

#[test]
fn policy_preview_unsupported_target_precedes_manual_review_attention() {
    let card = policy_preview_attention_card(&[
        (constants::field::POLICY_PREVIEW_SAVE_STATE, "blocked"),
        (
            constants::field::POLICY_PREVIEW_MANUAL_REVIEW_STATE,
            "required",
        ),
        (constants::field::POLICY_PREVIEW_TARGET_STATE, "unsupported"),
    ]);
    assert_eq!(
        card["summary"],
        json!("This target is unsupported and cannot be saved from this policy path.")
    );
    assert_eq!(
        card["details"][0],
        json!({ "label": "Attention type", "value": "Unsupported target" })
    );
}

#[test]
fn policy_preview_offline_and_stale_target_attention_precedes_manual_review_without_save_state() {
    for target_state in ["offline", "stale"] {
        let card = policy_preview_attention_card(&[
            (constants::field::POLICY_PREVIEW_TARGET_STATE, target_state),
            (
                constants::field::POLICY_PREVIEW_MANUAL_REVIEW_STATE,
                "required",
            ),
        ]);
        assert_eq!(card["title"], json!("Parent attention"));
        assert_eq!(
            card["details"][0],
            json!({ "label": "Attention type", "value": format!("Target {target_state}") })
        );
        assert_eq!(
            card["details"][1]["value"]
                .as_str()
                .map(str::to_ascii_lowercase),
            Some(target_state.to_string())
        );
        assert_eq!(
            card["details"][2],
            json!({ "label": "Save state", "value": "Not reported" })
        );
    }
}

#[test]
fn tracking_retention_settings_write_action_results_project_into_live_activity_snapshot() {
    let (address, capture) = start_local_server_with_capture_responses(vec![
        tracking_retention_settings_write_response_event(),
    ]);
    let tracking_result = with_agent_addr(&address, || {
        dispatch_parent_ui_action(&empty_action(
            ParentUiActionKind::TrackingRetentionSettingsWriteRequested,
            ParentRouteId::Activity,
        ))
    });
    let request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured tracking retention write command arrives",
    );

    assert_eq!(
        request.command["command"],
        json!("agent.activity.tracking.retention-settings.write")
    );

    let tracking_live_activity = require_result_live_activity(
        &tracking_result,
        TestContext("tracking retention action returns snapshot"),
        TestContext("tracking retention action returns live activity snapshot"),
    );
    assert_json_field_eq(
        tracking_live_activity
            .activity_tracking_retention_settings_write_result
            .as_ref(),
        TestLabel("commandId"),
        TestValue("tracking-retention-settings-write-command"),
    );
    assert_json_field_eq(
        tracking_live_activity
            .activity_tracking_retention_settings_write_result
            .as_ref(),
        TestLabel("writeState"),
        TestValue("service-write-command-accepted"),
    );
}

#[test]
fn app_game_adapter_dispatch_execute_action_results_project_into_live_activity_snapshot() {
    let mut responses = vec![app_game_adapter_dispatch_execute_response_event(
        PayloadText("latest-execute-command".to_string()),
    )];
    responses.extend(app_game_route_load_response_events());
    let (address, capture) = start_local_server_with_capture_responses(responses);
    let app_game_dispatch_result = with_agent_addr(&address, || {
        dispatch_parent_ui_action(&empty_action(
            ParentUiActionKind::AppGameAdapterDispatchExecuteRequested,
            ParentRouteId::AppGameSessions,
        ))
    });
    let requests = capture_app_game_dispatch_requests(&capture);
    assert_app_game_dispatch_request_commands(&requests);

    let app_game_dispatch_live_activity = require_result_live_activity(
        &app_game_dispatch_result,
        TestContext("app-game dispatch execute action returns snapshot"),
        TestContext("app-game dispatch execute action returns live activity snapshot"),
    );
    let app_game_dispatch_panel = require_some(
        app_game_dispatch_live_activity
            .app_game_adapter_dispatch_panel
            .as_ref(),
        TestContext("app-game dispatch execute action returns adapter dispatch panel"),
    );
    let app_game_dispatch_panel_json = serialize_json(
        app_game_dispatch_panel,
        TestContext("adapter dispatch panel serializes"),
    );
    assert_panel_detail_value(
        &app_game_dispatch_panel_json["resultPanel"]["summaryDetails"],
        TestLabel("Execute command"),
        TestValue("latest-execute-command"),
    );
    assert_panel_detail_value(
        &app_game_dispatch_panel_json["resultPanel"]["summaryDetails"],
        TestLabel("Execute status"),
        TestValue("actually-enforced"),
    );
    assert_panel_detail_value(
        &app_game_dispatch_panel_json["resultPanel"]["summaryDetails"],
        TestLabel("Adapter execution status"),
        TestValue("process-already-exited"),
    );
    assert_eq!(
        serialize_json(
            require_some(
                app_game_dispatch_live_activity
                    .app_game_policy_readiness_panel
                    .as_ref(),
                TestContext("app-game dispatch returns policy panel"),
            ),
            TestContext("policy panel serializes"),
        )["rows"][0]["title"],
        json!("Policy evidence")
    );
    assert_eq!(
        serialize_json(
            require_some(
                app_game_dispatch_live_activity
                    .app_game_notification_parent_surface_panel
                    .as_ref(),
                TestContext("app-game dispatch returns notification panel"),
            ),
            TestContext("notification panel serializes"),
        )["rows"][0]["key"],
        json!("notification-ready-row")
    );
}

#[test]
fn app_game_timer_parent_preference_setup_action_results_project_into_live_activity_snapshot() {
    let mut responses = vec![app_game_timer_parent_preference_setup_requested_response_event()];
    responses.extend(app_game_route_load_response_events());
    let (address, capture) = start_local_server_with_capture_responses(responses);
    let app_game_timer_result = with_agent_addr(&address, || {
        dispatch_parent_ui_action(&empty_action(
            ParentUiActionKind::AppGameTimerParentPreferenceSetupRequested,
            ParentRouteId::AppGameSessions,
        ))
    });
    let requests = capture_app_game_timer_requests(&capture);
    assert_app_game_timer_request_commands(&requests);

    let command_result_projection = require_some(
        app_game_timer_result
            .events
            .iter()
            .find_map(|event| event.command_result_projection.as_ref()),
        TestContext("app-game timer response projects Rust-owned command-result details"),
    );
    assert_eq!(
        command_result_projection.projection_kind,
        "app-game-timer-parent-preference-setup"
    );
    assert_eq!(command_result_projection.details[0].label, "Status");
    assert_eq!(command_result_projection.details[0].value, "Ready");
    assert!(command_result_projection.details.iter().any(|detail| {
        detail.label == "Provider delivery aggregate status"
            && detail.value.starts_with("Manual provider setup required")
    }));

    let app_game_timer_live_activity = require_result_live_activity(
        &app_game_timer_result,
        TestContext("app-game timer parent preference setup action returns snapshot"),
        TestContext("app-game timer parent preference setup action returns live activity snapshot"),
    );
    let app_game_timer_panel = require_some(
        app_game_timer_live_activity
            .app_game_timer_parent_surface_panel
            .as_ref(),
        TestContext("app-game timer action returns timer parent surface panel"),
    );
    assert_eq!(app_game_timer_panel.load_state, "unavailable");
    assert!(app_game_timer_panel.parent_preference_setup_rows.is_empty());
    assert_eq!(
        serialize_json(
            require_some(
                app_game_timer_live_activity
                    .app_game_platform_proof_status_panel
                    .as_ref(),
                TestContext("app-game timer returns platform panel"),
            ),
            TestContext("platform panel serializes"),
        )["rows"][0]["title"],
        json!("Windows")
    );
    assert_eq!(
        serialize_json(
            require_some(
                app_game_timer_live_activity
                    .app_game_child_runtime_transport_receipt_panel
                    .as_ref(),
                TestContext("app-game timer returns transport panel"),
            ),
            TestContext("child runtime transport panel serializes"),
        )["rows"][0]["title"],
        json!("transport-receipt-row-1")
    );
}

#[test]
fn screen_analysis_route_load_attaches_rust_owned_screen_summary_panel() {
    let (address, capture) =
        start_local_server_with_capture_responses(vec![screen_read_model_response_event()]);
    let value = with_agent_addr(&address, || {
        route_snapshot_json(
            ParentRouteId::ScreenAnalysis,
            TestContext("screen-analysis route snapshot serializes"),
        )
    });
    let request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured screen-analysis read-model load arrives",
    );

    assert_eq!(
        request.command["command"],
        json!("agent.activity.screen.read-model.get")
    );
    assert!(value["liveActivity"]["activityScreenReadModel"].is_null());
    assert_eq!(
        value["liveActivity"]["screenSummaryPanel"]["rows"][0]["title"],
        json!("screen-ready-row")
    );
    assert_panel_detail_value(
        &value["liveActivity"]["screenSummaryPanel"]["summaryDetails"],
        TestLabel("Status"),
        TestValue("Ready"),
    );
    assert_panel_detail_value(
        &value["liveActivity"]["screenSummaryPanel"]["rows"][0]["details"],
        TestLabel("Event ID"),
        TestValue("screen-ready-row-1"),
    );
}

#[test]
fn app_game_sessions_route_load_attaches_rust_owned_app_game_panels() {
    let (address, capture) =
        start_local_server_with_capture_responses(app_game_route_load_response_events());
    let value = with_agent_addr(&address, || {
        route_snapshot_json(
            ParentRouteId::AppGameSessions,
            TestContext("app-game sessions route snapshot serializes"),
        )
    });
    assert_app_game_route_load_requests(&capture);
    assert_eq!(
        value["liveActivity"]["appGameNotificationParentSurfacePanel"]["rows"][0]["key"],
        json!("notification-ready-row")
    );
    assert_eq!(
        value["liveActivity"]["appGamePolicyReadinessPanel"]["rows"][0]["title"],
        json!("Policy evidence")
    );
    assert_eq!(
        value["liveActivity"]["appGamePlatformProofStatusPanel"]["rows"][0]["title"],
        json!("Windows")
    );
    assert_eq!(
        value["liveActivity"]["appGameChildRuntimeTransportReceiptPanel"]["rows"][0]["title"],
        json!("transport-receipt-row-1")
    );
}

#[test]
fn proof_panels_route_load_attaches_network_flow_read_model() {
    let (address, capture) = start_local_server_with_capture_responses(vec![
        network_flow_response_event(),
        network_runtime_event_chain_response_event(),
        policy_preview_response_event(),
        tracking_read_model_response_event(),
    ]);
    let value = with_agent_addr(&address, || {
        route_snapshot_json(
            ParentRouteId::ProofPanels,
            TestContext("proof panels snapshot serializes"),
        )
    });
    let request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured proof-panels network flow load arrives",
    );
    let runtime_stream_request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured proof-panels network runtime load arrives",
    );
    let policy_preview_request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured proof-panels policy preview load arrives",
    );
    let tracking_request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured proof-panels tracking read model load arrives",
    );

    let mut commands = vec![
        command_text(
            &request.command["command"],
            TestContext("proof panels network flow command is a string"),
        )
        .0,
        command_text(
            &runtime_stream_request.command["command"],
            TestContext("proof panels network runtime command is a string"),
        )
        .0,
        command_text(
            &policy_preview_request.command["command"],
            TestContext("proof panels policy preview command is a string"),
        )
        .0,
        command_text(
            &tracking_request.command["command"],
            TestContext("proof panels tracking command is a string"),
        )
        .0,
    ];
    commands.sort();
    assert_eq!(
        commands,
        vec![
            "agent.activity.tracking.read-model.get".to_string(),
            "agent.network.flow.read-model.get".to_string(),
            "agent.network.runtime.event-chain.stream.get".to_string(),
            "agent.policy.preview.read-model.get".to_string(),
        ]
    );
    assert_eq!(
        value["liveActivity"]["networkFlowReadModel"]["rows"][0]["eventId"],
        json!("network-ui-flow-1")
    );
    assert_eq!(
        value["liveActivity"]["networkFlowReadModel"]["rows"][0]["evidence"][0]["evidenceId"],
        json!("network-ui-evidence-1")
    );
    assert_network_policy_bridge_snapshot(&value["liveActivity"], &json!(3));
    assert_eq!(
        value["liveActivity"]["activityTrackingReadModel"]["ok"],
        json!(true)
    );
    assert_eq!(
        value["liveActivity"]["activityTrackingReadModel"]["value"]["rows"][0]["deviceId"],
        json!("child-device-1")
    );
    assert_eq!(
        value["liveActivity"]["activityTrackingReadModelEvent"]["event"],
        json!("agent.activity.tracking.read-model.reported")
    );
    assert_eq!(
        value["liveActivity"]["activityTrackingPanel"]["summaryCards"][0]["title"],
        json!("Tracking live summary")
    );
    assert_eq!(
        value["liveActivity"]["activityTrackingPanel"]["cards"][0]["title"],
        json!("Family dashboard tracking rollup")
    );
}

#[test]
fn policy_tracking_route_load_attaches_tracking_read_model() {
    let (address, capture) =
        start_local_server_with_capture_responses(vec![tracking_read_model_response_event()]);
    let value = with_agent_addr(&address, || {
        route_snapshot_json(
            ParentRouteId::PolicyTracking,
            TestContext("policy-tracking route snapshot serializes"),
        )
    });
    let tracking_request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured policy-tracking tracking read model load arrives",
    );

    assert_eq!(
        tracking_request.command["command"],
        json!("agent.activity.tracking.read-model.get")
    );
    assert_eq!(value["route"], "policy-tracking");
    assert_eq!(value["dataSource"], "host-bridge");
    assert_eq!(value["connectionState"], "connected");
    assert!(value["liveActivity"]["lanAddDeviceReadModel"].is_null());
    assert_eq!(
        value["liveActivity"]["activityTrackingReadModel"]["ok"],
        json!(true)
    );
    assert_eq!(
        value["liveActivity"]["activityTrackingReadModel"]["value"]["rows"][0]["deviceId"],
        json!("child-device-1")
    );
    assert_eq!(
        value["liveActivity"]["activityTrackingReadModelEvent"]["event"],
        json!("agent.activity.tracking.read-model.reported")
    );
}

#[test]
fn network_flow_refresh_action_attaches_runtime_backed_snapshot() {
    let (address, capture) = start_local_server_with_capture_responses(vec![
        network_flow_response_event(),
        network_runtime_event_chain_response_event(),
        policy_preview_response_event(),
    ]);
    let result = with_agent_addr(&address, || {
        dispatch_parent_ui_action(&empty_action(
            ParentUiActionKind::NetworkFlowReadModelRefreshRequested,
            ParentRouteId::Activity,
        ))
    });
    let request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured network flow refresh action arrives",
    );
    let runtime_stream_request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured network runtime chain refresh action arrives",
    );
    let policy_preview_request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured policy preview refresh action arrives",
    );
    let mut commands = vec![
        command_text(
            &request.command["command"],
            TestContext("network flow command is a string"),
        )
        .0,
        command_text(
            &runtime_stream_request.command["command"],
            TestContext("network runtime command is a string"),
        )
        .0,
        command_text(
            &policy_preview_request.command["command"],
            TestContext("policy preview command is a string"),
        )
        .0,
    ];
    commands.sort();

    assert!(result.accepted);
    assert_eq!(
        commands,
        vec![
            "agent.network.flow.read-model.get".to_string(),
            "agent.network.runtime.event-chain.stream.get".to_string(),
            "agent.policy.preview.read-model.get".to_string(),
        ]
    );
    assert_eq!(
        result_network_flow_row_event_id(&result),
        Some(CommandText("network-ui-flow-1".to_string()))
    );
    let live_activity = serialize_json(
        require_result_live_activity(
            &result,
            TestContext("network flow refresh returns snapshot"),
            TestContext("network flow refresh returns live activity snapshot"),
        ),
        TestContext("live activity snapshot serializes"),
    );
    assert_network_policy_bridge_snapshot(&live_activity, &json!(3));
}

fn capture_app_game_dispatch_requests(
    capture: &std::sync::mpsc::Receiver<super::tests_support::CapturedLanRequest>,
) -> [super::tests_support::CapturedLanRequest; 10] {
    [
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game dispatch execute command arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-use load after dispatch arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured games load after dispatch arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game notification readiness load after dispatch arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game policy readiness load after dispatch arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game platform proof status load after dispatch arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game child transport receipt load after dispatch arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game adapter dispatch preflight load after dispatch arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game adapter dispatch result load after dispatch arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game timer parent surface load after dispatch arrives",
        ),
    ]
}

fn assert_app_game_route_load_requests(
    capture: &std::sync::mpsc::Receiver<super::tests_support::CapturedLanRequest>,
) {
    let mut commands = (0..9)
        .map(|_| {
            let request = require_ok(
                capture.recv_timeout(Duration::from_secs(1)),
                "captured app-game route load request arrives",
            );
            command_text(
                &request.command["command"],
                TestContext("app-game route load command is a string"),
            )
            .0
        })
        .collect::<Vec<_>>();
    commands.sort();
    assert_eq!(
        commands,
        vec![
            "agent.activity.app-game.adapter-dispatch-preflight.read-model.get".to_string(),
            "agent.activity.app-game.adapter-dispatch-result.read-model.get".to_string(),
            "agent.activity.app-game.child-runtime-transport-receipt.read-model.get".to_string(),
            "agent.activity.app-game.notification-readiness.read-model.get".to_string(),
            "agent.activity.app-game.platform-proof-status.read-model.get".to_string(),
            "agent.activity.app-game.policy-readiness.read-model.get".to_string(),
            "agent.activity.app-game.timer-parent-surface.read-model.get".to_string(),
            "agent.activity.app-use.read-model.get".to_string(),
            "agent.activity.games.read-model.get".to_string(),
        ]
    );
}

fn assert_app_game_dispatch_request_commands(
    requests: &[super::tests_support::CapturedLanRequest; 10],
) {
    for (request, expected_command) in [
        (
            &requests[0],
            "agent.activity.app-game.adapter-dispatch.execute",
        ),
        (&requests[1], "agent.activity.app-use.read-model.get"),
        (&requests[2], "agent.activity.games.read-model.get"),
        (
            &requests[3],
            "agent.activity.app-game.notification-readiness.read-model.get",
        ),
        (
            &requests[4],
            "agent.activity.app-game.policy-readiness.read-model.get",
        ),
        (
            &requests[5],
            "agent.activity.app-game.platform-proof-status.read-model.get",
        ),
        (
            &requests[6],
            "agent.activity.app-game.child-runtime-transport-receipt.read-model.get",
        ),
        (
            &requests[7],
            "agent.activity.app-game.adapter-dispatch-preflight.read-model.get",
        ),
        (
            &requests[8],
            "agent.activity.app-game.adapter-dispatch-result.read-model.get",
        ),
        (
            &requests[9],
            "agent.activity.app-game.timer-parent-surface.read-model.get",
        ),
    ] {
        assert_eq!(request.command["command"], json!(expected_command));
    }
}

fn capture_app_game_timer_requests(
    capture: &std::sync::mpsc::Receiver<super::tests_support::CapturedLanRequest>,
) -> [super::tests_support::CapturedLanRequest; 10] {
    [
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game parent preference setup command arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-use load after timer setup arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured games load after timer setup arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game notification readiness load after timer setup arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game policy readiness load after timer setup arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game platform proof status load after timer setup arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game child transport receipt load after timer setup arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game adapter dispatch preflight load after timer setup arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game adapter dispatch result load after timer setup arrives",
        ),
        require_ok(
            capture.recv_timeout(Duration::from_secs(1)),
            "captured app-game timer parent surface load after timer setup arrives",
        ),
    ]
}

fn assert_app_game_timer_request_commands(
    requests: &[super::tests_support::CapturedLanRequest; 10],
) {
    for (request, expected_command) in [
        (
            &requests[0],
            "agent.activity.app-game.timer-parent-surface.parent-preference-setup.request",
        ),
        (&requests[1], "agent.activity.app-use.read-model.get"),
        (&requests[2], "agent.activity.games.read-model.get"),
        (
            &requests[3],
            "agent.activity.app-game.notification-readiness.read-model.get",
        ),
        (
            &requests[4],
            "agent.activity.app-game.policy-readiness.read-model.get",
        ),
        (
            &requests[5],
            "agent.activity.app-game.platform-proof-status.read-model.get",
        ),
        (
            &requests[6],
            "agent.activity.app-game.child-runtime-transport-receipt.read-model.get",
        ),
        (
            &requests[7],
            "agent.activity.app-game.adapter-dispatch-preflight.read-model.get",
        ),
        (
            &requests[8],
            "agent.activity.app-game.adapter-dispatch-result.read-model.get",
        ),
        (
            &requests[9],
            "agent.activity.app-game.timer-parent-surface.read-model.get",
        ),
    ] {
        assert_eq!(request.command["command"], json!(expected_command));
    }
}

fn result_network_flow_row_event_id(result: &super::ParentUiActionResult) -> Option<CommandText> {
    result
        .snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.live_activity.as_ref())
        .and_then(|live_activity| live_activity.network_flow_read_model.as_ref())
        .and_then(|read_model| read_model.rows.first())
        .map(|row| CommandText(row.event_id.to_string()))
}

fn empty_action(action: ParentUiActionKind, route: ParentRouteId) -> ParentUiAction {
    ParentUiAction {
        action,
        route,
        command: None,
        payload: json!({}),
        context: None,
    }
}

fn route_snapshot_json(route: ParentRouteId, label: TestContext) -> serde_json::Value {
    require_ok(
        serde_json::to_value(load_parent_route_snapshot(route, None)),
        label.0,
    )
}

fn policy_preview_attention_card(fields: &[(&str, &str)]) -> serde_json::Value {
    let mut response = policy_preview_response_event();
    for (field, value) in fields {
        response.payload.insert(
            (*field).to_string(),
            LogFieldValue::String((*value).to_string()),
        );
    }
    let (address, capture) = start_local_server_with_capture_responses(vec![response]);
    let snapshot = with_agent_addr(&address, || {
        route_snapshot_json(
            ParentRouteId::Approvals,
            TestContext("policy preview attention route serializes"),
        )
    });
    let request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured policy preview attention load arrives",
    );
    assert_eq!(
        request.command["command"],
        json!("agent.policy.preview.read-model.get")
    );
    snapshot["liveActivity"]["policyPreviewPanel"]["cards"][0].clone()
}
