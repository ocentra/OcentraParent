use std::collections::HashMap;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdRouteState, LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind,
    LanDiscoveryEvidenceRecord, LanDiscoveryEvidenceSource, LanHouseholdDeviceActionKind,
    LanHouseholdDeviceDecision,
};

use super::values::surfaces_for;

pub(super) fn assigned_child_profiles(
    decisions: &[LanHouseholdDeviceDecision],
) -> HashMap<String, String> {
    let mut assignments = HashMap::new();
    for decision in decisions {
        if decision.revoked_at.is_some() {
            continue;
        }
        apply_assignment_decision(&mut assignments, decision);
    }
    assignments
}

pub(super) fn apply_household_device_decisions(
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

fn apply_household_device_decision(
    device: &mut LanCanonicalHouseholdDevice,
    decision: &LanHouseholdDeviceDecision,
) {
    apply_display_name(device, decision);
    match &decision.action_kind {
        LanHouseholdDeviceActionKind::Ignore | LanHouseholdDeviceActionKind::Revoke => {
            mark_device_revoked(device);
        }
        LanHouseholdDeviceActionKind::Restore => restore_device(device),
        LanHouseholdDeviceActionKind::Assign | LanHouseholdDeviceActionKind::Trust => {
            mark_device_paired(device);
        }
        LanHouseholdDeviceActionKind::Rename => {}
    }
    push_parent_decision_evidence(device, decision);
}

fn apply_display_name(
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
}

fn mark_device_revoked(device: &mut LanCanonicalHouseholdDevice) {
    device.discovery_state = LanPairingProductionDiscoveryState::Revoked;
    device.trust_state = LanPairingTrustState::Revoked;
    device.enrollable = false;
    device.route_id = None;
    device.route_state = LanCanonicalHouseholdRouteState::Unavailable;
    device.policy_target_surfaces = surfaces_for(false);
}

fn restore_device(device: &mut LanCanonicalHouseholdDevice) {
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

fn mark_device_paired(device: &mut LanCanonicalHouseholdDevice) {
    device.trust_state = LanPairingTrustState::Paired;
    if let Some(inventory) = device.child_agent_inventory.as_mut() {
        inventory.pairing_trust_state = LanPairingTrustState::Paired;
    }
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
        .push(parent_decision_evidence_record(decision, merge_key));
}

fn parent_decision_evidence_record(
    decision: &LanHouseholdDeviceDecision,
    merge_key: String,
) -> LanDiscoveryEvidenceRecord {
    LanDiscoveryEvidenceRecord {
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
    }
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
        LanHouseholdDeviceActionKind::Revoke => constants::lan_pairing::HOUSEHOLD_ACTION_REVOKE,
        LanHouseholdDeviceActionKind::Restore => constants::lan_pairing::HOUSEHOLD_ACTION_RESTORE,
        LanHouseholdDeviceActionKind::Trust => constants::lan_pairing::HOUSEHOLD_ACTION_TRUST,
    }
}

fn parent_decision_confidence(
    action_kind: &LanHouseholdDeviceActionKind,
) -> LanDiscoveryEvidenceConfidence {
    if matches!(
        action_kind,
        LanHouseholdDeviceActionKind::Ignore | LanHouseholdDeviceActionKind::Revoke
    ) {
        LanDiscoveryEvidenceConfidence::Rejected
    } else {
        LanDiscoveryEvidenceConfidence::Confirmed
    }
}
