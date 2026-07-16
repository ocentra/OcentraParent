use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

#[path = "reason/primary.rs"]
mod primary;
#[path = "reason/secondary.rs"]
mod secondary;

#[derive(Clone, Copy)]
struct ReasonText(&'static str);

pub(super) fn reason_value(reason: &LanPairingRejectionReason) -> LogFieldValue {
    if let Some(text) = primary::reason_text(reason) {
        return LogFieldValue::String(text.0.to_string());
    }

    LogFieldValue::String(secondary::reason_text(reason).0.to_string())
}
