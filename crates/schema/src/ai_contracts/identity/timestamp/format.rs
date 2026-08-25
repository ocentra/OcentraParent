pub(super) fn ascii_digits(value: &[u8]) -> Option<u32> {
    (!value.is_empty() && value.iter().all(u8::is_ascii_digit)).then(|| {
        value
            .iter()
            .fold(0_u32, |number, digit| number * 10 + u32::from(digit - b'0'))
    })
}

pub(super) fn fraction_nanos(value: &[u8]) -> Option<u32> {
    if value.is_empty() || value.len() > 9 {
        return None;
    }
    let number = ascii_digits(value)?;
    Some(number * 10_u32.pow(9 - value.len() as u32))
}
