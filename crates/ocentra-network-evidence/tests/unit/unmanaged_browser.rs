use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::unmanaged_browser::*;

#[test]
fn unmanaged_browser_correlation_records_known_browser_as_process_only_bypass() {
    let correlation =
        correlate_unmanaged_browser_activity(input(UnmanagedBrowserProcessKind::KnownBrowser))
            .expect_value("known unmanaged browser process should produce bypass evidence");

    assert_eq!(
        correlation.state,
        UnmanagedBrowserCorrelationState::ProcessOnlyBypassEvidence
    );
    assert_eq!(
        correlation.basis,
        UnmanagedBrowserCorrelationBasis::KnownBrowserProcess
    );
    assert_eq!(correlation.process_id, Some(4_242));
    assert_eq!(correlation.process_name, Some("chrome.exe".to_owned()));
    assert_eq!(correlation.browser_family, Some("chromium".to_owned()));
    assert!(correlation.possible_bypass);
    assert_eq!(correlation.evidence_grade, NetworkEvidenceGrade::C);
    assert_eq!(
        correlation.evidence_refs,
        vec!["unmanaged-browser-process-1"]
    );
    assert_no_exact_or_authority_claims(&correlation);
}

#[test]
fn unmanaged_browser_correlation_records_portable_browser_as_process_only_bypass() {
    let mut candidate = input(UnmanagedBrowserProcessKind::PortableBrowser);
    candidate.process_name = Some("PortableFox.exe".to_owned());
    candidate.browser_family = Some("firefox".to_owned());

    let correlation = correlate_unmanaged_browser_activity(candidate)
        .expect_value("portable browser process should produce bypass evidence");

    assert_eq!(
        correlation.state,
        UnmanagedBrowserCorrelationState::ProcessOnlyBypassEvidence
    );
    assert_eq!(
        correlation.basis,
        UnmanagedBrowserCorrelationBasis::PortableBrowserProcess
    );
    assert!(correlation.possible_bypass);
    assert_no_exact_or_authority_claims(&correlation);
}

#[test]
fn unmanaged_browser_correlation_keeps_browser_like_process_candidate() {
    let mut candidate = input(UnmanagedBrowserProcessKind::BrowserLikeProcess);
    candidate.process_name = Some("custom-browser.exe".to_owned());
    candidate.browser_family = None;

    let correlation = correlate_unmanaged_browser_activity(candidate)
        .expect_value("browser-like process should remain candidate-only");

    assert_eq!(
        correlation.state,
        UnmanagedBrowserCorrelationState::ProcessOnlyBypassCandidate
    );
    assert_eq!(
        correlation.basis,
        UnmanagedBrowserCorrelationBasis::BrowserLikeProcessName
    );
    assert_eq!(correlation.evidence_grade, NetworkEvidenceGrade::D);
    assert!(correlation.possible_bypass);
    assert_no_exact_or_authority_claims(&correlation);
}

#[test]
fn unmanaged_browser_correlation_does_not_treat_managed_browser_as_unmanaged() {
    let mut managed = input(UnmanagedBrowserProcessKind::ManagedOcentraBrowser);
    managed.process_name = Some("ocentra-browser.exe".to_owned());
    managed.browser_family = Some("ocentra-managed".to_owned());

    let correlation = correlate_unmanaged_browser_activity(managed)
        .expect_value("managed browser should stay inside managed boundary");

    assert_eq!(
        correlation.state,
        UnmanagedBrowserCorrelationState::ManagedBrowserBoundary
    );
    assert_eq!(
        correlation.basis,
        UnmanagedBrowserCorrelationBasis::ManagedBrowserProcess
    );
    assert!(!correlation.possible_bypass);
    assert_no_exact_or_authority_claims(&correlation);
}

#[test]
fn unmanaged_browser_correlation_preserves_no_browser_and_adapter_unavailable_states() {
    let mut no_browser = input(UnmanagedBrowserProcessKind::NonBrowserProcess);
    no_browser.process_name = Some("game.exe".to_owned());

    let missing = correlate_unmanaged_browser_activity(no_browser)
        .expect_value("non-browser process should not become unmanaged evidence");
    assert_eq!(
        missing.state,
        UnmanagedBrowserCorrelationState::NoUnmanagedBrowserEvidence
    );
    assert_eq!(
        missing.basis,
        UnmanagedBrowserCorrelationBasis::MissingBrowserLikeProcess
    );
    assert!(!missing.possible_bypass);
    assert_no_exact_or_authority_claims(&missing);

    let mut unavailable = input(UnmanagedBrowserProcessKind::KnownBrowser);
    unavailable.adapter_available = false;

    let adapter = correlate_unmanaged_browser_activity(unavailable)
        .expect_value("unavailable adapter should be explicit");
    assert_eq!(
        adapter.state,
        UnmanagedBrowserCorrelationState::AdapterUnavailable
    );
    assert_eq!(
        adapter.basis,
        UnmanagedBrowserCorrelationBasis::AdapterUnavailable
    );
    assert!(!adapter.possible_bypass);
    assert_no_exact_or_authority_claims(&adapter);
}

