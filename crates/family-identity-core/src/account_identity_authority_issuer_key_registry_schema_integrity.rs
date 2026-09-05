use rusqlite::Connection;

use crate::account_identity_authority_issuer::AccountIdentityIssuerError;

pub(super) fn validate(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    let integrity = connection
        .query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0))
        .map_err(|_error| AccountIdentityIssuerError::DurableIntegrityFailure)?;
    if integrity != "ok" {
        return Err(AccountIdentityIssuerError::DurableIntegrityFailure);
    }
    let mut statement = connection
        .prepare("PRAGMA foreign_key_check")
        .map_err(|_error| AccountIdentityIssuerError::DurableIntegrityFailure)?;
    let mut rows = statement
        .query([])
        .map_err(|_error| AccountIdentityIssuerError::DurableIntegrityFailure)?;
    if rows
        .next()
        .map_err(|_error| AccountIdentityIssuerError::DurableIntegrityFailure)?
        .is_some()
    {
        return Err(AccountIdentityIssuerError::DurableIntegrityFailure);
    }
    Ok(())
}
