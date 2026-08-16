use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;

pub(super) fn device_ref(
    paired_device_id: LanPairingText,
    platform: LanPairingText,
) -> LanPairingDeviceRef {
    LanPairingDeviceRef::new(
        paired_device_id.clone(),
        LanPairingOptionalText(None),
        paired_device_id,
        platform,
    )
}
