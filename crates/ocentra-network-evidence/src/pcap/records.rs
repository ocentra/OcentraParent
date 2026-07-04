use super::{
    PcapEndian, PcapPacket, PcapReplayError, PCAP_GLOBAL_HEADER_LEN, PCAP_PACKET_HEADER_LEN,
};

pub(super) fn parse_packet_records(
    bytes: &[u8],
    endian: PcapEndian,
    snaplen: u32,
) -> Result<Vec<PcapPacket>, PcapReplayError> {
    let mut offset = PCAP_GLOBAL_HEADER_LEN;
    let mut packets = Vec::new();
    while offset < bytes.len() {
        if bytes.len() - offset < PCAP_PACKET_HEADER_LEN {
            return Err(PcapReplayError::TruncatedPacketHeader { offset });
        }

        let header = &bytes[offset..offset + PCAP_PACKET_HEADER_LEN];
        let timestamp_seconds = super::read_u32(&header[0..4], endian);
        let timestamp_fraction = super::read_u32(&header[4..8], endian);
        let included_len = super::read_u32(&header[8..12], endian);
        let original_len = super::read_u32(&header[12..16], endian);
        if included_len > snaplen {
            return Err(PcapReplayError::PacketLengthExceedsSnaplen {
                packet_len: included_len,
                snaplen,
            });
        }

        let packet_len = usize::try_from(included_len)
            .map_err(|_error| PcapReplayError::PacketTooLarge(included_len))?;
        let data_start = offset + PCAP_PACKET_HEADER_LEN;
        let data_end = data_start + packet_len;
        if data_end > bytes.len() {
            return Err(PcapReplayError::TruncatedPacketData {
                offset: data_start,
                expected: packet_len,
                actual: bytes.len().saturating_sub(data_start),
            });
        }

        packets.push(PcapPacket {
            timestamp_seconds,
            timestamp_fraction,
            original_len,
            data: bytes[data_start..data_end].to_vec(),
        });
        offset = data_end;
    }

    Ok(packets)
}
