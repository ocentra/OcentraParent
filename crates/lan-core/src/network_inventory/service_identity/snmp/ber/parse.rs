pub(super) fn parse_ber_tlv(payload: &[u8], offset: usize) -> Option<(u8, &[u8], usize)> {
    let tag = *payload.get(offset)?;
    let length_first = *payload.get(offset + 1)?;
    let (length, value_offset) = if (length_first & 0x80) == 0 {
        (usize::from(length_first), offset + 2)
    } else {
        let length_len = usize::from(length_first & 0x7f);
        if length_len == 0 || length_len > std::mem::size_of::<usize>() {
            return None;
        }
        let mut decoded_length = 0_usize;
        for byte in payload.get((offset + 2)..(offset + 2 + length_len))? {
            decoded_length = (decoded_length << 8) | usize::from(*byte);
        }
        (decoded_length, offset + 2 + length_len)
    };
    let value_end = value_offset.checked_add(length)?;
    Some((tag, payload.get(value_offset..value_end)?, value_end))
}

pub(super) fn parse_ber_integer(payload: &[u8]) -> Option<i64> {
    if payload.is_empty() || payload.len() > 8 {
        return None;
    }
    let negative = (payload[0] & 0x80) != 0;
    let mut bytes = [if negative { 0xff } else { 0x00 }; 8];
    let start = 8_usize.checked_sub(payload.len())?;
    bytes[start..].copy_from_slice(payload);
    Some(i64::from_be_bytes(bytes))
}

pub(super) fn parse_ber_oid(payload: &[u8]) -> Option<Vec<u32>> {
    let first = *payload.first()?;
    let mut oid = vec![u32::from(first / 40), u32::from(first % 40)];
    let mut index = 1_usize;
    while index < payload.len() {
        let mut value = 0_u32;
        loop {
            let byte = *payload.get(index)?;
            index += 1;
            value = (value << 7) | u32::from(byte & 0x7f);
            if (byte & 0x80) == 0 {
                break;
            }
        }
        oid.push(value);
    }
    Some(oid)
}
