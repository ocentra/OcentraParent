use super::{NetworkAppleNetworkExtensionGateError, NetworkAppleNetworkExtensionGateInput};

pub(super) fn reject_unsupported_claims(
    input: &NetworkAppleNetworkExtensionGateInput,
) -> Result<(), NetworkAppleNetworkExtensionGateError> {
    for (claimed, error) in [
        (
            input.exact_url_claimed,
            NetworkAppleNetworkExtensionGateError::ExactUrlClaimRejected,
        ),
        (
            input.decrypted_payload_claimed,
            NetworkAppleNetworkExtensionGateError::DecryptedPayloadClaimRejected,
        ),
        (
            input.page_content_claimed,
            NetworkAppleNetworkExtensionGateError::PageContentClaimRejected,
        ),
        (
            input.simulator_only_product_support_claimed,
            NetworkAppleNetworkExtensionGateError::SimulatorOnlyProductSupportClaimRejected,
        ),
        (
            input.live_network_extension_claimed,
            NetworkAppleNetworkExtensionGateError::LiveNetworkExtensionClaimRejected,
        ),
        (
            input.packet_block_claimed,
            NetworkAppleNetworkExtensionGateError::PacketBlockClaimRejected,
        ),
        (
            input.app_level_control_claimed,
            NetworkAppleNetworkExtensionGateError::AppLevelControlClaimRejected,
        ),
    ] {
        if claimed {
            return Err(error);
        }
    }
    Ok(())
}
