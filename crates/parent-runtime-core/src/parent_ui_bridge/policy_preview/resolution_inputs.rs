use super::*;

pub(super) fn decision_from_payload(
    payload: &Value,
) -> Result<PolicyRequestParentResolutionDecision, String> {
    let object = payload
        .as_object()
        .ok_or_else(|| "parent resolution payload must be an object".to_string())?;
    if object.len() != 1 || !object.contains_key(RESOLUTION_PAYLOAD_FIELD) {
        return Err("parent resolution payload must contain only the decision field".to_string());
    }
    let value = object
        .get(RESOLUTION_PAYLOAD_FIELD)
        .ok_or_else(|| "parent resolution decision is missing".to_string())?;
    let value = match value {
        Value::String(text) => serde_json::from_str(text)
            .map_err(|error| format!("parent resolution decision payload is invalid: {error}"))?,
        value => value.clone(),
    };
    let input: ParentResolutionDecisionInput = serde_json::from_value(value).map_err(|error| {
        format!("parent resolution payload must contain only a valid decision: {error}")
    })?;
    Ok(input.decision)
}

pub(super) fn actor_role(
    value: &Option<String>,
) -> Result<PolicyRequestAssistantPreviewConfirmActorRole, String> {
    let value = required_context(value, "actor role")?;
    serde_json::from_value(Value::String(value.to_string())).map_err(|error| {
        format!("parent resolution actor role is unavailable; manual review required: {error}")
    })
}

pub(super) fn actor_state(
    value: &Option<String>,
) -> Result<PolicyRequestAssistantPreviewConfirmActorState, String> {
    let value = required_context(value, "actor state")?;
    serde_json::from_value(Value::String(value.to_string())).map_err(|error| {
        format!("parent resolution actor state is unavailable; manual review required: {error}")
    })
}

pub(super) fn single_audit_reference(value: Option<&str>) -> Result<&str, String> {
    let value = value
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            "parent resolution approval audit reference is unavailable; manual review required"
                .to_string()
        })?;
    let references = value
        .split(constants::delimiter::LIST)
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>();
    match references.as_slice() {
        [reference] => Ok(*reference),
        _ => Err(
            "parent resolution requires one exact approval audit reference; manual review required"
                .to_string(),
        ),
    }
}

pub(super) fn required_context<'a>(
    value: &'a Option<String>,
    label: &str,
) -> Result<&'a str, String> {
    value
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("parent resolution {label} is unavailable; manual review required"))
}
