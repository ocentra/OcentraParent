use super::super::types::{
    NetworkAndroidPhysicalTargetError, NetworkAndroidPhysicalTargetUnsupportedClaims,
};

pub(super) fn reject_unsupported_claims(
    claims: &NetworkAndroidPhysicalTargetUnsupportedClaims,
) -> Result<(), NetworkAndroidPhysicalTargetError> {
    for (claimed, error) in [
        (
            claims.exact_url_claimed,
            NetworkAndroidPhysicalTargetError::ExactUrlClaimRejected,
        ),
        (
            claims.decrypted_payload_claimed,
            NetworkAndroidPhysicalTargetError::DecryptedPayloadClaimRejected,
        ),
        (
            claims.page_content_claimed,
            NetworkAndroidPhysicalTargetError::PageContentClaimRejected,
        ),
        (
            claims.emulator_only_product_support_claimed,
            NetworkAndroidPhysicalTargetError::EmulatorOnlyProductSupportClaimRejected,
        ),
        (
            claims.live_vpn_service_execution_claimed,
            NetworkAndroidPhysicalTargetError::LiveVpnServiceExecutionClaimRejected,
        ),
        (
            claims.packet_capture_claimed,
            NetworkAndroidPhysicalTargetError::PacketCaptureClaimRejected,
        ),
        (
            claims.packet_block_claimed,
            NetworkAndroidPhysicalTargetError::PacketBlockClaimRejected,
        ),
        (
            claims.app_package_correlation_claimed,
            NetworkAndroidPhysicalTargetError::AppPackageCorrelationClaimRejected,
        ),
        (
            claims.adapter_authority_claimed,
            NetworkAndroidPhysicalTargetError::AdapterAuthorityClaimRejected,
        ),
        (
            claims.enforcement_command_claimed,
            NetworkAndroidPhysicalTargetError::EnforcementCommandClaimRejected,
        ),
        (
            claims.production_android_support_claimed,
            NetworkAndroidPhysicalTargetError::ProductionAndroidSupportClaimRejected,
        ),
    ] {
        if claimed {
            return Err(error);
        }
    }
    Ok(())
}
