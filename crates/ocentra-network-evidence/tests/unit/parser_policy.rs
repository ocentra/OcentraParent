use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::dns::types::NetworkEvidenceGrade;
use ocentra_network_evidence::http::parse_http_host;
use ocentra_network_evidence::parser_policy::{
    map_network_parser_evidence_to_policy, NetworkParserEvidence, NetworkParserPolicyHandoffError,
    NetworkParserPolicyHandoffInput,
};
use ocentra_network_evidence::policy::{
    NetworkEvidencePolicyAction, NetworkEvidencePolicyMappingError, NetworkEvidencePolicyMode,
};

const HTTP_REQUEST_WITH_HOST: &[u8] = b"GET / HTTP/1.1\r\nHost: Example.test\r\n\r\n";

#[test]
fn parsed_http_host_evidence_maps_to_parent_review_without_authority() {
    let observation = parse_http_host(HTTP_REQUEST_WITH_HOST)
        .expect_value("valid HTTP host fixture should parse")
        .expect_value("HTTP host fixture should produce metadata evidence");

    assert_eq!(observation.host, "example.test");
    assert!(!observation.exact_url_available);
    assert!(!observation.decrypted_payload_available);

    let mapping = map_network_parser_evidence_to_policy(handoff_input(NetworkParserEvidence {
        evidence_ref: "network-parser.http-host.fixture-1".to_owned(),
        evidence_grade: NetworkEvidenceGrade::C,
        exact_url_available: observation.exact_url_available,
        decrypted_payload_available: observation.decrypted_payload_available,
    }))
    .expect_value("parser evidence should reach the policy handoff");

    assert_eq!(mapping.mode, NetworkEvidencePolicyMode::ParentReview);
    assert_eq!(
        mapping.mapped_action,
        NetworkEvidencePolicyAction::AskParent
    );
    assert_eq!(
        mapping.evidence_refs,
        vec!["network-parser.http-host.fixture-1".to_owned()]
    );
    assert!(!mapping.adapter_action_authorized);
    assert!(!mapping.enforcement_command_authorized);
}

#[test]
fn malformed_http_parser_input_produces_no_policy_evidence() {
    assert_eq!(
        parse_http_host(&[0xff, 0xfe, 0xfd]).expect_value("malformed bytes are handled"),
        None
    );

    assert_eq!(
        map_network_parser_evidence_to_policy(handoff_input(NetworkParserEvidence {
            evidence_ref: " ".to_owned(),
            evidence_grade: NetworkEvidenceGrade::D,
            exact_url_available: false,
            decrypted_payload_available: false,
        })),
        Err(NetworkParserPolicyHandoffError::EmptyParserEvidenceRef)
    );
}

#[test]
fn parser_policy_handoff_rejects_unsupported_content_claims() {
    assert_eq!(
        map_network_parser_evidence_to_policy(handoff_input(NetworkParserEvidence {
            evidence_ref: "network-parser.http-host.fixture-unsupported-url".to_owned(),
            evidence_grade: NetworkEvidenceGrade::B,
            exact_url_available: true,
            decrypted_payload_available: false,
        })),
        Err(NetworkParserPolicyHandoffError::ExactUrlClaimRejected)
    );
    assert_eq!(
        map_network_parser_evidence_to_policy(handoff_input(NetworkParserEvidence {
            evidence_ref: "network-parser.http-host.fixture-unsupported-payload".to_owned(),
            evidence_grade: NetworkEvidenceGrade::B,
            exact_url_available: false,
            decrypted_payload_available: true,
        })),
        Err(NetworkParserPolicyHandoffError::DecryptedPayloadClaimRejected)
    );
}

#[test]
fn parser_policy_handoff_preserves_policy_ref_validation() {
    assert_eq!(
        map_network_parser_evidence_to_policy(NetworkParserPolicyHandoffInput {
            policy_decision_ref: " ".to_owned(),
            ..handoff_input(NetworkParserEvidence {
                evidence_ref: "network-parser.http-host.fixture-2".to_owned(),
                evidence_grade: NetworkEvidenceGrade::B,
                exact_url_available: false,
                decrypted_payload_available: false,
            })
        }),
        Err(NetworkParserPolicyHandoffError::PolicyMapping(
            NetworkEvidencePolicyMappingError::EmptyPolicyDecisionRef
        ))
    );
}

fn handoff_input(parser_evidence: NetworkParserEvidence) -> NetworkParserPolicyHandoffInput {
    NetworkParserPolicyHandoffInput {
        parser_evidence,
        policy_decision_ref: "policy-decision-parser-1".to_owned(),
        parent_rule_ref: "parent-rule-parser-1".to_owned(),
        local_ai_result_ref: None,
        requested_action: NetworkEvidencePolicyAction::Block,
        adapter_capability_proof_ref: None,
    }
}
