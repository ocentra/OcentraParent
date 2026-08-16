use std::fs;
use std::path::PathBuf;

#[test]
fn eventing_contracts_generated_typescript_matches_checked_in_file() {
    let generated = ocentra_schema::eventing_contracts_ts::eventing_contracts_typescript();
    let checked_in = match fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../packages/schema-domain/src/eventing.ts"),
    ) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };

    assert_eq!(generated, checked_in);
    assert!(
        generated.starts_with("/* generated from crates/schema/src/eventing_contracts_ts.rs */")
    );
}
