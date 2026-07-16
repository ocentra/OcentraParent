use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::cascade::*;
use ocentra_network_evidence::dns::types::*;

#[derive(Clone, Copy)]
struct SourceRef(&'static str);

#[test]
fn cascade_router_prefers_managed_browser_exact_url_over_network_domain() {
    let decision = route_network_evidence_cascade(NetworkEvidenceCascadeInput {
        sources: vec![
            source(
                NetworkCascadeSourceKind::DomainCategory,
                NetworkCascadeSignalStrength::Confirmed,
                SourceRef("domain-category-1"),
                false,
            ),
            source(
                NetworkCascadeSourceKind::ManagedBrowserExactUrl,
                NetworkCascadeSignalStrength::Confirmed,
                SourceRef("managed-browser-1"),
                true,
            ),
        ],
    })
    .expect_value("confirmed managed browser exact url should route");

    assert_eq!(
        decision.primary_source,
        Some(NetworkCascadeSourceKind::ManagedBrowserExactUrl)
    );
    assert!(decision.exact_url_available);
    assert!(!decision.adapter_action_authorized);
    assert!(!decision.policy_action_authority);
}

#[test]
fn cascade_router_orders_next_checks_for_weak_hint() {
    let decision = route_network_evidence_cascade(NetworkEvidenceCascadeInput {
        sources: vec![source(
            NetworkCascadeSourceKind::TransferCandidate,
            NetworkCascadeSignalStrength::WeakHint,
            SourceRef("transfer-hint-1"),
            false,
        )],
    })
    .expect_value("weak hint should request next checks");

    assert_eq!(
        decision.next_checks,
        vec![
            NetworkCascadeNextCheck::ManagedBrowserCorrelation,
            NetworkCascadeNextCheck::ProcessAppCorrelation,
            NetworkCascadeNextCheck::ScreenSummary,
            NetworkCascadeNextCheck::LocalAiReview,
        ]
    );
    assert!(!decision.parent_review_required);
    assert!(!decision.adapter_action_authorized);
}

#[test]
fn cascade_router_keeps_candidate_parent_review_manual() {
    let decision = route_network_evidence_cascade(NetworkEvidenceCascadeInput {
        sources: vec![source(
            NetworkCascadeSourceKind::TunnelIndicator,
            NetworkCascadeSignalStrength::Candidate,
            SourceRef("tunnel-candidate-1"),
            false,
        )],
    })
    .expect_value("candidate should request parent review only");

    assert_eq!(
        decision.next_checks,
        vec![NetworkCascadeNextCheck::ParentReview]
    );
    assert!(decision.parent_review_required);
    assert!(!decision.adapter_action_authorized);
    assert!(!decision.policy_action_authority);
}

#[test]
fn cascade_router_routes_missing_sources_to_correlation_checks() {
    let decision = route_network_evidence_cascade(NetworkEvidenceCascadeInput {
        sources: Vec::new(),
    })
    .expect_value("missing sources should produce next checks");

    assert_eq!(
        decision.next_checks,
        vec![
            NetworkCascadeNextCheck::ManagedBrowserCorrelation,
            NetworkCascadeNextCheck::ProcessAppCorrelation,
            NetworkCascadeNextCheck::ScreenSummary,
        ]
    );
    assert_eq!(decision.primary_source, None);
    assert!(!decision.exact_url_available);
}

#[test]
fn cascade_router_rejects_decrypted_payload_claim() {
    let mut source = source(
        NetworkCascadeSourceKind::DomainCategory,
        NetworkCascadeSignalStrength::Confirmed,
        SourceRef("domain-category-1"),
        false,
    );
    source.decrypted_payload_available = true;

    let result = route_network_evidence_cascade(NetworkEvidenceCascadeInput {
        sources: vec![source],
    });

    assert_eq!(
        result,
        Err(NetworkEvidenceCascadeError::UnsupportedDecryptedPayloadClaim)
    );
}

#[test]
fn cascade_router_rejects_network_exact_url_claim() {
    let result = route_network_evidence_cascade(NetworkEvidenceCascadeInput {
        sources: vec![source(
            NetworkCascadeSourceKind::DomainCategory,
            NetworkCascadeSignalStrength::Confirmed,
            SourceRef("domain-category-1"),
            true,
        )],
    });

    assert_eq!(
        result,
        Err(
            NetworkEvidenceCascadeError::UnsupportedNetworkExactUrlClaim(
                NetworkCascadeSourceKind::DomainCategory
            )
        )
    );
}

fn source(
    source_kind: NetworkCascadeSourceKind,
    signal_strength: NetworkCascadeSignalStrength,
    source_ref: SourceRef,
    exact_url_available: bool,
) -> NetworkCascadeSource {
    NetworkCascadeSource {
        source_kind,
        signal_strength,
        evidence_grade: NetworkEvidenceGrade::C,
        source_ref: source_ref.0.to_owned(),
        exact_url_available,
        decrypted_payload_available: false,
        policy_action_authority: false,
    }
}
