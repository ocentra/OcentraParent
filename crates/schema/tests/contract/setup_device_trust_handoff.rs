use crate::support::{error_or_unreachable, result_or_unreachable};
use ocentra_schema::setup_device_trust_handoff as contracts;
use serde_json::json;

macro_rules! option_or_abort {
    ($value:expr $(,)?) => {
        match $value {
            Some(value) => value,
            None => std::process::abort(),
        }
    };
}

macro_rules! timestamp {
    ($value:expr $(,)?) => {
        option_or_abort!(contracts::SetupDeviceTrustHandoffTimestamp::parse($value))
    };
}

macro_rules! claim_boundary {
    ($value:expr $(,)?) => {
        option_or_abort!(contracts::SetupDeviceTrustHandoffClaimBoundary::parse(
            $value
        ))
    };
}

fn artifact_requirement() -> contracts::SetupDeviceTrustHandoffArtifactRequirement {
    contracts::SetupDeviceTrustHandoffArtifactRequirement {
        requirement_ref: option_or_abort!(
            contracts::SetupDeviceTrustHandoffArtifactRequirementRef::parse(
                "child-package-artifact-required",
            )
        ),
        external_artifact_path: option_or_abort!(
            contracts::SetupDeviceTrustHandoffExternalArtifactPath::parse(
                "output/child-agent-runtime-distribution-plan-proof/02-child-windows-service-package/artifacts/windows/child-agent-service.msi",
            )
        ),
        claim_boundary: claim_boundary!(
            "package artifact path only; no package readiness; no install/runtime readiness",
        ),
    }
}

fn request() -> contracts::SetupDeviceTrustHandoffRequest {
    contracts::SetupDeviceTrustHandoffRequest {
        schema_version: contracts::SETUP_DEVICE_TRUST_HANDOFF_SCHEMA_VERSION.to_string(),
        handoff_id: option_or_abort!(contracts::SetupDeviceTrustHandoffId::parse(
            "handoff-setup-device-trust-1",
        )),
        household_ref: option_or_abort!(contracts::SetupDeviceTrustHandoffHouseholdRef::parse(
            "household-alpha",
        )),
        child_profile_ref: option_or_abort!(
            contracts::SetupDeviceTrustHandoffChildProfileRef::parse("child-profile-alpha")
        ),
        target_device_ref: option_or_abort!(
            contracts::SetupDeviceTrustHandoffTargetDeviceRef::parse("target-device-windows-alpha")
        ),
        setup_session_ref: option_or_abort!(
            contracts::SetupDeviceTrustHandoffSetupSessionRef::parse("setup-session-alpha")
        ),
        trust_bootstrap_ref: option_or_abort!(
            contracts::SetupDeviceTrustHandoffTrustBootstrapRef::parse("trust-bootstrap-alpha")
        ),
        child_package_target_ref: option_or_abort!(
            contracts::SetupDeviceTrustHandoffChildPackageTargetRef::parse(
                "child-package-target-windows-service",
            )
        ),
        platform: contracts::SetupDeviceTrustHandoffPlatform::Windows,
        setup_state: contracts::SetupDeviceTrustHandoffSetupState::TrustBootstrapIssued,
        trust_bootstrap_state:
            contracts::SetupDeviceTrustHandoffTrustBootstrapState::BootstrapIssued,
        artifact_requirement: artifact_requirement(),
        expiry_or_replay_guard_ref: option_or_abort!(
            contracts::SetupDeviceTrustHandoffExpiryOrReplayGuardRef::parse(
                "setup-session-alpha:trust-bootstrap-alpha:nonce-001",
            )
        ),
        requested_at: timestamp!("2026-06-28T17:55:00Z"),
        no_claim: vec![
            contracts::SetupDeviceTrustHandoffNoClaim::ParentBootstrapProof,
            contracts::SetupDeviceTrustHandoffNoClaim::ChildPairingCode,
            contracts::SetupDeviceTrustHandoffNoClaim::PackageReadiness,
            contracts::SetupDeviceTrustHandoffNoClaim::InstallRuntimeReadiness,
            contracts::SetupDeviceTrustHandoffNoClaim::ParentClientParity,
        ],
    }
}

fn response() -> contracts::SetupDeviceTrustHandoffResponse {
    let request = request();
    contracts::SetupDeviceTrustHandoffResponse {
        schema_version: contracts::SETUP_DEVICE_TRUST_HANDOFF_SCHEMA_VERSION.to_string(),
        handoff_id: request.handoff_id,
        household_ref: request.household_ref,
        child_profile_ref: request.child_profile_ref,
        target_device_ref: request.target_device_ref,
        setup_session_ref: request.setup_session_ref,
        trust_bootstrap_ref: request.trust_bootstrap_ref,
        child_package_target_ref: request.child_package_target_ref,
        platform: request.platform,
        setup_state: contracts::SetupDeviceTrustHandoffSetupState::TrustBootstrapIssued,
        trust_bootstrap_state:
            contracts::SetupDeviceTrustHandoffTrustBootstrapState::BootstrapBoundToDevice,
        artifact_requirement: request.artifact_requirement,
        install_precondition_state:
            contracts::SetupDeviceTrustHandoffInstallPreconditionState::ArtifactProofRequired,
        manual_required_state: contracts::SetupDeviceTrustHandoffManualRequiredState::ParentAction,
        expiry_or_replay_guard_ref: request.expiry_or_replay_guard_ref,
        handoff_status: contracts::SetupDeviceTrustHandoffStatus::ReadyForChildPackageDistribution,
        no_claim: vec![
            contracts::SetupDeviceTrustHandoffNoClaim::PackageArtifactProof,
            contracts::SetupDeviceTrustHandoffNoClaim::PackageReadiness,
            contracts::SetupDeviceTrustHandoffNoClaim::InstallRuntimeReadiness,
            contracts::SetupDeviceTrustHandoffNoClaim::ServiceHealthProof,
            contracts::SetupDeviceTrustHandoffNoClaim::ParentClientParity,
        ],
        updated_at: timestamp!("2026-06-28T18:00:00Z"),
    }
}

