use ocentra_parent_agent_protocol::lan_pairing::LanPairingIntentKind;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::LanPairingParentAuthority;

#[path = "values/authentication_state.rs"]
mod authentication_state;
#[path = "values/intent_kind.rs"]
mod intent_kind;
#[path = "values/parent_authority.rs"]
mod parent_authority;
#[path = "values/reason.rs"]
mod reason;

pub(crate) fn intent_kind_value(intent_kind: &LanPairingIntentKind) -> LogFieldValue {
    intent_kind::intent_kind_value(intent_kind)
}

pub(crate) fn parent_authority_value(
    parent_authority: &LanPairingParentAuthority,
) -> LogFieldValue {
    parent_authority::parent_authority_value(parent_authority)
}

pub(crate) fn reason_value(reason: &LanPairingRejectionReason) -> LogFieldValue {
    reason::reason_value(reason)
}

pub(crate) fn authentication_state_value(
    reason: Option<&LanPairingRejectionReason>,
) -> LogFieldValue {
    authentication_state::authentication_state_value(reason)
}
