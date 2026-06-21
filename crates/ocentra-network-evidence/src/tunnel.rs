use serde::{Deserialize, Serialize};

use crate::dns::types::NetworkEvidenceGrade;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkTunnelKind {
    Vpn,
    Proxy,
    Tor,
    Tunnel,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkTunnelIndicator {
    VpnAdapter,
    HttpProxyPort,
    SocksProxyPort,
    TorKnownPort,
    TorBootstrapDomain,
    WireGuardUdpPort,
    OpenVpnUdpPort,
    EncryptedDnsOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkTunnelBasis {
    VpnAdapterIndicator,
    ProxyPortIndicator,
    TorIndicator,
    TunnelProtocolIndicator,
    EncryptedDnsOnlyNoTunnel,
    NoIndicator,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTunnelIndicatorEvidence {
    pub indicator: NetworkTunnelIndicator,
    pub confidence_percent: u8,
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTunnelClassifierInput {
    pub indicators: Vec<NetworkTunnelIndicatorEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTunnelClassification {
    pub tunnel_kind: NetworkTunnelKind,
    pub basis: NetworkTunnelBasis,
    pub confidence_percent: u8,
    pub evidence_refs: Vec<String>,
    pub hidden_destination_claimed: bool,
    pub evidence_grade: NetworkEvidenceGrade,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkTunnelClassifierError {
    InvalidIndicatorConfidence(u8),
    EmptyIndicatorSourceRef,
}

pub fn classify_vpn_proxy_tunnel_activity(
    input: NetworkTunnelClassifierInput,
) -> Result<NetworkTunnelClassification, NetworkTunnelClassifierError> {
    validate_tunnel_input(&input)?;

    if let Some(classification) = known_tunnel_classification(&input.indicators) {
        return Ok(classification);
    }

    Ok(unknown_tunnel_classification(input.indicators))
}

fn known_tunnel_classification(
    indicators: &[NetworkTunnelIndicatorEvidence],
) -> Option<NetworkTunnelClassification> {
    if let Some(indicator) = strongest_indicator(indicators, tor_indicator) {
        return Some(tunnel_classification(
            NetworkTunnelKind::Tor,
            NetworkTunnelBasis::TorIndicator,
            indicator,
            NetworkEvidenceGrade::C,
        ));
    }
    if let Some(indicator) = strongest_indicator(indicators, vpn_indicator) {
        return Some(tunnel_classification(
            NetworkTunnelKind::Vpn,
            NetworkTunnelBasis::VpnAdapterIndicator,
            indicator,
            NetworkEvidenceGrade::C,
        ));
    }
    if let Some(indicator) = strongest_indicator(indicators, proxy_indicator) {
        return Some(tunnel_classification(
            NetworkTunnelKind::Proxy,
            NetworkTunnelBasis::ProxyPortIndicator,
            indicator,
            NetworkEvidenceGrade::D,
        ));
    }
    strongest_indicator(indicators, tunnel_protocol_indicator).map(|indicator| {
        tunnel_classification(
            NetworkTunnelKind::Tunnel,
            NetworkTunnelBasis::TunnelProtocolIndicator,
            indicator,
            NetworkEvidenceGrade::D,
        )
    })
}

fn unknown_tunnel_classification(
    indicators: Vec<NetworkTunnelIndicatorEvidence>,
) -> NetworkTunnelClassification {
    if encrypted_dns_only(&indicators) {
        return NetworkTunnelClassification {
            tunnel_kind: NetworkTunnelKind::Unknown,
            basis: NetworkTunnelBasis::EncryptedDnsOnlyNoTunnel,
            confidence_percent: 0,
            evidence_refs: indicators
                .into_iter()
                .map(|indicator| indicator.source_ref)
                .collect(),
            hidden_destination_claimed: false,
            evidence_grade: NetworkEvidenceGrade::D,
            exact_url_available: false,
            decrypted_payload_available: false,
        };
    }

    NetworkTunnelClassification {
        tunnel_kind: NetworkTunnelKind::Unknown,
        basis: NetworkTunnelBasis::NoIndicator,
        confidence_percent: 0,
        evidence_refs: Vec::new(),
        hidden_destination_claimed: false,
        evidence_grade: NetworkEvidenceGrade::D,
        exact_url_available: false,
        decrypted_payload_available: false,
    }
}

fn encrypted_dns_only(indicators: &[NetworkTunnelIndicatorEvidence]) -> bool {
    !indicators.is_empty()
        && indicators
            .iter()
            .all(|indicator| indicator.indicator == NetworkTunnelIndicator::EncryptedDnsOnly)
}

fn tunnel_classification(
    tunnel_kind: NetworkTunnelKind,
    basis: NetworkTunnelBasis,
    indicator: &NetworkTunnelIndicatorEvidence,
    evidence_grade: NetworkEvidenceGrade,
) -> NetworkTunnelClassification {
    NetworkTunnelClassification {
        tunnel_kind,
        basis,
        confidence_percent: indicator.confidence_percent,
        evidence_refs: vec![indicator.source_ref.clone()],
        hidden_destination_claimed: false,
        evidence_grade,
        exact_url_available: false,
        decrypted_payload_available: false,
    }
}

fn strongest_indicator(
    indicators: &[NetworkTunnelIndicatorEvidence],
    predicate: fn(NetworkTunnelIndicator) -> bool,
) -> Option<&NetworkTunnelIndicatorEvidence> {
    indicators
        .iter()
        .filter(|indicator| predicate(indicator.indicator))
        .max_by_key(|indicator| indicator.confidence_percent)
}

fn validate_tunnel_input(
    input: &NetworkTunnelClassifierInput,
) -> Result<(), NetworkTunnelClassifierError> {
    for indicator in &input.indicators {
        if indicator.confidence_percent > 100 {
            return Err(NetworkTunnelClassifierError::InvalidIndicatorConfidence(
                indicator.confidence_percent,
            ));
        }
        if indicator.source_ref.trim().is_empty() {
            return Err(NetworkTunnelClassifierError::EmptyIndicatorSourceRef);
        }
    }

    Ok(())
}

fn tor_indicator(indicator: NetworkTunnelIndicator) -> bool {
    matches!(
        indicator,
        NetworkTunnelIndicator::TorKnownPort | NetworkTunnelIndicator::TorBootstrapDomain
    )
}

fn vpn_indicator(indicator: NetworkTunnelIndicator) -> bool {
    indicator == NetworkTunnelIndicator::VpnAdapter
}

fn proxy_indicator(indicator: NetworkTunnelIndicator) -> bool {
    matches!(
        indicator,
        NetworkTunnelIndicator::HttpProxyPort | NetworkTunnelIndicator::SocksProxyPort
    )
}

fn tunnel_protocol_indicator(indicator: NetworkTunnelIndicator) -> bool {
    matches!(
        indicator,
        NetworkTunnelIndicator::WireGuardUdpPort | NetworkTunnelIndicator::OpenVpnUdpPort
    )
}
