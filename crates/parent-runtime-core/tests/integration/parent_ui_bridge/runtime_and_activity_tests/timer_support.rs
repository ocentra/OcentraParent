use super::*;

pub(super) fn replace_timer_parent_surface_response(
    responses: &mut Vec<AgentEventEnvelope>,
    response: AgentEventEnvelope,
) {
    let timer_response = responses
        .last_mut()
        .unwrap_or_else(|| std::panic::resume_unwind(Box::new("timer response is present")));
    *timer_response = response;
}

pub(super) fn app_game_timer_parent_surface_response_with_valid_row() -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_mutation(|read_model| {
        read_model["capabilityStatus"] = json!("timer-parent-surface-ready");
        read_model["returned"] = json!(1);
        read_model["readyForParentSurfaceCount"] = json!(1);
        read_model["rows"] = json!([{
            "schemaVersion": 1,
            "rowId": "timer-parent-surface-row-1",
            "targetDomain": "native-game",
            "timerSurfaceState": "ready-for-parent-surface",
            "rowCount": 2,
            "evidenceReferenceIds": ["evidence.timer.1", "evidence.timer.2"],
            "evidence": [
                {
                    "evidenceId": "evidence.timer.1",
                    "kind": "localDbRow",
                    "digest": null,
                    "uri": null
                },
                {
                    "evidenceId": "evidence.timer.2",
                    "kind": "localDbRow",
                    "digest": null,
                    "uri": null
                }
            ]
        }]);
    })
}

pub(super) fn app_game_timer_parent_surface_response_with_unknown_target() -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_mutation(|read_model| {
        read_model["capabilityStatus"] = json!("timer-parent-surface-ready");
        read_model["returned"] = json!(1);
        read_model["readyForParentSurfaceCount"] = json!(1);
        read_model["rows"] = json!([{
            "schemaVersion": 1,
            "rowId": "timer-parent-surface-row-unknown-target",
            "targetDomain": "unknown-target",
            "timerSurfaceState": "ready-for-parent-surface",
            "rowCount": 0,
            "evidenceReferenceIds": [],
            "evidence": []
        }]);
    })
}

pub(super) fn app_game_timer_parent_surface_response_with_unknown_state() -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_mutation(|read_model| {
        read_model["capabilityStatus"] = json!("timer-parent-surface-ready");
        read_model["returned"] = json!(1);
        read_model["readyForParentSurfaceCount"] = json!(1);
        read_model["rows"] = json!([{
            "schemaVersion": 1,
            "rowId": "timer-parent-surface-row-unknown-state",
            "targetDomain": "native-game",
            "timerSurfaceState": "unknown-state",
            "rowCount": 0,
            "evidenceReferenceIds": [],
            "evidence": []
        }]);
    })
}

pub(super) fn app_game_timer_parent_surface_response_with_inconsistent_counts() -> AgentEventEnvelope
{
    app_game_timer_parent_surface_response_with_mutation(|read_model| {
        read_model["capabilityStatus"] = json!("timer-parent-surface-partial");
        read_model["returned"] = json!(2);
        read_model["readyForParentSurfaceCount"] = json!(1);
    })
}

pub(super) fn app_game_timer_parent_surface_response_with_state_count_mismatch(
) -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_mutation(|read_model| {
        read_model["capabilityStatus"] = json!("timer-parent-surface-ready");
        read_model["returned"] = json!(2);
        read_model["readyForParentSurfaceCount"] = json!(2);
        read_model["rows"] = json!([
            {
                "schemaVersion": 1,
                "rowId": "timer-parent-surface-row-ready",
                "targetDomain": "native-game",
                "timerSurfaceState": "ready-for-parent-surface",
                "rowCount": 0,
                "evidenceReferenceIds": [],
                "evidence": []
            },
            {
                "schemaVersion": 1,
                "rowId": "timer-parent-surface-row-blocked",
                "targetDomain": "native-app",
                "timerSurfaceState": "blocked-by-source-freshness",
                "rowCount": 0,
                "evidenceReferenceIds": [],
                "evidence": []
            }
        ]);
    })
}

