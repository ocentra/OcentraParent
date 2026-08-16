use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind,
    LanDiscoveryEvidenceRecord, LanDiscoveryEvidenceSource, LanHouseholdDeviceActionKind,
    LanHouseholdDeviceDecision,
};

pub(super) fn push_parent_decision_evidence(
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
    let value = household_action_value(&decision.action_kind).to_string();
    LanDiscoveryEvidenceRecord {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        evidence_id: parent_decision_evidence_id(&decision.action_id),
        source: LanDiscoveryEvidenceSource::ParentAssignment,
        evidence_kind: LanDiscoveryEvidenceKind::ParentDecision,
        device_id: decision.canonical_device_id.clone(),
        value: value.clone(),
        normalized_value: value,
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
