use std::fmt::Write;

/// Canonical sensitive-field policy shared by the Rust logger and its
/// generated TypeScript edge consumer.
pub const SENSITIVE_LOG_FIELD_NEEDLES: &[&str] = &[
    "authorization",
    "clipboard",
    "cookie",
    "keystroke",
    "password",
    "screenshot",
    "secret",
    "token",
    "url",
    "childname",
    "accountname",
    "fullname",
    "commandline",
    "apikey",
    "session",
    "credential",
    "privatekey",
    "clientsecret",
];

pub(crate) fn is_sensitive_field_name(field_name: &str) -> bool {
    let normalized = field_name
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect::<String>();
    !normalized.is_empty()
        && SENSITIVE_LOG_FIELD_NEEDLES
            .iter()
            .any(|needle| normalized.contains(needle))
}

/// Render the checked-in TypeScript policy artifact from this Rust-owned list.
pub fn generated_typescript() -> String {
    let mut output =
        String::from("/* generated from crates/logging-core/src/redaction_policy.rs */\n\n");
    output.push_str("export const GeneratedSensitiveLogFieldNeedles = [\n");
    for needle in SENSITIVE_LOG_FIELD_NEEDLES {
        writeln!(&mut output, "  '{needle}',").expect("writing to a String cannot fail");
    }
    output.push_str("] as const;\n");
    output
}
