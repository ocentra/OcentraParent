use std::fs;
use std::path::PathBuf;

#[test]
fn family_reference_primitives_generated_typescript_matches_checked_in_file() {
    let generated =
        ocentra_schema::family_reference_primitives_ts::family_reference_primitives_typescript();
    let checked_in = match fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../packages/schema-domain/src/generated-family-reference-primitives.ts"),
    ) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };

    assert_eq!(generated, checked_in);
    assert!(generated
        .starts_with("/* generated from crates/schema/src/family_reference_primitives_ts.rs */"));
}
