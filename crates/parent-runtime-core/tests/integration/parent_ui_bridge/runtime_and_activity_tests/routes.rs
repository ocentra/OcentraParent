use super::support::*;
use super::timer_support::*;
use super::*;
use ocentra_parent_agent_protocol::transport::AgentCommandName;

#[test]
fn screen_analysis_route_load_attaches_rust_owned_screen_summary_panel() {
    let value = projected_route_snapshot_json(
        ParentRouteId::ScreenAnalysis,
        vec![(
            AgentCommandName::AgentActivityScreenReadModelGet,
            screen_read_model_response_event(),
        )],
        TestContext("screen-analysis route snapshot serializes"),
    );

    assert_eq!(
        serialize_json(
            &AgentCommandName::AgentActivityScreenReadModelGet,
            TestContext("screen read-model command serializes"),
        ),
        json!("agent.activity.screen.read-model.get")
    );
    assert_eq!(
        value["liveActivity"]["activityScreenReadModel"]["ok"],
        json!(true)
    );
    assert_eq!(
        value["liveActivity"]["activityScreenReadModel"]["state"],
        json!("ready")
    );
    assert_eq!(
        value["liveActivity"]["activityScreenReadModel"]["value"]["summary"],
        json!("1 screen row ready")
    );
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
    let value = projected_route_snapshot_json(
        ParentRouteId::AppGameSessions,
        app_game_route_projection(app_game_route_load_response_events()),
        TestContext("app-game sessions route snapshot serializes"),
    );
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
        Some(1)
    );
    assert_eq!(
        value["liveActivity"]["activityAppGamePlatformExtensionReadModel"]["value"]["rows"][0]
            ["platform"],
        json!("Windows")
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
    let value = projected_route_snapshot_json(
        ParentRouteId::AppGameSessions,
        app_game_route_projection(responses),
        TestContext("valid timer parent-surface response serializes"),
    );

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
        nested_claim_violation(NestedClaimViolationKind::Artifact, "adapterDispatchClaimed"),
        nested_claim_violation(
            NestedClaimViolationKind::Artifact,
            "platformEnforcementClaimed",
        ),
        nested_claim_violation(NestedClaimViolationKind::Intent, "adapterDispatchClaimed"),
        nested_claim_violation(
            NestedClaimViolationKind::Intent,
            "platformEnforcementClaimed",
        ),
        nested_claim_violation(
            NestedClaimViolationKind::Preference,
            "adapterDispatchClaimed",
        ),
        nested_claim_violation(
            NestedClaimViolationKind::Preference,
            "platformEnforcementClaimed",
        ),
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
        let value = projected_route_snapshot_json(
            ParentRouteId::AppGameSessions,
            app_game_route_projection(responses),
            TestContext("invalid timer parent-surface response serializes"),
        );

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
    let value = projected_route_snapshot_json(
        ParentRouteId::AppGameSessions,
        app_game_route_projection(responses),
        TestContext("valid local artifact response serializes"),
    );
    let row =
        &value["liveActivity"]["appGameTimerParentSurfacePanel"]["localHandoffArtifactRows"][0];
    assert_eq!(row["title"], json!("result-1"));
    assert_panel_detail_value(
        &row["details"],
        TestLabel("Target"),
        TestValue("Native app"),
    );
    assert_panel_detail_value(
        &row["details"],
        TestLabel("Child reason refs"),
        TestValue("reason-1"),
    );
    assert_panel_detail_value(
        &row["details"],
        TestLabel("Child status refs"),
        TestValue("status-1"),
    );
    assert_panel_detail_value(
        &row["details"],
        TestLabel("Delivery"),
        TestValue("not-claimed"),
    );
    assert_panel_detail_value(
        &row["details"],
        TestLabel("Notification delivery"),
        TestValue("not-claimed"),
    );
    assert_panel_detail_value(
        &row["details"],
        TestLabel("Adapter dispatch"),
        TestValue("not-claimed"),
    );
    assert_panel_detail_value(
        &row["details"],
        TestLabel("Platform state"),
        TestValue("not-claimed"),
    );
    assert_panel_detail_value(
        &row["details"],
        TestLabel("Raw private source rows"),
        TestValue("not-claimed"),
    );
}

