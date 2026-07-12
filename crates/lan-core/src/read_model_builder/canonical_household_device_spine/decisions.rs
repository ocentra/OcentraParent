#[path = "decision_application.rs"]
mod application;
#[path = "decision_assignments.rs"]
mod assignments;

use std::collections::HashMap;

use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanHouseholdDeviceDecision,
};

pub(super) fn assigned_child_profiles(
    decisions: &[LanHouseholdDeviceDecision],
) -> HashMap<String, String> {
    assignments::assigned_child_profiles(decisions)
}

pub(super) fn apply_household_device_decisions(
    devices: &mut [LanCanonicalHouseholdDevice],
    decisions: &[LanHouseholdDeviceDecision],
) {
    application::apply_household_device_decisions(devices, decisions)
}
