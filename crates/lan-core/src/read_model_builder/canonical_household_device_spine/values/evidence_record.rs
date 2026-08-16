use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord,
    LanDiscoveryEvidenceSource,
};

use super::super::value_support::compact_identifier;
use super::EvidenceContext;

pub(super) struct EvidenceRecordInput<'a> {
    pub(super) device: &'a LanPairingDeviceRef,
    pub(super) source: LanDiscoveryEvidenceSource,
    pub(super) evidence_kind: LanDiscoveryEvidenceKind,
    pub(super) value: &'a str,
    pub(super) merge_key_prefix: &'a str,
    pub(super) confidence: LanDiscoveryEvidenceConfidence,
    pub(super) observed_at: &'a str,
    pub(super) note: Option<String>,
}

pub(super) struct OptionalEvidenceInput<'a> {
    pub(super) device: &'a LanPairingDeviceRef,
    pub(super) source: LanDiscoveryEvidenceSource,
    pub(super) evidence_kind: LanDiscoveryEvidenceKind,
    pub(super) value: Option<&'a str>,
    pub(super) merge_key_prefix: &'a str,
    pub(super) confidence: LanDiscoveryEvidenceConfidence,
    pub(super) observed_at: &'a str,
}

pub(super) fn push_optional_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    context: &EvidenceContext,
    evidence_kind: LanDiscoveryEvidenceKind,
    value: Option<&str>,
    merge_key_prefix: &str,
    observed_at: &str,
) {
    push_optional_evidence_with_confidence(
        records,
        OptionalEvidenceInput {
            device,
            source: context.source.clone(),
            evidence_kind,
            value,
            merge_key_prefix,
            confidence: context.confidence.clone(),
            observed_at,
        },
    );
}

pub(super) fn push_optional_evidence_with_confidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    input: OptionalEvidenceInput<'_>,
) {
    let Some(value) = input.value else {
        return;
    };
    push_evidence_record(
        records,
        EvidenceRecordInput {
            device: input.device,
            source: input.source,
            evidence_kind: input.evidence_kind,
            value,
            merge_key_prefix: input.merge_key_prefix,
            confidence: input.confidence,
            observed_at: input.observed_at,
            note: None,
        },
    );
}

pub(super) fn push_evidence_record(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    input: EvidenceRecordInput<'_>,
) {
    let normalized_value = normalized_evidence_value(input.value);
    let merge_key = evidence_key(input.merge_key_prefix, &normalized_value);
    if records.iter().any(|record| {
        same_evidence_record_identity(
            record,
            &input.source,
            &input.evidence_kind,
            &merge_key,
            &input.device.device_id,
        )
    }) {
        return;
    }
    let identity_key = evidence_identity_key(
        &input.source,
        &input.evidence_kind,
        &input.device.device_id,
        &merge_key,
    );
    records.push(LanDiscoveryEvidenceRecord {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        evidence_id: evidence_id(&identity_key),
        source: input.source,
        evidence_kind: input.evidence_kind,
        device_id: input.device.device_id.clone(),
        value: input.value.to_string(),
        normalized_value,
        first_seen_at: input.observed_at.to_string(),
        last_seen_at: input.observed_at.to_string(),
        expires_at: None,
        confidence: input.confidence,
        merge_key,
        note: input.note,
    });
}

fn same_evidence_record_identity(
    record: &LanDiscoveryEvidenceRecord,
    source: &LanDiscoveryEvidenceSource,
    evidence_kind: &LanDiscoveryEvidenceKind,
    merge_key: &str,
    device_id: &str,
) -> bool {
    record.source == *source
        && record.evidence_kind == *evidence_kind
        && record.merge_key.eq_ignore_ascii_case(merge_key)
        && record.device_id.eq_ignore_ascii_case(device_id)
}

fn evidence_key(prefix: &str, normalized_value: &str) -> String {
    let mut key = String::from(prefix);
    key.push_str(normalized_value);
    key
}

fn evidence_identity_key(
    source: &LanDiscoveryEvidenceSource,
    evidence_kind: &LanDiscoveryEvidenceKind,
    device_id: &str,
    merge_key: &str,
) -> String {
    format!("{source:?}:{evidence_kind:?}:{device_id}:{merge_key}")
}

fn evidence_id(merge_key: &str) -> String {
    let mut id = String::from(constants::lan_pairing::LAN_EVIDENCE_ID_PREFIX);
    id.push_str(&compact_identifier(merge_key));
    id
}

fn normalized_evidence_value(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric() || *character == '.')
        .flat_map(char::to_lowercase)
        .collect()
}
