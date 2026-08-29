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

pub(super) fn nested_claim_violation(kind: &str, field: &str) -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_mutation(|model| {
        model["childUxHandoffReadyCount"] = json!(1);
        model["childUxLocalHandoffArtifactRecordCount"] = json!(1);
        model["childUxLocalHandoffArtifactReferenceIds"] = json!(["artifact-1"]);
        let mut value = match kind {
            "artifact" => json!({
                "schemaVersion": 1, "artifactReferenceId": "artifact-1", "sourceResultId": "result-1",
                "targetDomain": "native-app", "childReasonReferenceIds": ["reason-1"],
                "childStatusReferenceIds": ["status-1"], "childDeliveryClaimed": false,
                "notificationDeliveryClaimed": false, "adapterDispatchClaimed": false,
                "platformEnforcementClaimed": false, "rawPrivateSourceRowsIncluded": false
            }),
            "intent" => json!({
                "schemaVersion": 1, "parentSurfaceIntentReferenceId": "intent-1", "sourceResultId": "result-1",
                "sourceArtifactReferenceId": "artifact-1", "targetDomain": "native-app",
                "historyVisibility": "history-row-visible", "parentSurfaceStatus": "manual-action-required",
                "preferenceVisibility": "preference-setup-required", "drillInReferenceIds": [],
                "manualProofReferenceIds": [], "sensitiveDetailIncluded": false,
                "parentNotificationUiRendered": false, "parentPreferenceMutationClaimed": false,
                "providerDeliveryClaimed": false, "childDeliveryClaimed": false,
                "adapterDispatchClaimed": false, "platformEnforcementClaimed": false,
                "rawPrivateSourceRowsIncluded": false
            }),
            "preference" => json!({
                "schemaVersion": 1, "parentPreferenceSetupReferenceId": "setup-1",
                "sourceParentSurfaceIntentReferenceId": "intent-1", "sourceResultId": "result-1",
                "sourceArtifactReferenceId": "artifact-1", "targetDomain": "native-app",
                "draftStatus": "draft-ready", "parentPreferenceSetupRequestStatus": "request-ready",
                "parentPreferenceSetupRequestReferenceIds": [], "drillInReferenceIds": [],
                "manualProofReferenceIds": [], "parentPreferenceUiRendered": false,
                "parentFrequencyControlUiRendered": false, "parentPreferenceMutationClaimed": false,
                "notificationRuleMutationClaimed": false, "providerDeliveryClaimed": false,
                "childDeliveryClaimed": false, "adapterDispatchClaimed": false,
                "platformEnforcementClaimed": false, "rawPrivateSourceRowsIncluded": false
            }),
        };
        value[field] = json!(true);
        let field_name = match kind {
            "artifact" => "childUxLocalHandoffArtifactRecords",
            "intent" => "childUxParentSurfaceIntentRecords",
            "preference" => "childUxParentPreferenceSetupRecords",
            _ => unreachable!("known nested record kind"),
        };
        model[field_name] = json!([value]);
    })
}

pub(super) fn app_game_timer_parent_surface_response_with_local_artifact() -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_mutation(|model| {
        model["childUxHandoffReadyCount"] = json!(1);
        model["childUxLocalHandoffArtifactRecordCount"] = json!(1);
        model["childUxLocalHandoffArtifactReferenceIds"] = json!(["artifact-1"]);
        model["childUxLocalHandoffArtifactRecords"] = json!([{
            "schemaVersion": 1, "artifactReferenceId": "artifact-1", "sourceResultId": "result-1",
            "targetDomain": "native-app", "childReasonReferenceIds": ["reason-1"],
            "childStatusReferenceIds": ["status-1"], "childDeliveryClaimed": false,
            "notificationDeliveryClaimed": false, "adapterDispatchClaimed": false,
            "platformEnforcementClaimed": false, "rawPrivateSourceRowsIncluded": false
        }]);
    })
}

pub(super) fn local_artifact_mismatch(field: &str) -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_mutation(|model| {
        model["childUxHandoffReadyCount"] = json!(1);
        model["childUxLocalHandoffArtifactRecordCount"] = json!(1);
        model["childUxLocalHandoffArtifactReferenceIds"] = json!(["artifact-1"]);
        model["childUxLocalHandoffArtifactRecords"] = json!([{
            "schemaVersion": 1, "artifactReferenceId": "artifact-1", "sourceResultId": "result-1",
            "targetDomain": "native-app", "childReasonReferenceIds": ["reason-1"],
            "childStatusReferenceIds": ["status-1"], "childDeliveryClaimed": false,
            "notificationDeliveryClaimed": false, "adapterDispatchClaimed": false,
            "platformEnforcementClaimed": false, "rawPrivateSourceRowsIncluded": false
        }]);
        if field == "childUxLocalHandoffArtifactReferenceIds" {
            model[field] = json!(["artifact-other"]);
        } else {
            model[field] = json!(0);
        }
    })
}

pub(super) fn local_artifact_invalid_record(field: &str) -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_local_artifact_mutation(|model| {
        let record = &mut model["childUxLocalHandoffArtifactRecords"][0];
        record[field] = if field == "childReasonReferenceIds" || field == "childStatusReferenceIds" { json!([""]) } else { json!("") };
    })
}

pub(super) fn local_artifact_duplicate_refs(field: &str) -> AgentEventEnvelope {
    local_artifact_invalid_record_with_value(field, json!(["duplicate", "duplicate"]))
}

fn local_artifact_invalid_record_with_value(field: &str, value: serde_json::Value) -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_local_artifact_mutation(|model| {
        model["childUxLocalHandoffArtifactRecords"][0][field] = value;
    })
}

fn app_game_timer_parent_surface_response_with_local_artifact_mutation(
    mutate: impl FnOnce(&mut serde_json::Value),
) -> AgentEventEnvelope {
    let mut response = app_game_timer_parent_surface_response_with_local_artifact();
    let payload = require_some(response.payload.get(constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL), TestContext("artifact response payload exists"));
    let text = match payload {
        LogFieldValue::String(value) => value.clone(),
        _ => require_some(None, TestContext("artifact response payload is serialized")),
    };
    let mut model = require_ok(serde_json::from_str::<serde_json::Value>(&text), "artifact response read model parses");
    mutate(&mut model);
    response.payload.insert(constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL.to_string(), LogFieldValue::String(require_ok(serde_json::to_string(&model), "artifact response read model serializes")));
    response
}
