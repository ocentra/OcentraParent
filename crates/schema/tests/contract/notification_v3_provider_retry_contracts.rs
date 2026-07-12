use std::fs::read_to_string;
use std::path::PathBuf;

#[test]
fn notification_v3_provider_retry_generated_typescript_matches_checked_in_file() {
    let generated = ocentra_schema::notification_v3_provider_retry_ts::notification_v3_provider_retry_typescript();
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/schema-domain/src/generated-notification-v3-provider-retry.ts");
    let checked_in =
        read_to_string(file_path).expect("read generated notification v3 provider retry ts");

    assert!(checked_in.starts_with(
        "/* generated from crates/schema/src/notification_v3_provider_retry_ts.rs */"
    ));
    assert_eq!(generated, checked_in);
}
