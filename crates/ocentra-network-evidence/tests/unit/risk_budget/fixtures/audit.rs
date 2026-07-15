use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::ai_audit::*;
use ocentra_network_evidence::ai_detection::*;

use super::{detection, AuditFixtureCase};

pub(super) fn high_risk_audit_report(fixture: AuditFixtureCase) -> NetworkAiAuditReport {
    audit_report(
        fixture,
        detection::detection_case(
            fixture,
            NetworkAiDetectionLabel::SignatureThreat,
            NetworkAiDetectionLabel::SignatureThreat,
            9_100,
            8_900,
            NetworkAiDetectionRiskLevel::Critical,
            true,
        ),
    )
}

pub(super) fn benign_audit_report(fixture: AuditFixtureCase) -> NetworkAiAuditReport {
    audit_report(
        fixture,
        detection::detection_case(
            fixture,
            NetworkAiDetectionLabel::BenignExpected,
            NetworkAiDetectionLabel::BenignExpected,
            8_200,
            8_100,
            NetworkAiDetectionRiskLevel::Low,
            false,
        ),
    )
}

fn audit_report(
    fixture: AuditFixtureCase,
    detection_case: NetworkAiDetectionFixtureCase,
) -> NetworkAiAuditReport {
    let audit_report_ref = match fixture {
        AuditFixtureCase::Block => "network-ai-audit-row48-block",
        AuditFixtureCase::Manual => "network-ai-audit-row48-manual",
        AuditFixtureCase::Safe => "network-ai-audit-row48-safe",
        AuditFixtureCase::MissingProof => "network-ai-audit-row48-missing",
        AuditFixtureCase::Signature => "network-ai-audit-row48-signature",
        AuditFixtureCase::Unsupported => "network-ai-audit-row48-unsupported",
        AuditFixtureCase::Low => "network-ai-audit-row48-low",
    };

    build_network_ai_audit_report(&NetworkAiAuditReportInput {
        audit_report_ref: audit_report_ref.to_owned(),
        narrative_template_ref: "network-ai-audit-template-row48".to_owned(),
        model_version_ref: "local-ai-model-version-fixture-row48".to_owned(),
        policy_context_ref: "network-policy-context-row48".to_owned(),
        detection_results: detection::detection_results(vec![detection_case]),
        parent_rule_refs: vec!["parent-rule-network-review".to_owned()],
        remote_ai_claimed: false,
        raw_pcap_input_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
    })
    .expect_value("AI audit report should build from detection fixture")
}
