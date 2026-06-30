use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidence;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind;

use super::{compact_identifier, known_hostname};
use crate::mac_identity::{LanMacIdentityAssessment, LanMacIdentityDisposition};
use crate::network_inventory::api::{is_confirmed_agent_status, is_service_identity_probe_status};

const LAN_EVIDENCE_KEY_INSTALL_ID_PREFIX: &str = "install:";
const LAN_EVIDENCE_KEY_PAIRING_ID_PREFIX: &str = "pairing:";

struct EvidenceContext {
    source: LanDiscoveryEvidenceSource,
    confidence: LanDiscoveryEvidenceConfidence,
}

pub(super) struct EvidenceRecordsInput<'a> {
    pub(super) device: &'a LanPairingDeviceRef,
    pub(super) pairing_id: Option<&'a str>,
    pub(super) source: &'a LanCanonicalHouseholdDeviceSource,
    pub(super) evidence_sources: &'a [LanDiscoveryEvidenceSource],
    pub(super) hint_sources: &'a [LanDiscoveryEvidenceSource],
    pub(super) service_identity_probe_evidence: &'a [LanServiceIdentityProbeEvidence],
    pub(super) observed_at: &'a str,
    pub(super) mac_assessment: Option<&'a LanMacIdentityAssessment>,
}

struct EvidenceRecordInput<'a> {
    device: &'a LanPairingDeviceRef,
    source: LanDiscoveryEvidenceSource,
    evidence_kind: LanDiscoveryEvidenceKind,
    value: &'a str,
    merge_key_prefix: &'a str,
    confidence: LanDiscoveryEvidenceConfidence,
    observed_at: &'a str,
    note: Option<String>,
}

struct OptionalEvidenceInput<'a> {
    device: &'a LanPairingDeviceRef,
    source: LanDiscoveryEvidenceSource,
    evidence_kind: LanDiscoveryEvidenceKind,
    value: Option<&'a str>,
    merge_key_prefix: &'a str,
    confidence: LanDiscoveryEvidenceConfidence,
    observed_at: &'a str,
}

pub(super) fn evidence_records_for(
    input: &EvidenceRecordsInput<'_>,
) -> Vec<LanDiscoveryEvidenceRecord> {
    let device = input.device;
    let pairing_id = input.pairing_id;
    let source = input.source;
    let evidence_sources = input.evidence_sources;
    let hint_sources = input.hint_sources;
    let service_identity_probe_evidence = input.service_identity_probe_evidence;
    let observed_at = input.observed_at;
    let mac_assessment = input.mac_assessment;
    let context = evidence_context_for(source, evidence_sources);
    let mut records = Vec::new();
    push_network_identity_evidence(
        &mut records,
        device,
        &context,
        evidence_sources,
        observed_at,
        mac_assessment,
    );
    push_strong_identity_evidence(&mut records, device, pairing_id, &context, observed_at);
    push_weak_name_evidence(&mut records, device, evidence_sources, observed_at);
    push_vendor_evidence(&mut records, device, &context, observed_at, mac_assessment);
    push_agent_evidence(&mut records, device, observed_at);
    push_service_probe_evidence(
        &mut records,
        device,
        service_identity_probe_evidence,
        observed_at,
    );
    push_hint_evidence(&mut records, device, hint_sources, observed_at);
    push_trusted_registry_evidence(&mut records, device, source, observed_at);
    push_router_evidence(&mut records, device, &context, observed_at);
    push_fallback_evidence(&mut records, device, observed_at);
    records
}

fn push_strong_identity_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    pairing_id: Option<&str>,
    context: &EvidenceContext,
    observed_at: &str,
) {
    push_optional_evidence_with_confidence(
        records,
        OptionalEvidenceInput {
            device,
            source: context.source.clone(),
            evidence_kind: LanDiscoveryEvidenceKind::InstallId,
            value: device.install_id.as_deref(),
            merge_key_prefix: LAN_EVIDENCE_KEY_INSTALL_ID_PREFIX,
            confidence: strong_identity_confidence(&context.source),
            observed_at,
        },
    );
    push_optional_evidence_with_confidence(
        records,
        OptionalEvidenceInput {
            device,
            source: context.source.clone(),
            evidence_kind: LanDiscoveryEvidenceKind::PairingId,
            value: pairing_id,
            merge_key_prefix: LAN_EVIDENCE_KEY_PAIRING_ID_PREFIX,
            confidence: strong_identity_confidence(&context.source),
            observed_at,
        },
    );
}

