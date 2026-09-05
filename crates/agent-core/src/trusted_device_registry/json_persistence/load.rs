use ocentra_parent_agent_protocol::{
    constants, lan_pairing::LanTrustedDeviceRegistryEntry,
    lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice,
};
use serde_json::Value;

use super::super::TrustedDeviceRegistry;
use super::{
    ACCEPTED_CHALLENGE_IDS_KEY, ACCEPTED_INTENT_IDS_KEY, CONTROLLER_LEASE_KEY, SIGNER_ANCHORS_KEY,
    SIGNER_ANCHOR_GENERATIONS_KEY,
};

mod authority;
mod fields;
mod replay;

pub(super) fn from_json_text(content: &str) -> Option<TrustedDeviceRegistry> {
    if let Ok(entries) = serde_json::from_str::<Vec<LanTrustedDeviceRegistryEntry>>(content) {
        return Some(TrustedDeviceRegistry::from_entries(entries));
    }

    let value = serde_json::from_str::<Value>(content).ok()?;
    let entries = serde_json::from_value::<Vec<LanTrustedDeviceRegistryEntry>>(
        value.get(constants::field::ENTRIES)?.clone(),
    )
    .ok()?;
    let mut registry = TrustedDeviceRegistry::from_entries(entries);
    registry.selected_pairing_id =
        fields::strict_optional_string(&value, constants::field::LAN_SELECTED_PAIRING_ID)?;
    registry.selected_route_stale_at =
        fields::strict_optional_string(&value, constants::field::LAN_SELECTED_ROUTE_STALE_AT)?;
    registry.selected_route_offline_at =
        fields::strict_optional_string(&value, constants::field::LAN_SELECTED_ROUTE_OFFLINE_AT)?;
    registry.household_device_decisions = fields::strict_household_decisions(&value)?;
    registry.known_household_devices = fields::strict_optional_array::<LanCanonicalHouseholdDevice>(
        &value,
        constants::lan_pairing::REGISTRY_KEY_KNOWN_HOUSEHOLD_DEVICES,
    )?;
    registry.signer_anchor_generations =
        fields::strict_optional_object(&value, SIGNER_ANCHOR_GENERATIONS_KEY)?;
    registry.controller_lease = match value.get(CONTROLLER_LEASE_KEY) {
        None | Some(Value::Null) => None,
        Some(value) => Some(serde_json::from_value(value.clone()).ok()?),
    };
    // Missing replay fields are an explicit legacy migration to empty history;
    // a present field must deserialize or the whole registry load fails closed.
    registry.accepted_intent_ids = replay::strict_replay_ids(&value, ACCEPTED_INTENT_IDS_KEY)?;
    registry.accepted_challenge_ids =
        replay::strict_replay_ids(&value, ACCEPTED_CHALLENGE_IDS_KEY)?;
    Some(registry)
}

pub(super) fn reject_untrusted_paired_entries(
    registry: &TrustedDeviceRegistry,
) -> std::io::Result<()> {
    authority::reject_untrusted_paired_entries(registry)
}

pub(super) fn reject_untrusted_signer_anchors(value: &Value) -> std::io::Result<()> {
    authority::reject_untrusted_signer_anchors(value, SIGNER_ANCHORS_KEY)
}
