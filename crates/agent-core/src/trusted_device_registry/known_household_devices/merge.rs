use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;

use super::rank::{stronger_discovery_state, stronger_route_state, stronger_trust_state};
use super::{classification, display, identity, inventory, network_identity, sources};

pub(super) fn same_known_household_device(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    identity::same_known_household_device(existing, incoming)
}

pub(super) fn merge_known_household_device(
    existing: &mut LanCanonicalHouseholdDevice,
    incoming: LanCanonicalHouseholdDevice,
) {
    existing.display_name = display::preferred_display_name(
        &existing.display_name,
        &incoming.display_name,
        &incoming.network_identity.evidence_records,
    );
    existing.classification = classification::preferred_classification(
        existing.classification.clone(),
        incoming.classification.clone(),
    );
    existing.enrollable = existing.enrollable || incoming.enrollable;
    let merged_discovery_state = stronger_discovery_state(
        existing.discovery_state.clone(),
        incoming.discovery_state.clone(),
    );
    existing.discovery_state = merged_discovery_state;
    let merged_trust_state = stronger_trust_state(existing.trust_state, incoming.trust_state);
    existing.trust_state = merged_trust_state;
    existing.route_id = existing.route_id.clone().or(incoming.route_id);
    existing.route_state =
        stronger_route_state(existing.route_state.clone(), incoming.route_state.clone());
    existing.network_mode = incoming.network_mode;
    existing.network_identity = network_identity::merge_network_identity(
        existing.network_identity.clone(),
        incoming.network_identity,
        &merged_trust_state,
        &existing.source_labels,
        &incoming.source_labels,
    );
    sources::merge_source_labels(&mut existing.source_labels, incoming.source_labels);
    sources::merge_surfaces(
        &mut existing.policy_target_surfaces,
        incoming.policy_target_surfaces,
    );
    sources::merge_roles(&mut existing.role_badges, incoming.role_badges);
    inventory::merge_child_agent_inventory(
        existing,
        incoming.child_agent_inventory,
        merged_trust_state,
    );
}
