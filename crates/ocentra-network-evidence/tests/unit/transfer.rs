use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::transfer::*;

#[derive(Clone, Copy)]
struct SourceRef(&'static str);

#[test]
fn transfer_classifier_flags_remote_desktop_candidate() {
    let classification =
        classify_remote_torrent_download_activity(NetworkTransferClassifierInput {
            indicators: vec![NetworkTransferIndicatorEvidence {
                indicator: NetworkTransferIndicator::RemoteDesktopPort,
                confidence_percent: 83,
                source_ref: "flow-port-3389".to_owned(),
                observed_bytes: Some(42_000),
            }],
        })
        .expect_value("remote desktop indicator should classify");

    assert_transfer_classification(
        &classification,
        NetworkTransferActivityKind::RemoteDesktop,
        NetworkTransferBasis::RemoteDesktopCandidate,
        83,
        SourceRef("flow-port-3389"),
    );
}

#[test]
fn transfer_classifier_flags_torrent_candidate() {
    let classification =
        classify_remote_torrent_download_activity(NetworkTransferClassifierInput {
            indicators: vec![NetworkTransferIndicatorEvidence {
                indicator: NetworkTransferIndicator::TorrentTrackerDomain,
                confidence_percent: 87,
                source_ref: "tracker-domain-1".to_owned(),
                observed_bytes: None,
            }],
        })
        .expect_value("torrent indicator should classify");

    assert_transfer_classification(
        &classification,
        NetworkTransferActivityKind::Torrent,
        NetworkTransferBasis::TorrentCandidate,
        87,
        SourceRef("tracker-domain-1"),
    );
}

#[test]
fn transfer_classifier_flags_large_download_without_file_name_claim() {
    let classification =
        classify_remote_torrent_download_activity(NetworkTransferClassifierInput {
            indicators: vec![NetworkTransferIndicatorEvidence {
                indicator: NetworkTransferIndicator::LargeDownloadByteCount,
                confidence_percent: 74,
                source_ref: "flow-bytes-1".to_owned(),
                observed_bytes: Some(4_294_967_296),
            }],
        })
        .expect_value("large download indicator should classify");

    assert_transfer_classification(
        &classification,
        NetworkTransferActivityKind::LargeDownload,
        NetworkTransferBasis::LargeDownloadCandidate,
        74,
        SourceRef("flow-bytes-1"),
    );
    assert!(!classification.file_name_available);
}

#[test]
fn transfer_classifier_keeps_unattributed_high_volume_uncertain() {
    let classification =
        classify_remote_torrent_download_activity(NetworkTransferClassifierInput {
            indicators: vec![NetworkTransferIndicatorEvidence {
                indicator: NetworkTransferIndicator::UnattributedHighVolume,
                confidence_percent: 70,
                source_ref: "high-volume-flow-1".to_owned(),
                observed_bytes: Some(8_589_934_592),
            }],
        })
        .expect_value("unattributed high volume should remain uncertain");

    assert_eq!(
        classification.activity_kind,
        NetworkTransferActivityKind::Unknown
    );
    assert_eq!(
        classification.basis,
        NetworkTransferBasis::UnattributedHighVolume
    );
    assert_eq!(
        classification.uncertainty,
        NetworkTransferUncertainty::UnattributedHighVolume
    );
    assert_eq!(classification.confidence_percent, 0);
    assert_eq!(
        classification.evidence_refs,
        vec!["high-volume-flow-1".to_owned()]
    );
    assert!(!classification.exact_url_available);
    assert!(!classification.decrypted_payload_available);
    assert!(!classification.file_name_available);
}

#[test]
fn transfer_classifier_rejects_invalid_confidence() {
    let result = classify_remote_torrent_download_activity(NetworkTransferClassifierInput {
        indicators: vec![NetworkTransferIndicatorEvidence {
            indicator: NetworkTransferIndicator::ParallelRangeRequests,
            confidence_percent: 101,
            source_ref: "range-request-1".to_owned(),
            observed_bytes: Some(1024),
        }],
    });

    assert_eq!(
        result,
        Err(NetworkTransferClassifierError::InvalidIndicatorConfidence(
            101
        ))
    );
}

fn assert_transfer_classification(
    classification: &NetworkTransferClassification,
    activity_kind: NetworkTransferActivityKind,
    basis: NetworkTransferBasis,
    confidence_percent: u8,
    source_ref: SourceRef,
) {
    assert_eq!(classification.activity_kind, activity_kind);
    assert_eq!(classification.basis, basis);
    assert_eq!(
        classification.uncertainty,
        NetworkTransferUncertainty::CandidateNeedsConfirmation
    );
    assert_eq!(classification.confidence_percent, confidence_percent);
    assert_eq!(classification.evidence_refs, vec![source_ref.0.to_owned()]);
    assert!(!classification.exact_url_available);
    assert!(!classification.decrypted_payload_available);
    assert!(!classification.file_name_available);
}
