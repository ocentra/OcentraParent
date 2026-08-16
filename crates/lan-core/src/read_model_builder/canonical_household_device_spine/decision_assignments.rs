use std::collections::HashMap;

use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanHouseholdDeviceActionKind, LanHouseholdDeviceDecision,
};

pub(super) fn assigned_child_profiles(
    decisions: &[LanHouseholdDeviceDecision],
) -> HashMap<String, String> {
    let mut assignments = HashMap::new();
    for decision in decisions {
        if decision.revoked_at.is_none() {
            apply_assignment_decision(&mut assignments, decision);
        }
    }
    assignments
}

fn apply_assignment_decision(
    assignments: &mut HashMap<String, String>,
    decision: &LanHouseholdDeviceDecision,
) {
    match decision.action_kind {
        LanHouseholdDeviceActionKind::Assign | LanHouseholdDeviceActionKind::Trust => {
            let Some(child_profile_id) =
                normalized_child_profile_id(decision.child_profile_id.as_deref())
            else {
                return;
            };
            assignments.insert(decision.canonical_device_id.clone(), child_profile_id);
        }
        LanHouseholdDeviceActionKind::Ignore | LanHouseholdDeviceActionKind::Revoke => {
            assignments.remove(&decision.canonical_device_id);
        }
        LanHouseholdDeviceActionKind::Rename | LanHouseholdDeviceActionKind::Restore => {}
    }
}

fn normalized_child_profile_id(child_profile_id: Option<&str>) -> Option<String> {
    let normalized = child_profile_id?
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect::<String>();
    (!normalized.is_empty()).then_some(normalized)
}
