#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

pub(crate) fn required_contract_text(
    field: &'static str,
    value: impl Into<String>,
) -> Result<String, EventingError> {
    let value = value.into();
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(EventingError::EmptyValue { field });
    }

    Ok(trimmed.to_string())
}

pub(crate) fn optional_contract_text(
    field: &'static str,
    value: Option<String>,
) -> Result<Option<String>, EventingError> {
    value
        .map(|value| required_contract_text(field, value))
        .transpose()
}
