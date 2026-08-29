use super::support::*;
use super::*;

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
    let details = serialize_json(
        &command_result_projection.details,
        TestContext("command-result projection details serialize"),
    );
    for (label, value) in [
        ("Provider delivery receipt-required refs", "app-game-parent-preference-setup-provider-receipt-required::request-1"),
        ("Provider delivery receipt-required status", "Required"),
        ("Provider delivery receipt-pending refs", "app-game-parent-preference-setup-provider-receipt-pending::request-1"),
        ("Provider delivery receipt-pending status", "Pending"),
        ("Provider delivery aggregate status", "Manual provider setup required; local outbox, queue, and receipt tracking are recorded."),
        ("Provider delivery next action", "Configure provider adapter and credential proof before external delivery."),
        ("Provider delivery proof state", "Local durable outbox, provider queue, receipt-required, pending, and ingested refs are visible."),
        ("Provider delivery no-claim boundary", "Provider delivery execution and external provider receipt ingestion are not claimed."),
        ("Adapter dispatch", "Not claimed"),
        ("Platform state", "Not claimed"),
        ("Child delivery", "Not claimed"),
    ] {
        assert_panel_detail_value(&details, TestLabel(label), TestValue(value));
    }

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
