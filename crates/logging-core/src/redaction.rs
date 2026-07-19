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
    let normalized = field_name
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect::<String>();
    [
        "token",
        "secret",
        "password",
        "authorization",
        "apikey",
        "cookie",
        "session",
        "credential",
        "privatekey",
        "clientsecret",
    ]
    .iter()
    .any(|needle| normalized.contains(needle))
}
