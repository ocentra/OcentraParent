use serde_json::Value;

use super::super::record_text_values_any;

pub(super) fn flag_state(record: &Value) -> Option<(bool, bool, bool)> {
    let normalized_flags = record_text_values_any(record, &["flags", "Flags"])
        .iter()
        .map(|flag| flag.to_ascii_lowercase())
        .collect::<Vec<_>>();
    if normalized_flags.iter().any(|flag| {
        matches!(
            flag.as_str(),
            "down" | "no-carrier" | "lowerlayerdown" | "disconnected"
        )
    }) {
        return Some((false, false, true));
    }
    if normalized_flags
        .iter()
        .any(|flag| matches!(flag.as_str(), "up" | "lower_up" | "lower-up" | "connected"))
    {
        return Some((true, true, true));
    }
    None
}
