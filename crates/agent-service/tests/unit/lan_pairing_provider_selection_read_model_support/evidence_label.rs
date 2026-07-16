use super::{LanPairingRejectionReason, LanSelectedRouteTarget};
use std::string::String as TestString;

pub(super) fn evidence_label_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
    rejection_reason: Option<&LanPairingRejectionReason>,
) -> TestString {
    match (selected, rejection_reason) {
        (None, _) | (_, Some(LanPairingRejectionReason::Anonymous)) => {
            super::constants::value::LAN_REASON_ANONYMOUS
        }
        (_, Some(LanPairingRejectionReason::Offline)) => {
            super::constants::value::LAN_REASON_OFFLINE
        }
        (_, Some(LanPairingRejectionReason::Stale)) => super::constants::value::LAN_REASON_STALE,
        (_, Some(LanPairingRejectionReason::Revoked)) => {
            super::constants::value::LAN_REASON_REVOKED
        }
        (_, Some(LanPairingRejectionReason::Expired)) => {
            super::constants::value::LAN_REASON_EXPIRED
        }
        (_, Some(LanPairingRejectionReason::LanAiProviderUnavailable)) => {
            super::constants::value::LAN_REASON_LAN_AI_PROVIDER_UNAVAILABLE
        }
        (_, Some(_)) => super::constants::value::LAN_CONTROL_REJECTED,
        (_, None) => super::constants::value::LAN_AUDIT_LAN_AI_JOB_COMPLETED,
    }
    .to_string()
}
