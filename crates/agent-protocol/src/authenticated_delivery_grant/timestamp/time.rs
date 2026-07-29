pub(super) fn parse(value: &[u8]) -> Option<(u8, u8, u8, &[u8])> {
    if value.len() < 9 || value.get(2) != Some(&b':') || value.get(5) != Some(&b':') {
        return None;
    }
    let hour = decimal_u8(value.get(..2)?)?;
    let minute = decimal_u8(value.get(3..5)?)?;
    let second = decimal_u8(value.get(6..8)?)?;
    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    let offset_index = offset_index(value)?;
    Some((hour, minute, second, value.get(offset_index..)?))
}

fn offset_index(value: &[u8]) -> Option<usize> {
    if value.get(8) != Some(&b'.') {
        return Some(8);
    }
    let mut index = 9;
    while value.get(index).is_some_and(u8::is_ascii_digit) {
        index += 1;
    }
    (index > 9).then_some(index)
}

fn decimal_u8(value: &[u8]) -> Option<u8> {
    std::str::from_utf8(value).ok()?.parse().ok()
}
