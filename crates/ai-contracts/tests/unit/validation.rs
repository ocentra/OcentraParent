use ocentra_ai_contracts::ai_contracts::identity::{AiFamilyId, AiSchemaVersion};
use ocentra_ai_contracts::ai_contracts::{AiConfidence, AI_CONTRACT_SCHEMA_VERSION};

#[test]
fn confidence_parser_accepts_only_finite_unit_interval_values() {
    assert_eq!(
        AiConfidence::parse(0.0).map(|confidence| confidence.as_f64()),
        Some(0.0)
    );
    assert_eq!(
        AiConfidence::parse(1.0).map(|confidence| confidence.as_f64()),
        Some(1.0)
    );
    assert_eq!(
        AiConfidence::parse(-0.01).map(|confidence| confidence.as_f64()),
        None
    );
    assert_eq!(
        AiConfidence::parse(1.01).map(|confidence| confidence.as_f64()),
        None
    );
    assert_eq!(
        AiConfidence::parse(f64::NAN).map(|confidence| confidence.as_f64()),
        None
    );
}

#[test]
fn schema_and_family_identifiers_enforce_current_and_trimmed_boundaries() {
    let current = AiSchemaVersion::current();
    assert_eq!(current.as_str(), AI_CONTRACT_SCHEMA_VERSION);
    assert!(current.is_current());
    assert_eq!(
        AiSchemaVersion::parse_current(AI_CONTRACT_SCHEMA_VERSION),
        Some(current)
    );
    assert_eq!(AiSchemaVersion::parse_current("ai-contracts-v0"), None);

    let family = AiFamilyId::parse("family-1");
    assert_eq!(family.as_ref().map(AiFamilyId::as_str), Some("family-1"));
    assert_eq!(AiFamilyId::parse(" family-1"), None);
    assert_eq!(AiFamilyId::parse(""), None);
}
