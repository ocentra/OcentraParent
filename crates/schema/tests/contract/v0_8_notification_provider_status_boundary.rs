use std::fs;
use std::path::PathBuf;

#[test]
fn v0_8_notification_provider_status_boundary_generated_typescript_matches_checked_in_file() {
    let generated = ocentra_schema::v0_8_notification_provider_status_boundary_ts::v0_8_notification_provider_status_boundary_typescript();
    let checked_in = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../packages/schema-domain/src/generated-v0-8-notification-provider-status-boundary.ts"),
    )
    .expect("v0.8 notification provider status boundary source should be readable");

    assert_eq!(generated, checked_in);
    assert!(generated.starts_with(
        "/* generated from crates/schema/src/v0_8_notification_provider_status_boundary_ts.rs */"
    ));
}
