use chrono::DateTime;
use ocentra_parent_agent_protocol::lan_pairing::signed_household_mesh_ingress::transport::LanSignedHouseholdMeshMessageType;
use rusqlite::Connection;

use super::{
    LanHouseholdMeshIngressCustodyError, IDEMPOTENCY_INDEX, MESSAGE_INDEX, REPLAY_INDEX, TABLE,
};

pub(super) fn validate_owned_schema_objects(
    connection: &Connection,
) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    let mut statement = connection
        .prepare(
            "SELECT type, name, tbl_name FROM sqlite_master WHERE name NOT LIKE 'sqlite_%' ORDER BY type, name",
        )
        .map_err(storage_error)?;
    let actual = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .map_err(storage_error)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(storage_error)?;
    let expected = vec![
        (
            "index".to_string(),
            IDEMPOTENCY_INDEX.to_string(),
            TABLE.to_string(),
        ),
        (
            "index".to_string(),
            MESSAGE_INDEX.to_string(),
            TABLE.to_string(),
        ),
        (
            "index".to_string(),
            REPLAY_INDEX.to_string(),
            TABLE.to_string(),
        ),
        ("table".to_string(), TABLE.to_string(), TABLE.to_string()),
        (
            "table".to_string(),
            super::super::rejection::TABLE.to_string(),
            super::super::rejection::TABLE.to_string(),
        ),
    ];
    if actual != expected {
        return Err(LanHouseholdMeshIngressCustodyError::SchemaRejected);
    }
    Ok(())
}

pub(super) fn validate_timestamp_and_message_semantics(
    connection: &Connection,
) -> Result<(), LanHouseholdMeshIngressCustodyError> {
    let mut statement = connection
        .prepare("SELECT issued_at, expires_at, reserved_at, lan_message_type FROM lan_household_mesh_ingress_receipts_v2")
        .map_err(storage_error)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .map_err(storage_error)?;
    for row in rows {
        let (issued_at, expires_at, reserved_at, message_type) =
            row.map_err(|_error| LanHouseholdMeshIngressCustodyError::IntegrityRejected)?;
        let issued_at = DateTime::parse_from_rfc3339(&issued_at)
            .map_err(|_error| LanHouseholdMeshIngressCustodyError::IntegrityRejected)?;
        let expires_at = DateTime::parse_from_rfc3339(&expires_at)
            .map_err(|_error| LanHouseholdMeshIngressCustodyError::IntegrityRejected)?;
        let reserved_at = DateTime::parse_from_rfc3339(&reserved_at)
            .map_err(|_error| LanHouseholdMeshIngressCustodyError::IntegrityRejected)?;
        if expires_at <= issued_at || reserved_at < issued_at || reserved_at >= expires_at {
            return Err(LanHouseholdMeshIngressCustodyError::IntegrityRejected);
        }
        LanSignedHouseholdMeshMessageType::try_from(message_type)
            .map_err(|_error| LanHouseholdMeshIngressCustodyError::IntegrityRejected)?;
    }
    Ok(())
}

fn storage_error(_error: rusqlite::Error) -> LanHouseholdMeshIngressCustodyError {
    LanHouseholdMeshIngressCustodyError::StorageUnavailable
}
