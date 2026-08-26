use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;
use rusqlite::{Connection, Row};

use crate::account_identity_authority_producer_v2;

use super::super::AccountIdentityAuthorityIssuerClientError;

const MAX_VALIDATED_KEY_ROWS: i64 = 4096;

pub(super) fn validate_key_rows(
    connection: &Connection,
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    let row_count = connection
        .query_row(
            "SELECT COUNT(*) FROM account_identity_issuer_v2_key_registry",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    if row_count > MAX_VALIDATED_KEY_ROWS {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
    }
    let mut statement = connection
        .prepare(
            "SELECT account_id, household_id, service, service_binding_id, key_id, public_key,
                    key_generation, enrollment_generation, authority_generation, key_state
               FROM account_identity_issuer_v2_key_registry
              ORDER BY account_id, household_id, service, key_generation",
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let mut rows = statement
        .query([])
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    while let Some(row) = rows
        .next()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
    {
        if validate_key_row(row)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn validate_key_row(row: &Row<'_>) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    let account_id = text(row, 0)?;
    let household_id = text(row, 1)?;
    let service = text(row, 2)?;
    let service_binding_id = text(row, 3)?;
    let key_id = text(row, 4)?;
    let public_key_bytes = row
        .get::<_, Vec<u8>>(5)
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    let key_generation = integer(row, 6)?;
    let enrollment_generation = integer(row, 7)?;
    let authority_generation = integer(row, 8)?;
    let key_state = text(row, 9)?;
    if ![
        account_id.as_str(),
        household_id.as_str(),
        service.as_str(),
        service_binding_id.as_str(),
        key_id.as_str(),
    ]
    .into_iter()
    .all(valid_storage_text)
    {
        return Ok(true);
    }
    let public_key: [u8; 65] = match public_key_bytes.try_into() {
        Ok(public_key) => public_key,
        Err(_) => return Ok(true),
    };
    let expected_binding =
        super::super::super::transaction::service_binding_id_for_values(&account_id, &household_id);
    Ok(service != ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE
        || service_binding_id != expected_binding
        || key_id != account_identity_authority_producer_v2::expected_key_id(&public_key)
        || account_identity_authority_producer_v2::validate_public_key(&public_key).is_err()
        || key_generation == 0
        || key_generation > 9_007_199_254_740_991
        || enrollment_generation == 0
        || enrollment_generation > 9_007_199_254_740_991
        || authority_generation == 0
        || authority_generation > 9_007_199_254_740_991
        || key_state != "active" && key_state != "revoked")
}

fn text(row: &Row<'_>, index: usize) -> Result<String, AccountIdentityAuthorityIssuerClientError> {
    row.get(index)
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn integer(row: &Row<'_>, index: usize) -> Result<i64, AccountIdentityAuthorityIssuerClientError> {
    row.get(index)
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn valid_storage_text(value: &str) -> bool {
    !value.trim().is_empty()
        && value.len() <= 1_024
        && !value
            .chars()
            .any(|character| character <= '\u{001f}' || character == '\u{007f}')
}
