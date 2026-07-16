use std::path::Path;

const POLICY_CONTRACT_HELPERS_SOURCE_DIRECTORY: &str = "src";
const POLICY_CONTRACT_HELPERS_TYPESCRIPT_CONTRACTS_PATH: &str =
    "policy_contract_helpers_ts.contracts.txt";
const POLICY_CONTRACT_HELPERS_TYPESCRIPT_HELPERS_PATH: &str =
    "policy_contract_helpers_ts.helpers.txt";
const POLICY_CONTRACT_HELPERS_TYPESCRIPT_HELPER_FRAGMENTS: [(&str, &str); 8] = [
    (
        "/* __OCENTRA_POLICY_HELPER_SCHEDULE__ */",
        "policy_contract_helpers_ts.helpers/schedule.template.txt",
    ),
    (
        "/* __OCENTRA_POLICY_HELPER_PREVIEW__ */",
        "policy_contract_helpers_ts.helpers/preview.template.txt",
    ),
    (
        "/* __OCENTRA_POLICY_HELPER_AUTHORITY__ */",
        "policy_contract_helpers_ts.helpers/authority.template.txt",
    ),
    (
        "/* __OCENTRA_POLICY_HELPER_APP_GAME__ */",
        "policy_contract_helpers_ts.helpers/app-game.template.txt",
    ),
    (
        "/* __OCENTRA_POLICY_HELPER_SCREEN_AI__ */",
        "policy_contract_helpers_ts.helpers/screen-ai.template.txt",
    ),
    (
        "/* __OCENTRA_POLICY_HELPER_SCHEDULE_INTERNALS__ */",
        "policy_contract_helpers_ts.helpers/schedule-internals.template.txt",
    ),
    (
        "/* __OCENTRA_POLICY_HELPER_APPROVAL_INTERNALS__ */",
        "policy_contract_helpers_ts.helpers/approval-internals.template.txt",
    ),
    (
        "/* __OCENTRA_POLICY_HELPER_VALIDATION_INTERNALS__ */",
        "policy_contract_helpers_ts.helpers/validation-internals.template.txt",
    ),
];
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
    let mut source = read_policy_contract_helpers_typescript_sidecar(
        POLICY_CONTRACT_HELPERS_TYPESCRIPT_HELPERS_PATH,
    )?;
    for (marker, path) in POLICY_CONTRACT_HELPERS_TYPESCRIPT_HELPER_FRAGMENTS {
        let fragment = read_policy_contract_helpers_typescript_sidecar(path)?;
        let marker_line = format!("{marker}\n");
        source = source.replace(&marker_line, &fragment);
    }
    Ok(source)
}
