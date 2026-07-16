use std::path::Path;

const BROWSER_POLICY_CONTROL_CATALOG_SCHEMA_SOURCE_DIRECTORY: &str = "src";
const BROWSER_POLICY_CONTROL_CATALOG_CONTRACTS_TYPESCRIPT_PATH: &str =
    "browser_policy_control_catalog_ts.contracts.txt";
const BROWSER_POLICY_CONTROL_CATALOG_HELPERS_TYPESCRIPT_PATH: &str =
    "browser_policy_control_catalog_ts.helpers.txt";
const BROWSER_POLICY_CONTROL_CATALOG_HELPERS_TYPESCRIPT_FRAGMENT_PATHS: &[&str] = &[
    BROWSER_POLICY_CONTROL_CATALOG_HELPERS_TYPESCRIPT_PATH,
    "browser_policy_control_catalog_ts.helpers.policy.txt",
    "browser_policy_control_catalog_ts.helpers.policy.validation.txt",
    "browser_policy_control_catalog_ts.helpers.policy.browser-games.txt",
    "browser_policy_control_catalog_ts.helpers.defaults.txt",
    "browser_policy_control_catalog_ts.helpers.catalog.core.txt",
    "browser_policy_control_catalog_ts.helpers.catalog.effect-status.txt",
    "browser_policy_control_catalog_ts.helpers.catalog.runtime-owner.txt",
    "browser_policy_control_catalog_ts.helpers.catalog.capability-state.txt",
    "browser_policy_control_catalog_ts.helpers.catalog.capability-requirement.txt",
    "browser_policy_control_catalog_ts.helpers.catalog.proof-requirement.txt",
    "browser_policy_control_catalog_ts.helpers.catalog.fallback.txt",
    "browser_policy_control_catalog_ts.helpers.catalog.visibility.txt",
    "browser_policy_control_catalog_ts.helpers.catalog.validation.txt",
    "browser_policy_control_catalog_ts.helpers.catalog.ui-tab.txt",
    "browser_policy_control_catalog_ts.helpers.manifest.txt",
    "browser_policy_control_catalog_ts.helpers.manifest.options.txt",
];
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
    BROWSER_POLICY_CONTROL_CATALOG_HELPERS_TYPESCRIPT_FRAGMENT_PATHS
        .iter()
        .map(|path| read_browser_policy_control_catalog_typescript_sidecar(path))
        .collect::<Vec<_>>()
        .join("")
}
