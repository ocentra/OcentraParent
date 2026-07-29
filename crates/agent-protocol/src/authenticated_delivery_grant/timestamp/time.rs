pub(super) fn parse(value: &[u8]) -> Option<(u8, u8, u8, i128, &[u8])> {
    if value.len() < 9 || value.get(2) != Some(&b':') || value.get(5) != Some(&b':') {
        return None;
    }
    let hour = decimal_u8(value.get(..2)?)?;
    let minute = decimal_u8(value.get(3..5)?)?;
    let second = decimal_u8(value.get(6..8)?)?;
    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    let (fraction, offset_index) = fraction_and_offset_index(value)?;
    Some((hour, minute, second, fraction, value.get(offset_index..)?))
}

fn fraction_and_offset_index(value: &[u8]) -> Option<(i128, usize)> {
    if value.get(8) != Some(&b'.') {
        return Some((0, 8));
    }
    let mut index = 9;
    while value.get(index).is_some_and(u8::is_ascii_digit) {
        index += 1;
    }
    if index == 9 || index > 18 {
        return None;
    }
    let digits = std::str::from_utf8(value.get(9..index)?)
        .ok()?
        .parse::<i128>()
        .ok()?;
    let scale = 10_i128.checked_pow(u32::try_from(index - 9).ok()?)?;
    Some((digits.checked_mul(1_000_000_000)? / scale, index))
}

fn decimal_u8(value: &[u8]) -> Option<u8> {
    if !value.iter().all(u8::is_ascii_digit) {
        return None;
    }
    std::str::from_utf8(value).ok()?.parse().ok()
}
