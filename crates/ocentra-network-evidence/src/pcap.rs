mod parsing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PcapPacket {
    pub timestamp_seconds: u32,
    pub timestamp_fraction: u32,
    pub original_len: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PcapReplayError {
    TruncatedGlobalHeader,
    UnsupportedMagic([u8; 4]),
    UnsupportedVersion {
        major: u16,
        minor: u16,
    },
    UnsupportedLinkType(u32),
    PacketLengthExceedsSnaplen {
        packet_len: u32,
        snaplen: u32,
    },
    PacketTooLarge(u32),
    TruncatedPacketHeader {
        offset: usize,
    },
    TruncatedPacketData {
        offset: usize,
        expected: usize,
        actual: usize,
    },
}

#[derive(Clone, Copy)]
enum PcapEndian {
    Little,
    Big,
}

const PCAP_GLOBAL_HEADER_LEN: usize = 24;
const PCAP_PACKET_HEADER_LEN: usize = 16;
const LINKTYPE_ETHERNET: u32 = 1;

pub fn parse_pcap_packets(bytes: &[u8]) -> Result<Vec<PcapPacket>, PcapReplayError> {
    if bytes.len() < PCAP_GLOBAL_HEADER_LEN {
        return Err(PcapReplayError::TruncatedGlobalHeader);
    }

    let endian = parsing::parse_endian(bytes)?;
    let major = parsing::read_u16(&bytes[4..6], endian);
    let minor = parsing::read_u16(&bytes[6..8], endian);
    if major != 2 || minor != 4 {
        return Err(PcapReplayError::UnsupportedVersion { major, minor });
    }

    let snaplen = parsing::read_u32(&bytes[16..20], endian);
    let link_type = parsing::read_u32(&bytes[20..24], endian);
    if link_type != LINKTYPE_ETHERNET {
        return Err(PcapReplayError::UnsupportedLinkType(link_type));
    }

    parsing::parse_packet_records(bytes, endian, snaplen)
}
