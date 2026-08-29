#[path = "decision_application_actions.rs"]
mod actions;
#[path = "decision_evidence.rs"]
mod evidence;
#[path = "decision_application_fields.rs"]
mod fields;

use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanHouseholdDeviceActionKind, LanHouseholdDeviceDecision,
};

pub(super) fn apply_household_device_decisions(
    devices: &mut [LanCanonicalHouseholdDevice],
    decisions: &[LanHouseholdDeviceDecision],
) {
    for decision in decisions.iter().filter(|decision| {
        match &decision.action_kind {
            LanHouseholdDeviceActionKind::Revoke => decision.revoked_at.is_some(),
            _ => decision.revoked_at.is_none(),
        }
    }) {
        let Some(device) = devices
            .iter_mut()
            .find(|device| device.canonical_device_id == decision.canonical_device_id)
        else {
            continue;
        };
        fields::apply_display_name(device, decision);
        actions::apply(device, decision);
        evidence::push_parent_decision_evidence(device, decision);
    }
}
