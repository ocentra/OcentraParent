use rusqlite::{params, Transaction};

use crate::{
    device_trust_lifecycle::{to_sql_generation, DeviceTrustLifecycleError},
    device_trust_lifecycle_current_authority::redacted_signer_binding,
    device_trust_signer_registration::validate_persisted_rows,
};

const ACTIVE: &str = "active";
const REVOKED: &str = "revoked";

pub(crate) fn revoke_for_lifecycle(
    transaction: &Transaction<'_>,
    family_id: &str,
    trust_subject: &str,
    parent_device_id: &str,
    authority_generation: u64,
) -> Result<Vec<String>, DeviceTrustLifecycleError> {
    validate_persisted_rows(transaction)?;
    let signer_rows = current_signer_rows(transaction, family_id, trust_subject, parent_device_id)?;
    let event_bindings = signer_rows
        .iter()
        .map(|(child_device_id, installation_id, signer_key_id)| {
            redacted_signer_binding(
                family_id,
                trust_subject,
                parent_device_id,
                child_device_id,
                installation_id,
                signer_key_id,
            )
        })
        .collect::<Vec<_>>();
    let changed = transaction
        .execute(
            "UPDATE device_trust_signer_registration
             SET registration_state = ?4, authority_generation = ?5
             WHERE family_id = ?1 AND trust_subject = ?2 AND parent_device_id = ?3
               AND registration_state = ?6",
            params![
                family_id,
                trust_subject,
                parent_device_id,
                REVOKED,
                to_sql_generation(authority_generation)?,
                ACTIVE,
            ],
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    if changed != event_bindings.len() {
        return Err(DeviceTrustLifecycleError::Unavailable);
    }
    Ok(event_bindings)
}

fn current_signer_rows(
    transaction: &Transaction<'_>,
    family_id: &str,
    trust_subject: &str,
    parent_device_id: &str,
) -> Result<Vec<(String, String, String)>, DeviceTrustLifecycleError> {
    let mut statement = transaction
        .prepare(
            "SELECT child_device_id, installation_id, signer_key_id
             FROM device_trust_signer_registration
             WHERE family_id = ?1 AND trust_subject = ?2 AND parent_device_id = ?3
               AND registration_state = ?4
             ORDER BY child_device_id, installation_id, signer_key_id",
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let rows = statement
        .query_map(
            params![family_id, trust_subject, parent_device_id, ACTIVE],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .collect::<Result<_, _>>()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    Ok(rows)
}
