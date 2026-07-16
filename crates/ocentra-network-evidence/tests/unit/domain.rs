use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::domain::*;

#[test]
fn domain_normalization_lowercases_and_trims_root_dot() {
    let model = PublicSuffixModel::ocentra_fixture();

    let evidence = normalize_domain_with_public_suffix("Video.Example.TEST.", &model)
        .expect_value("fixture domain should normalize");

    assert_eq!(evidence.normalized_domain, "video.example.test");
    assert_eq!(
        evidence.labels,
        vec!["video".to_owned(), "example".to_owned(), "test".to_owned()]
    );
    assert_eq!(evidence.public_suffix, Some("test".to_owned()));
    assert_eq!(evidence.registrable_domain, Some("example.test".to_owned()));
    assert_eq!(evidence.evidence_grade, NetworkEvidenceGrade::C);
    assert!(!evidence.exact_url_available);
    assert!(!evidence.decrypted_payload_available);
}

#[test]
fn domain_public_suffix_model_selects_longest_suffix() {
    let model = PublicSuffixModel::ocentra_fixture();

    let evidence = normalize_domain_with_public_suffix("media.child.example.co.uk", &model)
        .expect_value("fixture domain should match longest suffix");

    assert_eq!(evidence.public_suffix, Some("co.uk".to_owned()));
    assert_eq!(
        evidence.registrable_domain,
        Some("example.co.uk".to_owned())
    );
}

#[test]
fn domain_public_suffix_without_private_label_has_no_registrable_domain() {
    let model = PublicSuffixModel::ocentra_fixture();

    let evidence = normalize_domain_with_public_suffix("co.uk", &model)
        .expect_value("public suffix fixture should normalize");

    assert_eq!(evidence.public_suffix, Some("co.uk".to_owned()));
    assert_eq!(evidence.registrable_domain, None);
}

#[test]
fn domain_normalization_rejects_empty_interior_label() {
    let model = PublicSuffixModel::ocentra_fixture();

    let result = normalize_domain_with_public_suffix("bad..example.test", &model);

    assert_eq!(result, Err(DomainNormalizationError::EmptyLabel));
}

#[test]
fn domain_normalization_rejects_invalid_label_character() {
    let model = PublicSuffixModel::ocentra_fixture();

    let result = normalize_domain_with_public_suffix("bad_label.example.test", &model);

    assert_eq!(
        result,
        Err(DomainNormalizationError::InvalidLabelCharacter {
            label: "bad_label".to_owned(),
            character: '_'
        })
    );
}
