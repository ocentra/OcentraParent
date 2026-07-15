use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::bundle::*;
use ocentra_network_evidence::cascade::{
    NetworkCascadeNextCheck, NetworkCascadeSignalStrength, NetworkCascadeSourceKind,
};
use ocentra_network_evidence::dns::types::NetworkEvidenceGrade;

#[derive(Clone, Copy)]
struct EvidenceRef(&'static str);

#[test]
fn evidence_bundle_preserves_cross_slice_refs_without_action_authority() {
    let bundle = build_network_cross_slice_evidence_bundle(NetworkCrossSliceEvidenceBundleInput {
        trigger_ref: "network-trigger-1".to_owned(),
        sources: vec![
            source(
                NetworkCascadeSourceKind::DomainCategory,
                NetworkCascadeSignalStrength::Confirmed,
                NetworkEvidenceGrade::B,
                EvidenceRef("domain-category-1"),
                false,
            ),
            source(
                NetworkCascadeSourceKind::ManagedBrowserExactUrl,
                NetworkCascadeSignalStrength::Confirmed,
                NetworkEvidenceGrade::B,
                EvidenceRef("managed-browser-1"),
                true,
            ),
            source(
                NetworkCascadeSourceKind::ProcessAppCorrelation,
                NetworkCascadeSignalStrength::Candidate,
                NetworkEvidenceGrade::C,
                EvidenceRef("process-app-1"),
                false,
            ),
        ],
    })
    .expect_value("cross-slice bundle should build from typed evidence");

    assert_eq!(bundle.trigger_ref, "network-trigger-1");
    assert_eq!(
        bundle.primary_source,
        Some(NetworkCascadeSourceKind::ManagedBrowserExactUrl)
    );
    assert_eq!(
        bundle.evidence_refs,
        vec![
            "domain-category-1".to_owned(),
            "managed-browser-1".to_owned(),
            "process-app-1".to_owned()
        ]
    );
    assert_eq!(bundle.exact_url_evidence_refs, vec!["managed-browser-1"]);
    assert!(bundle.exact_url_available);
    assert!(!bundle.decrypted_payload_available);
    assert!(!bundle.policy_action_authority);
    assert!(!bundle.adapter_action_authorized);
}

#[test]
fn evidence_bundle_keeps_weak_signal_on_review_route() {
    let bundle = build_network_cross_slice_evidence_bundle(NetworkCrossSliceEvidenceBundleInput {
        trigger_ref: "network-trigger-weak".to_owned(),
        sources: vec![source(
            NetworkCascadeSourceKind::TransferCandidate,
            NetworkCascadeSignalStrength::WeakHint,
            NetworkEvidenceGrade::D,
            EvidenceRef("transfer-hint-1"),
            false,
        )],
    })
    .expect_value("weak signal should build review bundle");

    assert_eq!(
        bundle.next_checks,
        vec![
            NetworkCascadeNextCheck::ManagedBrowserCorrelation,
            NetworkCascadeNextCheck::ProcessAppCorrelation,
            NetworkCascadeNextCheck::ScreenSummary,
            NetworkCascadeNextCheck::LocalAiReview,
        ]
    );
    assert!(bundle.local_ai_review_recommended);
    assert!(!bundle.parent_review_required);
    assert!(!bundle.policy_action_authority);
    assert!(!bundle.adapter_action_authorized);
}

#[test]
fn evidence_bundle_deduplicates_refs_and_trims_trigger_ref() {
    let bundle = build_network_cross_slice_evidence_bundle(NetworkCrossSliceEvidenceBundleInput {
        trigger_ref: " network-trigger-trimmed ".to_owned(),
        sources: vec![
            source(
                NetworkCascadeSourceKind::DomainCategory,
                NetworkCascadeSignalStrength::Confirmed,
                NetworkEvidenceGrade::C,
                EvidenceRef(" duplicate-ref "),
                false,
            ),
            source(
                NetworkCascadeSourceKind::TunnelIndicator,
                NetworkCascadeSignalStrength::Candidate,
                NetworkEvidenceGrade::C,
                EvidenceRef("duplicate-ref"),
                false,
            ),
        ],
    })
    .expect_value("duplicate refs should not duplicate evidence custody");

    assert_eq!(bundle.trigger_ref, "network-trigger-trimmed");
    assert_eq!(bundle.evidence_refs, vec!["duplicate-ref"]);
}

#[test]
fn evidence_bundle_rejects_network_only_exact_url_claim() {
    let result = build_network_cross_slice_evidence_bundle(NetworkCrossSliceEvidenceBundleInput {
        trigger_ref: "network-trigger-1".to_owned(),
        sources: vec![source(
            NetworkCascadeSourceKind::DomainCategory,
            NetworkCascadeSignalStrength::Confirmed,
            NetworkEvidenceGrade::B,
            EvidenceRef("domain-category-1"),
            true,
        )],
    });

    assert_eq!(
        result,
        Err(
            NetworkCrossSliceEvidenceBundleError::UnsupportedNetworkExactUrlClaim(
                NetworkCascadeSourceKind::DomainCategory
            )
        )
    );
}

#[test]
fn evidence_bundle_rejects_decrypted_payload_or_action_authority() {
    let mut decrypted = source(
        NetworkCascadeSourceKind::DomainCategory,
        NetworkCascadeSignalStrength::Confirmed,
        NetworkEvidenceGrade::B,
        EvidenceRef("domain-category-1"),
        false,
    );
    decrypted.decrypted_payload_available = true;

    assert_eq!(
        build_network_cross_slice_evidence_bundle(NetworkCrossSliceEvidenceBundleInput {
            trigger_ref: "network-trigger-1".to_owned(),
            sources: vec![decrypted],
        }),
        Err(NetworkCrossSliceEvidenceBundleError::UnsupportedDecryptedPayloadClaim)
    );

    let mut policy = source(
        NetworkCascadeSourceKind::ProcessAppCorrelation,
        NetworkCascadeSignalStrength::Confirmed,
        NetworkEvidenceGrade::B,
        EvidenceRef("process-app-1"),
        false,
    );
    policy.policy_action_authority = true;

    assert_eq!(
        build_network_cross_slice_evidence_bundle(NetworkCrossSliceEvidenceBundleInput {
            trigger_ref: "network-trigger-1".to_owned(),
            sources: vec![policy],
        }),
        Err(NetworkCrossSliceEvidenceBundleError::UnsupportedPolicyAuthorityClaim)
    );

    let mut adapter = source(
        NetworkCascadeSourceKind::TunnelIndicator,
        NetworkCascadeSignalStrength::Confirmed,
        NetworkEvidenceGrade::C,
        EvidenceRef("tunnel-1"),
        false,
    );
    adapter.adapter_action_authority = true;

    assert_eq!(
        build_network_cross_slice_evidence_bundle(NetworkCrossSliceEvidenceBundleInput {
            trigger_ref: "network-trigger-1".to_owned(),
            sources: vec![adapter],
        }),
        Err(NetworkCrossSliceEvidenceBundleError::UnsupportedAdapterAuthorityClaim)
    );
}

fn source(
    source_kind: NetworkCascadeSourceKind,
    signal_strength: NetworkCascadeSignalStrength,
    evidence_grade: NetworkEvidenceGrade,
    evidence_ref: EvidenceRef,
    exact_url_available: bool,
) -> NetworkCrossSliceEvidenceSource {
    NetworkCrossSliceEvidenceSource {
        source_kind,
        signal_strength,
        evidence_grade,
        evidence_ref: evidence_ref.0.to_owned(),
        exact_url_available,
        decrypted_payload_available: false,
        policy_action_authority: false,
        adapter_action_authority: false,
    }
}
