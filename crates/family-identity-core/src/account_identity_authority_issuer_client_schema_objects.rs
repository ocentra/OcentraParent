use std::collections::HashSet;

use rusqlite::Connection;

use super::super::{has_legacy_table, AccountIdentityAuthorityIssuerClientError};

pub(super) fn validate_previous_metadata_if_present(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if super::has_legacy_table(connection, "account_identity_issuer_v2_schema")? {
        super::validate_previous_metadata(connection)?;
    }
    Ok(())
}

pub(super) fn validate_owned_objects(
    connection: &Connection,
    allowed_tables: &[&str],
    allowed_indexes: &[&str],
    schema_table_optional: bool,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let mut statement = connection
        .prepare(
            "SELECT type, name FROM sqlite_master
             WHERE name LIKE 'account_identity_issuer_v2_%'
                OR (type IN ('trigger', 'view')
                    AND lower(COALESCE(sql, '')) LIKE '%account_identity_issuer_v2_%')",
        )
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let mut rows = statement
        .query([])
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let mut tables = HashSet::new();
    let mut indexes = HashSet::new();
    while let Some(row) = rows
        .next()
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
    {
        let object_type = row
            .get::<_, String>(0)
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
        let name = row
            .get::<_, String>(1)
            .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
        match object_type.as_str() {
            "table" if allowed_tables.contains(&name.as_str()) => {
                tables.insert(name);
            }
            "index" if allowed_indexes.contains(&name.as_str()) => {
                indexes.insert(name);
            }
            _ => return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema),
        }
    }
    let expected_tables = if schema_table_optional
        && !has_legacy_table(connection, "account_identity_issuer_v2_schema")?
    {
        allowed_tables.len() - 1
    } else {
        allowed_tables.len()
    };
    (tables.len() == expected_tables && indexes.len() == allowed_indexes.len())
        .then_some(())
        .ok_or(AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}
