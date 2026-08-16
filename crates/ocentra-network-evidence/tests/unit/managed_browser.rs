use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::managed_browser::*;

#[test]
fn managed_browser_correlation_uses_browser_url_for_matching_domain() {
    let correlation = correlate_managed_browser_activity(ManagedBrowserCorrelationInput {
        network_flow: network_flow(Some(DomainCase::Video)),
        managed_browser: Some(browser_evidence(DomainCase::Video)),
    })
    .expect_value("matching managed browser evidence should confirm exact url");

    assert_eq!(
        correlation.state,
        ManagedBrowserCorrelationState::ExactUrlConfirmed
    );
    assert_eq!(
        correlation.basis,
        ManagedBrowserCorrelationBasis::ManagedBrowserUrlEvidence
    );
    assert_eq!(
        correlation.exact_url,
        Some("https://video.example.test/watch/123".to_owned())
    );
    assert_eq!(
        correlation.exact_url_source_ref,
        Some("managed-browser-page-1".to_owned())
    );
    assert!(correlation.exact_url_from_managed_browser);
    assert_no_network_url_claim(&correlation);
}

#[test]
fn managed_browser_correlation_keeps_network_domain_without_exact_url() {
    let correlation = correlate_managed_browser_activity(ManagedBrowserCorrelationInput {
        network_flow: network_flow(Some(DomainCase::Video)),
        managed_browser: None,
    })
    .expect_value("network domain alone should remain domain-only");

    assert_eq!(
        correlation.state,
        ManagedBrowserCorrelationState::NetworkDomainOnly
    );
    assert_eq!(
        correlation.basis,
        ManagedBrowserCorrelationBasis::NetworkDomainEvidenceOnly
    );
    assert_eq!(
        correlation.network_domain,
        Some("video.example.test".to_owned())
    );
    assert_eq!(correlation.exact_url, None);
    assert_eq!(correlation.evidence_grade, NetworkEvidenceGrade::C);
    assert_no_network_url_claim(&correlation);
}

#[test]
fn managed_browser_correlation_rejects_mismatched_browser_domain() {
    let correlation = correlate_managed_browser_activity(ManagedBrowserCorrelationInput {
        network_flow: network_flow(Some(DomainCase::Video)),
        managed_browser: Some(browser_evidence(DomainCase::Social)),
    })
    .expect_value("mismatched browser evidence should not confirm url");

    assert_eq!(
        correlation.state,
        ManagedBrowserCorrelationState::BrowserDomainMismatch
    );
    assert_eq!(
        correlation.basis,
        ManagedBrowserCorrelationBasis::MismatchedBrowserDomain
    );
    assert_eq!(correlation.exact_url, None);
    assert_eq!(correlation.evidence_grade, NetworkEvidenceGrade::D);
    assert_no_network_url_claim(&correlation);
}

#[test]
fn managed_browser_correlation_requires_browser_evidence_for_exact_url() {
    let correlation = correlate_managed_browser_activity(ManagedBrowserCorrelationInput {
        network_flow: network_flow(None),
        managed_browser: None,
    })
    .expect_value("missing browser evidence should keep exact url unavailable");

    assert_eq!(
        correlation.state,
        ManagedBrowserCorrelationState::MissingManagedBrowserEvidence
    );
    assert_eq!(
        correlation.basis,
        ManagedBrowserCorrelationBasis::NoManagedBrowserEvidence
    );
    assert_eq!(correlation.exact_url, None);
    assert_no_network_url_claim(&correlation);
}

#[test]
fn managed_browser_correlation_rejects_empty_browser_source_ref() {
    let mut browser = browser_evidence(DomainCase::Video);
    browser.source_ref = " ".to_owned();

    let result = correlate_managed_browser_activity(ManagedBrowserCorrelationInput {
        network_flow: network_flow(Some(DomainCase::Video)),
        managed_browser: Some(browser),
    });

    assert_eq!(
        result,
        Err(ManagedBrowserCorrelationError::EmptyBrowserSourceRef)
    );
}

#[derive(Clone, Copy)]
enum DomainCase {
    Video,
    Social,
}

fn network_flow(domain: Option<DomainCase>) -> NetworkManagedBrowserFlowEvidence {
    let observed_domain = domain.map(|domain| {
        match domain {
            DomainCase::Video => "video.example.test",
            DomainCase::Social => "social.example.test",
        }
        .to_owned()
    });

    NetworkManagedBrowserFlowEvidence {
        flow_ref: "flow-1".to_owned(),
        observed_domain,
    }
}

fn browser_evidence(page_domain: DomainCase) -> ManagedBrowserPageEvidence {
    let page_domain = match page_domain {
        DomainCase::Video => "video.example.test",
        DomainCase::Social => "social.example.test",
    };

    ManagedBrowserPageEvidence {
        browser_ref: "managed-browser-1".to_owned(),
        tab_ref: "tab-1".to_owned(),
        page_url: "https://video.example.test/watch/123".to_owned(),
        page_domain: page_domain.to_owned(),
        source_ref: "managed-browser-page-1".to_owned(),
    }
}

fn assert_no_network_url_claim(correlation: &ManagedBrowserCorrelation) {
    assert!(!correlation.exact_url_from_network);
    assert!(!correlation.decrypted_payload_available);
}
