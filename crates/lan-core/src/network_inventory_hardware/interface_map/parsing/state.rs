use serde_json::Value;

use super::{record_text_values_any, LOOPBACK_KEYS};

#[path = "state_explicit.rs"]
mod state_explicit;
#[path = "state_flags.rs"]
mod state_flags;

pub(super) fn interface_state(record: &Value) -> (bool, bool, bool) {
    state_explicit::explicit_state(record)
        .or_else(|| state_flags::flag_state(record))
        .unwrap_or((true, true, false))
}

pub(super) fn interface_is_loopback(record: &Value, interface_name: &str) -> bool {
    super::record_bool_any(record, LOOPBACK_KEYS).unwrap_or(false)
        || record_text_values_any(record, &["flags", "Flags"])
            .iter()
            .any(|flag| flag.eq_ignore_ascii_case("loopback"))
        || interface_name.trim().eq_ignore_ascii_case("lo")
        || interface_name.to_ascii_lowercase().contains("loopback")
}
