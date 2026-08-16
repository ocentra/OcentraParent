use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::android_physical_target::prove_network_android_physical_target;
use ocentra_network_evidence::android_physical_target::types::{
    NetworkAndroidPhysicalTargetBoundaryReason, NetworkAndroidPhysicalTargetError,
    NetworkAndroidPhysicalTargetExpected, NetworkAndroidPhysicalTargetField,
    NetworkAndroidPhysicalTargetInput, NetworkAndroidPhysicalTargetMismatch,
    NetworkAndroidPhysicalTargetObserved, NetworkAndroidPhysicalTargetState,
    NetworkAndroidPhysicalTargetUnsupportedClaims,
};

#[test]
fn android_physical_target_proves_matching_physical_device_identity_without_live_claims() {
    let proof = prove_network_android_physical_target(valid_input())
        .expect_value("matching physical Android target should prove identity");

    assert_eq!(proof.proof_ref, "android-physical-target-proof-row40a");
    assert_eq!(
        proof.state,
        NetworkAndroidPhysicalTargetState::PhysicalDeviceObserved
    );
    assert_eq!(proof.boundary_reasons, Vec::new());
    assert_eq!(proof.mismatches, Vec::new());
    assert_eq!(proof.expected.serial, "192.168.2.45:5555");
    assert_eq!(proof.expected.product, "star2qltecs");
    assert_eq!(proof.expected.model, "SM_G965W");
    assert_eq!(proof.expected.device, "star2qltecs");
    assert_eq!(proof.expected.android_release, "10");
    assert_eq!(proof.expected.abi, "arm64-v8a");
    assert_eq!(
        proof.evidence_refs,
        vec![
            "adb-connect-proof-ref-row40a",
            "adb-devices-proof-ref-row40a",
            "adb-getprop-proof-ref-row40a"
        ]
    );
    assert!(proof.adb_available);
    assert!(proof.target_connected);
    assert!(proof.read_only_adb_probe_executed);
    assert!(proof.physical_device_identity_proved);
    assert!(!proof.live_vpn_service_executed);
    assert!(!proof.packet_capture_executed);
    assert!(!proof.packet_blocked);
    assert!(!proof.app_package_correlation_claimed);
    assert!(!proof.adapter_authority_claimed);
    assert!(!proof.enforcement_command_published);
    assert!(!proof.production_android_support_claimed);
    assert!(!proof.exact_url_available);
    assert!(!proof.decrypted_payload_available);
    assert!(!proof.page_content_available);
}

#[test]
fn android_physical_target_stays_manual_required_without_connected_target_or_observation() {
    let disconnected = prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
        target_connected: false,
        observed: None,
        ..valid_input()
    })
    .expect_value("disconnected target should stay manual-required");
    assert_eq!(
        disconnected.state,
        NetworkAndroidPhysicalTargetState::ManualRequired
    );
    assert_eq!(
        disconnected.boundary_reasons,
        vec![NetworkAndroidPhysicalTargetBoundaryReason::TargetNotConnected]
    );
    assert!(!disconnected.read_only_adb_probe_executed);
    assert!(!disconnected.physical_device_identity_proved);

    let missing_observation =
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            observed: None,
            ..valid_input()
        })
        .expect_value("missing getprop observation should stay manual-required");
    assert_eq!(
        missing_observation.state,
        NetworkAndroidPhysicalTargetState::ManualRequired
    );
    assert_eq!(
        missing_observation.boundary_reasons,
        vec![NetworkAndroidPhysicalTargetBoundaryReason::ObservationMissing]
    );
    assert!(!missing_observation.physical_device_identity_proved);
}

