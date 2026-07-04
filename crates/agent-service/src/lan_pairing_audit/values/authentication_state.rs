use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

pub(super) fn authentication_state_value(
    reason: Option<&LanPairingRejectionReason>,
) -> LogFieldValue {
    LogFieldValue::String(
        match reason {
            Some(_) => constants::value::LAN_CONTROL_REJECTED,
            None => constants::value::LAN_CONTROL_ACCEPTED,
        }
        .to_string(),
    )
}
