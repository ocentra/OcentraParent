use rusqlite::Connection;

use super::super::AccountIdentityAuthorityIssuerClientError;

pub(super) fn validate_text_rows(
    connection: &Connection,
    table: &str,
    columns: &[&str],
    max_rows: i64,
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    let row_count = connection
        .query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
            row.get::<_, i64>(0)
        })
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    if row_count > max_rows {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
    }
    let query = format!("SELECT {} FROM {table} ORDER BY rowid", columns.join(", "));
    let mut statement = connection
        .prepare(&query)
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let mut rows = statement
        .query([])
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    while let Some(row) = rows
        .next()
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
    {
        for index in 0..columns.len() {
            let value = row
                .get::<_, Option<String>>(index)
                .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
            if value
                .as_deref()
                .is_some_and(|value| !valid_storage_text(value))
            {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn valid_storage_text(value: &str) -> bool {
    !value.trim().is_empty()
        && value.len() <= 1_024
        && !value
            .chars()
            .any(|character| character <= '\u{001f}' || character == '\u{007f}')
}
