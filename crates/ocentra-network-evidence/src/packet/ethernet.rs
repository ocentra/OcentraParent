use super::{EthernetFrameMetadata, PacketParseError, ETHERNET_HEADER_LEN};

pub(super) fn parse_ethernet_frame(
    frame: &[u8],
) -> Result<EthernetFrameMetadata, PacketParseError> {
    if frame.len() < ETHERNET_HEADER_LEN {
        return Err(PacketParseError::EthernetFrameTooShort);
    }

    Ok(EthernetFrameMetadata {
        destination_mac: mac_text(&frame[0..6]),
        source_mac: mac_text(&frame[6..12]),
        ether_type: ether_type(frame),
    })
}

pub(super) fn ether_type(frame: &[u8]) -> u16 {
    if frame.len() < ETHERNET_HEADER_LEN {
        return 0;
    }
    u16::from_be_bytes([frame[12], frame[13]])
}

fn mac_text(bytes: &[u8]) -> String {
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    )
}
