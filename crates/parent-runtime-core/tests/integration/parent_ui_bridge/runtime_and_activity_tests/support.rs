use super::*;

pub(super) fn capture_app_game_dispatch_requests(
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

pub(super) fn assert_app_game_route_load_requests(
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

pub(super) fn assert_app_game_dispatch_request_commands(
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

pub(super) fn capture_app_game_timer_requests(
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

pub(super) fn assert_app_game_timer_request_commands(
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

pub(super) fn result_network_flow_row_event_id(
    result: &super::ParentUiActionResult,
) -> Option<CommandText> {
    result
        .snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.live_activity.as_ref())
        .and_then(|live_activity| live_activity.network_flow_read_model.as_ref())
        .and_then(|read_model| read_model.rows.first())
        .map(|row| CommandText(row.event_id.to_string()))
}

pub(super) fn empty_action(action: ParentUiActionKind, route: ParentRouteId) -> ParentUiAction {
    ParentUiAction {
        action,
        route,
        command: None,
        payload: json!({}),
        context: None,
    }
}

pub(super) fn route_snapshot_json(route: ParentRouteId, label: TestContext) -> serde_json::Value {
    require_ok(
        serde_json::to_value(load_parent_route_snapshot(route, None)),
        label.0,
    )
}

pub(super) fn policy_preview_attention_card(fields: &[(&str, &str)]) -> serde_json::Value {
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