#[test]
fn android_physical_target_marks_adb_unavailable_without_device_support_claims() {
    let proof = prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
        adb_available: false,
        target_connected: false,
        observed: None,
        ..valid_input()
    })
    .expect_value("missing adb should stay reportable as unavailable");

    assert_eq!(proof.state, NetworkAndroidPhysicalTargetState::Unavailable);
    assert_eq!(
        proof.boundary_reasons,
        vec![NetworkAndroidPhysicalTargetBoundaryReason::AdbUnavailable]
    );
    assert!(!proof.read_only_adb_probe_executed);
    assert!(!proof.physical_device_identity_proved);
    assert!(!proof.production_android_support_claimed);
}

#[test]
fn android_physical_target_records_identity_mismatch_without_upgrading_readiness() {
    let proof = prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
        observed: Some(NetworkAndroidPhysicalTargetObserved {
            model: "SM_G960W".to_owned(),
            android_release: "9".to_owned(),
            ..matching_observation()
        }),
        ..valid_input()
    })
    .expect_value("mismatched target should produce mismatch proof");

    assert_eq!(proof.state, NetworkAndroidPhysicalTargetState::Mismatch);
    assert_eq!(
        proof.boundary_reasons,
        vec![NetworkAndroidPhysicalTargetBoundaryReason::IdentityMismatch]
    );
    assert_eq!(
        proof.mismatches,
        vec![
            NetworkAndroidPhysicalTargetMismatch {
                field: NetworkAndroidPhysicalTargetField::Model,
                expected: "SM_G965W".to_owned(),
                observed: "SM_G960W".to_owned(),
            },
            NetworkAndroidPhysicalTargetMismatch {
                field: NetworkAndroidPhysicalTargetField::AndroidRelease,
                expected: "10".to_owned(),
                observed: "9".to_owned(),
            }
        ]
    );
    assert!(!proof.physical_device_identity_proved);
    assert!(!proof.live_vpn_service_executed);
}

#[test]
fn android_physical_target_rejects_content_live_capture_and_enforcement_claims() {
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            unsupported_claims: NetworkAndroidPhysicalTargetUnsupportedClaims {
                exact_url_claimed: true,
                ..no_unsupported_claims()
            },
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::ExactUrlClaimRejected)
    );
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            unsupported_claims: NetworkAndroidPhysicalTargetUnsupportedClaims {
                decrypted_payload_claimed: true,
                ..no_unsupported_claims()
            },
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::DecryptedPayloadClaimRejected)
    );
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            unsupported_claims: NetworkAndroidPhysicalTargetUnsupportedClaims {
                page_content_claimed: true,
                ..no_unsupported_claims()
            },
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::PageContentClaimRejected)
    );
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            unsupported_claims: NetworkAndroidPhysicalTargetUnsupportedClaims {
                live_vpn_service_execution_claimed: true,
                ..no_unsupported_claims()
            },
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::LiveVpnServiceExecutionClaimRejected)
    );
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            unsupported_claims: NetworkAndroidPhysicalTargetUnsupportedClaims {
                packet_capture_claimed: true,
                ..no_unsupported_claims()
            },
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::PacketCaptureClaimRejected)
    );
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            unsupported_claims: NetworkAndroidPhysicalTargetUnsupportedClaims {
                enforcement_command_claimed: true,
                ..no_unsupported_claims()
            },
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::EnforcementCommandClaimRejected)
    );
}

#[test]
fn android_physical_target_rejects_emulator_product_support_and_adapter_authority() {
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            unsupported_claims: NetworkAndroidPhysicalTargetUnsupportedClaims {
                emulator_only_product_support_claimed: true,
                ..no_unsupported_claims()
            },
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::EmulatorOnlyProductSupportClaimRejected)
    );
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            unsupported_claims: NetworkAndroidPhysicalTargetUnsupportedClaims {
                packet_block_claimed: true,
                ..no_unsupported_claims()
            },
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::PacketBlockClaimRejected)
    );
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            unsupported_claims: NetworkAndroidPhysicalTargetUnsupportedClaims {
                app_package_correlation_claimed: true,
                ..no_unsupported_claims()
            },
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::AppPackageCorrelationClaimRejected)
    );
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            unsupported_claims: NetworkAndroidPhysicalTargetUnsupportedClaims {
                adapter_authority_claimed: true,
                ..no_unsupported_claims()
            },
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::AdapterAuthorityClaimRejected)
    );
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            unsupported_claims: NetworkAndroidPhysicalTargetUnsupportedClaims {
                production_android_support_claimed: true,
                ..no_unsupported_claims()
            },
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::ProductionAndroidSupportClaimRejected)
    );
}