fn route_sync() -> Vec<contracts::SetupDeviceTrustHandoffRouteSyncRequirement> {
    vec![
        contracts::SetupDeviceTrustHandoffRouteSyncRequirement {
            plan: contracts::SetupDeviceTrustHandoffRouteSyncPlan::SetupInstallProvisioningPlan,
            status: contracts::SetupDeviceTrustHandoffRouteSyncStatus::NamedExternalOwner,
            claim_boundary: claim_boundary!(
                "setup journey success stays in setup-install-provisioning-plan",
            ),
        },
        contracts::SetupDeviceTrustHandoffRouteSyncRequirement {
            plan: contracts::SetupDeviceTrustHandoffRouteSyncPlan::DeviceTrustBootstrapPlan,
            status: contracts::SetupDeviceTrustHandoffRouteSyncStatus::RequiredForRuntimeProof,
            claim_boundary: claim_boundary!(
                "trusted-device bootstrap ownership stays in device-trust-bootstrap-plan",
            ),
        },
    ]
}

fn proof() -> contracts::SetupDeviceTrustHandoffContractProof {
    contracts::SetupDeviceTrustHandoffContractProof {
        schema_version: contracts::SETUP_DEVICE_TRUST_HANDOFF_SCHEMA_VERSION.to_string(),
        request: request(),
        response: response(),
        route_sync: route_sync(),
        updated_at: timestamp!("2026-06-28T18:00:00Z"),
    }
}

#[test]
fn setup_device_trust_handoff_contract_round_trips_through_rust_owned_shape() {
    let proof = proof();
    let encoded = result_or_unreachable(
        serde_json::to_value(&proof),
        crate::assert_context!("proof serializes"),
    );

    assert_eq!(
        encoded["schemaVersion"],
        json!(contracts::SETUP_DEVICE_TRUST_HANDOFF_SCHEMA_VERSION)
    );
    assert_eq!(
        encoded["request"]["childPackageTargetRef"],
        json!("child-package-target-windows-service")
    );
    assert_eq!(
        encoded["request"]["setupState"],
        json!("trust-bootstrap-issued")
    );
    assert_eq!(
        encoded["response"]["installPreconditionState"],
        json!("artifact-proof-required")
    );
    assert_eq!(
        encoded["response"]["manualRequiredState"],
        json!("parent-action-required")
    );
    assert_eq!(
        encoded["response"]["handoffStatus"],
        json!("ready-for-child-package-distribution")
    );
    assert_eq!(
        encoded["response"]["artifactRequirement"]["externalArtifactPath"],
        json!(
            "output/child-agent-runtime-distribution-plan-proof/02-child-windows-service-package/artifacts/windows/child-agent-service.msi"
        )
    );
    assert_eq!(
        encoded["routeSync"][0]["plan"],
        json!("setup-install-provisioning-plan")
    );
    assert!(encoded.get("schema_version").is_none());

    let decoded: contracts::SetupDeviceTrustHandoffContractProof = result_or_unreachable(
        serde_json::from_value(encoded),
        crate::assert_context!("proof deserializes"),
    );
    assert_eq!(decoded, proof);
}

#[test]
fn setup_device_trust_handoff_proof_keeps_no_claim_boundaries_explicit() {
    let proof = proof();

    assert_eq!(
        proof.request.no_claim,
        vec![
            contracts::SetupDeviceTrustHandoffNoClaim::ParentBootstrapProof,
            contracts::SetupDeviceTrustHandoffNoClaim::ChildPairingCode,
            contracts::SetupDeviceTrustHandoffNoClaim::PackageReadiness,
            contracts::SetupDeviceTrustHandoffNoClaim::InstallRuntimeReadiness,
            contracts::SetupDeviceTrustHandoffNoClaim::ParentClientParity,
        ]
    );
    assert_eq!(
        proof.response.no_claim,
        vec![
            contracts::SetupDeviceTrustHandoffNoClaim::PackageArtifactProof,
            contracts::SetupDeviceTrustHandoffNoClaim::PackageReadiness,
            contracts::SetupDeviceTrustHandoffNoClaim::InstallRuntimeReadiness,
            contracts::SetupDeviceTrustHandoffNoClaim::ServiceHealthProof,
            contracts::SetupDeviceTrustHandoffNoClaim::ParentClientParity,
        ]
    );
    assert_eq!(proof.route_sync.len(), 2);
}

#[test]
fn setup_device_trust_handoff_contract_rejects_missing_target_package_field() {
    let mut encoded = result_or_unreachable(
        serde_json::to_value(proof()),
        crate::assert_context!("proof serializes"),
    );
    let response = option_or_abort!(encoded["response"].as_object_mut());
    response.remove("childPackageTargetRef");

    let decoded =
        serde_json::from_value::<contracts::SetupDeviceTrustHandoffContractProof>(encoded);
    let err = error_or_unreachable(
        decoded,
        crate::assert_context!("missing childPackageTargetRef should fail validation"),
    );
    assert_eq!(err.to_string(), "missing field `childPackageTargetRef`");
}
