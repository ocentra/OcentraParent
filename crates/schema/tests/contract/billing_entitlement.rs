use std::fs::read_to_string;
use std::path::PathBuf;

#[path = "../../src/billing_entitlement_ts.rs"]
mod billing_entitlement_ts;

#[test]
fn billing_entitlement_generated_typescript_matches_checked_in_file() {
    let generated = billing_entitlement_ts::BILLING_ENTITLEMENT_TYPESCRIPT;
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/schema-domain/src/generated-billing-entitlement.ts");
    let checked_in = match read_to_string(file_path) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };

    assert!(
        generated.starts_with("/* generated from crates/schema/src/billing_entitlement_ts.rs */")
    );
    assert_eq!(generated, checked_in);
}
