use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use super::app_game_adapter_dispatch_result_payload::{DispatchExecutionStatusText, StaticText};

pub(crate) fn required_string(
    payload: &LogFields,
    field: StaticText,
) -> Result<DispatchExecutionStatusText, StaticText> {
    match payload.get(field.0) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Ok(DispatchExecutionStatusText(value.trim().to_string()))
        }
        _ => Err(StaticText(
            constants::enforcement::REJECTION_COMMAND_PAYLOAD_INVALID,
        )),
    }
}
