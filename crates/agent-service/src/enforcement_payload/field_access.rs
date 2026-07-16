use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;

use super::EnforcementFieldKey;
use super::EnforcementPayloadError;
use super::EnforcementText;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;

pub(super) fn required_string(
    payload: &LogFields,
    field: EnforcementFieldKey,
) -> Result<EnforcementText, EnforcementPayloadError> {
    match payload.get(field.0) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Ok(EnforcementText(value.trim().to_string()))
        }
        _ => Err(EnforcementPayloadError::CommandPayloadInvalid),
    }
}

pub(super) fn optional_string(
    payload: &LogFields,
    field: EnforcementFieldKey,
) -> Option<EnforcementText> {
    match payload.get(field.0) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Some(EnforcementText(value.trim().to_string()))
        }
        _ => None,
    }
}

pub(super) fn required_boolean(
    payload: &LogFields,
    field: EnforcementFieldKey,
) -> Result<bool, EnforcementPayloadError> {
    match payload.get(field.0) {
        Some(LogFieldValue::Boolean(value)) => Ok(*value),
        _ => Err(EnforcementPayloadError::CommandPayloadInvalid),
    }
}

pub(super) fn required_string_list(
    payload: &LogFields,
    field: EnforcementFieldKey,
    error: EnforcementPayloadError,
) -> Result<Vec<EnforcementText>, EnforcementPayloadError> {
    let values = split_list(&(required_string(payload, field)?));
    if values.is_empty() {
        return Err(error);
    }
    Ok(values)
}

pub(super) fn evidence_references(
    payload: &LogFields,
    observed_at: &EnforcementText,
    field: EnforcementFieldKey,
) -> Result<Vec<ParentEvidenceReference>, EnforcementPayloadError> {
    let references =
        required_string_list(payload, field, EnforcementPayloadError::MissingEvidence)?
            .into_iter()
            .map(|evidence_reference_id| ParentEvidenceReference {
                evidence_reference_id: evidence_reference_id.0,
                kind: ParentEvidenceReferenceKind::ActivityEvent,
                observed_at: observed_at.0.clone(),
            })
            .collect::<Vec<_>>();

    Ok(references)
}

pub(super) fn split_list(value: &EnforcementText) -> Vec<EnforcementText> {
    value
        .0
        .as_str()
        .split(constants::delimiter::LIST)
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(|item| EnforcementText(item.to_string()))
        .collect()
}
