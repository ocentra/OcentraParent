use crate::support::{ErrorOrUnreachable as _, ValueOrUnreachable as _};
use ocentra_schema::child_ios_entitlement_capability_proof as contracts;
use ocentra_schema::child_ios_entitlement_capability_proof_ts::child_ios_entitlement_capability_proof_contracts_typescript;
use serde_json::json;

#[test]
fn child_ios_entitlement_capability_round_trips_through_rust_owned_shape() {
    let proof = contracts::sample_child_ios_entitlement_capability_read_model();
    let encoded = serde_json::to_value(&proof)
        .value_or_unreachable(crate::assert_context!("proof serializes"));

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::CHILD_IOS_ENTITLEMENT_CAPABILITY_PROOF_SCHEMA_VERSION)
    );
    assert_eq!(encoded["bundleId"], json!("ca.ocentra.parent.agent"));
    assert_eq!(
        encoded["protocolBridgeProof"]["commands"][0],
        json!("child.ios.entitlement.capability.snapshot.get")
    );
    assert_eq!(
        encoded["surfaceProofs"][10]["proofState"],
        json!("device-proof-required")
    );
    assert_eq!(
        encoded["packageLifecycleProofs"][12]["proofState"],
        json!("not-implemented")
    );
    assert_eq!(
        encoded["claimBoundaries"]["capabilityOnlyState"],
        json!(
            "iOS child runtime remains capability-only; no hidden daemon or persistent background service is claimed"
        )
    );
    assert!(encoded.get("schema_version").is_none());

    let decoded: contracts::ChildIosEntitlementCapabilityReadModel =
        serde_json::from_value(encoded)
            .value_or_unreachable(crate::assert_context!("proof deserializes"));
    assert_eq!(decoded, proof);
}

#[test]
fn child_ios_entitlement_capability_keeps_manual_and_no_claim_boundaries_explicit() {
    let proof = contracts::sample_child_ios_entitlement_capability_read_model();

    assert_eq!(proof.surface_proofs.len(), 15);
    assert_eq!(
        proof.surface_proofs[3].parent_capability_status.as_str(),
        "manual-required"
    );
    assert_eq!(
        proof.surface_proofs[14].parent_capability_status.as_str(),
        "planned"
    );
    assert_eq!(
        proof.package_lifecycle_proofs[6].proof_state.as_str(),
        "manual-required"
    );
    assert_eq!(
        proof.package_lifecycle_proofs[12].proof_state.as_str(),
        "not-implemented"
    );
    assert_eq!(
        proof.claim_boundaries.recovery_behavior.as_str(),
        "launch recovery remains not-implemented; no iOS daemon, relaunch, or persistent background recovery is claimed"
    );
}

#[test]
fn child_ios_entitlement_capability_rejects_missing_surface_field() {
    let mut encoded =
        serde_json::to_value(contracts::sample_child_ios_entitlement_capability_read_model())
            .value_or_unreachable(crate::assert_context!("proof serializes"));
    encoded["surfaceProofs"][2]
        .as_object_mut()
        .value_or_unreachable(crate::assert_context!("surface object"))
        .remove("runtimeOwner");

    let decoded =
        serde_json::from_value::<contracts::ChildIosEntitlementCapabilityReadModel>(encoded);
    assert_eq!(
        decoded
            .error_or_unreachable(crate::assert_context!("missing runtimeOwner should fail"))
            .to_string(),
        "missing field `runtimeOwner`"
    );
}

#[test]
fn generated_child_ios_entitlement_contracts_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-child-ios-entitlement-capability-proof-contracts.ts"
    );
    let generated = child_ios_entitlement_capability_proof_contracts_typescript();
    let generated_lines: Vec<&str> = generated.lines().collect();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated_lines.first().copied(),
        Some("/* generated from crates/schema/src/child_ios_entitlement_capability_proof.rs */")
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| {
                **line == "export interface GeneratedChildIosEntitlementCapabilityReadModelShape {"
            })
            .count(),
        1
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line == "export interface GeneratedChildIosEntitlementSurfaceProof {")
            .count(),
        1
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| { **line == "export type GeneratedChildIosEntitlementProofState =" })
            .count(),
        1
    );
    assert_eq!(
        generated_lines
            .iter()
            .filter(|line| **line == "export const GeneratedChildIosEntitlementSurfaceNames = [")
            .count(),
        1
    );
}