#[test]
fn unmanaged_browser_correlation_rejects_url_tab_content_and_authority_claims() {
    let mut exact_url = input(UnmanagedBrowserProcessKind::KnownBrowser);
    exact_url.exact_url_claimed = true;
    assert_eq!(
        correlate_unmanaged_browser_activity(exact_url),
        Err(UnmanagedBrowserCorrelationError::UnsupportedExactUrlClaim)
    );

    let mut active_tab = input(UnmanagedBrowserProcessKind::KnownBrowser);
    active_tab.active_tab_claimed = true;
    assert_eq!(
        correlate_unmanaged_browser_activity(active_tab),
        Err(UnmanagedBrowserCorrelationError::UnsupportedActiveTabClaim)
    );

    let mut page_title = input(UnmanagedBrowserProcessKind::KnownBrowser);
    page_title.page_title_claimed = true;
    assert_eq!(
        correlate_unmanaged_browser_activity(page_title),
        Err(UnmanagedBrowserCorrelationError::UnsupportedPageTitleClaim)
    );

    let mut page_content = input(UnmanagedBrowserProcessKind::KnownBrowser);
    page_content.page_content_claimed = true;
    assert_eq!(
        correlate_unmanaged_browser_activity(page_content),
        Err(UnmanagedBrowserCorrelationError::UnsupportedPageContentClaim)
    );

    let mut decrypted = input(UnmanagedBrowserProcessKind::KnownBrowser);
    decrypted.decrypted_payload_claimed = true;
    assert_eq!(
        correlate_unmanaged_browser_activity(decrypted),
        Err(UnmanagedBrowserCorrelationError::UnsupportedDecryptedPayloadClaim)
    );

    let mut policy = input(UnmanagedBrowserProcessKind::KnownBrowser);
    policy.policy_action_authority = true;
    assert_eq!(
        correlate_unmanaged_browser_activity(policy),
        Err(UnmanagedBrowserCorrelationError::UnsupportedPolicyAuthorityClaim)
    );

    let mut adapter = input(UnmanagedBrowserProcessKind::KnownBrowser);
    adapter.adapter_action_authorized = true;
    assert_eq!(
        correlate_unmanaged_browser_activity(adapter),
        Err(UnmanagedBrowserCorrelationError::UnsupportedAdapterAuthorityClaim)
    );

    let mut command = input(UnmanagedBrowserProcessKind::KnownBrowser);
    command.enforcement_command_published = true;
    assert_eq!(
        correlate_unmanaged_browser_activity(command),
        Err(UnmanagedBrowserCorrelationError::UnsupportedEnforcementCommandClaim)
    );
}

#[test]
fn unmanaged_browser_correlation_rejects_empty_refs_and_invalid_confidence() {
    let mut empty_ref = input(UnmanagedBrowserProcessKind::KnownBrowser);
    empty_ref.observation_ref = " ".to_owned();
    assert_eq!(
        correlate_unmanaged_browser_activity(empty_ref),
        Err(UnmanagedBrowserCorrelationError::EmptyObservationRef)
    );

    let mut empty_name = input(UnmanagedBrowserProcessKind::KnownBrowser);
    empty_name.process_name = Some(" ".to_owned());
    assert_eq!(
        correlate_unmanaged_browser_activity(empty_name),
        Err(UnmanagedBrowserCorrelationError::EmptyProcessName)
    );

    let mut invalid_confidence = input(UnmanagedBrowserProcessKind::KnownBrowser);
    invalid_confidence.confidence = Some(101);
    assert_eq!(
        correlate_unmanaged_browser_activity(invalid_confidence),
        Err(UnmanagedBrowserCorrelationError::InvalidConfidence(101))
    );
}

fn input(process_kind: UnmanagedBrowserProcessKind) -> UnmanagedBrowserCorrelationInput {
    UnmanagedBrowserCorrelationInput {
        observation_ref: "unmanaged-browser-process-1".to_owned(),
        adapter_available: true,
        process_id: Some(4_242),
        process_name: Some("chrome.exe".to_owned()),
        redacted_executable_path_ref: Some("path-ref-1".to_owned()),
        signature_ref: Some("signature-ref-1".to_owned()),
        hash_ref: Some("hash-ref-1".to_owned()),
        browser_family: Some("chromium".to_owned()),
        process_kind: Some(process_kind),
        confidence: Some(88),
        possible_bypass_reason_ref: Some("bypass-reason-1".to_owned()),
        exact_url_claimed: false,
        active_tab_claimed: false,
        page_title_claimed: false,
        page_content_claimed: false,
        decrypted_payload_claimed: false,
        policy_action_authority: false,
        adapter_action_authorized: false,
        enforcement_command_published: false,
    }
}

fn assert_no_exact_or_authority_claims(correlation: &UnmanagedBrowserCorrelation) {
    assert!(!correlation.exact_url_available);
    assert!(!correlation.active_tab_available);
    assert!(!correlation.page_title_available);
    assert!(!correlation.page_content_available);
    assert!(!correlation.decrypted_payload_available);
    assert!(!correlation.policy_action_authority);
    assert!(!correlation.adapter_action_authorized);
    assert!(!correlation.enforcement_command_published);
}
