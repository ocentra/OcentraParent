use crate::field::{LogFieldValue, LogFields};
use crate::redaction_policy::is_sensitive_field_name;

pub const REDACTED_VALUE: &str = "[REDACTED]";

pub fn redact_fields(fields: &LogFields) -> LogFields {
    fields
        .iter()
        .map(|(key, value)| {
            if is_sensitive_field_name(key) {
                (
                    key.clone(),
                    LogFieldValue::String(REDACTED_VALUE.to_owned()),
                )
            } else {
                (key.clone(), value.clone())
            }
        })
        .collect()
}
