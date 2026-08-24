use std::collections::{BTreeMap, BTreeSet};
use std::io;

use ocentra_parent_agent_protocol::{constants, lan_pairing::LanTrustedDeviceRegistryEntry};
use serde_json::Value;

use super::super::{
    known_household_devices::{
        household_device_decisions_from_json, known_household_devices_from_json, optional_string,
    },
    signer_authority_types::LanTrustedDeviceSignerAnchor,
    TrustedDeviceRegistry,
};
use super::{
    ACCEPTED_CHALLENGE_IDS_KEY, ACCEPTED_INTENT_IDS_KEY, SIGNER_ANCHORS_KEY,
    SIGNER_ANCHOR_GENERATIONS_KEY,
};

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
        optional_string(&value, constants::field::LAN_SELECTED_PAIRING_ID);
    registry.selected_route_stale_at =
        optional_string(&value, constants::field::LAN_SELECTED_ROUTE_STALE_AT);
    registry.selected_route_offline_at =
        optional_string(&value, constants::field::LAN_SELECTED_ROUTE_OFFLINE_AT);
    registry.household_device_decisions = bounded_household_decisions(
        household_device_decisions_from_json(&value).unwrap_or_default(),
    );
    registry.known_household_devices =
        known_household_devices_from_json(&value).unwrap_or_default();
    registry.signer_anchor_generations = value
        .get(SIGNER_ANCHOR_GENERATIONS_KEY)
        .and_then(|generations| serde_json::from_value(generations.clone()).ok())
        .unwrap_or_default();
    // Missing replay fields are an explicit legacy migration to empty history;
    // a present field must deserialize or the whole registry load fails closed.
    registry.accepted_intent_ids = bounded_replay_ids(match value.get(ACCEPTED_INTENT_IDS_KEY) {
        Some(intent_ids) => serde_json::from_value(intent_ids.clone()).ok()?,
        None => BTreeSet::new(),
    });
    registry.accepted_challenge_ids =
        bounded_replay_ids(match value.get(ACCEPTED_CHALLENGE_IDS_KEY) {
            Some(challenge_ids) => serde_json::from_value(challenge_ids.clone()).ok()?,
            None => BTreeSet::new(),
        });
    Some(registry)
}

fn bounded_replay_ids(mut ids: BTreeSet<String>) -> BTreeSet<String> {
    while ids.len() > constants::lan_pairing::LAN_PAIRING_MAX_ACCEPTED_INTENT_HISTORY {
        if let Some(oldest) = ids.iter().next().cloned() {
            ids.remove(&oldest);
        }
    }
    ids
}

fn bounded_household_decisions(
    decisions: Vec<ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision>,
) -> Vec<
    ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision,
> {
    let max = constants::lan_pairing::LAN_PAIRING_MAX_HOUSEHOLD_DECISION_HISTORY;
    if decisions.len() <= max {
        return decisions;
    }
    let skip = decisions.len() - max;
    decisions.into_iter().skip(skip).collect()
}

pub(super) fn reject_untrusted_signer_anchors(value: &Value) -> io::Result<()> {
    let Some(anchors) = value.get(SIGNER_ANCHORS_KEY) else {
        return Ok(());
    };
    let persisted =
        serde_json::from_value::<BTreeMap<String, LanTrustedDeviceSignerAnchor>>(anchors.clone())
            .map_err(|_error| io::Error::from(io::ErrorKind::InvalidData))?;
    if persisted.is_empty() {
        Ok(())
    } else {
        Err(io::Error::from(io::ErrorKind::InvalidData))
    }
}
