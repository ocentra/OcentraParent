use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;

use super::collections::{merge_roles, merge_sources, merged_surfaces};
use super::identity::merge_network_identity;
use super::merge_ranking::{
    stronger_classification, stronger_discovery_state, stronger_route_state, stronger_trust_state,
};

pub(super) fn merge_device(
    existing: &mut LanCanonicalHouseholdDevice,
    incoming: LanCanonicalHouseholdDevice,
) {
    existing.classification = stronger_classification(
        existing.classification.clone(),
        incoming.classification.clone(),
    );
    existing.enrollable = existing.enrollable || incoming.enrollable;
    existing.discovery_state = stronger_discovery_state(
        existing.discovery_state.clone(),
        incoming.discovery_state.clone(),
    );
    let merged_trust_state = stronger_trust_state(existing.trust_state, incoming.trust_state);
    existing.trust_state = merged_trust_state;
    existing.route_id = existing.route_id.clone().or(incoming.route_id);
    existing.route_state = stronger_route_state(existing.route_state.clone(), incoming.route_state);
    existing.display_name = preferred_display_name(&existing.display_name, &incoming.display_name);
    existing.network_identity = merge_network_identity(
        existing.network_identity.clone(),
        incoming.network_identity,
        &merged_trust_state,
        &existing.source_labels,
        &incoming.source_labels,
    );
    merge_sources(&mut existing.source_labels, incoming.source_labels);
    existing.policy_target_surfaces =
        merged_surfaces(existing.enrollable, incoming.policy_target_surfaces);
    merge_roles(&mut existing.role_badges, incoming.role_badges);
    if incoming.child_agent_inventory.is_some()
        && (merged_trust_state == LanPairingTrustState::Paired
            || existing.child_agent_inventory.is_none())
    {
        existing.child_agent_inventory = incoming.child_agent_inventory;
    }
}

fn preferred_display_name(existing: &str, incoming: &str) -> String {
    if existing.starts_with(
        ocentra_parent_agent_protocol::constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX,
    ) {
        incoming.to_string()
    } else {
        existing.to_string()
    }
}
