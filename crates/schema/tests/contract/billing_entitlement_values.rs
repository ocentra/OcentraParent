use std::fs::read_to_string;
use std::path::PathBuf;

use ocentra_schema::billing_entitlement_values_ts::billing_entitlement_values_typescript;

#[test]
fn billing_entitlement_values_generated_typescript_matches_checked_in_file() {
    let generated = billing_entitlement_values_typescript();
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/schema-domain/src/generated-billing-entitlement-values.ts");
    let checked_in = match read_to_string(file_path) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };

    assert_eq!(generated, checked_in);
}
