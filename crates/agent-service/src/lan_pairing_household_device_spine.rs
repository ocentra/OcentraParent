mod builders;
mod merge;
mod values;

use builders::{device_from_discovery, device_from_registry};
use merge::merge_device;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing::LanTrustedDeviceRegistryEntry;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceActionKind;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision;
use values::{option_overlaps, surfaces_for};

pub(crate) fn canonical_household_devices(
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
    trusted_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
) -> Vec<LanCanonicalHouseholdDevice> {
    let mut devices: Vec<LanCanonicalHouseholdDevice> = Vec::new();

    for discovered in discovered_devices {
        upsert_device(
            &mut devices,
            device_from_discovery(discovered),
            &discovered.child_device,
        );
    }

    for entry in trusted_registry {
        upsert_device(
            &mut devices,
            device_from_registry(entry),
            &entry.child_device,
        );
    }

    apply_household_device_decisions(&mut devices, household_device_decisions);
    devices
}

fn upsert_device(
    devices: &mut Vec<LanCanonicalHouseholdDevice>,
    device: LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
) {
    if let Some(existing) = devices
        .iter_mut()
        .find(|candidate| devices_match(candidate, source_ref, &device))
    {
        merge_device(existing, device);
        return;
    }

    devices.push(device);
}

fn devices_match(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
) -> bool {
    existing.canonical_device_id == device.canonical_device_id
        || option_overlaps(
            existing.network_identity.mac_address.as_ref(),
            source_ref.mac_address.as_ref(),
        )
        || authoritative_ip_overlap(existing, source_ref, device)
}

fn authoritative_ip_overlap(
    existing: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    device: &LanCanonicalHouseholdDevice,
) -> bool {
    let ip_matches = source_ref
        .ip_address
        .as_ref()
        .map(|ip| existing.network_identity.ip_addresses.contains(ip))
        .unwrap_or(false);
    ip_matches
        && (has_agent_or_registry_evidence(existing) || has_agent_or_registry_evidence(device))
}

fn has_agent_or_registry_evidence(device: &LanCanonicalHouseholdDevice) -> bool {
    device.classification == LanCanonicalHouseholdDeviceClassification::ChildAgent
        || device.child_agent_inventory.is_some()
        || device.source_labels.iter().any(|source| {
            matches!(
                source,
                LanCanonicalHouseholdDeviceSource::LocalService
                    | LanCanonicalHouseholdDeviceSource::TrustedRegistry
            )
        })
}

fn apply_household_device_decisions(
    devices: &mut [LanCanonicalHouseholdDevice],
    decisions: &[LanHouseholdDeviceDecision],
) {
    for decision in decisions
        .iter()
        .filter(|decision| decision.revoked_at.is_none())
    {
        if let Some(device) = devices
            .iter_mut()
            .find(|device| device.canonical_device_id == decision.canonical_device_id)
        {
            apply_household_device_decision(device, decision);
        }
    }
}

fn apply_household_device_decision(
    device: &mut LanCanonicalHouseholdDevice,
    decision: &LanHouseholdDeviceDecision,
) {
    if let Some(display_name) = decision
        .display_name
        .as_ref()
        .filter(|value| !value.is_empty())
    {
        device.display_name = display_name.clone();
        if let Some(inventory) = device.child_agent_inventory.as_mut() {
            inventory.device_name = display_name.clone();
        }
    }

    match &decision.action_kind {
        LanHouseholdDeviceActionKind::Ignore => {
            device.discovery_state = LanPairingProductionDiscoveryState::Revoked;
            device.trust_state = LanPairingTrustState::Revoked;
            device.enrollable = false;
            device.route_id = None;
            device.route_state = LanCanonicalHouseholdRouteState::Unavailable;
            device.policy_target_surfaces = surfaces_for(false);
        }
        LanHouseholdDeviceActionKind::Restore => {
            if device.discovery_state == LanPairingProductionDiscoveryState::Revoked {
                device.discovery_state = LanPairingProductionDiscoveryState::Discovered;
            }
            if device.trust_state == LanPairingTrustState::Revoked {
                device.trust_state = LanPairingTrustState::Unpaired;
            }
            device.enrollable =
                device.classification == LanCanonicalHouseholdDeviceClassification::ChildAgent;
            device.policy_target_surfaces = surfaces_for(device.enrollable);
        }
        LanHouseholdDeviceActionKind::Assign
        | LanHouseholdDeviceActionKind::Rename
        | LanHouseholdDeviceActionKind::Trust => {}
    }

    push_parent_decision_evidence(device, decision);
}

fn push_parent_decision_evidence(
    device: &mut LanCanonicalHouseholdDevice,
    decision: &LanHouseholdDeviceDecision,
) {
    let merge_key = parent_decision_merge_key(&decision.action_id);
    if device
        .network_identity
        .evidence_records
        .iter()
        .any(|record| record.merge_key == merge_key)
    {
        return;
    }
    device
        .network_identity
        .evidence_records
        .push(LanDiscoveryEvidenceRecord {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            evidence_id: parent_decision_evidence_id(&decision.action_id),
            source: LanDiscoveryEvidenceSource::ParentAssignment,
            evidence_kind: LanDiscoveryEvidenceKind::ParentDecision,
            device_id: decision.canonical_device_id.clone(),
            value: household_action_value(&decision.action_kind).to_string(),
            normalized_value: household_action_value(&decision.action_kind).to_string(),
            first_seen_at: decision.decided_at.clone(),
            last_seen_at: decision.decided_at.clone(),
            expires_at: None,
            confidence: parent_decision_confidence(&decision.action_kind),
            merge_key,
            note: decision.display_name.clone(),
        });
}

fn parent_decision_merge_key(action_id: &str) -> String {
    let mut key = String::from(constants::lan_pairing::LAN_EVIDENCE_KEY_PARENT_DECISION_PREFIX);
    key.push_str(action_id);
    key
}

fn parent_decision_evidence_id(action_id: &str) -> String {
    let mut id = String::from(constants::lan_pairing::LAN_EVIDENCE_ID_PREFIX);
    id.push_str(action_id);
    id
}

fn household_action_value(action_kind: &LanHouseholdDeviceActionKind) -> &'static str {
    match action_kind {
        LanHouseholdDeviceActionKind::Assign => constants::lan_pairing::HOUSEHOLD_ACTION_ASSIGN,
        LanHouseholdDeviceActionKind::Rename => constants::lan_pairing::HOUSEHOLD_ACTION_RENAME,
        LanHouseholdDeviceActionKind::Ignore => constants::lan_pairing::HOUSEHOLD_ACTION_IGNORE,
        LanHouseholdDeviceActionKind::Restore => constants::lan_pairing::HOUSEHOLD_ACTION_RESTORE,
        LanHouseholdDeviceActionKind::Trust => constants::lan_pairing::HOUSEHOLD_ACTION_TRUST,
    }
}

fn parent_decision_confidence(
    action_kind: &LanHouseholdDeviceActionKind,
) -> LanDiscoveryEvidenceConfidence {
    if matches!(action_kind, LanHouseholdDeviceActionKind::Ignore) {
        LanDiscoveryEvidenceConfidence::Rejected
    } else {
        LanDiscoveryEvidenceConfidence::Confirmed
    }
}
