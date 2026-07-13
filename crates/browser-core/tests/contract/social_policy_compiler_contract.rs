use ocentra_browser_core::social_policy_compiler_contract::social_policy_compiler_contract_typescript;

#[test]
fn social_policy_compiler_contract_stays_rust_owned_and_replaces_schema_domain_owner() {
    let source = social_policy_compiler_contract_typescript();

    assert_eq!(
        source.lines().next(),
        Some("/* generated from crates/browser-core/src/social_policy_compiler_contract.rs */")
    );
    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/social-policy-compiler';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/policy-compiler';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/social-policy-compiler-values';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("} from './social_policy_compiler_contract_support';")
            .count(),
        1
    );
    assert_eq!(
        source
            .matches("export const SocialParentPolicyCompilerInputSchema = withParser(")
            .count(),
        1
    );
    assert_eq!(
        source
            .matches("export const SocialParentPolicyDecisionCandidateSchema = withParser(")
            .count(),
        1
    );
    assert_eq!(
        source
            .matches(
                "export const decodeSocialParentPolicyCompilerInput = Schema.decodeUnknownSync("
            )
            .count(),
        1
    );
    assert_eq!(
        source
            .matches(
                "export const decodeSocialParentPolicyDecisionCandidate = Schema.decodeUnknownSync("
            )
            .count(),
        1
    );
}
