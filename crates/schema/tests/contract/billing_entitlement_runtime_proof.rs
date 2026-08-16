use std::fs::read_to_string;
use std::path::PathBuf;

#[test]
fn billing_entitlement_runtime_proof_generated_typescript_matches_checked_in_file() {
    let generated =
        ocentra_schema::billing_entitlement_runtime_proof_ts::billing_entitlement_runtime_proof_typescript();
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/schema-domain/src/generated-billing-entitlement-runtime-proof.ts");
    let checked_in = match read_to_string(file_path) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };

    assert!(generated.starts_with(
        "/* generated from crates/schema/src/billing_entitlement_runtime_proof_ts.rs */"
    ));
    assert_eq!(generated, checked_in);
}
