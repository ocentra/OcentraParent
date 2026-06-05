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

    let endian = parse_endian(bytes)?;
    let major = read_u16(&bytes[4..6], endian);
    let minor = read_u16(&bytes[6..8], endian);
    if major != 2 || minor != 4 {
        return Err(PcapReplayError::UnsupportedVersion { major, minor });
    }

    let snaplen = read_u32(&bytes[16..20], endian);
    let link_type = read_u32(&bytes[20..24], endian);
    if link_type != LINKTYPE_ETHERNET {
        return Err(PcapReplayError::UnsupportedLinkType(link_type));
    }

    let mut offset = PCAP_GLOBAL_HEADER_LEN;
    let mut packets = Vec::new();
    while offset < bytes.len() {
        if bytes.len() - offset < PCAP_PACKET_HEADER_LEN {
            return Err(PcapReplayError::TruncatedPacketHeader { offset });
        }

        let header = &bytes[offset..offset + PCAP_PACKET_HEADER_LEN];
        let timestamp_seconds = read_u32(&header[0..4], endian);
        let timestamp_fraction = read_u32(&header[4..8], endian);
        let included_len = read_u32(&header[8..12], endian);
        let original_len = read_u32(&header[12..16], endian);
        if included_len > snaplen {
            return Err(PcapReplayError::PacketLengthExceedsSnaplen {
                packet_len: included_len,
                snaplen,
            });
        }

        let packet_len = usize::try_from(included_len)
            .map_err(|_| PcapReplayError::PacketTooLarge(included_len))?;
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

fn parse_endian(bytes: &[u8]) -> Result<PcapEndian, PcapReplayError> {
    let mut magic = [0_u8; 4];
    magic.copy_from_slice(&bytes[0..4]);
    match magic {
        [0xd4, 0xc3, 0xb2, 0xa1] => Ok(PcapEndian::Little),
        [0xa1, 0xb2, 0xc3, 0xd4] => Ok(PcapEndian::Big),
        _ => Err(PcapReplayError::UnsupportedMagic(magic)),
    }
}

fn read_u16(bytes: &[u8], endian: PcapEndian) -> u16 {
    let mut raw = [0_u8; 2];
    raw.copy_from_slice(bytes);
    match endian {
        PcapEndian::Little => u16::from_le_bytes(raw),
        PcapEndian::Big => u16::from_be_bytes(raw),
    }
}

fn read_u32(bytes: &[u8], endian: PcapEndian) -> u32 {
    let mut raw = [0_u8; 4];
    raw.copy_from_slice(bytes);
    match endian {
        PcapEndian::Little => u32::from_le_bytes(raw),
        PcapEndian::Big => u32::from_be_bytes(raw),
    }
}
