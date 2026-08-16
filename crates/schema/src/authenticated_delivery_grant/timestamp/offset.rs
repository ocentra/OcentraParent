pub(super) fn parse(value: &[u8]) -> Option<i32> {
    match value {
        [b'Z' | b'z'] => Some(0),
        [sign @ (b'+' | b'-'), hour_a, hour_b, b':', minute_a, minute_b] => signed_seconds(
            *sign,
            decimal_u8(&[*hour_a, *hour_b])?,
            decimal_u8(&[*minute_a, *minute_b])?,
        ),
        _ => None,
    }
}

fn signed_seconds(sign: u8, hours: u8, minutes: u8) -> Option<i32> {
    if hours > 23 || minutes > 59 {
        return None;
    }
    let seconds = i32::from(hours) * 3_600 + i32::from(minutes) * 60;
    Some(if sign == b'+' { seconds } else { -seconds })
}

fn decimal_u8(value: &[u8; 2]) -> Option<u8> {
    if !value.iter().all(u8::is_ascii_digit) {
        return None;
    }
    std::str::from_utf8(value).ok()?.parse().ok()
}
