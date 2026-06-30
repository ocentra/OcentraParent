use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceSource,
};

pub const MAX_NAME_EVIDENCE_BYTES: usize = 255;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanNeighborNameEvidence {
    pub source: LanDiscoveryEvidenceSource,
    pub confidence: LanDiscoveryEvidenceConfidence,
    pub value: String,
    pub normalized_value: String,
    pub first_seen_at: String,
    pub last_seen_at: String,
    pub network_interface: Option<String>,
}

impl LanNeighborNameEvidence {
    pub fn source_label(&self) -> &'static str {
        match &self.source {
            LanDiscoveryEvidenceSource::DnsCache => {
                constants::lan_pairing::LAN_SCAN_SOURCE_DNS_CACHE
            }
            LanDiscoveryEvidenceSource::Netbios => constants::lan_pairing::LAN_SCAN_SOURCE_NETBIOS,
            LanDiscoveryEvidenceSource::Llmnr => constants::lan_pairing::LAN_SCAN_SOURCE_LLMNR,
            _ => "unsupported-name-source",
        }
    }

    pub fn confidence_label(&self) -> &'static str {
        match &self.confidence {
            LanDiscoveryEvidenceConfidence::Weak => "weak",
            LanDiscoveryEvidenceConfidence::Confirmed => "confirmed",
            LanDiscoveryEvidenceConfidence::Strong => "strong",
            LanDiscoveryEvidenceConfidence::ManualRequired => "manual-required",
            LanDiscoveryEvidenceConfidence::Rejected => "rejected",
        }
    }
}

pub fn dns_cache_name_evidence(
    value: &str,
    observed_at: &str,
    network_interface: Option<&str>,
) -> Option<LanNeighborNameEvidence> {
    name_evidence(
        LanDiscoveryEvidenceSource::DnsCache,
        value,
        observed_at,
        network_interface,
    )
}

pub fn reverse_dns_name_evidence(
    value: &str,
    observed_at: &str,
    network_interface: Option<&str>,
) -> Option<LanNeighborNameEvidence> {
    name_evidence(
        LanDiscoveryEvidenceSource::DnsCache,
        value,
        observed_at,
        network_interface,
    )
}

pub fn netbios_name_evidence(
    value: &str,
    observed_at: &str,
    network_interface: Option<&str>,
) -> Option<LanNeighborNameEvidence> {
    name_evidence(
        LanDiscoveryEvidenceSource::Netbios,
        value,
        observed_at,
        network_interface,
    )
}

pub fn llmnr_name_evidence(
    value: &str,
    observed_at: &str,
    network_interface: Option<&str>,
) -> Option<LanNeighborNameEvidence> {
    name_evidence(
        LanDiscoveryEvidenceSource::Llmnr,
        value,
        observed_at,
        network_interface,
    )
}

pub fn normalize_name_evidence_value(value: &str) -> Option<String> {
    let candidate = value.trim();
    let candidate = candidate.strip_suffix('.').unwrap_or(candidate);
    if candidate.is_empty()
        || candidate == constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME
        || candidate.len() > MAX_NAME_EVIDENCE_BYTES
        || !candidate.is_ascii()
    {
        return None;
    }
    if candidate.split('.').any(invalid_hostname_label) {
        return None;
    }
    Some(candidate.to_string())
}

pub fn name_evidence(
    source: LanDiscoveryEvidenceSource,
    value: &str,
    observed_at: &str,
    network_interface: Option<&str>,
) -> Option<LanNeighborNameEvidence> {
    let observed_at = observed_at.trim();
    if observed_at.is_empty() {
        return None;
    }
    let value = normalize_name_evidence_value(value)?;
    let normalized_value = value.to_ascii_lowercase();
    Some(LanNeighborNameEvidence {
        source,
        confidence: LanDiscoveryEvidenceConfidence::Weak,
        value,
        normalized_value,
        first_seen_at: observed_at.to_string(),
        last_seen_at: observed_at.to_string(),
        network_interface: network_interface.and_then(trim_optional_text),
    })
}

pub fn invalid_hostname_label(label: &str) -> bool {
    label.is_empty()
        || label.len() > 63
        || label.starts_with('-')
        || label.ends_with('-')
        || !label
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-')
}

pub fn trim_optional_text(value: &str) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_string())
}
