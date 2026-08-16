use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;

use super::ReasonText;

pub(super) fn reason_text(reason: &LanPairingRejectionReason) -> ReasonText {
    match reason {
        LanPairingRejectionReason::ControllerLeaseMissing => {
            ReasonText(constants::value::LAN_REASON_CONTROLLER_LEASE_MISSING)
        }
        LanPairingRejectionReason::ControllerLeaseExpired => {
            ReasonText(constants::value::LAN_REASON_CONTROLLER_LEASE_EXPIRED)
        }
        LanPairingRejectionReason::WrongController => {
            ReasonText(constants::value::LAN_REASON_WRONG_CONTROLLER)
        }
        LanPairingRejectionReason::SignedChildAgentContextUnavailable => {
            ReasonText(constants::value::LAN_REASON_SIGNED_CHILD_AGENT_CONTEXT_UNAVAILABLE)
        }
        LanPairingRejectionReason::LocalNetworkDisabled => {
            ReasonText(constants::value::LAN_REASON_UNSUPPORTED_ROUTE)
        }
        LanPairingRejectionReason::UnselectedDevice => {
            ReasonText(constants::value::LAN_REASON_UNSELECTED_DEVICE)
        }
        LanPairingRejectionReason::ObserverReadOnly => {
            ReasonText(constants::value::LAN_REASON_OBSERVER_READ_ONLY)
        }
        LanPairingRejectionReason::TakeoverDenied => {
            ReasonText(constants::value::LAN_REASON_TAKEOVER_DENIED)
        }
        LanPairingRejectionReason::LanAiProviderUnavailable => {
            ReasonText(constants::value::LAN_REASON_LAN_AI_PROVIDER_UNAVAILABLE)
        }
        LanPairingRejectionReason::LanAiJobUnauthorized => {
            ReasonText(constants::value::LAN_REASON_LAN_AI_JOB_UNAUTHORIZED)
        }
        _ => ReasonText(constants::value::LAN_REASON_UNSUPPORTED_ROUTE),
    }
}
