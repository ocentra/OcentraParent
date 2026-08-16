use super::{PcapEndian, PcapReplayError};

pub(super) fn parse_endian(bytes: &[u8]) -> Result<PcapEndian, PcapReplayError> {
    let mut magic = [0_u8; 4];
    magic.copy_from_slice(&bytes[0..4]);
    match magic {
        [0xd4, 0xc3, 0xb2, 0xa1] => Ok(PcapEndian::Little),
        [0xa1, 0xb2, 0xc3, 0xd4] => Ok(PcapEndian::Big),
        _ => Err(PcapReplayError::UnsupportedMagic(magic)),
    }
}

pub(super) fn read_u16(bytes: &[u8], endian: PcapEndian) -> u16 {
    let mut raw = [0_u8; 2];
    raw.copy_from_slice(bytes);
    match endian {
        PcapEndian::Little => u16::from_le_bytes(raw),
        PcapEndian::Big => u16::from_be_bytes(raw),
    }
}

pub(super) fn read_u32(bytes: &[u8], endian: PcapEndian) -> u32 {
    let mut raw = [0_u8; 4];
    raw.copy_from_slice(bytes);
    match endian {
        PcapEndian::Little => u32::from_le_bytes(raw),
        PcapEndian::Big => u32::from_be_bytes(raw),
    }
}
