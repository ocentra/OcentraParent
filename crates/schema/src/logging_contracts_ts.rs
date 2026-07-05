use super::logging_contracts::{LOG_SCHEMA_VERSION, LOG_SNAPSHOT_SCHEMA_VERSION};

const LOGGING_CONTRACTS_SCHEMA_VERSION_TOKEN: &str = "__LOG_SCHEMA_VERSION__";
const LOGGING_CONTRACTS_SNAPSHOT_SCHEMA_VERSION_TOKEN: &str = "__LOG_SNAPSHOT_SCHEMA_VERSION__";
const LOGGING_CONTRACTS_TYPESCRIPT_TEMPLATE: &str = include_str!("logging_contracts.template.txt");

pub fn logging_contracts_typescript() -> String {
    LOGGING_CONTRACTS_TYPESCRIPT_TEMPLATE
        .replace(
            LOGGING_CONTRACTS_SCHEMA_VERSION_TOKEN,
            &LOG_SCHEMA_VERSION.to_string(),
        )
        .replace(
            LOGGING_CONTRACTS_SNAPSHOT_SCHEMA_VERSION_TOKEN,
            &LOG_SNAPSHOT_SCHEMA_VERSION.to_string(),
        )
}
