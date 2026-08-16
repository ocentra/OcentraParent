use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::category::*;
use ocentra_network_evidence::domain::{normalize_domain_with_public_suffix, PublicSuffixModel};

#[test]
fn category_database_matches_registrable_domain_with_fresh_source() {
    let model = PublicSuffixModel::ocentra_fixture();
    let evidence = normalize_domain_with_public_suffix("watch.video.example.test", &model)
        .expect_value("fixture domain should normalize");
    let source = DomainCategorySource {
        source_id: "ocentra-category-fixture-v1".to_owned(),
        retrieved_at_epoch_seconds: 1_000,
        max_age_seconds: 300,
        custody: CategorySourceCustody::SignedLocalSnapshot,
        signature_required: true,
    };
    let database = DomainCategoryDatabase::from_records(vec![DomainCategoryRecord {
        domain: "example.test".to_owned(),
        category: NetworkCategory::Video,
        source,
        confidence_percent: 92,
    }])
    .expect_value("fixture category database should be valid");

    let lookup = lookup_domain_category(&database, &evidence, 1_050);

    assert_eq!(lookup.normalized_domain, "watch.video.example.test");
    assert_eq!(lookup.matched_domain, Some("example.test".to_owned()));
    assert_eq!(lookup.match_kind, CategoryMatchKind::RegistrableDomain);
    assert_eq!(lookup.category, NetworkCategory::Video);
    assert_eq!(
        lookup.source_id,
        Some("ocentra-category-fixture-v1".to_owned())
    );
    assert_eq!(
        lookup.source_custody,
        Some(CategorySourceCustody::SignedLocalSnapshot)
    );
    assert_eq!(
        lookup.freshness,
        CategoryFreshnessState::Fresh {
            age_seconds: 50,
            max_age_seconds: 300
        }
    );
    assert_eq!(lookup.confidence_percent, Some(92));
    assert!(!lookup.exact_url_available);
    assert!(!lookup.decrypted_payload_available);
}

#[test]
fn category_database_marks_stale_source_without_upgrading_claim() {
    let model = PublicSuffixModel::ocentra_fixture();
    let evidence = normalize_domain_with_public_suffix("video.example.test", &model)
        .expect_value("fixture domain should normalize");
    let source = DomainCategorySource {
        source_id: "ocentra-category-fixture-v1".to_owned(),
        retrieved_at_epoch_seconds: 100,
        max_age_seconds: 10,
        custody: CategorySourceCustody::BuiltInFixture,
        signature_required: false,
    };
    let database = DomainCategoryDatabase::from_records(vec![DomainCategoryRecord {
        domain: "video.example.test".to_owned(),
        category: NetworkCategory::Video,
        source,
        confidence_percent: 80,
    }])
    .expect_value("fixture category database should be valid");

    let lookup = lookup_domain_category(&database, &evidence, 111);

    assert_eq!(lookup.match_kind, CategoryMatchKind::ExactDomain);
    assert_eq!(lookup.category, NetworkCategory::Video);
    assert_eq!(
        lookup.freshness,
        CategoryFreshnessState::Stale {
            age_seconds: 11,
            max_age_seconds: 10
        }
    );
    assert!(!lookup.exact_url_available);
    assert!(!lookup.decrypted_payload_available);
}

#[test]
fn category_update_policy_rejects_unsigned_required_snapshot() {
    let candidate = DomainCategorySource {
        source_id: "vendor-category-snapshot-v2".to_owned(),
        retrieved_at_epoch_seconds: 2_000,
        max_age_seconds: 86_400,
        custody: CategorySourceCustody::Unverified,
        signature_required: true,
    };

    let decision = evaluate_category_source_update(None, &candidate);

    assert_eq!(decision, CategoryUpdateDecision::RejectMissingSignature);
}

#[test]
fn category_update_policy_accepts_newer_signed_snapshot() {
    let current = DomainCategorySource {
        source_id: "vendor-category-snapshot-v1".to_owned(),
        retrieved_at_epoch_seconds: 1_000,
        max_age_seconds: 86_400,
        custody: CategorySourceCustody::SignedLocalSnapshot,
        signature_required: true,
    };
    let candidate = DomainCategorySource {
        source_id: "vendor-category-snapshot-v2".to_owned(),
        retrieved_at_epoch_seconds: 2_000,
        max_age_seconds: 86_400,
        custody: CategorySourceCustody::SignedLocalSnapshot,
        signature_required: true,
    };

    let decision = evaluate_category_source_update(Some(&current), &candidate);

    assert_eq!(decision, CategoryUpdateDecision::Accept);
}

#[test]
fn category_update_policy_rejects_older_snapshot() {
    let current = DomainCategorySource {
        source_id: "vendor-category-snapshot-v2".to_owned(),
        retrieved_at_epoch_seconds: 2_000,
        max_age_seconds: 86_400,
        custody: CategorySourceCustody::SignedLocalSnapshot,
        signature_required: true,
    };
    let candidate = DomainCategorySource {
        source_id: "vendor-category-snapshot-v1".to_owned(),
        retrieved_at_epoch_seconds: 1_000,
        max_age_seconds: 86_400,
        custody: CategorySourceCustody::SignedLocalSnapshot,
        signature_required: true,
    };

    let decision = evaluate_category_source_update(Some(&current), &candidate);

    assert_eq!(decision, CategoryUpdateDecision::RejectOlderSnapshot);
}