#[test]
fn proof_panels_route_load_attaches_network_flow_read_model() {
    let commands = [
        AgentCommandName::AgentNetworkFlowReadModelGet,
        AgentCommandName::AgentNetworkRuntimeEventChainStreamGet,
        AgentCommandName::AgentPolicyPreviewReadModelGet,
        AgentCommandName::AgentActivityTrackingReadModelGet,
    ];
    let value = projected_route_snapshot_json(
        ParentRouteId::ProofPanels,
        vec![
            (commands[0].clone(), network_flow_response_event()),
            (
                commands[1].clone(),
                network_runtime_event_chain_response_event(),
            ),
            (commands[2].clone(), policy_preview_response_event()),
            (commands[3].clone(), tracking_read_model_response_event()),
        ],
        TestContext("proof panels snapshot serializes"),
    );
    let mut commands = commands
        .into_iter()
        .map(|command| {
            command_text(
                &serialize_json(&command, TestContext("proof command serializes")),
                TestContext("proof command is a string"),
            )
            .0
        })
        .collect::<Vec<_>>();
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
    assert_panel_detail_value(
        &value["liveActivity"]["activityTrackingPanel"]["cards"][0]["details"],
        TestLabel("Visible devices"),
        TestValue("1"),
    );
    assert_panel_detail_value(
        &value["liveActivity"]["activityTrackingPanel"]["cards"][0]["details"],
        TestLabel("Active tracking rows"),
        TestValue("1"),
    );
    assert_panel_detail_value(
        &value["liveActivity"]["activityTrackingPanel"]["cards"][1]["details"],
        TestLabel("Status"),
        TestValue("Unavailable"),
    );
    assert_eq!(
        value["liveActivity"]["activityTrackingPanel"]["cards"][1]["details"]
            .as_array()
            .map(Vec::len),
        Some(3)
    );
    assert_panel_detail_value(
        &value["liveActivity"]["activityTrackingPanel"]["cards"][8]["details"],
        TestLabel("Reason"),
        TestValue(
            "No authenticated child check-in request or receipt is supplied to this surface.",
        ),
    );
    assert_eq!(
        value["liveActivity"]["activityTrackingPanel"]["cards"][8]["details"]
            .as_array()
            .map(Vec::len),
        Some(3)
    );
}

