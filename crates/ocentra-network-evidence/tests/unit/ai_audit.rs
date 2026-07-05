use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::ai_audit::*;

mod fixtures;

use self::fixtures::{
    audit_input, detection_results, signature_detection_case, unknown_detection_case,
    update_detection_case,
};

#[test]
fn ai_audit_report_generates_parent_readable_narrative_with_cited_refs() {
    let report = build_network_ai_audit_report(&audit_input(detection_results(vec![
        signature_detection_case(),
    ])))
    .expect_value("high-risk detection should produce a parent-readable audit");

    assert_eq!(report.narrative_state, NetworkAiAuditNarrativeState::Ready);
    assert_eq!(
        report.narrative_headline,
        "Network AI audit recommends parent review for cited high-risk network detections."
    );
    assert_eq!(report.cited_detection_refs, vec!["detect-signature-1"]);
    assert_eq!(
        report.cited_evidence_refs,
        vec!["evidence-detect-signature-1"]
    );
    assert_eq!(
        report.cited_analyzer_alert_refs,
        vec!["analyzer-alert-critical"]
    );
    assert_eq!(
        report.cited_parent_rule_refs,
        vec!["parent-rule-network-review"]
    );
    assert_eq!(report.recommendations.len(), 2);
    assert_eq!(
        report.recommendations[0].kind,
        NetworkAiAuditRecommendationKind::ReviewWithParent
    );
    assert_eq!(
        report.recommendations[1].kind,
        NetworkAiAuditRecommendationKind::ReviewPolicyRule
    );
    assert!(report.recommendations.iter().all(|recommendation| {
        recommendation.advisory_only
            && !recommendation.policy_authority
            && !recommendation.adapter_authority
            && !recommendation.enforcement_command_published
            && recommendation.cited_evidence_refs == report.cited_evidence_refs
    }));
    assert!(report.parent_readable);
    assert!(report.advisory_only);
    assert!(!report.remote_ai_used);
    assert!(!report.policy_authority);
    assert!(!report.adapter_authority);
    assert_eq!(report.enforcement_commands_published, 0);
}

#[test]
fn ai_audit_report_recommends_confirmation_for_uncertain_detection() {
    let report = build_network_ai_audit_report(&audit_input(detection_results(vec![
        unknown_detection_case(),
    ])))
    .expect_value("uncertain detection should produce advisory confirmation");

    assert_eq!(
        report.narrative_state,
        NetworkAiAuditNarrativeState::UncertainReviewRequired
    );
    assert_eq!(
        report.narrative_headline,
        "Network AI audit found uncertainty and recommends evidence confirmation before policy action."
    );
    assert!(report
        .uncertainty_codes
        .contains(&NetworkAiAuditUncertaintyCode::UnknownPrediction));
    assert!(report
        .uncertainty_codes
        .contains(&NetworkAiAuditUncertaintyCode::LowConfidence));
    assert_eq!(report.recommendations.len(), 2);
    assert_eq!(
        report.recommendations[0].kind,
        NetworkAiAuditRecommendationKind::ConfirmWithManagedBrowser
    );
    assert_eq!(
        report.recommendations[1].kind,
        NetworkAiAuditRecommendationKind::ConfirmWithScreenSummary
    );
    assert!(!report.exact_url_available);
    assert!(!report.page_content_available);
    assert!(!report.decrypted_payload_available);
}

#[test]
fn ai_audit_report_uses_monitor_only_for_non_high_risk_cited_detection() {
    let report = build_network_ai_audit_report(&audit_input(detection_results(vec![
        update_detection_case(),
    ])))
    .expect_value("benign cited detection should produce monitor-only audit");

    assert_eq!(
        report.narrative_state,
        NetworkAiAuditNarrativeState::MonitorOnly
    );
    assert_eq!(report.recommendations.len(), 1);
    assert_eq!(
        report.recommendations[0].kind,
        NetworkAiAuditRecommendationKind::MonitorOnly
    );
    assert_eq!(report.cited_evidence_refs, vec!["evidence-detect-update-1"]);
    assert!(report.uncertainty_codes.is_empty());
}

#[test]
fn ai_audit_report_rejects_unsupported_content_and_authority_claims() {
    assert_eq!(
        build_network_ai_audit_report(&NetworkAiAuditReportInput {
            remote_ai_claimed: true,
            ..audit_input(detection_results(vec![signature_detection_case()]))
        }),
        Err(NetworkAiAuditReportError::RemoteAiClaimRejected)
    );
    assert_eq!(
        build_network_ai_audit_report(&NetworkAiAuditReportInput {
            exact_url_claimed: true,
            ..audit_input(detection_results(vec![signature_detection_case()]))
        }),
        Err(NetworkAiAuditReportError::ExactUrlClaimRejected)
    );
    assert_eq!(
        build_network_ai_audit_report(&NetworkAiAuditReportInput {
            private_message_claimed: true,
            ..audit_input(detection_results(vec![signature_detection_case()]))
        }),
        Err(NetworkAiAuditReportError::PrivateMessageClaimRejected)
    );
    assert_eq!(
        build_network_ai_audit_report(&NetworkAiAuditReportInput {
            policy_authority_claimed: true,
            ..audit_input(detection_results(vec![signature_detection_case()]))
        }),
        Err(NetworkAiAuditReportError::PolicyAuthorityClaimRejected)
    );
    let mut detection = detection_results(vec![signature_detection_case()])
        .pop()
        .expect_value("test detection exists");
    detection.adapter_authority = true;
    assert_eq!(
        build_network_ai_audit_report(&audit_input(vec![detection])),
        Err(NetworkAiAuditReportError::AdapterAuthorityClaimRejected)
    );
}

#[test]
fn ai_audit_report_rejects_missing_citations_and_duplicate_detections() {
    assert_eq!(
        build_network_ai_audit_report(&NetworkAiAuditReportInput {
            audit_report_ref: " ".to_owned(),
            ..audit_input(detection_results(vec![signature_detection_case()]))
        }),
        Err(NetworkAiAuditReportError::EmptyAuditReportRef)
    );
    assert_eq!(
        build_network_ai_audit_report(&NetworkAiAuditReportInput {
            detection_results: vec![],
            ..audit_input(detection_results(vec![signature_detection_case()]))
        }),
        Err(NetworkAiAuditReportError::EmptyDetectionResults)
    );
    assert_eq!(
        build_network_ai_audit_report(&NetworkAiAuditReportInput {
            parent_rule_refs: vec![],
            ..audit_input(detection_results(vec![signature_detection_case()]))
        }),
        Err(NetworkAiAuditReportError::EmptyParentRuleRefs)
    );

    let duplicate = detection_results(vec![signature_detection_case()]);
    assert_eq!(
        build_network_ai_audit_report(&audit_input(vec![
            duplicate[0].clone(),
            duplicate[0].clone()
        ])),
        Err(NetworkAiAuditReportError::DuplicateDetectionRef)
    );
}
