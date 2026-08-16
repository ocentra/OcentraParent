use super::authenticated_delivery_grant_fixture::executable_conflict_decision;
use super::authenticated_delivery_grant_provenance::{
    household_authority_proof, resolved_decision, ProvenanceFixture,
};
use super::TestResult;
use ocentra_family_identity_core::household_authority_proof::HouseholdAuthorityProofSigner;
use ocentra_policy_control_core::authenticated_delivery_grant::authority::{
    AuthenticatedDeliveryGrantAuthoritySigner, AuthenticatedDeliveryGrantAuthorityVerifier,
    SignedAuthorityBindings,
};
use ocentra_policy_control_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantIssuanceError;
use ocentra_policy_control_core::policy_authority_resolved_decision::ResolvedPolicyDecision;
use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAssertionSnapshot, AuthenticatedDeliveryGrantCapabilityAssertion,
    AuthenticatedDeliveryGrantEvidenceAssertion, AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
};

#[test]
fn authority_envelope_bounds_resolved_policy_identity_before_signing_or_verification() -> TestResult
{
    let fixture = ProvenanceFixture::new();
    let signer = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let household_authority = HouseholdAuthorityProofSigner::from_platform_key([6; 32]);
    let assertions = AuthenticatedDeliveryGrantAssertionSnapshot {
        capability: AuthenticatedDeliveryGrantCapabilityAssertion::Available,
        evidence: AuthenticatedDeliveryGrantEvidenceAssertion::Stable,
    };

    let valid = test_ok!(
        signer.sign(
            fixture.bindings.clone(),
            assertions.clone(),
            household_authority_proof(fixture.authority),
            resolved_decision(&fixture.bindings, fixture.decision),
            fixture.contract_authority.clone(),
        ),
        "bounded authority envelope signs"
    );
    let verifier = AuthenticatedDeliveryGrantAuthorityVerifier::new(
        signer.verifying_key(),
        household_authority.verifying_key(),
    );
    assert!(
        verifier.verify(&valid).is_ok(),
        "a bounded authority envelope remains verifiable"
    );

    let oversized_decision = test_ok!(
        ResolvedPolicyDecision::for_delivery_grant(
            "x".repeat(AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES + 1),
            fixture.bindings.policy_decision_id.clone(),
            fixture.decision,
            executable_conflict_decision(),
        ),
        "oversized policy identity remains syntactically deserializable"
    );
    assert_eq!(
        signer.sign(
            fixture.bindings.clone(),
            assertions,
            household_authority_proof(fixture.authority),
            oversized_decision,
            fixture.contract_authority,
        ),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected),
        "the signer rejects an oversized policy identity before serializing signing bytes"
    );

    let mut untrusted_wire = serde_json::to_value(valid)?;
    untrusted_wire["resolved_policy_decision"]["aggregate_id"] =
        serde_json::Value::String("x".repeat(AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES + 1));
    let oversized_from_wire: SignedAuthorityBindings = serde_json::from_value(untrusted_wire)?;
    assert_eq!(
        verifier.verify(&oversized_from_wire),
        Err(AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected),
        "the public verifier rejects a deserialized oversized policy identity before signing-byte serialization"
    );
    Ok(())
}
