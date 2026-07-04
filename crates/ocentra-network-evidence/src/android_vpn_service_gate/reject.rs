use super::{NetworkAndroidVpnServiceGateError, NetworkAndroidVpnServiceGateInput};

pub(super) fn reject_unsupported_claims(
    input: &NetworkAndroidVpnServiceGateInput,
) -> Result<(), NetworkAndroidVpnServiceGateError> {
    for (claimed, error) in [
        (
            input.exact_url_claimed,
            NetworkAndroidVpnServiceGateError::ExactUrlClaimRejected,
        ),
        (
            input.decrypted_payload_claimed,
            NetworkAndroidVpnServiceGateError::DecryptedPayloadClaimRejected,
        ),
        (
            input.page_content_claimed,
            NetworkAndroidVpnServiceGateError::PageContentClaimRejected,
        ),
        (
            input.emulator_only_product_support_claimed,
            NetworkAndroidVpnServiceGateError::EmulatorOnlyProductSupportClaimRejected,
        ),
        (
            input.live_vpn_tunnel_claimed,
            NetworkAndroidVpnServiceGateError::LiveVpnTunnelClaimRejected,
        ),
        (
            input.packet_block_claimed,
            NetworkAndroidVpnServiceGateError::PacketBlockClaimRejected,
        ),
        (
            input.app_package_correlation_claimed,
            NetworkAndroidVpnServiceGateError::AppPackageCorrelationClaimRejected,
        ),
    ] {
        if claimed {
            return Err(error);
        }
    }
    Ok(())
}
