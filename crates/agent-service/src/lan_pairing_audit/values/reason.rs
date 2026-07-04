use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

pub(super) fn reason_value(reason: &LanPairingRejectionReason) -> LogFieldValue {
    LogFieldValue::String(
        match reason {
            LanPairingRejectionReason::Malformed => constants::value::LAN_REASON_MALFORMED,
            LanPairingRejectionReason::WrongDevice => constants::value::LAN_REASON_WRONG_DEVICE,
            LanPairingRejectionReason::WrongOrigin => constants::value::LAN_REASON_WRONG_ORIGIN,
            LanPairingRejectionReason::UnsupportedRoute => {
                constants::value::LAN_REASON_UNSUPPORTED_ROUTE
            }
            LanPairingRejectionReason::Expired => constants::value::LAN_REASON_EXPIRED,
            LanPairingRejectionReason::Replayed => constants::value::LAN_REASON_REPLAYED,
            LanPairingRejectionReason::Stale => constants::value::LAN_REASON_STALE,
            LanPairingRejectionReason::Offline => constants::value::LAN_REASON_OFFLINE,
            LanPairingRejectionReason::Anonymous => constants::value::LAN_REASON_ANONYMOUS,
            LanPairingRejectionReason::Revoked => constants::value::LAN_REASON_REVOKED,
            LanPairingRejectionReason::ControllerLeaseMissing => {
                constants::value::LAN_REASON_CONTROLLER_LEASE_MISSING
            }
            LanPairingRejectionReason::ControllerLeaseExpired => {
                constants::value::LAN_REASON_CONTROLLER_LEASE_EXPIRED
            }
            LanPairingRejectionReason::WrongController => {
                constants::value::LAN_REASON_WRONG_CONTROLLER
            }
            LanPairingRejectionReason::SignedChildAgentContextUnavailable => {
                constants::value::LAN_REASON_SIGNED_CHILD_AGENT_CONTEXT_UNAVAILABLE
            }
            LanPairingRejectionReason::LocalNetworkDisabled => {
                constants::value::LAN_REASON_UNSUPPORTED_ROUTE
            }
            LanPairingRejectionReason::UnselectedDevice => {
                constants::value::LAN_REASON_UNSELECTED_DEVICE
            }
            LanPairingRejectionReason::ObserverReadOnly => {
                constants::value::LAN_REASON_OBSERVER_READ_ONLY
            }
            LanPairingRejectionReason::TakeoverDenied => {
                constants::value::LAN_REASON_TAKEOVER_DENIED
            }
            LanPairingRejectionReason::LanAiProviderUnavailable => {
                constants::value::LAN_REASON_LAN_AI_PROVIDER_UNAVAILABLE
            }
            LanPairingRejectionReason::LanAiJobUnauthorized => {
                constants::value::LAN_REASON_LAN_AI_JOB_UNAUTHORIZED
            }
        }
        .to_string(),
    )
}
