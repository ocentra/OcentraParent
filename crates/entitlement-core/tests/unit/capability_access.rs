use ocentra_entitlement_core::entitlement_access::{
    evaluate_entitlement_capability, EntitlementCapability, EntitlementCapabilityAccessState,
    EntitlementCapabilityInput, EntitlementCapabilityRejectionReason, EntitlementManualReviewState,
};
use serde_json::{json, Value};

#[test]
fn public_untrusted_input_is_blocked_without_verifier_context() {
    let decision = evaluate_entitlement_capability(parse_input("tracking", "local-child-runtime"));

    assert_eq!(decision.capability, EntitlementCapability::Tracking);
    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
    assert_eq!(
        decision.manual_review_state,
        EntitlementManualReviewState::Required
    );
    assert_eq!(
        decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::MissingSignature)
    );
}

#[test]
fn public_deserializer_ignores_untrusted_snapshot_context() {
    let mut wire = input_wire("remote-access", "local-child-runtime");
    wire["snapshot_context"] = json!({
        "signature_state": "trusted",
        "freshness_state": "fresh",
        "household_binding_state": "matched",
        "device_binding_state": "matched",
        "device_trust_state": "present",
        "package_build_state": "valid"
    });

    let decision = evaluate_entitlement_capability(
        serde_json::from_value::<EntitlementCapabilityInput>(wire)
            .expect("untrusted input with context-shaped data decodes"),
    );
    assert_eq!(decision.capability, EntitlementCapability::RemoteAccess);
    assert_eq!(
        decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::MissingSignature)
    );
}

#[test]
fn public_capability_input_rejects_unknown_fields() {
    let mut wire = input_wire("tracking", "local-child-runtime");
    wire["unexpected_field"] = json!(true);
    let error = serde_json::from_value::<EntitlementCapabilityInput>(wire)
        .expect_err("unknown capability input fields are rejected");
    assert!(error.to_string().contains("unknown field"));
    assert!(error.to_string().contains("unexpected_field"));
}

#[test]
fn public_capability_input_requires_all_decision_fields() {
    let mut wire = input_wire("tracking", "local-child-runtime");
    wire.as_object_mut()
        .expect("capability input is an object")
        .remove("policy_state");
    let error = serde_json::from_value::<EntitlementCapabilityInput>(wire)
        .expect_err("missing capability input fields are rejected");
    assert!(error.to_string().contains("missing field `policy_state`"));
}

#[test]
fn public_capability_input_cannot_serialize_verifier_context() {
    let input = parse_input("screen-evidence", "parent-portal-only");
    let error = serde_json::to_value(&input)
        .expect_err("verifier-owned context cannot be serialized through public input");
    assert!(error
        .to_string()
        .contains("entitlement snapshot context is verifier-owned"));
}

fn parse_input(capability: &str, scope: &str) -> EntitlementCapabilityInput {
    serde_json::from_value(input_wire(capability, scope)).expect("public capability input decodes")
}

fn input_wire(capability: &str, scope: &str) -> Value {
    json!({
        "capability": capability,
        "subscription_state": "active",
        "offline_grace_state": "inactive",
        "family_setup_state": "complete",
        "policy_state": "clean",
        "capability_scope": scope
    })
}
