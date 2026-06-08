use super::types::{
    normalize_text, NetworkAndroidPhysicalTargetError, NetworkAndroidPhysicalTargetExpected,
    NetworkAndroidPhysicalTargetField, NetworkAndroidPhysicalTargetObserved,
    NetworkAndroidPhysicalTargetUnsupportedClaims,
};

pub(super) fn normalize_expected(
    expected: NetworkAndroidPhysicalTargetExpected,
) -> Result<NetworkAndroidPhysicalTargetExpected, NetworkAndroidPhysicalTargetError> {
    Ok(NetworkAndroidPhysicalTargetExpected {
        target_ref: expected_text(
            expected.target_ref,
            NetworkAndroidPhysicalTargetField::TargetRef,
        )?,
        serial: expected_text(expected.serial, NetworkAndroidPhysicalTargetField::Serial)?,
        product: expected_text(expected.product, NetworkAndroidPhysicalTargetField::Product)?,
        model: expected_text(expected.model, NetworkAndroidPhysicalTargetField::Model)?,
        device: expected_text(expected.device, NetworkAndroidPhysicalTargetField::Device)?,
        android_release: expected_text(
            expected.android_release,
            NetworkAndroidPhysicalTargetField::AndroidRelease,
        )?,
        abi: expected_text(expected.abi, NetworkAndroidPhysicalTargetField::Abi)?,
        adb_connect_command_ref: expected_text(
            expected.adb_connect_command_ref,
            NetworkAndroidPhysicalTargetField::AdbConnectCommandRef,
        )?,
        adb_devices_command_ref: expected_text(
            expected.adb_devices_command_ref,
            NetworkAndroidPhysicalTargetField::AdbDevicesCommandRef,
        )?,
        adb_getprop_command_ref: expected_text(
            expected.adb_getprop_command_ref,
            NetworkAndroidPhysicalTargetField::AdbGetpropCommandRef,
        )?,
        evidence_refs: normalize_evidence_refs(expected.evidence_refs)?,
    })
}

pub(super) fn normalize_observed(
    observed: NetworkAndroidPhysicalTargetObserved,
) -> Result<NetworkAndroidPhysicalTargetObserved, NetworkAndroidPhysicalTargetError> {
    Ok(NetworkAndroidPhysicalTargetObserved {
        serial: observed_text(observed.serial, NetworkAndroidPhysicalTargetField::Serial)?,
        product: observed_text(observed.product, NetworkAndroidPhysicalTargetField::Product)?,
        model: observed_text(observed.model, NetworkAndroidPhysicalTargetField::Model)?,
        device: observed_text(observed.device, NetworkAndroidPhysicalTargetField::Device)?,
        android_release: observed_text(
            observed.android_release,
            NetworkAndroidPhysicalTargetField::AndroidRelease,
        )?,
        abi: observed_text(observed.abi, NetworkAndroidPhysicalTargetField::Abi)?,
    })
}

pub(super) fn reject_unsupported_claims(
    claims: &NetworkAndroidPhysicalTargetUnsupportedClaims,
) -> Result<(), NetworkAndroidPhysicalTargetError> {
    if claims.exact_url_claimed {
        return Err(NetworkAndroidPhysicalTargetError::ExactUrlClaimRejected);
    }
    if claims.decrypted_payload_claimed {
        return Err(NetworkAndroidPhysicalTargetError::DecryptedPayloadClaimRejected);
    }
    if claims.page_content_claimed {
        return Err(NetworkAndroidPhysicalTargetError::PageContentClaimRejected);
    }
    if claims.emulator_only_product_support_claimed {
        return Err(NetworkAndroidPhysicalTargetError::EmulatorOnlyProductSupportClaimRejected);
    }
    if claims.live_vpn_service_execution_claimed {
        return Err(NetworkAndroidPhysicalTargetError::LiveVpnServiceExecutionClaimRejected);
    }
    if claims.packet_capture_claimed {
        return Err(NetworkAndroidPhysicalTargetError::PacketCaptureClaimRejected);
    }
    if claims.packet_block_claimed {
        return Err(NetworkAndroidPhysicalTargetError::PacketBlockClaimRejected);
    }
    if claims.app_package_correlation_claimed {
        return Err(NetworkAndroidPhysicalTargetError::AppPackageCorrelationClaimRejected);
    }
    if claims.adapter_authority_claimed {
        return Err(NetworkAndroidPhysicalTargetError::AdapterAuthorityClaimRejected);
    }
    if claims.enforcement_command_claimed {
        return Err(NetworkAndroidPhysicalTargetError::EnforcementCommandClaimRejected);
    }
    if claims.production_android_support_claimed {
        return Err(NetworkAndroidPhysicalTargetError::ProductionAndroidSupportClaimRejected);
    }
    Ok(())
}

fn normalize_evidence_refs(
    refs: Vec<String>,
) -> Result<Vec<String>, NetworkAndroidPhysicalTargetError> {
    let mut normalized = Vec::new();
    for value in refs {
        normalized.push(
            normalize_text(&value).ok_or(NetworkAndroidPhysicalTargetError::EmptyEvidenceRef)?,
        );
    }
    if normalized.is_empty() {
        return Err(NetworkAndroidPhysicalTargetError::EmptyEvidenceRef);
    }
    Ok(normalized)
}

fn expected_text(
    value: String,
    field: NetworkAndroidPhysicalTargetField,
) -> Result<String, NetworkAndroidPhysicalTargetError> {
    normalize_text(&value).ok_or(NetworkAndroidPhysicalTargetError::EmptyExpectedField(field))
}

fn observed_text(
    value: String,
    field: NetworkAndroidPhysicalTargetField,
) -> Result<String, NetworkAndroidPhysicalTargetError> {
    normalize_text(&value).ok_or(NetworkAndroidPhysicalTargetError::EmptyObservedField(field))
}
