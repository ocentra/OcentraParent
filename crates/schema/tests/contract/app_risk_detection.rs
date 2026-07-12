use crate::support::ValueOrUnreachable as _;
use ocentra_schema::app_risk_detection as contracts;
use ocentra_schema::app_risk_detection_ts::{
    app_risk_detection_contract_rules_typescript, app_risk_detection_contracts_typescript,
};
use serde_json::json;

#[test]
fn app_risk_detection_matrix_round_trips_through_rust_owned_shape() {
    let matrix = contracts::sample_app_risk_detection_matrix();
    let encoded = serde_json::to_value(&matrix)
        .value_or_unreachable(crate::assert_context!("matrix serializes"));

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::APP_RISK_DETECTION_SCHEMA_VERSION)
    );
    assert_eq!(encoded["candidates"][0]["riskSignal"], json!("vpnProxy"));
    assert_eq!(
        encoded["candidates"][6]["sourceKind"],
        json!("localAiDigest")
    );
    assert_eq!(
        encoded["candidates"][7]["parentOverride"]["parentDisplayLabel"],
        json!("Homework AI tool")
    );
    assert!(encoded.get("schema_version").is_none());

    let decoded: contracts::AppRiskDetectionMatrix = serde_json::from_value(encoded)
        .value_or_unreachable(crate::assert_context!("matrix deserializes"));
    assert_eq!(decoded, matrix);
}

#[test]
fn app_risk_detection_sample_matrix_covers_known_heuristic_ai_and_override_paths() {
    let matrix = contracts::sample_app_risk_detection_matrix();

    assert_eq!(matrix.candidates.len(), 8);
    assert!(
        matrix
            .candidates
            .iter()
            .filter(|candidate| candidate.source_kind
                == contracts::AppRiskDetectionSourceKind::KnownCatalog)
            .count()
            == 4
    );
    assert!(matrix.candidates.iter().any(|candidate| {
        candidate.source_kind == contracts::AppRiskDetectionSourceKind::LocalAiDigest
            && candidate.local_ai_digest_ref.is_some()
    }));
    assert!(matrix.candidates.iter().any(|candidate| {
        candidate.source_kind == contracts::AppRiskDetectionSourceKind::ParentOverride
            && candidate.parent_override.is_some()
    }));
    assert!(matrix
        .candidates
        .iter()
        .all(|candidate| candidate.not_direct_enforcement && candidate.no_content_claim));
}

#[test]
fn generated_app_risk_detection_contracts_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-app-riskdetection-contracts.ts"
    );
    let generated = app_risk_detection_contracts_typescript();

    assert_eq!(checked_in, generated);
    assert_generated_app_risk_detection_contracts(crate::contract_text!(&generated));
}

fn assert_generated_app_risk_detection_contracts(generated: crate::support::ContractText<'_>) {
    let generated_lines: Vec<&str> = generated.0.lines().collect();

    assert_eq!(
        generated_lines.first().copied(),
        Some("/* generated from crates/schema/src/app_risk_detection.rs */")
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line == "export interface GeneratedAppRiskDetectionCandidate {")
            .count(),
        1
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line == "export const GeneratedAppRiskDetectionMatrix = {")
            .count(),
        1
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line == "export type GeneratedParentContractSchemaVersion = 'v0.6';")
            .count(),
        1
    );
}

#[test]
fn generated_app_risk_detection_contract_rules_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-app-riskdetection-contract-rules.ts"
    );
    let generated = app_risk_detection_contract_rules_typescript();
    let generated_lines: Vec<&str> = generated.lines().collect();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated_lines.first().copied(),
        Some("/* generated from crates/schema/src/app_risk_detection.rs */")
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| line
                .starts_with("export function appRiskDetectionCandidateIsHonestGenerated("))
            .count(),
        1
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| line
                .starts_with("function appRiskDetectionAiCandidateCitesDigestGenerated("))
            .count(),
        1
    );
}
