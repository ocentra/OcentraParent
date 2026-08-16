use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;

use super::EnforcementPayloadError;

pub(super) fn optional_process_id(
    payload: &LogFields,
) -> Result<Option<u32>, EnforcementPayloadError> {
    match payload.get(constants::field::PROCESS_ID) {
        Some(LogFieldValue::Number(value)) => parse_process_id(*value).map(Some),
        Some(_) => Err(EnforcementPayloadError::ProcessIdRequired),
        None => Ok(None),
    }
}

fn parse_process_id(value: f64) -> Result<u32, EnforcementPayloadError> {
    if !value.is_finite() || value <= 0.0 || value.fract() != 0.0 || value > f64::from(u32::MAX) {
        return Err(EnforcementPayloadError::ProcessIdRequired);
    }

    Ok(value as u32)
}
