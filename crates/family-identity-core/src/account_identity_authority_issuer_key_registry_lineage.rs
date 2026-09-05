use rusqlite::{Connection, OptionalExtension};

use super::AccountIdentityIssuerError;

pub(super) fn validate(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    reject_query_match(
        connection,
        "SELECT 1 FROM account_identity_issuer_key_registry
         WHERE key_state = 'active'
         GROUP BY account_id, household_id, service_label
         HAVING COUNT(*) > 1 LIMIT 1",
    )?;
    reject_query_match(
        connection,
        "SELECT 1 FROM account_identity_issuer_key_registry AS active
         WHERE active.key_state = 'active' AND EXISTS (
            SELECT 1 FROM account_identity_issuer_key_registry AS newer
             WHERE newer.account_id = active.account_id
               AND newer.household_id = active.household_id
               AND newer.service_label = active.service_label
               AND newer.key_version > active.key_version
         ) LIMIT 1",
    )?;
    reject_query_match(
        connection,
        "SELECT 1 FROM account_identity_issuer_key_registry
         GROUP BY account_id, household_id, service_label
         HAVING MIN(key_version) != 1
             OR MAX(key_version) != COUNT(*)
             OR COUNT(DISTINCT key_version) != COUNT(*)
         LIMIT 1",
    )
}

fn reject_query_match(
    connection: &Connection,
    query: &str,
) -> Result<(), AccountIdentityIssuerError> {
    let invalid = connection
        .query_row(query, [], |_row| Ok(()))
        .optional()
        .map_err(|_error| AccountIdentityIssuerError::Unavailable)?
        .is_some();
    (!invalid)
        .then_some(())
        .ok_or(AccountIdentityIssuerError::InvalidKeyRecord)
}
