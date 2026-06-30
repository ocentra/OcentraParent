use std::path::Path;

const BROWSER_POLICY_CONTROL_CATALOG_SCHEMA_SOURCE_DIRECTORY: &str = "src";
const BROWSER_POLICY_CONTROL_CATALOG_CONTRACTS_TYPESCRIPT_PATH: &str =
    "browser_policy_control_catalog_ts.contracts.txt";
const BROWSER_POLICY_CONTROL_CATALOG_HELPERS_TYPESCRIPT_PATH: &str =
    "browser_policy_control_catalog_ts.helpers.txt";
const BROWSER_POLICY_CONTROL_CATALOG_TYPESCRIPT_SIDECAR_READ_ERROR: &str =
    "browser policy control catalog TypeScript sidecar should be readable";

fn read_browser_policy_control_catalog_typescript_sidecar(path: &str) -> String {
    crate::schema_result_or_unreachable(
        std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(BROWSER_POLICY_CONTROL_CATALOG_SCHEMA_SOURCE_DIRECTORY)
                .join(path),
        ),
        BROWSER_POLICY_CONTROL_CATALOG_TYPESCRIPT_SIDECAR_READ_ERROR,
    )
}

pub fn browser_policy_control_catalog_contracts_typescript() -> String {
    read_browser_policy_control_catalog_typescript_sidecar(
        BROWSER_POLICY_CONTROL_CATALOG_CONTRACTS_TYPESCRIPT_PATH,
    )
}

pub fn browser_policy_control_catalog_helpers_typescript() -> String {
    read_browser_policy_control_catalog_typescript_sidecar(
        BROWSER_POLICY_CONTROL_CATALOG_HELPERS_TYPESCRIPT_PATH,
    )
}
