use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::category::{
    lookup_domain_category, CategorySourceCustody, DomainCategoryDatabase, DomainCategoryRecord,
    DomainCategorySource, NetworkCategory,
};
use ocentra_network_evidence::classifier::NetworkClassifierBasis;
use ocentra_network_evidence::classifier::*;
use ocentra_network_evidence::domain::{normalize_domain_with_public_suffix, PublicSuffixModel};

#[test]
fn classifier_uses_fresh_domain_category_for_video() {
    let model = PublicSuffixModel::ocentra_fixture();
    let evidence = normalize_domain_with_public_suffix("watch.video.example.test", &model)
        .expect_value("fixture domain should normalize");
    let database = DomainCategoryDatabase::from_records(vec![DomainCategoryRecord {
        domain: "example.test".to_owned(),
        category: NetworkCategory::Video,
        source: DomainCategorySource {
            source_id: "category-source-video".to_owned(),
            retrieved_at_epoch_seconds: 1_000,
            max_age_seconds: 300,
            custody: CategorySourceCustody::SignedLocalSnapshot,
            signature_required: true,
        },
        confidence_percent: 92,
    }])
    .expect_value("fixture category database should be valid");
    let domain_lookup = lookup_domain_category(&database, &evidence, 1_050);

    let classification = classify_social_video_game_activity(NetworkActivityClassifierInput {
        domain_lookup,
        cdn_hint: None,
        process_hint: None,
        browser_confirmation: None,
    })
    .expect_value("fresh domain category should classify");

    assert_eq!(classification.category, NetworkCategory::Video);
    assert_eq!(classification.basis, NetworkClassifierBasis::DomainCategory);
    assert_eq!(classification.confidence_percent, 92);
    assert_eq!(
        classification.evidence_refs,
        vec!["category-source-video".to_owned()]
    );
    assert!(!classification.browser_confirmation_required);
    assert!(!classification.exact_url_available);
    assert!(!classification.decrypted_payload_available);
}

#[test]
fn classifier_keeps_cdn_hint_confirmation_required_without_browser_evidence() {
    let input = unknown_domain_input(
        Some(CdnClassifierHint {
            provider_domain: "cdn.example.test".to_owned(),
            category_hint: NetworkCategory::CloudGaming,
            confidence_percent: 88,
            source_ref: "cdn-hint-1".to_owned(),
        }),
        None,
        None,
    );

    let classification =
        classify_social_video_game_activity(input).expect_value("cdn candidate should classify");

    assert_eq!(classification.category, NetworkCategory::CloudGaming);
    assert_eq!(
        classification.basis,
        NetworkClassifierBasis::CdnCandidateNeedsConfirmation
    );
    assert_eq!(classification.confidence_percent, 60);
    assert_eq!(classification.evidence_refs, vec!["cdn-hint-1".to_owned()]);
    assert!(classification.browser_confirmation_required);
    assert!(!classification.exact_url_available);
    assert!(!classification.decrypted_payload_available);
}

#[test]
fn classifier_promotes_matching_cdn_when_browser_confirmation_exists() {
    let input = unknown_domain_input(
        Some(CdnClassifierHint {
            provider_domain: "cdn.example.test".to_owned(),
            category_hint: NetworkCategory::CloudGaming,
            confidence_percent: 88,
            source_ref: "cdn-hint-1".to_owned(),
        }),
        None,
        Some(BrowserClassifierConfirmation {
            confirmed_domain: "play.example.test".to_owned(),
            category: NetworkCategory::CloudGaming,
            source_ref: "browser-confirmation-1".to_owned(),
        }),
    );

    let classification = classify_social_video_game_activity(input)
        .expect_value("browser-confirmed cdn should classify");

    assert_eq!(classification.category, NetworkCategory::CloudGaming);
    assert_eq!(
        classification.basis,
        NetworkClassifierBasis::BrowserConfirmedCdn
    );
    assert_eq!(classification.confidence_percent, 88);
    assert_eq!(
        classification.evidence_refs,
        vec!["cdn-hint-1".to_owned(), "browser-confirmation-1".to_owned()]
    );
    assert!(!classification.browser_confirmation_required);
    assert!(!classification.exact_url_available);
    assert!(!classification.decrypted_payload_available);
}

#[test]
fn classifier_uses_process_hint_as_confirmation_required_game_candidate() {
    let input = unknown_domain_input(
        None,
        Some(ProcessClassifierHint {
            process_name: "launcher.exe".to_owned(),
            category_hint: NetworkCategory::Game,
            confidence_percent: 95,
            source_ref: "process-hint-1".to_owned(),
        }),
        None,
    );

    let classification = classify_social_video_game_activity(input)
        .expect_value("process candidate should classify");

    assert_eq!(classification.category, NetworkCategory::Game);
    assert_eq!(
        classification.basis,
        NetworkClassifierBasis::ProcessCandidateNeedsConfirmation
    );
    assert_eq!(classification.confidence_percent, 70);
    assert_eq!(
        classification.evidence_refs,
        vec!["process-hint-1".to_owned()]
    );
    assert!(classification.browser_confirmation_required);
}

#[test]
fn classifier_rejects_invalid_cdn_confidence() {
    let input = unknown_domain_input(
        Some(CdnClassifierHint {
            provider_domain: "cdn.example.test".to_owned(),
            category_hint: NetworkCategory::Video,
            confidence_percent: 101,
            source_ref: "cdn-hint-1".to_owned(),
        }),
        None,
        None,
    );

    let result = classify_social_video_game_activity(input);

    assert_eq!(
        result,
        Err(NetworkClassifierError::InvalidCdnConfidence(101))
    );
}

fn unknown_domain_input(
    cdn_hint: Option<CdnClassifierHint>,
    process_hint: Option<ProcessClassifierHint>,
    browser_confirmation: Option<BrowserClassifierConfirmation>,
) -> NetworkActivityClassifierInput {
    let model = PublicSuffixModel::ocentra_fixture();
    let evidence = normalize_domain_with_public_suffix("unknown.example.test", &model)
        .expect_value("fixture domain should normalize");
    let database = DomainCategoryDatabase::from_records(Vec::new())
        .expect_value("empty category database should be valid");
    let domain_lookup = lookup_domain_category(&database, &evidence, 1_050);

    NetworkActivityClassifierInput {
        domain_lookup,
        cdn_hint,
        process_hint,
        browser_confirmation,
    }
}
