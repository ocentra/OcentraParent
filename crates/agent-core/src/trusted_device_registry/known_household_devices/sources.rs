use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceSource, LanCanonicalHouseholdSurface,
};

pub(super) fn merge_source_labels(
    existing: &mut Vec<LanCanonicalHouseholdDeviceSource>,
    incoming: Vec<LanCanonicalHouseholdDeviceSource>,
) {
    for source in incoming {
        if !existing.contains(&source) {
            existing.push(source);
        }
    }
}

pub(super) fn merge_surfaces(
    existing: &mut Vec<LanCanonicalHouseholdSurface>,
    incoming: Vec<LanCanonicalHouseholdSurface>,
) {
    for surface in incoming {
        if !existing.contains(&surface) {
            existing.push(surface);
        }
    }
}

pub(super) fn merge_roles(
    existing: &mut Vec<
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceRole,
    >,
    incoming: Vec<
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceRole,
    >,
) {
    for role in incoming {
        if !existing.contains(&role) {
            existing.push(role);
        }
    }
}
