use rusqlite::Connection;

use super::hex_digest;

pub(super) fn validate(connection: &Connection) -> Result<(), ()> {
    let mut statement = connection
        .prepare(
            "SELECT h.handoff_id, h.correlation_id, h.household_id, h.account_id,
                    h.member_id, h.device_id, h.kind, h.requested_at_epoch_millis,
                    h.state, h.active_attempt_id, h.lease_expires_at_epoch_millis,
                    h.attempt_count, h.owner_transition_id, h.owner_receipt_digest,
                    r.household_id, r.account_id, r.requester_member_id, r.requester_device_id,
                    r.kind, r.state, r.reserved_owner_receipt_id,
                    r.reserved_owner_transition_id
             FROM account_identity_recovery_custody_handoff h
             JOIN account_identity_recovery r ON r.recovery_id = h.recovery_id",
        )
        .map_err(|_error| ())?;
    let mut rows = statement.query([]).map_err(|_error| ())?;
    while let Some(row) = rows.next().map_err(|_error| ())? {
        let state = row.get::<_, String>(8).map_err(|_error| ())?;
        let recovery_state = row.get::<_, String>(19).map_err(|_error| ())?;
        if !binding_valid(row)? || !state_valid(row, &state, &recovery_state)? {
            return Err(());
        }
    }
    Ok(())
}

fn binding_valid(row: &rusqlite::Row<'_>) -> Result<bool, ()> {
    let nonempty = [
        row.get::<_, String>(0).map_err(|_error| ())?,
        row.get::<_, String>(1).map_err(|_error| ())?,
    ];
    Ok(nonempty.iter().all(|value| !value.trim().is_empty())
        && row.get::<_, String>(2).map_err(|_error| ())?
            == row.get::<_, String>(14).map_err(|_error| ())?
        && row.get::<_, String>(3).map_err(|_error| ())?
            == row.get::<_, String>(15).map_err(|_error| ())?
        && row.get::<_, String>(4).map_err(|_error| ())?
            == row.get::<_, String>(16).map_err(|_error| ())?
        && row.get::<_, String>(5).map_err(|_error| ())?
            == row.get::<_, String>(17).map_err(|_error| ())?
        && row.get::<_, String>(6).map_err(|_error| ())?
            == row.get::<_, String>(18).map_err(|_error| ())?
        && row.get::<_, i64>(7).map_err(|_error| ())? > 0
        && row.get::<_, i64>(11).map_err(|_error| ())? >= 0)
}

fn state_valid(row: &rusqlite::Row<'_>, state: &str, recovery_state: &str) -> Result<bool, ()> {
    let attempt = row.get::<_, Option<String>>(9).map_err(|_error| ())?;
    let lease = row.get::<_, Option<i64>>(10).map_err(|_error| ())?;
    let owner_transition = row.get::<_, Option<String>>(12).map_err(|_error| ())?;
    let owner_receipt = row.get::<_, Option<String>>(13).map_err(|_error| ())?;
    let recovery_receipt = row.get::<_, Option<String>>(20).map_err(|_error| ())?;
    let recovery_transition = row.get::<_, Option<String>>(21).map_err(|_error| ())?;
    Ok(handoff_state_valid(
        state,
        recovery_state,
        row.get::<_, i64>(7).map_err(|_error| ())?,
        attempt.as_deref(),
        lease,
        owner_transition.as_deref(),
        owner_receipt.as_deref(),
    ) && owner_transition == recovery_transition
        && owner_receipt == recovery_receipt)
}

fn handoff_state_valid(
    state: &str,
    recovery_state: &str,
    requested: i64,
    attempt: Option<&str>,
    lease: Option<i64>,
    owner_transition: Option<&str>,
    owner_receipt: Option<&str>,
) -> bool {
    let owner_pair = match (owner_transition, owner_receipt) {
        (Some(transition), Some(receipt)) => !transition.trim().is_empty() && hex_digest(receipt),
        (None, None) => false,
        _ => return false,
    };
    match state {
        "pending" => {
            recovery_state == "approved" && attempt.is_none() && lease.is_none() && !owner_pair
        }
        "in-flight" => {
            recovery_state == "approved"
                && attempt.is_some_and(|value| !value.trim().is_empty())
                && lease.is_some_and(|value| value > requested)
                && !owner_pair
        }
        "delivered" => {
            recovery_state == "completed" && attempt.is_none() && lease.is_none() && owner_pair
        }
        _ => false,
    }
}
