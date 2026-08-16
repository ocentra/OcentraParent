use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;

use super::{EnforcementTimerPayloadError, EnforcementTimerText, EnforcementTimerTextRef};

pub(crate) fn optional_string(
    payload: &LogFields,
    field: EnforcementTimerTextRef<'_>,
) -> Option<EnforcementTimerText> {
    match payload.get(field.0) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Some(EnforcementTimerText(value.trim().to_string()))
        }
        _ => None,
    }
}

pub(crate) fn required_process_id(
    payload: &LogFields,
) -> Result<u32, EnforcementTimerPayloadError> {
    match payload.get(constants::field::PROCESS_ID) {
        Some(LogFieldValue::Number(value))
            if value.is_finite()
                && *value > 0.0
                && value.fract() == 0.0
                && *value <= f64::from(u32::MAX) =>
        {
            Ok(*value as u32)
        }
        _ => Err(EnforcementTimerPayloadError::ProcessIdRequired),
    }
}

pub(crate) fn string_or_prefixed(
    payload: &LogFields,
    field: EnforcementTimerTextRef<'_>,
    prefix: EnforcementTimerTextRef<'_>,
    suffix: EnforcementTimerTextRef<'_>,
) -> EnforcementTimerText {
    optional_string(payload, field).unwrap_or_else(|| prefixed_id(prefix, suffix))
}

fn prefixed_id(
    prefix: EnforcementTimerTextRef<'_>,
    suffix: EnforcementTimerTextRef<'_>,
) -> EnforcementTimerText {
    let mut value = String::from(prefix.0);
    value.push_str(suffix.0);
    EnforcementTimerText(value)
}