fn push_service_probe_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    service_identity_probe_evidence: &[LanServiceIdentityProbeEvidence],
    observed_at: &str,
) {
    for evidence in service_identity_probe_evidence {
        let value = service_probe_evidence_value(evidence);
        push_evidence_record(
            records,
            EvidenceRecordInput {
                device,
                source: LanDiscoveryEvidenceSource::ServiceIdentityProbe,
                evidence_kind: LanDiscoveryEvidenceKind::ServiceProbeHint,
                value: &value,
                merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_SERVICE_PROBE_PREFIX,
                confidence: service_probe_evidence_confidence(&evidence.evidence_kind),
                observed_at,
                note: service_probe_evidence_note(&evidence.evidence_kind),
            },
        );
    }
}

fn service_probe_evidence_value(evidence: &LanServiceIdentityProbeEvidence) -> String {
    let mut value = String::from(service_probe_kind_value(&evidence.evidence_kind));
    value.push(':');
    value.push_str(&evidence.value);
    value
}

fn service_probe_kind_value(kind: &LanServiceIdentityProbeEvidenceKind) -> &'static str {
    match kind {
        LanServiceIdentityProbeEvidenceKind::HttpStatus => "http-status",
        LanServiceIdentityProbeEvidenceKind::HtmlTitle => "html-title",
        LanServiceIdentityProbeEvidenceKind::ServerHeader => "server-header",
        LanServiceIdentityProbeEvidenceKind::Banner => "banner",
        LanServiceIdentityProbeEvidenceKind::RedirectLocation => "redirect-location",
        LanServiceIdentityProbeEvidenceKind::CertificateSubject => "certificate-subject",
        LanServiceIdentityProbeEvidenceKind::DescriptorLink => "descriptor-link",
        LanServiceIdentityProbeEvidenceKind::WsdEndpointAddress => "wsd-endpoint-address",
        LanServiceIdentityProbeEvidenceKind::WsdTypes => "wsd-types",
        LanServiceIdentityProbeEvidenceKind::SnmpSysDescr => "snmp-sys-descr",
        LanServiceIdentityProbeEvidenceKind::SnmpSysName => "snmp-sys-name",
        LanServiceIdentityProbeEvidenceKind::MdnsServiceType => "mdns-service-type",
        LanServiceIdentityProbeEvidenceKind::MdnsInstanceName => "mdns-instance-name",
        LanServiceIdentityProbeEvidenceKind::SsdpUdn => "ssdp-udn",
        LanServiceIdentityProbeEvidenceKind::SsdpDeviceType => "ssdp-device-type",
    }
}

fn service_probe_evidence_confidence(
    kind: &LanServiceIdentityProbeEvidenceKind,
) -> LanDiscoveryEvidenceConfidence {
    match kind {
        LanServiceIdentityProbeEvidenceKind::MdnsServiceType
        | LanServiceIdentityProbeEvidenceKind::MdnsInstanceName
        | LanServiceIdentityProbeEvidenceKind::SsdpUdn
        | LanServiceIdentityProbeEvidenceKind::SsdpDeviceType => {
            LanDiscoveryEvidenceConfidence::Strong
        }
        LanServiceIdentityProbeEvidenceKind::HttpStatus
        | LanServiceIdentityProbeEvidenceKind::HtmlTitle
        | LanServiceIdentityProbeEvidenceKind::ServerHeader
        | LanServiceIdentityProbeEvidenceKind::Banner
        | LanServiceIdentityProbeEvidenceKind::RedirectLocation
        | LanServiceIdentityProbeEvidenceKind::CertificateSubject
        | LanServiceIdentityProbeEvidenceKind::DescriptorLink
        | LanServiceIdentityProbeEvidenceKind::WsdEndpointAddress
        | LanServiceIdentityProbeEvidenceKind::WsdTypes
        | LanServiceIdentityProbeEvidenceKind::SnmpSysDescr
        | LanServiceIdentityProbeEvidenceKind::SnmpSysName => LanDiscoveryEvidenceConfidence::Weak,
    }
}

