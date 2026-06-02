use ocentra_parent_agent_protocol::{
    constants, LanCanonicalHouseholdDeviceSource, LanDiscoveryEvidenceConfidence,
    LanDiscoveryEvidenceKind, LanDiscoveryEvidenceRecord, LanDiscoveryEvidenceSource,
    LanPairingDeviceRef,
};

use super::{compact_identifier, known_hostname};

struct EvidenceContext {
    source: LanDiscoveryEvidenceSource,
    confidence: LanDiscoveryEvidenceConfidence,
}

pub(super) fn evidence_records_for(
    device: &LanPairingDeviceRef,
    source: LanCanonicalHouseholdDeviceSource,
) -> Vec<LanDiscoveryEvidenceRecord> {
    let context = evidence_context_for(&source);
    let mut records = Vec::new();
    push_network_identity_evidence(&mut records, device, &context);
    push_agent_evidence(&mut records, device);
    push_trusted_registry_evidence(&mut records, device, &source);
    push_router_evidence(&mut records, device, &context);
    push_fallback_evidence(&mut records, device);
    records
}

fn push_network_identity_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    context: &EvidenceContext,
) {
    push_optional_evidence(
        records,
        device,
        context,
        LanDiscoveryEvidenceKind::IpAddress,
        device.ip_address.as_deref(),
        constants::lan_pairing::LAN_EVIDENCE_KEY_IP_PREFIX,
    );
    push_optional_evidence(
        records,
        device,
        context,
        LanDiscoveryEvidenceKind::MacAddress,
        device.mac_address.as_deref(),
        constants::lan_pairing::LAN_EVIDENCE_KEY_MAC_PREFIX,
    );
    push_optional_evidence(
        records,
        device,
        context,
        LanDiscoveryEvidenceKind::Hostname,
        known_hostname(device).as_deref(),
        constants::lan_pairing::LAN_EVIDENCE_KEY_HOSTNAME_PREFIX,
    );
    push_optional_evidence(
        records,
        device,
        context,
        LanDiscoveryEvidenceKind::Interface,
        device.network_interface.as_deref(),
        constants::lan_pairing::LAN_EVIDENCE_KEY_INTERFACE_PREFIX,
    );
}

fn push_optional_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    context: &EvidenceContext,
    evidence_kind: LanDiscoveryEvidenceKind,
    value: Option<&str>,
    merge_key_prefix: &str,
) {
    if let Some(value) = value {
        push_evidence_record(
            records,
            device,
            context.source.clone(),
            evidence_kind,
            value,
            merge_key_prefix,
            context.confidence.clone(),
        );
    }
}

fn push_agent_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
) {
    if let Some(agent_status) = device.agent_status.as_ref() {
        push_evidence_record(
            records,
            device,
            LanDiscoveryEvidenceSource::LocalService,
            LanDiscoveryEvidenceKind::ChildAgentPresence,
            agent_status,
            constants::lan_pairing::LAN_EVIDENCE_KEY_AGENT_PREFIX,
            LanDiscoveryEvidenceConfidence::Confirmed,
        );
    }
}

fn push_trusted_registry_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    source: &LanCanonicalHouseholdDeviceSource,
) {
    if *source == LanCanonicalHouseholdDeviceSource::TrustedRegistry {
        push_evidence_record(
            records,
            device,
            LanDiscoveryEvidenceSource::TrustedRegistry,
            LanDiscoveryEvidenceKind::TrustedRegistry,
            &device.device_id,
            constants::lan_pairing::LAN_EVIDENCE_KEY_TRUSTED_PREFIX,
            LanDiscoveryEvidenceConfidence::ManualRequired,
        );
    }
}

fn push_router_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    context: &EvidenceContext,
) {
    if device.platform == constants::lan_pairing::PLATFORM_ROUTER {
        let router_value = device.ip_address.as_ref().unwrap_or(&device.device_id);
        push_evidence_record(
            records,
            device,
            context.source.clone(),
            LanDiscoveryEvidenceKind::RouterClassification,
            router_value,
            constants::lan_pairing::LAN_EVIDENCE_KEY_ROUTER_PREFIX,
            context.confidence.clone(),
        );
    }
}

fn push_fallback_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
) {
    if records.is_empty() {
        push_evidence_record(
            records,
            device,
            LanDiscoveryEvidenceSource::ParentAssignment,
            LanDiscoveryEvidenceKind::ParentDecision,
            &device.device_id,
            constants::lan_pairing::LAN_EVIDENCE_KEY_TRUSTED_PREFIX,
            LanDiscoveryEvidenceConfidence::ManualRequired,
        );
    }
}

fn evidence_context_for(source: &LanCanonicalHouseholdDeviceSource) -> EvidenceContext {
    EvidenceContext {
        source: evidence_source_for(source),
        confidence: evidence_confidence_for(source),
    }
}

fn evidence_source_for(source: &LanCanonicalHouseholdDeviceSource) -> LanDiscoveryEvidenceSource {
    match source {
        LanCanonicalHouseholdDeviceSource::LocalService => LanDiscoveryEvidenceSource::LocalService,
        LanCanonicalHouseholdDeviceSource::NetworkNeighbor => {
            LanDiscoveryEvidenceSource::WindowsNeighborTable
        }
        LanCanonicalHouseholdDeviceSource::TrustedRegistry => {
            LanDiscoveryEvidenceSource::TrustedRegistry
        }
    }
}

fn evidence_confidence_for(
    source: &LanCanonicalHouseholdDeviceSource,
) -> LanDiscoveryEvidenceConfidence {
    match source {
        LanCanonicalHouseholdDeviceSource::LocalService => {
            LanDiscoveryEvidenceConfidence::Confirmed
        }
        LanCanonicalHouseholdDeviceSource::NetworkNeighbor => {
            LanDiscoveryEvidenceConfidence::Strong
        }
        LanCanonicalHouseholdDeviceSource::TrustedRegistry => {
            LanDiscoveryEvidenceConfidence::ManualRequired
        }
    }
}

fn push_evidence_record(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    source: LanDiscoveryEvidenceSource,
    evidence_kind: LanDiscoveryEvidenceKind,
    value: &str,
    merge_key_prefix: &str,
    confidence: LanDiscoveryEvidenceConfidence,
) {
    let normalized_value = normalized_evidence_value(value);
    let merge_key = evidence_key(merge_key_prefix, &normalized_value);
    if records.iter().any(|record| record.merge_key == merge_key) {
        return;
    }
    records.push(LanDiscoveryEvidenceRecord {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        evidence_id: evidence_id(&merge_key),
        source,
        evidence_kind,
        device_id: device.device_id.clone(),
        value: value.to_string(),
        normalized_value,
        first_seen_at: crate::time::timestamp_now(),
        last_seen_at: crate::time::timestamp_now(),
        expires_at: None,
        confidence,
        merge_key,
        note: None,
    });
}

fn evidence_key(prefix: &str, normalized_value: &str) -> String {
    let mut key = String::from(prefix);
    key.push_str(normalized_value);
    key
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
