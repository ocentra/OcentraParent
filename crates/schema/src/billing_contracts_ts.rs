use std::path::Path;

const BILLING_CONTRACTS_TYPESCRIPT_SOURCE_DIRECTORY: &str = "../../infra/cloudflare/src/generated";
const BILLING_CONTRACTS_TYPESCRIPT_PATH: &str = "billing-contracts.ts";
const BILLING_CONTRACTS_TYPESCRIPT_SIDECAR_READ_ERROR: &str =
    "billing contracts TypeScript sidecar should be readable";

fn read_billing_contracts_typescript_sidecar(path: &str) -> String {
    crate::schema_result_or_unreachable(
        std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(BILLING_CONTRACTS_TYPESCRIPT_SOURCE_DIRECTORY)
                .join(path),
        ),
        BILLING_CONTRACTS_TYPESCRIPT_SIDECAR_READ_ERROR,
    )
}

pub fn billing_contracts_typescript() -> String {
    read_billing_contracts_typescript_sidecar(BILLING_CONTRACTS_TYPESCRIPT_PATH)
}
