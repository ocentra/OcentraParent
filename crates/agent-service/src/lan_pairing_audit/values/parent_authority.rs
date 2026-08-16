use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::LanPairingParentAuthority;

pub(super) fn parent_authority_value(
    parent_authority: &LanPairingParentAuthority,
) -> LogFieldValue {
    LogFieldValue::String(
        match parent_authority {
            LanPairingParentAuthority::ActiveController => {
                constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER
            }
            LanPairingParentAuthority::Observer => constants::value::LAN_PARENT_AUTHORITY_OBSERVER,
        }
        .to_string(),
    )
}
