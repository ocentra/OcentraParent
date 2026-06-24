use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;

use super::{compact_identifier, known_hostname};
use crate::mac_identity::{LanMacIdentityAssessment, LanMacIdentityDisposition};
use crate::network_inventory::{is_confirmed_agent_status, is_service_identity_probe_status};

struct EvidenceContext {
    source: LanDiscoveryEvidenceSource,
    confidence: LanDiscoveryEvidenceConfidence,
}

pub(super) fn evidence_records_for(
    device: &LanPairingDeviceRef,
    source: &LanCanonicalHouseholdDeviceSource,
    evidence_sources: &[LanDiscoveryEvidenceSource],
    hint_sources: &[LanDiscoveryEvidenceSource],
    observed_at: &str,
    mac_assessment: Option<&LanMacIdentityAssessment>,
) -> Vec<LanDiscoveryEvidenceRecord> {
    let context = evidence_context_for(source, evidence_sources);
    let mut records = Vec::new();
    push_network_identity_evidence(&mut records, device, &context, observed_at, mac_assessment);
    push_vendor_evidence(&mut records, device, &context, observed_at, mac_assessment);
    push_agent_evidence(&mut records, device, observed_at);
    push_hint_evidence(&mut records, device, hint_sources, observed_at);
    push_trusted_registry_evidence(&mut records, device, source, observed_at);
    push_router_evidence(&mut records, device, &context, observed_at);
    push_fallback_evidence(&mut records, device, observed_at);
    records
}

fn push_hint_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    hint_sources: &[LanDiscoveryEvidenceSource],
    observed_at: &str,
) {
    if !hint_sources.contains(&LanDiscoveryEvidenceSource::PreviousScanSnapshot) {
        return;
    }
    push_evidence_record(
        records,
        device,
        LanDiscoveryEvidenceSource::PreviousScanSnapshot,
        LanDiscoveryEvidenceKind::HistoricalIdentityHint,
        constants::lan_pairing::LAN_PREVIOUS_SCAN_CONTINUITY_VALUE,
        constants::lan_pairing::LAN_EVIDENCE_KEY_PREVIOUS_SCAN_PREFIX,
        LanDiscoveryEvidenceConfidence::Weak,
        observed_at,
        Some(constants::lan_pairing::LAN_PREVIOUS_SCAN_CONTINUITY_NOTE.to_string()),
    );
}

fn push_network_identity_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    context: &EvidenceContext,
    observed_at: &str,
    mac_assessment: Option<&LanMacIdentityAssessment>,
) {
    push_optional_evidence(
        records,
        device,
        context,
        LanDiscoveryEvidenceKind::IpAddress,
        device.ip_address.as_deref(),
        constants::lan_pairing::LAN_EVIDENCE_KEY_IP_PREFIX,
        observed_at,
    );
    push_optional_evidence(
        records,
        device,
        context,
        LanDiscoveryEvidenceKind::MacAddress,
        mac_assessment.and_then(LanMacIdentityAssessment::normalized),
        constants::lan_pairing::LAN_EVIDENCE_KEY_MAC_PREFIX,
        observed_at,
    );
    push_optional_evidence(
        records,
        device,
        context,
        LanDiscoveryEvidenceKind::Hostname,
        known_hostname(device).as_deref(),
        constants::lan_pairing::LAN_EVIDENCE_KEY_HOSTNAME_PREFIX,
        observed_at,
    );
    push_optional_evidence(
        records,
        device,
        context,
        LanDiscoveryEvidenceKind::Interface,
        device.network_interface.as_deref(),
        constants::lan_pairing::LAN_EVIDENCE_KEY_INTERFACE_PREFIX,
        observed_at,
    );
}

fn push_vendor_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    context: &EvidenceContext,
    observed_at: &str,
    mac_assessment: Option<&LanMacIdentityAssessment>,
) {
    let Some(mac_assessment) = mac_assessment else {
        return;
    };
    let note = mac_assessment.vendor_evidence_note().map(str::to_string);
    let confidence = vendor_evidence_confidence(mac_assessment);
    push_evidence_record(
        records,
        device,
        context.source.clone(),
        LanDiscoveryEvidenceKind::Vendor,
        &mac_assessment.vendor_evidence_value(),
        constants::lan_pairing::LAN_EVIDENCE_KEY_VENDOR_PREFIX,
        confidence,
        observed_at,
        note,
    );
}

