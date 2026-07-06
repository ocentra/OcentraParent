use std::fs::read_to_string;
use std::path::PathBuf;

use ocentra_schema::billing_entitlement_runtime_proof_schema_ts::billing_entitlement_runtime_proof_schema_typescript;

#[test]
fn billing_entitlement_runtime_proof_schema_generated_typescript_matches_checked_in_file() {
    let generated = billing_entitlement_runtime_proof_schema_typescript();
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/schema-domain/src/generated-billing-entitlement-runtime-proof-schema.ts");
    let checked_in =
        read_to_string(file_path).expect("read generated billing entitlement runtime proof schema ts");

    assert_eq!(generated, checked_in);
}