fn service_probe_evidence_note(kind: &LanServiceIdentityProbeEvidenceKind) -> Option<String> {
    match kind {
        LanServiceIdentityProbeEvidenceKind::MdnsServiceType
        | LanServiceIdentityProbeEvidenceKind::MdnsInstanceName
        | LanServiceIdentityProbeEvidenceKind::SsdpUdn
        | LanServiceIdentityProbeEvidenceKind::SsdpDeviceType => {
            Some("network discovery identity hint".to_string())
        }
        LanServiceIdentityProbeEvidenceKind::HttpStatus
        | LanServiceIdentityProbeEvidenceKind::HtmlTitle
        | LanServiceIdentityProbeEvidenceKind::ServerHeader
        | LanServiceIdentityProbeEvidenceKind::Banner
        | LanServiceIdentityProbeEvidenceKind::RedirectLocation
        | LanServiceIdentityProbeEvidenceKind::CertificateSubject
        | LanServiceIdentityProbeEvidenceKind::DescriptorLink => {
            Some(constants::lan_pairing::LAN_SERVICE_PROBE_HINT_NOTE.to_string())
        }
        LanServiceIdentityProbeEvidenceKind::WsdEndpointAddress
        | LanServiceIdentityProbeEvidenceKind::WsdTypes => {
            Some("ws-discovery metadata hint".to_string())
        }
        LanServiceIdentityProbeEvidenceKind::SnmpSysDescr
        | LanServiceIdentityProbeEvidenceKind::SnmpSysName => {
            Some("snmp metadata hint".to_string())
        }
    }
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
        EvidenceRecordInput {
            device,
            source: LanDiscoveryEvidenceSource::PreviousScanSnapshot,
            evidence_kind: LanDiscoveryEvidenceKind::HistoricalIdentityHint,
            value: constants::lan_pairing::LAN_PREVIOUS_SCAN_CONTINUITY_VALUE,
            merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_PREVIOUS_SCAN_PREFIX,
            confidence: LanDiscoveryEvidenceConfidence::Weak,
            observed_at,
            note: Some(constants::lan_pairing::LAN_PREVIOUS_SCAN_CONTINUITY_NOTE.to_string()),
        },
    );
}

fn push_network_identity_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    context: &EvidenceContext,
    evidence_sources: &[LanDiscoveryEvidenceSource],
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
    if !has_weak_name_source(evidence_sources) {
        push_optional_evidence(
            records,
            device,
            context,
            LanDiscoveryEvidenceKind::Hostname,
            known_hostname(device).as_deref(),
            constants::lan_pairing::LAN_EVIDENCE_KEY_HOSTNAME_PREFIX,
            observed_at,
        );
    }
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

fn push_weak_name_evidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    device: &LanPairingDeviceRef,
    evidence_sources: &[LanDiscoveryEvidenceSource],
    observed_at: &str,
) {
    let Some(hostname) = known_hostname(device) else {
        return;
    };
    if evidence_sources.contains(&LanDiscoveryEvidenceSource::DnsCache) {
        push_evidence_record(
            records,
            EvidenceRecordInput {
                device,
                source: LanDiscoveryEvidenceSource::DnsCache,
                evidence_kind: LanDiscoveryEvidenceKind::Hostname,
                value: &hostname,
                merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_DNS_CACHE_PREFIX,
                confidence: LanDiscoveryEvidenceConfidence::Weak,
                observed_at,
                note: None,
            },
        );
    }
    if evidence_sources.contains(&LanDiscoveryEvidenceSource::Netbios) {
        push_evidence_record(
            records,
            EvidenceRecordInput {
                device,
                source: LanDiscoveryEvidenceSource::Netbios,
                evidence_kind: LanDiscoveryEvidenceKind::Hostname,
                value: &hostname,
                merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_NETBIOS_PREFIX,
                confidence: LanDiscoveryEvidenceConfidence::Weak,
                observed_at,
                note: None,
            },
        );
    }
    if evidence_sources.contains(&LanDiscoveryEvidenceSource::Llmnr) {
        push_evidence_record(
            records,
            EvidenceRecordInput {
                device,
                source: LanDiscoveryEvidenceSource::Llmnr,
                evidence_kind: LanDiscoveryEvidenceKind::Hostname,
                value: &hostname,
                merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_LLMNR_PREFIX,
                confidence: LanDiscoveryEvidenceConfidence::Weak,
                observed_at,
                note: None,
            },
        );
    }
}

