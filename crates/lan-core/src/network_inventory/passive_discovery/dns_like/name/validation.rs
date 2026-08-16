pub(super) fn pointer_at(payload: &[u8], cursor: usize, label_len: u8) -> Option<Option<usize>> {
    if label_len & 0b1100_0000 != 0b1100_0000 {
        return Some(None);
    }
    let low = *payload.get(cursor + 1)?;
    let pointer = usize::from((u16::from(label_len & 0x3f) << 8) | u16::from(low));
    (pointer < payload.len()).then_some(Some(pointer))
}

pub(super) fn is_valid_label_len(label_len: u8) -> bool {
    label_len <= 63 && label_len & 0b1100_0000 == 0
}
