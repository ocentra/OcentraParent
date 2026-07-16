#[path = "classification_scores.rs"]
mod classification_scores;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceClassification, LanCanonicalHouseholdDeviceConfidence,
    LanServiceIdentityProbeEvidence, LanServiceIdentityProbeEvidenceKind,
};

use crate::mac_identity::{
    assess_mac_address, LanMacIdentityAssessment, LanMacIdentityDisposition,
};

const LABEL_HINT_WEIGHT: u16 = 1;
const PLATFORM_HINT_WEIGHT: u16 = 1;
const HOSTNAME_HINT_WEIGHT: u16 = 2;
const HARDWARE_HINT_WEIGHT: u16 = 2;
const SERVICE_PROBE_WEAK_HINT_WEIGHT: u16 = 3;
const SERVICE_PROBE_STRONG_HINT_WEIGHT: u16 = 5;
const MIN_DECISIVE_SCORE_MARGIN: u16 = 2;

use classification_scores::classification_scores;

#[derive(Clone)]
struct WeightedHintText {
    text: String,
    weight: u16,
}

pub(super) fn option_overlaps(first: Option<&String>, second: Option<&String>) -> bool {
    first
        .zip(second)
        .and_then(|(left, right)| {
            let left = assess_mac_address(Some(left.as_str()))?;
            let right = assess_mac_address(Some(right.as_str()))?;
            (left.stable_identity_key_allowed() && right.stable_identity_key_allowed())
                .then_some((left, right))
        })
        .map(|(left, right)| left.normalized() == right.normalized())
        .unwrap_or(false)
}

pub(super) fn compact_identifier(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

pub(super) fn known_hostname(device: &LanPairingDeviceRef) -> Option<String> {
    device
        .hostname
        .as_ref()
        .filter(|hostname| *hostname != constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME)
        .cloned()
}

pub(super) fn inferred_household_device_classification(
    device: &LanPairingDeviceRef,
    service_identity_probe_evidence: &[LanServiceIdentityProbeEvidence],
) -> Option<LanCanonicalHouseholdDeviceClassification> {
    let texts = classification_hint_texts(device, service_identity_probe_evidence);
    let scores = classification_scores(&texts);
    let mut ordered_scores = scores
        .iter()
        .filter(|&score| score.total > 0)
        .cloned()
        .collect::<Vec<_>>();
    ordered_scores.sort_by(|left, right| {
        right
            .total
            .cmp(&left.total)
            .then(right.strongest_signal.cmp(&left.strongest_signal))
    });
    let best = ordered_scores.first()?;
    if let Some(second) = ordered_scores.get(1) {
        let unresolved_strong_conflict = best.total == second.total
            && best.strongest_signal >= SERVICE_PROBE_STRONG_HINT_WEIGHT
            && second.strongest_signal >= SERVICE_PROBE_STRONG_HINT_WEIGHT
            && best.classification != second.classification;
        let weak_or_mixed_margin_too_small = second.total > 0
            && best.strongest_signal < SERVICE_PROBE_STRONG_HINT_WEIGHT
            && best.total.saturating_sub(second.total) < MIN_DECISIVE_SCORE_MARGIN;
        if unresolved_strong_conflict || weak_or_mixed_margin_too_small {
            return None;
        }
    }
    Some(best.classification.clone())
}

fn classification_hint_texts(
    device: &LanPairingDeviceRef,
    service_identity_probe_evidence: &[LanServiceIdentityProbeEvidence],
) -> Vec<WeightedHintText> {
    let mut texts = Vec::new();
    push_hint_text(&mut texts, Some(device.label.as_str()), LABEL_HINT_WEIGHT);
    push_hint_text(&mut texts, device.hostname.as_deref(), HOSTNAME_HINT_WEIGHT);
    push_hint_text(
        &mut texts,
        Some(device.platform.as_str()),
        PLATFORM_HINT_WEIGHT,
    );
    if let Some(hardware_profile) = device.hardware_profile.as_ref() {
        push_hint_text(
            &mut texts,
            hardware_profile.manufacturer.as_deref(),
            HARDWARE_HINT_WEIGHT,
        );
        push_hint_text(
            &mut texts,
            hardware_profile.model.as_deref(),
            HARDWARE_HINT_WEIGHT,
        );
    }
    for evidence in service_identity_probe_evidence {
        push_hint_text(
            &mut texts,
            Some(evidence.value.as_str()),
            service_probe_hint_weight(&evidence.evidence_kind),
        );
    }
    texts
}

fn push_hint_text(texts: &mut Vec<WeightedHintText>, value: Option<&str>, weight: u16) {
    let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) else {
        return;
    };
    texts.push(WeightedHintText {
        text: value.to_ascii_lowercase(),
        weight,
    });
}

fn service_probe_hint_weight(kind: &LanServiceIdentityProbeEvidenceKind) -> u16 {
    match kind {
        LanServiceIdentityProbeEvidenceKind::MdnsServiceType
        | LanServiceIdentityProbeEvidenceKind::MdnsInstanceName
        | LanServiceIdentityProbeEvidenceKind::SsdpUdn
        | LanServiceIdentityProbeEvidenceKind::SsdpDeviceType => SERVICE_PROBE_STRONG_HINT_WEIGHT,
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
        | LanServiceIdentityProbeEvidenceKind::SnmpSysName => SERVICE_PROBE_WEAK_HINT_WEIGHT,
    }
}

pub(super) fn stale_at_for(
    reachability: &LanPairingDeviceReachability,
    observed_at: &str,
) -> Option<String> {
    if *reachability == LanPairingDeviceReachability::Stale {
        Some(observed_at.to_string())
    } else {
        None
    }
}

pub(super) fn offline_at_for(
    reachability: &LanPairingDeviceReachability,
    observed_at: &str,
) -> Option<String> {
    if *reachability == LanPairingDeviceReachability::Offline {
        Some(observed_at.to_string())
    } else {
        None
    }
}

pub(super) fn child_agent_capabilities() -> Vec<String> {
    vec![
        constants::lan_pairing::CHILD_AGENT_CAPABILITY_DIRECT_WEBSOCKET.to_string(),
        constants::lan_pairing::CHILD_AGENT_CAPABILITY_DEVICE_INVENTORY.to_string(),
        constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE.to_string(),
    ]
}

pub(super) fn has_child_agent_evidence(evidence_sources: &[LanDiscoveryEvidenceSource]) -> bool {
    evidence_sources.iter().any(|source| {
        matches!(
            source,
            LanDiscoveryEvidenceSource::ChildAgentHello
                | LanDiscoveryEvidenceSource::ChildAgentHeartbeat
        )
    })
}

pub(super) fn preferred_mac_identity(device: &LanPairingDeviceRef) -> Option<String> {
    let assessment = assess_mac_address(device.mac_address.as_deref())?;
    assessment
        .stable_identity_key_allowed()
        .then_some(assessment.normalized_owned())
        .flatten()
}

pub(super) fn confidence_for_mac_identity(
    confidence: LanCanonicalHouseholdDeviceConfidence,
    mac_assessment: Option<&LanMacIdentityAssessment>,
) -> LanCanonicalHouseholdDeviceConfidence {
    if matches!(
        confidence,
        LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor
            | LanCanonicalHouseholdDeviceConfidence::MacIpMatch
    ) && mac_assessment.is_some_and(|assessment| {
        !assessment.identity_key_allowed()
            || assessment.disposition() == LanMacIdentityDisposition::LocallyAdministered
    }) {
        LanCanonicalHouseholdDeviceConfidence::ManualRequired
    } else {
        confidence
    }
}
