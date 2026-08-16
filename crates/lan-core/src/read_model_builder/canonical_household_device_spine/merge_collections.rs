use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceRole, LanCanonicalHouseholdDeviceSource,
    LanCanonicalHouseholdSurface,
};

use super::super::values::surfaces_for;

pub(super) fn merge_roles(
    existing: &mut Vec<LanCanonicalHouseholdDeviceRole>,
    incoming: Vec<LanCanonicalHouseholdDeviceRole>,
) {
    append_unique(existing, incoming, |left, right| left == right);
}

pub(super) fn merge_sources(
    existing: &mut Vec<LanCanonicalHouseholdDeviceSource>,
    incoming: Vec<LanCanonicalHouseholdDeviceSource>,
) {
    append_unique(existing, incoming, |left, right| left == right);
}

pub(super) fn merged_surfaces(
    is_child_agent: bool,
    incoming: Vec<LanCanonicalHouseholdSurface>,
) -> Vec<LanCanonicalHouseholdSurface> {
    let mut surfaces = surfaces_for(is_child_agent);
    append_unique(&mut surfaces, incoming, |left, right| left == right);
    surfaces
}

pub(super) fn merge_strings(existing: &mut Vec<String>, incoming: Vec<String>) {
    append_unique(existing, incoming, |left, right| {
        left.eq_ignore_ascii_case(right)
    });
}

fn append_unique<T>(existing: &mut Vec<T>, incoming: Vec<T>, matches: impl Fn(&T, &T) -> bool) {
    for value in incoming {
        if !existing.iter().any(|entry| matches(entry, &value)) {
            existing.push(value);
        }
    }
}