fn has_weak_name_source(evidence_sources: &[LanDiscoveryEvidenceSource]) -> bool {
    evidence_sources.contains(&LanDiscoveryEvidenceSource::DnsCache)
        || evidence_sources.contains(&LanDiscoveryEvidenceSource::Netbios)
        || evidence_sources.contains(&LanDiscoveryEvidenceSource::Llmnr)
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
    let vendor_value = mac_assessment.vendor_evidence_value();
    push_evidence_record(
        records,
        EvidenceRecordInput {
            device,
            source: context.source.clone(),
            evidence_kind: LanDiscoveryEvidenceKind::Vendor,
            value: &vendor_value,
            merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_VENDOR_PREFIX,
            confidence,
            observed_at,
            note,
        },
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
            EvidenceRecordInput {
                device,
                source: context.source.clone(),
                evidence_kind,
                value,
                merge_key_prefix,
                confidence: context.confidence.clone(),
                observed_at,
                note: None,
            },
        );
    }
}

fn push_optional_evidence_with_confidence(
    records: &mut Vec<LanDiscoveryEvidenceRecord>,
    input: OptionalEvidenceInput<'_>,
) {
    let OptionalEvidenceInput {
        device,
        source,
        evidence_kind,
        value,
        merge_key_prefix,
        confidence,
        observed_at,
    } = input;
    if let Some(value) = value {
        push_evidence_record(
            records,
            EvidenceRecordInput {
                device,
                source,
                evidence_kind,
                value,
                merge_key_prefix,
                confidence,
                observed_at,
                note: None,
            },
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
            EvidenceRecordInput {
                device,
                source,
                evidence_kind: LanDiscoveryEvidenceKind::ChildAgentPresence,
                value: agent_status,
                merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_AGENT_PREFIX,
                confidence,
                observed_at,
                note: None,
            },
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
            EvidenceRecordInput {
                device,
                source: LanDiscoveryEvidenceSource::TrustedRegistry,
                evidence_kind: LanDiscoveryEvidenceKind::TrustedRegistry,
                value: &device.device_id,
                merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_TRUSTED_PREFIX,
                confidence: LanDiscoveryEvidenceConfidence::ManualRequired,
                observed_at,
                note: None,
            },
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
            EvidenceRecordInput {
                device,
                source: context.source.clone(),
                evidence_kind: LanDiscoveryEvidenceKind::RouterClassification,
                value: router_value,
                merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_ROUTER_PREFIX,
                confidence: context.confidence.clone(),
                observed_at,
                note: None,
            },
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
            EvidenceRecordInput {
                device,
                source: LanDiscoveryEvidenceSource::ParentAssignment,
                evidence_kind: LanDiscoveryEvidenceKind::ParentDecision,
                value: &device.device_id,
                merge_key_prefix: constants::lan_pairing::LAN_EVIDENCE_KEY_TRUSTED_PREFIX,
                confidence: LanDiscoveryEvidenceConfidence::ManualRequired,
                observed_at,
                note: None,
            },
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
                    | LanDiscoveryEvidenceSource::MdnsDnsSdQuery
                    | LanDiscoveryEvidenceSource::SsdpUpnpQuery
                    | LanDiscoveryEvidenceSource::ServiceIdentityProbe
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

fn strong_identity_confidence(
    source: &LanDiscoveryEvidenceSource,
) -> LanDiscoveryEvidenceConfidence {
    match source {
        LanDiscoveryEvidenceSource::LocalService => LanDiscoveryEvidenceConfidence::Confirmed,
        LanDiscoveryEvidenceSource::TrustedRegistry => LanDiscoveryEvidenceConfidence::Strong,
        _ => LanDiscoveryEvidenceConfidence::Strong,
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
    input: EvidenceRecordInput<'_>,
) {
    let EvidenceRecordInput {
        device,
        source,
        evidence_kind,
        value,
        merge_key_prefix,
        confidence,
        observed_at,
        note,
    } = input;
    let normalized_value = normalized_evidence_value(value);
    let merge_key = evidence_key(merge_key_prefix, &normalized_value);
    if records.iter().any(|record| {
        same_evidence_record_identity(
            record,
            &source,
            &evidence_kind,
            &merge_key,
            &device.device_id,
        )
    }) {
        return;
    }
    let identity_key =
        evidence_identity_key(&source, &evidence_kind, &device.device_id, &merge_key);
    records.push(LanDiscoveryEvidenceRecord {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        evidence_id: evidence_id(&identity_key),
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
