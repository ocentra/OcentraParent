use rusqlite::Connection;

use crate::{
    device_trust_lifecycle::DeviceTrustLifecycleError,
    device_trust_lifecycle_schema::{
        is_lower_hex, validate_columns, validate_indexes, validate_table_sql,
    },
};

pub(crate) const TABLE: &str = "device_trust_authority_transition";
const TABLE_SQL: &str = concat!(
    "CREATETABLEDEVICE_TRUST_AUTHORITY_TRANSITION(",
    "AUTHORITY_KEYTEXTNOTNULLPRIMARYKEYCHECK(LENGTH(AUTHORITY_KEY)=64),",
    "OPERATION_IDTEXTNOTNULLCHECK(LENGTH(OPERATION_ID)=64),",
    "FROM_GENERATIONINTEGERCHECK(FROM_GENERATIONISNULLORFROM_GENERATION>0),",
    "TO_GENERATIONINTEGERNOTNULLCHECK(TO_GENERATION>0),",
    "CHECK((FROM_GENERATIONISNULLANDTO_GENERATION=1)OR(",
    "FROM_GENERATIONISNOTNULLANDTO_GENERATION=FROM_GENERATION+1))",
    ")STRICT"
);

pub(crate) fn validate(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    validate_table_sql(connection, TABLE, TABLE_SQL)?;
    validate_columns(
        connection,
        TABLE,
        &[
            ("authority_key", "TEXT", 1, 1),
            ("operation_id", "TEXT", 1, 0),
            ("from_generation", "INTEGER", 0, 0),
            ("to_generation", "INTEGER", 1, 0),
        ],
    )?;
    validate_indexes(connection, TABLE, &["authority_key"], "pk")?;
    validate_rows(connection)
}

fn validate_rows(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    let mut statement = connection
        .prepare(
            "SELECT authority_key, operation_id, from_generation, to_generation
             FROM device_trust_authority_transition
             ORDER BY authority_key",
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<i64>>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    for row in rows {
        let (authority_key, operation_id, from, to) =
            row.map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        let valid_generation = match from {
            None => to == 1,
            Some(from) => from > 0 && from.checked_add(1) == Some(to),
        };
        if !is_lower_hex(&authority_key, 64)
            || !is_lower_hex(&operation_id, 64)
            || !valid_generation
        {
            return Err(DeviceTrustLifecycleError::Unavailable);
        }
    }
    Ok(())
}
