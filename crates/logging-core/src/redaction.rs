use crate::field::{LogFieldValue, LogFields};

pub const REDACTED_VALUE: &str = "[REDACTED]";

pub fn redact_fields(fields: &LogFields) -> LogFields {
    fields
        .iter()
        .map(|(key, value)| {
            if is_secret_key(key) {
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

fn is_secret_key(field_name: &str) -> bool {
    let lowercase_field_name = field_name.to_ascii_lowercase();
    lowercase_field_name.contains("token")
        || lowercase_field_name.contains("secret")
        || lowercase_field_name.contains("password")
        || lowercase_field_name.contains("authorization")
        || lowercase_field_name.contains("apikey")
        || lowercase_field_name.contains("api_key")
        || lowercase_field_name.contains("cookie")
        || lowercase_field_name.contains("session")
}
