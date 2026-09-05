use std::collections::HashSet;

use rusqlite::Connection;

use super::super::AccountIdentityIssuerError;

pub(super) fn validate(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    let allowed_tables = [
        "account_identity_issuer_key_registry",
        "account_identity_issuer_transport_receipt",
        "account_identity_issuer_clock",
        "account_identity_issuer_transport_outbox",
    ];
    let allowed_indexes = [
        "account_identity_issuer_key_registry_current",
        "account_identity_issuer_transport_receipt_lookup",
        "account_identity_issuer_transport_outbox_delivery",
    ];
    let mut statement = connection
        .prepare(
            "SELECT type, name FROM sqlite_master
             WHERE name LIKE 'account_identity_issuer_%'
                OR (type IN ('trigger', 'view')
                    AND lower(COALESCE(sql, '')) LIKE '%account_identity_issuer_%')",
        )
        .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?;
    let mut rows = statement
        .query([])
        .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?;
    let mut tables = HashSet::new();
    let mut indexes = HashSet::new();
    while let Some(row) = rows
        .next()
        .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?
    {
        let object_type = row
            .get::<_, String>(0)
            .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?;
        let name = row
            .get::<_, String>(1)
            .map_err(|_error| AccountIdentityIssuerError::InvalidDurableSchema)?;
        match object_type.as_str() {
            "table" if allowed_tables.contains(&name.as_str()) => {
                tables.insert(name);
            }
            "index" if allowed_indexes.contains(&name.as_str()) => {
                indexes.insert(name);
            }
            _ => return Err(AccountIdentityIssuerError::InvalidDurableSchema),
        }
    }
    (tables.len() == allowed_tables.len() && indexes.len() == allowed_indexes.len())
        .then_some(())
        .ok_or(AccountIdentityIssuerError::InvalidDurableSchema)
}
