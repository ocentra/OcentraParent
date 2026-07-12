use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

pub(super) fn authentication_state_value(
    reason: Option<&LanPairingRejectionReason>,
) -> LogFieldValue {
    LogFieldValue::String(
        match reason {
            Some(
                LanPairingRejectionReason::Anonymous
                | LanPairingRejectionReason::ControllerLeaseMissing
                | LanPairingRejectionReason::ControllerLeaseExpired
                | LanPairingRejectionReason::ObserverReadOnly
                | LanPairingRejectionReason::TakeoverDenied
                | LanPairingRejectionReason::LanAiProviderUnavailable
                | LanPairingRejectionReason::LanAiJobUnauthorized
                | LanPairingRejectionReason::WrongOrigin
                | LanPairingRejectionReason::WrongController
                | LanPairingRejectionReason::Malformed
                | LanPairingRejectionReason::SignedChildAgentContextUnavailable,
            ) => constants::value::LAN_AUTH_UNAUTHENTICATED,
            Some(_) | None => constants::value::LAN_AUTH_PAIRED,
        }
        .to_string(),
    )
}
