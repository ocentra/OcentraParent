use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;

use super::ReasonText;

pub(super) fn reason_text(reason: &LanPairingRejectionReason) -> Option<ReasonText> {
    match reason {
        LanPairingRejectionReason::Malformed => {
            Some(ReasonText(constants::value::LAN_REASON_MALFORMED))
        }
        LanPairingRejectionReason::WrongDevice => {
            Some(ReasonText(constants::value::LAN_REASON_WRONG_DEVICE))
        }
        LanPairingRejectionReason::WrongOrigin => {
            Some(ReasonText(constants::value::LAN_REASON_WRONG_ORIGIN))
        }
        LanPairingRejectionReason::UnsupportedRoute => {
            Some(ReasonText(constants::value::LAN_REASON_UNSUPPORTED_ROUTE))
        }
        LanPairingRejectionReason::Expired => {
            Some(ReasonText(constants::value::LAN_REASON_EXPIRED))
        }
        LanPairingRejectionReason::Replayed => {
            Some(ReasonText(constants::value::LAN_REASON_REPLAYED))
        }
        LanPairingRejectionReason::Stale => Some(ReasonText(constants::value::LAN_REASON_STALE)),
        LanPairingRejectionReason::Offline => {
            Some(ReasonText(constants::value::LAN_REASON_OFFLINE))
        }
        LanPairingRejectionReason::Anonymous => {
            Some(ReasonText(constants::value::LAN_REASON_ANONYMOUS))
        }
        LanPairingRejectionReason::Revoked => {
            Some(ReasonText(constants::value::LAN_REASON_REVOKED))
        }
        _ => None,
    }
}