#[test]
fn android_physical_target_rejects_empty_expected_observed_and_evidence_refs() {
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            proof_ref: " ".to_owned(),
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::EmptyProofRef)
    );
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            expected: NetworkAndroidPhysicalTargetExpected {
                model: " ".to_owned(),
                ..expected_target()
            },
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::EmptyExpectedField(
            NetworkAndroidPhysicalTargetField::Model
        ))
    );
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            expected: NetworkAndroidPhysicalTargetExpected {
                evidence_refs: vec![" ".to_owned()],
                ..expected_target()
            },
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::EmptyEvidenceRef)
    );
    assert_eq!(
        prove_network_android_physical_target(NetworkAndroidPhysicalTargetInput {
            observed: Some(NetworkAndroidPhysicalTargetObserved {
                abi: " ".to_owned(),
                ..matching_observation()
            }),
            ..valid_input()
        }),
        Err(NetworkAndroidPhysicalTargetError::EmptyObservedField(
            NetworkAndroidPhysicalTargetField::Abi
        ))
    );
}

fn valid_input() -> NetworkAndroidPhysicalTargetInput {
    NetworkAndroidPhysicalTargetInput {
        proof_ref: " android-physical-target-proof-row40a ".to_owned(),
        adb_available: true,
        target_connected: true,
        expected: expected_target(),
        observed: Some(matching_observation()),
        unsupported_claims: no_unsupported_claims(),
    }
}

fn expected_target() -> NetworkAndroidPhysicalTargetExpected {
    NetworkAndroidPhysicalTargetExpected {
        target_ref: " android-target-ref-sm-g965w ".to_owned(),
        serial: " 192.168.2.45:5555 ".to_owned(),
        product: " star2qltecs ".to_owned(),
        model: " SM_G965W ".to_owned(),
        device: " star2qltecs ".to_owned(),
        android_release: " 10 ".to_owned(),
        abi: " arm64-v8a ".to_owned(),
        adb_connect_command_ref: " adb-connect-proof-ref-row40a ".to_owned(),
        adb_devices_command_ref: " adb-devices-proof-ref-row40a ".to_owned(),
        adb_getprop_command_ref: " adb-getprop-proof-ref-row40a ".to_owned(),
        evidence_refs: vec![
            " adb-connect-proof-ref-row40a ".to_owned(),
            " adb-devices-proof-ref-row40a ".to_owned(),
            " adb-getprop-proof-ref-row40a ".to_owned(),
        ],
    }
}

fn matching_observation() -> NetworkAndroidPhysicalTargetObserved {
    NetworkAndroidPhysicalTargetObserved {
        serial: " 192.168.2.45:5555 ".to_owned(),
        product: " star2qltecs ".to_owned(),
        model: " SM_G965W ".to_owned(),
        device: " star2qltecs ".to_owned(),
        android_release: " 10 ".to_owned(),
        abi: " arm64-v8a ".to_owned(),
    }
}

fn no_unsupported_claims() -> NetworkAndroidPhysicalTargetUnsupportedClaims {
    NetworkAndroidPhysicalTargetUnsupportedClaims {
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        emulator_only_product_support_claimed: false,
        live_vpn_service_execution_claimed: false,
        packet_capture_claimed: false,
        packet_block_claimed: false,
        app_package_correlation_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
        production_android_support_claimed: false,
    }
}