fn push_optional_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    context: &EvidenceContext,
    evidence_kind: LanDiscoveryEvidenceKind,
    value: Option<&str>,
    merge_key_prefix: &str,
    observed_at: &str,
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
            observed_at,
            None,
        );
    }
}

fn push_agent_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    observed_at: &str,
) {
    if let Some(agent_status) = device.agent_status.as_ref() {
        let (source, confidence) = if is_confirmed_agent_status(Some(agent_status.as_str())) {
            (
                LanDiscoveryEvidenceSource::LocalService,
                LanDiscoveryEvidenceConfidence::Confirmed,
            )
        } else if is_service_identity_probe_status(Some(agent_status.as_str())) {
            (
                LanDiscoveryEvidenceSource::ServiceIdentityProbe,
                LanDiscoveryEvidenceConfidence::Weak,
            )
        } else {
            return;
        };
        push_evidence_record(
            records,
            device,
            source,
            LanDiscoveryEvidenceKind::ChildAgentPresence,
            agent_status,
            constants::lan_pairing::LAN_EVIDENCE_KEY_AGENT_PREFIX,
            confidence,
            observed_at,
            None,
        );
    }
}

fn push_trusted_registry_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    source: &LanCanonicalHouseholdDeviceSource,
    observed_at: &str,
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
            observed_at,
            None,
        );
    }
}

fn push_router_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    context: &EvidenceContext,
    observed_at: &str,
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
            observed_at,
            None,
        );
    }
}

fn push_fallback_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    observed_at: &str,
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
            observed_at,
            None,
        );
    }
}

fn evidence_context_for(
    source: &LanCanonicalHouseholdDeviceSource,
    evidence_sources: &[LanDiscoveryEvidenceSource],
) -> EvidenceContext {
    EvidenceContext {
        source: evidence_source_for(source, evidence_sources),
        confidence: evidence_confidence_for(source),
    }
}

fn evidence_source_for(
    source: &LanCanonicalHouseholdDeviceSource,
    evidence_sources: &[LanDiscoveryEvidenceSource],
) -> LanDiscoveryEvidenceSource {
    match source {
        LanCanonicalHouseholdDeviceSource::LocalService => LanDiscoveryEvidenceSource::LocalService,
        LanCanonicalHouseholdDeviceSource::NetworkNeighbor => {
            primary_network_evidence_source(evidence_sources)
        }
        LanCanonicalHouseholdDeviceSource::TrustedRegistry => {
            LanDiscoveryEvidenceSource::TrustedRegistry
        }
    }
}

fn primary_network_evidence_source(
    evidence_sources: &[LanDiscoveryEvidenceSource],
) -> LanDiscoveryEvidenceSource {
    evidence_sources
        .iter()
        .find(|source| {
            matches!(
                source,
                LanDiscoveryEvidenceSource::WindowsNeighborTable
                    | LanDiscoveryEvidenceSource::LinuxProcNetArp
                    | LanDiscoveryEvidenceSource::LinuxIpNeigh
                    | LanDiscoveryEvidenceSource::MacosArp
            )
        })
        .cloned()
        .unwrap_or(LanDiscoveryEvidenceSource::WindowsNeighborTable)
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

fn vendor_evidence_confidence(
    mac_assessment: &LanMacIdentityAssessment,
) -> LanDiscoveryEvidenceConfidence {
    match mac_assessment.disposition() {
        LanMacIdentityDisposition::KnownVendor => LanDiscoveryEvidenceConfidence::Strong,
        LanMacIdentityDisposition::UnknownVendor => LanDiscoveryEvidenceConfidence::Weak,
        LanMacIdentityDisposition::LocallyAdministered => {
            LanDiscoveryEvidenceConfidence::ManualRequired
        }
        LanMacIdentityDisposition::RejectedMulticast
        | LanMacIdentityDisposition::RejectedMalformed => LanDiscoveryEvidenceConfidence::Rejected,
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
    observed_at: &str,
    note: Option<String>,
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
        first_seen_at: observed_at.to_string(),
        last_seen_at: observed_at.to_string(),
        expires_at: None,
        confidence,
        merge_key,
        note,
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
