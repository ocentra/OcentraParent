use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanHouseholdDeviceActionKind, LanHouseholdDeviceDecision,
};

use super::fields::{mark_device_paired, mark_device_revoked, restore_device};

pub(super) fn apply(
    device: &mut LanCanonicalHouseholdDevice,
    decision: &LanHouseholdDeviceDecision,
) {
    match decision.action_kind {
        LanHouseholdDeviceActionKind::Ignore | LanHouseholdDeviceActionKind::Revoke => {
            mark_device_revoked(device);
        }
        LanHouseholdDeviceActionKind::Restore => restore_device(device),
        LanHouseholdDeviceActionKind::Assign | LanHouseholdDeviceActionKind::Trust => {
            mark_device_paired(device);
        }
        LanHouseholdDeviceActionKind::Rename => {}
    }
}
