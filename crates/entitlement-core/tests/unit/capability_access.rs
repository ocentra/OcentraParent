use ocentra_entitlement_core::entitlement_access::{
    evaluate_entitlement_capability, EntitlementCapability, EntitlementCapabilityAccessState,
    EntitlementCapabilityInput, EntitlementCapabilityRejectionReason, EntitlementManualReviewState,
};
use serde_json::{json, Value};
use std::error::Error;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

#[test]
fn public_untrusted_input_is_blocked_without_verifier_context() -> TestResult {
    let decision = evaluate_entitlement_capability(parse_input("tracking", "local-child-runtime")?);

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
    Ok(())
}

#[test]
fn public_deserializer_ignores_untrusted_snapshot_context() -> TestResult {
    let mut wire = input_wire("remote-access", "local-child-runtime");
    wire["snapshot_context"] = json!({
        "signature_state": "trusted",
        "freshness_state": "fresh",
        "household_binding_state": "matched",
        "device_binding_state": "matched",
        "device_trust_state": "present",
        "package_build_state": "valid"
    });

    let decision = evaluate_entitlement_capability(serde_json::from_value::<
        EntitlementCapabilityInput,
    >(wire)?);
    assert_eq!(decision.capability, EntitlementCapability::RemoteAccess);
    assert_eq!(
        decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::MissingSignature)
    );
    Ok(())
}

#[test]
fn public_capability_input_rejects_unknown_fields() -> TestResult {
    let mut wire = input_wire("tracking", "local-child-runtime");
    wire["unexpected_field"] = json!(true);
    let error = require_json_error(
        serde_json::from_value::<EntitlementCapabilityInput>(wire),
        "unknown capability input fields must be rejected",
    )?;
    assert!(error.to_string().contains("unknown field"));
    assert!(error.to_string().contains("unexpected_field"));
    Ok(())
}

#[test]
fn public_capability_input_requires_all_decision_fields() -> TestResult {
    let mut wire = input_wire("tracking", "local-child-runtime");
    wire.as_object_mut()
        .ok_or("capability input must be an object")?
        .remove("policy_state");
    let error = require_json_error(
        serde_json::from_value::<EntitlementCapabilityInput>(wire),
        "missing capability input fields must be rejected",
    )?;
    assert!(error.to_string().contains("missing field `policy_state`"));
    Ok(())
}

#[test]
fn public_capability_input_cannot_serialize_verifier_context() -> TestResult {
    let input = parse_input("screen-evidence", "parent-portal-only")?;
    let error = require_json_error(
        serde_json::to_value(input),
        "verifier-owned context must not serialize through public input",
    )?;
    assert!(error
        .to_string()
        .contains("entitlement snapshot context is verifier-owned"));
    Ok(())
}

fn parse_input(capability: &str, scope: &str) -> TestResult<EntitlementCapabilityInput> {
    Ok(serde_json::from_value(input_wire(capability, scope))?)
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

fn require_json_error<T>(
    result: Result<T, serde_json::Error>,
    message: &'static str,
) -> TestResult<serde_json::Error> {
    match result {
        Ok(_) => Err(message.into()),
        Err(error) => Ok(error),
    }
}
