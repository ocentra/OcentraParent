use std::fs::read_to_string;
use std::path::PathBuf;

#[test]
fn parent_control_capability_data_generated_typescript_matches_checked_in_file() {
    let generated =
        ocentra_schema::parent_control_capability_data_ts::parent_control_capability_data_typescript();
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/schema-domain/src/capability-data.ts");
    let checked_in = match read_to_string(file_path) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };

    assert!(generated.starts_with(
        "/* generated from crates/schema/src/parent_control_capability_data_ts.rs */"
    ));
    assert_eq!(generated, checked_in);
}
