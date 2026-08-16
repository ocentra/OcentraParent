use std::fs;
use std::path::PathBuf;

#[test]
fn parent_control_capabilities_generated_typescript_matches_checked_in_file() {
    let generated =
        ocentra_schema::parent_control_capabilities_ts::parent_control_capabilities_typescript();
    let checked_in = match fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../packages/schema-domain/src/capabilities.ts"),
    ) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };

    assert_eq!(generated, checked_in);
    assert!(generated
        .starts_with("/* generated from crates/schema/src/parent_control_capabilities_ts.rs */"));
}
