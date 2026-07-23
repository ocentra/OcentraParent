#[path = "device_trust_registry_storage/decision.rs"]
mod decision;
#[path = "device_trust_registry_storage/state.rs"]
mod state;

use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};

use crate::device_trust_registry::{
    DeviceTrustLifecycleState, DeviceTrustRegistryDecision, DeviceTrustRegistryFailure,
    DeviceTrustRegistryRecord,
};
use decision::{mutation_plan, MutationPlan};
use state::{journal_fields, record_from_row};

#[derive(Clone, Copy)]
struct MutationInput<'a> {
    family_id: &'a str,
    parent_account_id: &'a str,
    device_id: &'a str,
    action: &'a str,
    correlation_id: &'a str,
    receipt_ref: &'a str,
}

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
    family_id: &str,
    device_id: &str,
) -> Result<Option<DeviceTrustRegistryRecord>, DeviceTrustRegistryFailure> {
    let row = connection
        .query_row(
            "SELECT device_id, state FROM device_trust_registry WHERE device_id = ?1 AND family_id = ?2",
            params![device_id, family_id],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()
        .map_err(|_error| DeviceTrustRegistryFailure::StorageUnavailable)?;
    row.map(|(device_id, state)| record_from_row(&device_id, &state))
        .transpose()
}

pub(crate) fn pair(
    connection: &mut Connection,
    family_id: &str,
    parent_account_id: &str,
    device_id: &str,
    correlation_id: &str,
    receipt_ref: &str,
) -> Result<DeviceTrustRegistryDecision, DeviceTrustRegistryFailure> {
    mutate(
        connection,
        MutationInput {
            family_id,
            parent_account_id,
            device_id,
            action: "pair-child-device",
            correlation_id,
            receipt_ref,
        },
    )
}

pub(crate) fn revoke(
    connection: &mut Connection,
    family_id: &str,
    parent_account_id: &str,
    device_id: &str,
    correlation_id: &str,
    receipt_ref: &str,
) -> Result<DeviceTrustRegistryDecision, DeviceTrustRegistryFailure> {
    mutate(
        connection,
        MutationInput {
            family_id,
            parent_account_id,
            device_id,
            action: "revoke-child-device",
            correlation_id,
            receipt_ref,
        },
    )
}

fn mutate(
    connection: &mut Connection,
    input: MutationInput<'_>,
) -> Result<DeviceTrustRegistryDecision, DeviceTrustRegistryFailure> {
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_error| DeviceTrustRegistryFailure::StorageUnavailable)?;
    let existing = transaction
        .query_row(
            "SELECT family_id, state FROM device_trust_registry WHERE device_id = ?1",
            [input.device_id],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()
        .map_err(|_error| DeviceTrustRegistryFailure::StorageUnavailable)?;
    let existing = existing
        .as_ref()
        .map(|(family, state)| (family.as_str(), state.as_str()));
    let decision = match mutation_plan(existing, input.family_id, input.action)? {
        MutationPlan::Rejected(rejection) => DeviceTrustRegistryDecision::Rejected(rejection),
        MutationPlan::PairPendingSealing => {
            transaction
        .execute(
            "INSERT INTO device_trust_registry (device_id, family_id, parent_account_id, state)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(device_id) DO UPDATE SET state = excluded.state",
             params![input.device_id, input.family_id, input.parent_account_id, "pending-sealing"],
        )
        .map_err(|_error| DeviceTrustRegistryFailure::StorageUnavailable)?;
            DeviceTrustRegistryDecision::PendingSealing(record_for(
                input.device_id,
                DeviceTrustLifecycleState::PendingSealing,
            ))
        }
        MutationPlan::Revoke => {
            transaction.execute(
                "INSERT INTO device_trust_registry (device_id, family_id, parent_account_id, state)
                 VALUES (?1, ?2, ?3, 'revoked')
                 ON CONFLICT(device_id) DO UPDATE SET state = 'revoked'",
                params![input.device_id, input.family_id, input.parent_account_id],
            ).map_err(|_error| DeviceTrustRegistryFailure::StorageUnavailable)?;
            DeviceTrustRegistryDecision::Revoked(record_for(
                input.device_id,
                DeviceTrustLifecycleState::Revoked,
            ))
        }
    };
    let (outcome, state) = journal_fields(&decision);
    transaction.execute(
        "INSERT INTO device_trust_registry_journal (operation_id, correlation_id, receipt_ref, device_id, family_id, acting_parent_account_id, action, outcome, state)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![input.receipt_ref, input.correlation_id, input.receipt_ref, input.device_id, input.family_id, input.parent_account_id, input.action, outcome, state],
    ).map_err(|_error| DeviceTrustRegistryFailure::StorageUnavailable)?;
    transaction
        .commit()
        .map_err(|_error| DeviceTrustRegistryFailure::StorageUnavailable)?;
    Ok(decision)
}

fn record_for(device_id: &str, state: DeviceTrustLifecycleState) -> DeviceTrustRegistryRecord {
    DeviceTrustRegistryRecord {
        device_id: device_id.to_owned(),
        state,
    }
}
