use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension};

use crate::device_trust_registry::{
    DeviceTrustLifecycleState, DeviceTrustRegistryDecision, DeviceTrustRegistryFailure,
    DeviceTrustRegistryRecord, DeviceTrustRegistryRejection,
};

pub(crate) fn validate_custody_path(path: &Path) -> Result<(), DeviceTrustRegistryFailure> {
    let Some(parent) = path.parent() else {
        return Err(DeviceTrustRegistryFailure::CustodyUnavailable);
    };
    if !path.is_absolute() || !parent.is_dir() {
        return Err(DeviceTrustRegistryFailure::CustodyUnavailable);
    }
    Ok(())
}

pub(crate) fn record(
    connection: &Connection,
    device_id: &str,
) -> Result<Option<DeviceTrustRegistryRecord>, DeviceTrustRegistryFailure> {
    let row = connection
        .query_row(
            "SELECT device_id, state FROM device_trust_registry WHERE device_id = ?1",
            [device_id],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()
        .map_err(|_error| DeviceTrustRegistryFailure::StorageUnavailable)?;
    row.map(|(device_id, state)| record_from_row(&device_id, &state))
        .transpose()
}

pub(crate) fn pair(
    connection: &Connection,
    family_id: &str,
    parent_account_id: &str,
    device_id: &str,
) -> Result<DeviceTrustRegistryDecision, DeviceTrustRegistryFailure> {
    if record(connection, device_id)?
        .as_ref()
        .is_some_and(|record| record.state == DeviceTrustLifecycleState::Revoked)
    {
        return Ok(DeviceTrustRegistryDecision::Rejected(
            DeviceTrustRegistryRejection::RevokedDeviceCannotRePair,
        ));
    }
    write_state(
        connection,
        family_id,
        parent_account_id,
        device_id,
        "pending-sealing",
    )?;
    Ok(DeviceTrustRegistryDecision::PendingSealing(record_for(
        device_id,
        DeviceTrustLifecycleState::PendingSealing,
    )))
}

pub(crate) fn revoke(
    connection: &Connection,
    family_id: &str,
    parent_account_id: &str,
    device_id: &str,
) -> Result<DeviceTrustRegistryDecision, DeviceTrustRegistryFailure> {
    write_state(
        connection,
        family_id,
        parent_account_id,
        device_id,
        "revoked",
    )?;
    Ok(DeviceTrustRegistryDecision::Revoked(record_for(
        device_id,
        DeviceTrustLifecycleState::Revoked,
    )))
}

fn write_state(
    connection: &Connection,
    family_id: &str,
    parent_account_id: &str,
    device_id: &str,
    state: &str,
) -> Result<(), DeviceTrustRegistryFailure> {
    connection
        .execute(
            "INSERT INTO device_trust_registry (device_id, family_id, parent_account_id, state)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(device_id) DO UPDATE SET family_id = excluded.family_id, parent_account_id = excluded.parent_account_id, state = excluded.state",
            params![device_id, family_id, parent_account_id, state],
        )
        .map_err(|_error| DeviceTrustRegistryFailure::StorageUnavailable)?;
    Ok(())
}

fn record_from_row(
    device_id: &str,
    state: &str,
) -> Result<DeviceTrustRegistryRecord, DeviceTrustRegistryFailure> {
    let state = match state {
        "pending-sealing" => DeviceTrustLifecycleState::PendingSealing,
        "trusted" => DeviceTrustLifecycleState::Trusted,
        "revoked" => DeviceTrustLifecycleState::Revoked,
        "reset-required" => DeviceTrustLifecycleState::ResetRequired,
        _ => return Err(DeviceTrustRegistryFailure::StorageIntegrityRejected),
    };
    Ok(record_for(device_id, state))
}

fn record_for(device_id: &str, state: DeviceTrustLifecycleState) -> DeviceTrustRegistryRecord {
    DeviceTrustRegistryRecord {
        device_id: device_id.to_owned(),
        state,
    }
}
