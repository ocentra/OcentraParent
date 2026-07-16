use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceActionKind;
use ocentra_parent_agent_protocol::logging::LogFields;

use super::field_values::required_payload_text;

pub(super) fn required_household_action_kind(
    fields: &LogFields,
) -> Result<LanHouseholdDeviceActionKind, LanPairingRejectionReason> {
    match required_payload_text(
        fields,
        constants::lan_pairing::HOUSEHOLD_ACTION_KIND_FIELD.into(),
    )?
    .as_payload_text_ref()
    .0
    {
        constants::lan_pairing::HOUSEHOLD_ACTION_ASSIGN => Ok(LanHouseholdDeviceActionKind::Assign),
        constants::lan_pairing::HOUSEHOLD_ACTION_RENAME => Ok(LanHouseholdDeviceActionKind::Rename),
        constants::lan_pairing::HOUSEHOLD_ACTION_IGNORE => Ok(LanHouseholdDeviceActionKind::Ignore),
        constants::lan_pairing::HOUSEHOLD_ACTION_REVOKE => Ok(LanHouseholdDeviceActionKind::Revoke),
        constants::lan_pairing::HOUSEHOLD_ACTION_RESTORE => {
            Ok(LanHouseholdDeviceActionKind::Restore)
        }
        constants::lan_pairing::HOUSEHOLD_ACTION_TRUST => Ok(LanHouseholdDeviceActionKind::Trust),
        _ => Err(LanPairingRejectionReason::Malformed),
    }
}

pub(super) fn required_parent_authority(
    fields: &LogFields,
) -> Result<LanPairingParentAuthority, LanPairingRejectionReason> {
    match required_payload_text(fields, constants::field::LAN_PARENT_AUTHORITY.into())?
        .as_payload_text_ref()
        .0
    {
        constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER => {
            Ok(LanPairingParentAuthority::ActiveController)
        }
        constants::value::LAN_PARENT_AUTHORITY_OBSERVER => Ok(LanPairingParentAuthority::Observer),
        _ => Err(LanPairingRejectionReason::Malformed),
    }
}
