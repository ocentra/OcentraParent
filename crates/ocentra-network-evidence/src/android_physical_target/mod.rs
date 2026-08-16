pub mod types;
mod validation;

use types::{
    NetworkAndroidPhysicalTargetBoundaryReason, NetworkAndroidPhysicalTargetError,
    NetworkAndroidPhysicalTargetExpected, NetworkAndroidPhysicalTargetField,
    NetworkAndroidPhysicalTargetInput, NetworkAndroidPhysicalTargetMismatch,
    NetworkAndroidPhysicalTargetObserved, NetworkAndroidPhysicalTargetProof,
    NetworkAndroidPhysicalTargetState,
};
use validation::{normalize_expected, normalize_observed, reject_unsupported_claims};

pub fn prove_network_android_physical_target(
    input: NetworkAndroidPhysicalTargetInput,
) -> Result<NetworkAndroidPhysicalTargetProof, NetworkAndroidPhysicalTargetError> {
    reject_unsupported_claims(&input.unsupported_claims)?;
    let proof_ref = types::normalize_text(&input.proof_ref)
        .ok_or(NetworkAndroidPhysicalTargetError::EmptyProofRef)?;
    let expected = normalize_expected(input.expected)?;
    let observed = input
        .observed
        .as_ref()
        .map(normalize_observed)
        .transpose()?;
    let (state, boundary_reasons, mismatches) = proof_state(
        input.adb_available,
        input.target_connected,
        &expected,
        observed.as_ref(),
    );
    let physical_device_identity_proved =
        state == NetworkAndroidPhysicalTargetState::PhysicalDeviceObserved;

    Ok(NetworkAndroidPhysicalTargetProof {
        proof_ref,
        evidence_refs: expected.evidence_refs.clone(),
        expected,
        observed,
        state,
        boundary_reasons,
        mismatches,
        adb_available: input.adb_available,
        target_connected: input.target_connected,
        read_only_adb_probe_executed: input.adb_available && input.target_connected,
        physical_device_identity_proved,
        live_vpn_service_executed: false,
        packet_capture_executed: false,
        packet_blocked: false,
        app_package_correlation_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_published: false,
        production_android_support_claimed: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
    })
}

fn proof_state(
    adb_available: bool,
    target_connected: bool,
    expected: &NetworkAndroidPhysicalTargetExpected,
    observed: Option<&NetworkAndroidPhysicalTargetObserved>,
) -> (
    NetworkAndroidPhysicalTargetState,
    Vec<NetworkAndroidPhysicalTargetBoundaryReason>,
    Vec<NetworkAndroidPhysicalTargetMismatch>,
) {
    if !adb_available {
        return (
            NetworkAndroidPhysicalTargetState::Unavailable,
            vec![NetworkAndroidPhysicalTargetBoundaryReason::AdbUnavailable],
            Vec::new(),
        );
    }
    if !target_connected {
        return (
            NetworkAndroidPhysicalTargetState::ManualRequired,
            vec![NetworkAndroidPhysicalTargetBoundaryReason::TargetNotConnected],
            Vec::new(),
        );
    }
    let Some(observed) = observed else {
        return (
            NetworkAndroidPhysicalTargetState::ManualRequired,
            vec![NetworkAndroidPhysicalTargetBoundaryReason::ObservationMissing],
            Vec::new(),
        );
    };
    let mismatches = identity_mismatches(expected, observed);
    if mismatches.is_empty() {
        (
            NetworkAndroidPhysicalTargetState::PhysicalDeviceObserved,
            Vec::new(),
            mismatches,
        )
    } else {
        (
            NetworkAndroidPhysicalTargetState::Mismatch,
            vec![NetworkAndroidPhysicalTargetBoundaryReason::IdentityMismatch],
            mismatches,
        )
    }
}

fn identity_mismatches(
    expected: &NetworkAndroidPhysicalTargetExpected,
    observed: &NetworkAndroidPhysicalTargetObserved,
) -> Vec<NetworkAndroidPhysicalTargetMismatch> {
    let mut mismatches = Vec::new();
    push_mismatch(
        &mut mismatches,
        NetworkAndroidPhysicalTargetField::Serial,
        &expected.serial,
        &observed.serial,
    );
    push_mismatch(
        &mut mismatches,
        NetworkAndroidPhysicalTargetField::Product,
        &expected.product,
        &observed.product,
    );
    push_mismatch(
        &mut mismatches,
        NetworkAndroidPhysicalTargetField::Model,
        &expected.model,
        &observed.model,
    );
    push_mismatch(
        &mut mismatches,
        NetworkAndroidPhysicalTargetField::Device,
        &expected.device,
        &observed.device,
    );
    push_mismatch(
        &mut mismatches,
        NetworkAndroidPhysicalTargetField::AndroidRelease,
        &expected.android_release,
        &observed.android_release,
    );
    push_mismatch(
        &mut mismatches,
        NetworkAndroidPhysicalTargetField::Abi,
        &expected.abi,
        &observed.abi,
    );
    mismatches
}

fn push_mismatch(
    mismatches: &mut Vec<NetworkAndroidPhysicalTargetMismatch>,
    field: NetworkAndroidPhysicalTargetField,
    expected: &str,
    observed: &str,
) {
    if expected != observed {
        mismatches.push(NetworkAndroidPhysicalTargetMismatch {
            field,
            expected: expected.to_owned(),
            observed: observed.to_owned(),
        });
    }
}
