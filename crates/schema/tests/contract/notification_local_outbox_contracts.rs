use std::fs::read_to_string;
use std::path::PathBuf;

#[test]
fn notification_local_outbox_generated_typescript_matches_checked_in_file() {
    let generated =
        ocentra_schema::notification_local_outbox_ts::notification_local_outbox_typescript();
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/schema-domain/src/generated-notification-local-outbox.ts");
    let checked_in = match read_to_string(file_path) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };

    assert!(checked_in
        .starts_with("/* generated from crates/schema/src/notification_local_outbox_ts.rs */"));
    assert_eq!(generated, checked_in);
}

#[test]
fn notification_local_outbox_uses_formatter_compact_scalar_arrays() {
    let generated =
        ocentra_schema::notification_local_outbox_ts::notification_local_outbox_typescript();
    let expected_line = "manualProofRequirements: ['provider setup review required'],";
    let manual_requirements_line = generated
        .lines()
        .map(str::trim)
        .find(|line| *line == expected_line);
    assert_eq!(manual_requirements_line, Some(expected_line));
}
