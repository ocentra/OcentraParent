use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::dns::types::NetworkEvidenceGrade;
use ocentra_network_evidence::domain::{
    normalize_domain_with_public_suffix, DomainNormalizationError, PublicSuffixModel,
};

const MIXED_CASE_DOMAIN_WITH_TRAILING_DOT: &str = "  Video.Example.Co.UK. ";
const TOO_LONG_LABEL: &str =
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.example.test";

#[test]
fn domain_normalization_extracts_suffix_and_registrable_domain() {
    let model = PublicSuffixModel::ocentra_fixture();

    let evidence = normalize_domain_with_public_suffix(MIXED_CASE_DOMAIN_WITH_TRAILING_DOT, &model)
        .expect_value("domain evidence should normalize");

    assert_eq!(evidence.normalized_domain, "video.example.co.uk");
    assert_eq!(
        evidence.labels,
        vec![
            String::from("video"),
            String::from("example"),
            String::from("co"),
            String::from("uk")
        ]
    );
    assert_eq!(evidence.public_suffix.as_deref(), Some("co.uk"));
    assert_eq!(
        evidence.registrable_domain.as_deref(),
        Some("example.co.uk")
    );
    assert_eq!(evidence.evidence_grade, NetworkEvidenceGrade::C);
    assert!(!evidence.exact_url_available);
    assert!(!evidence.decrypted_payload_available);
}

#[test]
fn domain_normalization_deduplicates_longest_public_suffix_rules() {
    let model = PublicSuffixModel::from_suffixes(&["uk", "co.uk", "co.uk"])
        .expect_value("suffix model should parse");

    let evidence = normalize_domain_with_public_suffix("shop.example.co.uk", &model)
        .expect_value("domain evidence should normalize");

    assert_eq!(evidence.public_suffix.as_deref(), Some("co.uk"));
    assert_eq!(
        evidence.registrable_domain.as_deref(),
        Some("example.co.uk")
    );
}

#[test]
fn domain_normalization_rejects_empty_or_malformed_domains() {
    let model = PublicSuffixModel::ocentra_fixture();

    assert_eq!(
        normalize_domain_with_public_suffix(" . ", &model),
        Err(DomainNormalizationError::EmptyDomain)
    );
    assert_eq!(
        normalize_domain_with_public_suffix("bad..example.test", &model),
        Err(DomainNormalizationError::EmptyLabel)
    );
    assert_eq!(
        normalize_domain_with_public_suffix("-bad.example.test", &model),
        Err(DomainNormalizationError::LabelStartsOrEndsWithHyphen)
    );
    assert_eq!(
        normalize_domain_with_public_suffix(TOO_LONG_LABEL, &model),
        Err(DomainNormalizationError::LabelTooLong)
    );
    assert!(matches!(
        normalize_domain_with_public_suffix("bad_underscore.example.test", &model),
        Err(DomainNormalizationError::InvalidLabelCharacter {
            label,
            character: '_'
        }) if label == "bad_underscore"
    ));
}
