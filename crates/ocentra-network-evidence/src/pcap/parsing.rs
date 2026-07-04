#[path = "endian.rs"]
mod endian;
#[path = "records.rs"]
mod records;

use super::{
    PcapEndian, PcapPacket, PcapReplayError, PCAP_GLOBAL_HEADER_LEN, PCAP_PACKET_HEADER_LEN,
};

pub(super) fn parse_endian(bytes: &[u8]) -> Result<PcapEndian, PcapReplayError> {
    endian::parse_endian(bytes)
}

pub(super) fn parse_packet_records(
    bytes: &[u8],
    endian: PcapEndian,
    snaplen: u32,
) -> Result<Vec<PcapPacket>, PcapReplayError> {
    records::parse_packet_records(bytes, endian, snaplen)
}

pub(super) fn read_u16(bytes: &[u8], endian: PcapEndian) -> u16 {
    endian::read_u16(bytes, endian)
}

pub(super) fn read_u32(bytes: &[u8], endian: PcapEndian) -> u32 {
    endian::read_u32(bytes, endian)
}
