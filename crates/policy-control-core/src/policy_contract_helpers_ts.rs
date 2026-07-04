use std::path::Path;

const POLICY_CONTRACT_HELPERS_SOURCE_DIRECTORY: &str = "src";
const POLICY_CONTRACT_HELPERS_TYPESCRIPT_CONTRACTS_PATH: &str =
    "policy_contract_helpers_ts.contracts.txt";
const POLICY_CONTRACT_HELPERS_TYPESCRIPT_HELPERS_PATH: &str =
    "policy_contract_helpers_ts.helpers.txt";
const POLICY_CONTRACT_HELPERS_TYPESCRIPT_READ_ERROR: &str =
    "policy contract helpers TypeScript sidecar should be readable";

fn read_policy_contract_helpers_typescript_sidecar(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(POLICY_CONTRACT_HELPERS_SOURCE_DIRECTORY)
            .join(path),
    )
}

pub fn policy_contract_helper_contracts_typescript() -> Result<String, std::io::Error> {
    read_policy_contract_helpers_typescript_sidecar(
        POLICY_CONTRACT_HELPERS_TYPESCRIPT_CONTRACTS_PATH,
    )
}

pub fn policy_contract_helper_helpers_typescript() -> Result<String, std::io::Error> {
    read_policy_contract_helpers_typescript_sidecar(POLICY_CONTRACT_HELPERS_TYPESCRIPT_HELPERS_PATH)
}