#[test]
fn policy_tracking_route_load_attaches_tracking_read_model() {
    let value = projected_route_snapshot_json(
        ParentRouteId::PolicyTracking,
        vec![(
            AgentCommandName::AgentActivityTrackingReadModelGet,
            tracking_read_model_response_event(),
        )],
        TestContext("policy-tracking route snapshot serializes"),
    );

    assert_eq!(
        serialize_json(
            &AgentCommandName::AgentActivityTrackingReadModelGet,
            TestContext("tracking read-model command serializes"),
        ),
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

    let panel = &value["liveActivity"]["activityTrackingPanel"];
    assert_eq!(
        panel["body"],
        json!("Current child tracking history, service coverage, custody, and honest connection gaps from the local Rust service.")
    );
    assert_eq!(
        panel["cards"].as_array().map(Vec::len),
        Some(9),
        "product tracking must expose real service rows and explicit unavailable boundaries",
    );
    assert_eq!(panel["cards"][0]["key"], json!("family-dashboard-rollup"));
    assert_eq!(panel["cards"][1]["key"], json!("tracking-current-device"));
    assert_panel_detail_value(
        &panel["cards"][1]["details"],
        TestLabel("Device"),
        TestValue("child-device-1"),
    );
    assert_panel_detail_value(
        &panel["cards"][1]["details"],
        TestLabel("Child or place"),
        TestValue("School"),
    );
    assert_panel_detail_value(
        &panel["cards"][1]["details"],
        TestLabel("Latest activity"),
        TestValue("Expected-place status"),
    );
    assert_eq!(panel["cards"][2]["key"], json!("tracking-location-surface"));
    assert_panel_detail_value(
        &panel["cards"][2]["details"],
        TestLabel("Status"),
        TestValue("Unavailable"),
    );
    assert_eq!(panel["cards"][3]["key"], json!("tracking-event-coverage"));
    assert_panel_detail_value(
        &panel["cards"][3]["details"],
        TestLabel("Expected-place states"),
        TestValue("1 reported"),
    );
    assert_eq!(
        panel["cards"][4]["key"],
        json!("tracking-retention-custody")
    );
    assert_panel_detail_value(
        &panel["cards"][4]["details"],
        TestLabel("Custody"),
        TestValue("child-device-query-store"),
    );
    assert_eq!(panel["cards"][5]["key"], json!("tracking-child-surface"));
    assert_eq!(panel["cards"][6]["key"], json!("tracking-action-readiness"));
    assert_eq!(
        panel["cards"][7]["key"],
        json!("tracking-evidence-drawer-ui")
    );
    assert_panel_detail_value(
        &panel["cards"][7]["details"],
        TestLabel("Observer source"),
        TestValue("tracking-engine"),
    );
    assert_eq!(panel["cards"][8]["key"], json!("tracking-citation-0"));
    assert_panel_detail_value(
        &panel["cards"][8]["details"],
        TestLabel("Evidence refs"),
        TestValue("tracking-evidence-1"),
    );
    assert_panel_detail_value(
        &panel["cards"][8]["details"],
        TestLabel("Deleted evidence refs"),
        TestValue("Not reported"),
    );
}

#[test]
fn policy_tracking_dependency_failure_keeps_rust_owned_unavailable_product_panel() {
    let value = projected_route_snapshot_json(
        ParentRouteId::PolicyTracking,
        Vec::new(),
        TestContext("unavailable policy-tracking route snapshot serializes"),
    );

    assert_eq!(value["route"], json!("policy-tracking"));
    assert_eq!(value["dataSource"], json!("unavailable"));
    assert_eq!(value["connectionState"], json!("error"));
    assert_eq!(value["commandEnabled"], json!(false));
    assert_eq!(value["diagnosticPanelsEnabled"], json!(false));
    assert!(value["parentPortalRows"].is_null());

    let panel = &value["liveActivity"]["activityTrackingPanel"];
    assert_eq!(
        panel["body"],
        json!("Current child tracking history, service coverage, custody, and honest connection gaps from the local Rust service.")
    );
    assert_panel_detail_value(
        &panel["summaryCards"][0]["details"],
        TestLabel("Status"),
        TestValue("Unavailable"),
    );
    assert_panel_detail_value(
        &panel["summaryCards"][0]["details"],
        TestLabel("Rows returned"),
        TestValue("0"),
    );
    assert_panel_detail_value(
        &panel["summaryCards"][0]["details"],
        TestLabel("Last observed"),
        TestValue("Not reported"),
    );
    assert_eq!(
        panel["cards"].as_array().map(Vec::len),
        Some(7),
        "unavailable product tracking must keep its honest feature boundaries visible",
    );
    assert_eq!(panel["cards"][0]["key"], json!("family-dashboard-rollup"));
    assert_eq!(panel["cards"][1]["key"], json!("tracking-current-device"));
    assert_panel_detail_value(
        &panel["cards"][1]["details"],
        TestLabel("Reason"),
        TestValue("No active tracking row is available."),
    );
    assert_eq!(panel["cards"][2]["key"], json!("tracking-location-surface"));
    assert_panel_detail_value(
        &panel["cards"][2]["details"],
        TestLabel("Map coordinates"),
        TestValue("Not supplied"),
    );
    assert_eq!(panel["cards"][5]["key"], json!("tracking-child-surface"));
    assert_eq!(panel["cards"][6]["key"], json!("tracking-action-readiness"));
}

#[test]
fn network_flow_refresh_action_attaches_runtime_backed_snapshot() {
    let action = empty_action(
        ParentUiActionKind::NetworkFlowReadModelRefreshRequested,
        ParentRouteId::Activity,
    );
    let mut responses = vec![(
        AgentCommandName::AgentNetworkFlowReadModelGet,
        network_flow_response_event(),
    )];
    responses.extend(activity_route_projection_after_network_refresh());
    let result = projected_action_result(&action, responses);

    assert!(result.accepted);
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
