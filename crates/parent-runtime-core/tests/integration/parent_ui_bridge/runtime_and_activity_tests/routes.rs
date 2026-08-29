use super::support::route_snapshot_json as local_route_snapshot_json;
use super::support::*;
use super::timer_support::*;
use super::*;

#[test]
fn screen_analysis_route_load_attaches_rust_owned_screen_summary_panel() {
    let (address, capture) =
        start_local_server_with_capture_responses(vec![screen_read_model_response_event()]);
    let value = with_agent_addr(&address, || {
        local_route_snapshot_json(
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
        local_route_snapshot_json(
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
        value["liveActivity"]["activityAppGamePlatformExtensionReadModel"]["ok"],
        json!(true)
    );
    assert_eq!(
        value["liveActivity"]["activityAppGamePlatformExtensionReadModel"]["value"]["state"],
        json!("manual-required")
    );
    assert_eq!(
        value["liveActivity"]["activityAppGamePlatformExtensionReadModel"]["value"]["rows"]
            .as_array()
            .map(Vec::len),
        Some(4)
    );
    assert_eq!(
        value["liveActivity"]["activityAppGamePlatformExtensionReadModel"]["value"]["rows"][0]
            ["platform"],
        json!("android")
    );
    assert_eq!(
        value["liveActivity"]["activityAppGamePlatformExtensionReadModel"]["value"]["rows"][0]
            ["setupState"],
        json!("manual-required")
    );
    assert_eq!(
        value["liveActivity"]["activityAppGamePlatformExtensionReadModel"]["value"]["rows"][0]
            ["proofPackState"],
        json!("manual-proof-pack-required")
    );
    assert_eq!(
        value["liveActivity"]["activityAppGamePlatformExtensionReadModel"]["value"]["rows"][0]
            ["adapterExecutionClaim"],
        json!("not-executed")
    );
    assert_eq!(
        value["liveActivity"]["activityAppGamePlatformExtensionReadModel"]["value"]["rows"][3]
            ["platform"],
        json!("ios")
    );
    assert_eq!(
        value["liveActivity"]["appGameChildRuntimeTransportReceiptPanel"]["rows"][0]["title"],
        json!("transport-receipt-row-1")
    );
    assert_eq!(
        value["liveActivity"]["appGameTimerParentSurfacePanel"]["loadState"],
        json!("unavailable")
    );
    assert_eq!(
        value["liveActivity"]["appGameTimerParentSurfacePanel"]["rows"],
        json!([])
    );
    assert_eq!(
        value["liveActivity"]["appGameTimerParentSurfacePanel"]["parentActionRows"],
        json!([])
    );
    assert_eq!(
        value["liveActivity"]["appGameTimerParentSurfacePanel"]["parentPreferenceSetupRows"],
        json!([])
    );
}

#[test]
fn app_game_timer_parent_surface_response_consumer_projects_valid_rows() {
    let mut responses = app_game_route_load_response_events();
    replace_timer_parent_surface_response(
        &mut responses,
        app_game_timer_parent_surface_response_with_valid_row(),
    );
    let (address, capture) = start_local_server_with_capture_responses(responses);
    let value = with_agent_addr(&address, || {
        local_route_snapshot_json(
            ParentRouteId::AppGameSessions,
            TestContext("valid timer parent-surface response serializes"),
        )
    });
    assert_app_game_route_load_requests(&capture);

    let panel = &value["liveActivity"]["appGameTimerParentSurfacePanel"];
    assert_eq!(panel["loadState"], json!("ready"));
    assert_eq!(panel["rows"].as_array().expect("rows array").len(), 1);
    assert_eq!(
        panel["rows"][0]["title"],
        json!("timer-parent-surface-row-1")
    );
    assert_panel_detail_value(
        &panel["rows"][0]["details"],
        TestLabel("Target"),
        TestValue("Native game"),
    );
    assert_panel_detail_value(
        &panel["rows"][0]["details"],
        TestLabel("Status"),
        TestValue("Ready for parent surface"),
    );
    assert_panel_detail_value(
        &panel["rows"][0]["details"],
        TestLabel("Row count"),
        TestValue("2"),
    );
    assert_panel_detail_value(
        &panel["rows"][0]["details"],
        TestLabel("Evidence references"),
        TestValue("evidence.timer.1 | evidence.timer.2"),
    );
    assert_eq!(panel["parentActionRows"], json!([]));
    assert_eq!(panel["parentPreferenceSetupRows"], json!([]));
}

#[test]
fn app_game_timer_parent_surface_response_consumer_fails_closed_on_invalid_rows() {
    for response in [
        app_game_timer_parent_surface_response_with_unknown_target(),
        app_game_timer_parent_surface_response_with_unknown_state(),
        app_game_timer_parent_surface_response_with_inconsistent_counts(),
        app_game_timer_parent_surface_response_with_state_count_mismatch(),
        app_game_timer_parent_surface_response_with_duplicate_row_id(),
        app_game_timer_parent_surface_response_with_duplicate_evidence_id(),
        app_game_timer_parent_surface_response_with_unowned_runtime_claim(),
        app_game_timer_parent_surface_response_with_unowned_adapter_dispatch_claim(),
        app_game_timer_parent_surface_response_with_unowned_platform_enforcement_claim(),
        nested_claim_violation("artifact", "adapterDispatchClaimed"),
        nested_claim_violation("artifact", "platformEnforcementClaimed"),
        nested_claim_violation("intent", "adapterDispatchClaimed"),
        nested_claim_violation("intent", "platformEnforcementClaimed"),
        nested_claim_violation("preference", "adapterDispatchClaimed"),
        nested_claim_violation("preference", "platformEnforcementClaimed"),
        local_artifact_mismatch("childUxHandoffReadyCount"),
        local_artifact_mismatch("childUxLocalHandoffArtifactRecordCount"),
        local_artifact_mismatch("childUxLocalHandoffArtifactReferenceIds"),
        local_artifact_invalid_record("sourceResultId"),
        local_artifact_invalid_record("artifactReferenceId"),
        local_artifact_invalid_record("childReasonReferenceIds"),
        local_artifact_invalid_record("childStatusReferenceIds"),
        local_artifact_duplicate_refs("childReasonReferenceIds"),
        local_artifact_duplicate_refs("childStatusReferenceIds"),
    ] {
        let mut responses = app_game_route_load_response_events();
        replace_timer_parent_surface_response(&mut responses, response);
        let (address, capture) = start_local_server_with_capture_responses(responses);
        let value = with_agent_addr(&address, || {
            local_route_snapshot_json(
                ParentRouteId::AppGameSessions,
                TestContext("invalid timer parent-surface response serializes"),
            )
        });
        assert_app_game_route_load_requests(&capture);

        let panel = &value["liveActivity"]["appGameTimerParentSurfacePanel"];
        assert_eq!(panel["loadState"], json!("unavailable"));
        assert_eq!(panel["rows"], json!([]));
        assert_eq!(panel["parentActionRows"], json!([]));
        assert_eq!(panel["parentPreferenceSetupRows"], json!([]));
    }
}

#[test]
fn app_game_timer_parent_surface_response_consumer_renders_local_artifact_record() {
    let mut responses = app_game_route_load_response_events();
    replace_timer_parent_surface_response(
        &mut responses,
        app_game_timer_parent_surface_response_with_local_artifact(),
    );
    let (address, capture) = start_local_server_with_capture_responses(responses);
    let value = with_agent_addr(&address, || {
        local_route_snapshot_json(
            ParentRouteId::AppGameSessions,
            TestContext("valid local artifact response serializes"),
        )
    });
    assert_app_game_route_load_requests(&capture);
    let row = &value["liveActivity"]["appGameTimerParentSurfacePanel"]["localHandoffArtifactRows"][0];
    assert_eq!(row["title"], json!("result-1"));
    assert_panel_detail_value(&row["details"], TestLabel("Target"), TestValue("Native app"));
    assert_panel_detail_value(&row["details"], TestLabel("Child reason refs"), TestValue("reason-1"));
    assert_panel_detail_value(&row["details"], TestLabel("Child status refs"), TestValue("status-1"));
    assert_panel_detail_value(&row["details"], TestLabel("Delivery"), TestValue("Not claimed"));
    assert_panel_detail_value(&row["details"], TestLabel("Notification delivery"), TestValue("Not claimed"));
    assert_panel_detail_value(&row["details"], TestLabel("Adapter dispatch"), TestValue("Not claimed"));
    assert_panel_detail_value(&row["details"], TestLabel("Platform state"), TestValue("Not claimed"));
    assert_panel_detail_value(&row["details"], TestLabel("Raw private source rows"), TestValue("Not claimed"));
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
        local_route_snapshot_json(
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
        local_route_snapshot_json(
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