pub(super) fn app_game_timer_parent_surface_response_with_duplicate_row_id() -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_mutation(|read_model| {
        read_model["capabilityStatus"] = json!("timer-parent-surface-ready");
        read_model["returned"] = json!(2);
        read_model["readyForParentSurfaceCount"] = json!(2);
        read_model["rows"] = json!([
            {
                "schemaVersion": 1,
                "rowId": "timer-parent-surface-row-duplicate",
                "targetDomain": "native-game",
                "timerSurfaceState": "ready-for-parent-surface",
                "rowCount": 0,
                "evidenceReferenceIds": [],
                "evidence": []
            },
            {
                "schemaVersion": 1,
                "rowId": "timer-parent-surface-row-duplicate",
                "targetDomain": "native-app",
                "timerSurfaceState": "ready-for-parent-surface",
                "rowCount": 0,
                "evidenceReferenceIds": [],
                "evidence": []
            }
        ]);
    })
}

pub(super) fn app_game_timer_parent_surface_response_with_duplicate_evidence_id(
) -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_mutation(|read_model| {
        read_model["capabilityStatus"] = json!("timer-parent-surface-ready");
        read_model["returned"] = json!(1);
        read_model["readyForParentSurfaceCount"] = json!(1);
        read_model["rows"] = json!([{
            "schemaVersion": 1,
            "rowId": "timer-parent-surface-row-duplicate-evidence",
            "targetDomain": "native-game",
            "timerSurfaceState": "ready-for-parent-surface",
            "rowCount": 2,
            "evidenceReferenceIds": ["evidence.timer.duplicate", "evidence.timer.duplicate"],
            "evidence": [
                {
                    "evidenceId": "evidence.timer.duplicate",
                    "kind": "localDbRow",
                    "digest": null,
                    "uri": null
                },
                {
                    "evidenceId": "evidence.timer.duplicate",
                    "kind": "localDbRow",
                    "digest": null,
                    "uri": null
                }
            ]
        }]);
    })
}

pub(super) fn app_game_timer_parent_surface_response_with_unowned_runtime_claim(
) -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_mutation(|read_model| {
        read_model["capabilityStatus"] = json!("timer-parent-surface-ready");
        read_model["returned"] = json!(1);
        read_model["readyForParentSurfaceCount"] = json!(1);
        read_model["timerRuntimeClaimed"] = json!(true);
        read_model["rows"] = json!([{
            "schemaVersion": 1,
            "rowId": "timer-parent-surface-row-unowned-runtime",
            "targetDomain": "native-game",
            "timerSurfaceState": "ready-for-parent-surface",
            "rowCount": 0,
            "evidenceReferenceIds": [],
            "evidence": []
        }]);
    })
}

pub(super) fn app_game_timer_parent_surface_response_with_unowned_adapter_dispatch_claim(
) -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_mutation(|read_model| {
        read_model["capabilityStatus"] = json!("timer-parent-surface-ready");
        read_model["returned"] = json!(1);
        read_model["readyForParentSurfaceCount"] = json!(1);
        read_model["adapterDispatchClaimed"] = json!(true);
        read_model["rows"] = json!([{
            "schemaVersion": 1,
            "rowId": "timer-parent-surface-row-unowned-adapter-dispatch",
            "targetDomain": "native-game",
            "timerSurfaceState": "ready-for-parent-surface",
            "rowCount": 0,
            "evidenceReferenceIds": [],
            "evidence": []
        }]);
    })
}

pub(super) fn app_game_timer_parent_surface_response_with_unowned_platform_enforcement_claim(
) -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_mutation(|read_model| {
        read_model["capabilityStatus"] = json!("timer-parent-surface-ready");
        read_model["returned"] = json!(1);
        read_model["readyForParentSurfaceCount"] = json!(1);
        read_model["platformEnforcementClaimed"] = json!(true);
        read_model["rows"] = json!([{
            "schemaVersion": 1,
            "rowId": "timer-parent-surface-row-unowned-platform-enforcement",
            "targetDomain": "native-game",
            "timerSurfaceState": "ready-for-parent-surface",
            "rowCount": 0,
            "evidenceReferenceIds": [],
            "evidence": []
        }]);
    })
}

pub(super) fn app_game_timer_parent_surface_response_with_mutation(
    mutate: impl FnOnce(&mut Value),
) -> AgentEventEnvelope {
    let mut response = app_game_timer_parent_surface_response_event();
    let read_model_json = require_some(
        response
            .payload
            .get(constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL)
            .and_then(|value| match value {
                LogFieldValue::String(value) => Some(value.clone()),
                _ => None,
            }),
        TestContext("timer parent-surface response contains serialized read model"),
    );
    let mut read_model = require_ok(
        serde_json::from_str::<Value>(&read_model_json),
        "timer parent-surface read model JSON parses",
    );
    mutate(&mut read_model);
    response.payload.insert(
        constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&read_model),
            "mutated timer parent-surface read model serializes",
        )),
    );
    response
}
