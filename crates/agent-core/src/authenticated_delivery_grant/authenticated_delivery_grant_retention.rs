use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantInstant,
};
use rusqlite::{params, Connection};

use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumeError;

const SELECT_CONSUMED_GRANTS: &str =
    "SELECT issuer_key_id, nonce, grant_json FROM authenticated_delivery_grant_consumes_v2";
const DELETE_CONSUMED_GRANT: &str =
    "DELETE FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2";
const DELETE_GRANT_AUDITS: &str =
    "DELETE FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2";

pub(super) fn purge_expired_replay_records(
    connection: &Connection,
    trusted_now: AuthenticatedDeliveryGrantInstant,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let expired = {
        let mut statement = connection
            .prepare(SELECT_CONSUMED_GRANTS)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        let mut expired = Vec::new();
        for row in rows {
            let (issuer_key_id, nonce, grant_json) =
                row.map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
            let grant: AuthenticatedDeliveryGrant = serde_json::from_str(&grant_json)
                .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
            if grant
                .expires_at_instant()
                .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?
                < trusted_now
            {
                expired.push((issuer_key_id, nonce));
            }
        }
        expired
    };
    for (issuer_key_id, nonce) in expired {
        connection
            .execute(DELETE_CONSUMED_GRANT, params![issuer_key_id, nonce])
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        connection
            .execute(DELETE_GRANT_AUDITS, params![issuer_key_id, nonce])
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    }
    Ok(())
}
