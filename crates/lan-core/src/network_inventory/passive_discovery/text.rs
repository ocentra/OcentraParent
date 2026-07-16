use super::LAN_PASSIVE_DISCOVERY_MAX_SUMMARY_BYTES;

pub fn compact_summary(summary: impl AsRef<str>) -> String {
    summary
        .as_ref()
        .chars()
        .map(|character| {
            if character.is_ascii() && !character.is_control() {
                character
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(LAN_PASSIVE_DISCOVERY_MAX_SUMMARY_BYTES)
        .collect()
}
