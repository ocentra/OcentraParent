use serde_json::Value;

use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumeError;

use super::storage_key_digest;

pub(super) fn redact(audit_json: &str) -> Result<String, AuthenticatedDeliveryGrantConsumeError> {
    let mut audit: Value = serde_json::from_str(audit_json)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
    redact_value(&mut audit)?;
    serde_json::to_string(&audit)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)
}

fn redact_value(value: &mut Value) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    match value {
        Value::Array(values) => values.iter_mut().try_for_each(redact_value),
        Value::Object(values) => values.iter_mut().try_for_each(redact_field),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => Ok(()),
    }
}

fn redact_field(
    (key, value): (&String, &mut Value),
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    if matches!(key.as_str(), "correlation_id" | "correlationId") {
        let correlation = value
            .as_str()
            .ok_or(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
        *value = Value::String(storage_key_digest(correlation));
        return Ok(());
    }
    redact_value(value)
}
