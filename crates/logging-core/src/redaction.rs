use crate::field::{LogFieldValue, LogFields};

pub const REDACTED_VALUE: &str = "[REDACTED]";

pub fn redact_fields(fields: &LogFields) -> LogFields {
    fields
        .iter()
        .map(|(key, value)| {
            if is_secret_key(key) {
                (key.clone(), LogFieldValue::String(REDACTED_VALUE.to_owned()))
            } else {
                (key.clone(), value.clone())
            }
        })
        .collect()
}

fn is_secret_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    key.contains("token")
        || key.contains("secret")
        || key.contains("password")
        || key.contains("authorization")
        || key.contains("apikey")
        || key.contains("api_key")
        || key.contains("cookie")
        || key.contains("session")
}
