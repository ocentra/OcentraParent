pub(super) const DNS_HEADER_LEN: usize = 12;
pub(super) const DNS_RESPONSE_FLAG: u16 = 0x8000;
pub(super) const DNS_POINTER_MASK: u8 = 0b1100_0000;
pub(super) const DNS_POINTER_VALUE: u8 = 0b1100_0000;
pub(super) const DNS_MAX_POINTER_JUMPS: usize = 8;
pub(super) const DNS_TYPE_A: u16 = 1;
pub(super) const DNS_TYPE_AAAA: u16 = 28;
pub(super) const IPV4_RDATA_LEN: usize = 4;
