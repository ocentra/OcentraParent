use std::fs;
use std::path::PathBuf;

#[test]
fn family_references_generated_typescript_matches_checked_in_file() {
    let generated = ocentra_schema::family_references_ts::family_references_typescript();
    let checked_in = match fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../packages/schema-domain/src/generated-family-references.ts"),
    ) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };

    assert_eq!(generated, checked_in);
    assert!(generated.starts_with("/* generated from crates/schema/src/family_references_ts.rs */"));
}
