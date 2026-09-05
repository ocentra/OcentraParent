use super::super::{
    constants, require_ok, require_some, AgentEventEnvelope, LogFieldValue, TestContext,
};
use super::NestedClaimViolationKind;
use serde_json::json;

pub(super) fn nested_claim_violation(
    kind: NestedClaimViolationKind,
    field: &str,
) -> AgentEventEnvelope {
    super::app_game_timer_parent_surface_response_with_mutation(|model| {
        model["childUxHandoffReadyCount"] = json!(1);
        model["childUxLocalHandoffArtifactRecordCount"] = json!(1);
        model["childUxLocalHandoffArtifactReferenceIds"] = json!(["artifact-1"]);
        let mut value = kind.value();
        value[field] = json!(true);
        model[kind.record_name()] = json!([value]);
    })
}

pub(super) fn app_game_timer_parent_surface_response_with_local_artifact() -> AgentEventEnvelope {
    base_local_artifact_response()
}

fn base_local_artifact_response() -> AgentEventEnvelope {
    super::app_game_timer_parent_surface_response_with_mutation(|model| {
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
    super::app_game_timer_parent_surface_response_with_mutation(|model| {
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
    local_artifact_invalid_record_with_value(
        field,
        if matches!(field, "childReasonReferenceIds" | "childStatusReferenceIds") {
            json!([""])
        } else {
            json!("")
        },
    )
}

pub(super) fn local_artifact_duplicate_refs(field: &str) -> AgentEventEnvelope {
    local_artifact_invalid_record_with_value(field, json!(["duplicate", "duplicate"]))
}

fn local_artifact_invalid_record_with_value(
    field: &str,
    value: serde_json::Value,
) -> AgentEventEnvelope {
    app_game_timer_parent_surface_response_with_local_artifact_mutation(|model| {
        model["childUxLocalHandoffArtifactRecords"][0][field] = value;
    })
}

fn app_game_timer_parent_surface_response_with_local_artifact_mutation(
    mutate: impl FnOnce(&mut serde_json::Value),
) -> AgentEventEnvelope {
    let mut response = base_local_artifact_response();
    let payload = require_some(
        response
            .payload
            .get(constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL),
        TestContext("artifact response payload exists"),
    );
    let text = match payload {
        LogFieldValue::String(value) => value.clone(),
        _ => require_some(None, TestContext("artifact response payload is serialized")),
    };
    let mut model = require_ok(
        serde_json::from_str::<serde_json::Value>(&text),
        "artifact response read model parses",
    );
    mutate(&mut model);
    response.payload.insert(
        constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&model),
            "artifact response read model serializes",
        )),
    );
    response
}
