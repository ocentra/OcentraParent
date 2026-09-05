use rusqlite::Connection;

use super::AccountIdentityAuthorityRepositoryError;

#[path = "account_identity_authority_repository_schema_rows.rs"]
mod rows;

pub(super) fn validate(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityRepositoryError> {
    validate_definition(connection)?;
    rows::validate(connection)
}

fn validate_definition(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityRepositoryError> {
    let definition = connection
        .query_row(
            "SELECT sql FROM sqlite_master
             WHERE type = 'table' AND name = 'account_identity_current_authority'",
            [],
            |row| row.get::<_, String>(0),
        )
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?;
    if !definition.to_ascii_uppercase().contains("STRICT") {
        return Err(AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority);
    }
    validate_columns(connection)?;
    validate_indexes(connection)
}

fn validate_columns(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityRepositoryError> {
    let expected = [
        ("provider", "TEXT", 1_i64, 1_i64),
        ("provider_subject", "TEXT", 1_i64, 2_i64),
        ("mapping_status", "TEXT", 1_i64, 0_i64),
        ("authority_generation", "INTEGER", 1_i64, 0_i64),
        ("session_id", "TEXT", 1_i64, 0_i64),
        ("session_generation", "INTEGER", 1_i64, 0_i64),
        ("authority_json", "TEXT", 1_i64, 0_i64),
    ];
    let mut statement = connection
        .prepare("PRAGMA table_info('account_identity_current_authority')")
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?;
    let mut rows = statement
        .query([])
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?;
    let mut actual = Vec::new();
    while let Some(row) = rows
        .next()
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?
    {
        actual.push((
            row.get::<_, String>(1)
                .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?,
            row.get::<_, String>(2)
                .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?,
            row.get::<_, i64>(3)
                .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?,
            row.get::<_, i64>(5)
                .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?,
        ));
    }
    if actual.len() != expected.len()
        || actual.iter().zip(expected).any(|(actual, expected)| {
            actual.0 != expected.0
                || actual.1.to_ascii_uppercase() != expected.1
                || actual.2 != expected.2
                || actual.3 != expected.3
        })
    {
        return Err(AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority);
    }
    Ok(())
}

fn validate_indexes(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityRepositoryError> {
    let mut statement = connection
        .prepare("PRAGMA index_list('account_identity_current_authority')")
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?;
    let mut rows = statement
        .query([])
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?;
    while let Some(row) = rows
        .next()
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?
    {
        let name = row
            .get::<_, String>(1)
            .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?;
        if !name.starts_with("sqlite_autoindex_") {
            return Err(AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority);
        }
    }
    Ok(())
}
