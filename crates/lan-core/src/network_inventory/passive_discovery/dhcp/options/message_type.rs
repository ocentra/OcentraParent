pub(super) fn label(value: u8) -> String {
    let label = match value {
        1 => "discover",
        2 => "offer",
        3 => "request",
        4 => "decline",
        5 => "ack",
        6 => "nak",
        7 => "release",
        8 => "inform",
        _ => return value.to_string(),
    };
    label.to_string()
}
