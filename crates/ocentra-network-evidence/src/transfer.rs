use serde::{Deserialize, Serialize};

use crate::dns::types::NetworkEvidenceGrade;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkTransferActivityKind {
    RemoteDesktop,
    Torrent,
    LargeDownload,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkTransferIndicator {
    RemoteDesktopPort,
    RemoteDesktopProcess,
    TorrentDhtPort,
    TorrentTrackerDomain,
    LargeDownloadByteCount,
    ParallelRangeRequests,
    UnattributedHighVolume,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkTransferBasis {
    RemoteDesktopCandidate,
    TorrentCandidate,
    LargeDownloadCandidate,
    UnattributedHighVolume,
    NoIndicator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkTransferUncertainty {
    CandidateNeedsConfirmation,
    UnattributedHighVolume,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTransferIndicatorEvidence {
    pub indicator: NetworkTransferIndicator,
    pub confidence_percent: u8,
    pub source_ref: String,
    pub observed_bytes: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTransferClassifierInput {
    pub indicators: Vec<NetworkTransferIndicatorEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTransferClassification {
    pub activity_kind: NetworkTransferActivityKind,
    pub basis: NetworkTransferBasis,
    pub uncertainty: NetworkTransferUncertainty,
    pub confidence_percent: u8,
    pub evidence_refs: Vec<String>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub file_name_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkTransferClassifierError {
    InvalidIndicatorConfidence(u8),
    EmptyIndicatorSourceRef,
}

pub fn classify_remote_torrent_download_activity(
    input: NetworkTransferClassifierInput,
) -> Result<NetworkTransferClassification, NetworkTransferClassifierError> {
    validate_transfer_input(&input)?;

    if let Some(classification) = known_transfer_classification(&input.indicators) {
        return Ok(classification);
    }

    Ok(unknown_transfer_classification(input.indicators))
}

fn known_transfer_classification(
    indicators: &[NetworkTransferIndicatorEvidence],
) -> Option<NetworkTransferClassification> {
    if let Some(indicator) = strongest_indicator(indicators, remote_desktop_indicator) {
        return Some(transfer_classification(
            NetworkTransferActivityKind::RemoteDesktop,
            NetworkTransferBasis::RemoteDesktopCandidate,
            indicator,
        ));
    }
    if let Some(indicator) = strongest_indicator(indicators, torrent_indicator) {
        return Some(transfer_classification(
            NetworkTransferActivityKind::Torrent,
            NetworkTransferBasis::TorrentCandidate,
            indicator,
        ));
    }
    strongest_indicator(indicators, large_download_indicator).map(|indicator| {
        transfer_classification(
            NetworkTransferActivityKind::LargeDownload,
            NetworkTransferBasis::LargeDownloadCandidate,
            indicator,
        )
    })
}

fn unknown_transfer_classification(
    indicators: Vec<NetworkTransferIndicatorEvidence>,
) -> NetworkTransferClassification {
    if unattributed_high_volume_only(&indicators) {
        return NetworkTransferClassification {
            activity_kind: NetworkTransferActivityKind::Unknown,
            basis: NetworkTransferBasis::UnattributedHighVolume,
            uncertainty: NetworkTransferUncertainty::UnattributedHighVolume,
            confidence_percent: 0,
            evidence_refs: indicators
                .into_iter()
                .map(|indicator| indicator.source_ref)
                .collect(),
            evidence_grade: NetworkEvidenceGrade::D,
            exact_url_available: false,
            decrypted_payload_available: false,
            file_name_available: false,
        };
    }

    NetworkTransferClassification {
        activity_kind: NetworkTransferActivityKind::Unknown,
        basis: NetworkTransferBasis::NoIndicator,
        uncertainty: NetworkTransferUncertainty::Unknown,
        confidence_percent: 0,
        evidence_refs: Vec::new(),
        evidence_grade: NetworkEvidenceGrade::D,
        exact_url_available: false,
        decrypted_payload_available: false,
        file_name_available: false,
    }
}

fn transfer_classification(
    activity_kind: NetworkTransferActivityKind,
    basis: NetworkTransferBasis,
    indicator: &NetworkTransferIndicatorEvidence,
) -> NetworkTransferClassification {
    NetworkTransferClassification {
        activity_kind,
        basis,
        uncertainty: NetworkTransferUncertainty::CandidateNeedsConfirmation,
        confidence_percent: indicator.confidence_percent,
        evidence_refs: vec![indicator.source_ref.clone()],
        evidence_grade: NetworkEvidenceGrade::D,
        exact_url_available: false,
        decrypted_payload_available: false,
        file_name_available: false,
    }
}

fn strongest_indicator(
    indicators: &[NetworkTransferIndicatorEvidence],
    predicate: fn(NetworkTransferIndicator) -> bool,
) -> Option<&NetworkTransferIndicatorEvidence> {
    indicators
        .iter()
        .filter(|indicator| predicate(indicator.indicator))
        .max_by_key(|indicator| indicator.confidence_percent)
}

fn validate_transfer_input(
    input: &NetworkTransferClassifierInput,
) -> Result<(), NetworkTransferClassifierError> {
    for indicator in &input.indicators {
        if indicator.confidence_percent > 100 {
            return Err(NetworkTransferClassifierError::InvalidIndicatorConfidence(
                indicator.confidence_percent,
            ));
        }
        if indicator.source_ref.trim().is_empty() {
            return Err(NetworkTransferClassifierError::EmptyIndicatorSourceRef);
        }
    }

    Ok(())
}

fn unattributed_high_volume_only(indicators: &[NetworkTransferIndicatorEvidence]) -> bool {
    !indicators.is_empty()
        && indicators.iter().all(|indicator| {
            indicator.indicator == NetworkTransferIndicator::UnattributedHighVolume
        })
}

fn remote_desktop_indicator(indicator: NetworkTransferIndicator) -> bool {
    matches!(
        indicator,
        NetworkTransferIndicator::RemoteDesktopPort
            | NetworkTransferIndicator::RemoteDesktopProcess
    )
}

fn torrent_indicator(indicator: NetworkTransferIndicator) -> bool {
    matches!(
        indicator,
        NetworkTransferIndicator::TorrentDhtPort | NetworkTransferIndicator::TorrentTrackerDomain
    )
}

fn large_download_indicator(indicator: NetworkTransferIndicator) -> bool {
    matches!(
        indicator,
        NetworkTransferIndicator::LargeDownloadByteCount
            | NetworkTransferIndicator::ParallelRangeRequests
    )
}
