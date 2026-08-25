use rusqlite::Connection;

pub(super) fn validate(connection: &Connection) -> Result<(), ()> {
    let mut statement = connection
        .prepare(
            "SELECT p.household_id, p.recipient_provider, p.recipient_provider_subject,
                    p.recipient_account_id, p.target_role, p.state, p.created_at_epoch_millis,
                    p.active_attempt_id, p.lease_expires_at_epoch_millis, p.attempt_count,
                    i.state, i.household_id, i.recipient_provider,
                    i.recipient_provider_subject, i.recipient_account_id, i.target_role
             FROM account_identity_pending_invite_membership p
             JOIN account_identity_setup_invite i ON i.invite_id = p.invite_id",
        )
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    while let Some(row) = rows.next().map_err(|_| ())? {
        let state = row.get::<_, String>(5).map_err(|_| ())?;
        let created = row.get::<_, i64>(6).map_err(|_| ())?;
        let attempt = row.get::<_, Option<String>>(7).map_err(|_| ())?;
        let lease = row.get::<_, Option<i64>>(8).map_err(|_| ())?;
        let count = row.get::<_, i64>(9).map_err(|_| ())?;
        if !identity_valid(row)?
            || !invite_binding_valid(row)?
            || created <= 0
            || count < 0
            || !state_valid(&state, created, attempt.as_deref(), lease)
        {
            return Err(());
        }
    }
    Ok(())
}

fn identity_valid(row: &rusqlite::Row<'_>) -> Result<bool, ()> {
    let strings = [
        row.get::<_, String>(0).map_err(|_| ())?,
        row.get::<_, String>(2).map_err(|_| ())?,
        row.get::<_, String>(3).map_err(|_| ())?,
    ];
    Ok(strings.iter().all(|value| !value.trim().is_empty())
        && matches!(
            row.get::<_, String>(1).map_err(|_| ())?.as_str(),
            "authjs" | "firebase"
        )
        && matches!(
            row.get::<_, String>(4).map_err(|_| ())?.as_str(),
            "co-parent-guardian" | "observer" | "child-device-agent" | "parent-owner"
        )
        && matches!(
            row.get::<_, String>(5).map_err(|_| ())?.as_str(),
            "pending" | "in-flight" | "committed" | "rejected"
        ))
}

fn invite_binding_valid(row: &rusqlite::Row<'_>) -> Result<bool, ()> {
    Ok(row.get::<_, String>(10).map_err(|_| ())? == "accepted"
        && row.get::<_, String>(0).map_err(|_| ())? == row.get::<_, String>(11).map_err(|_| ())?
        && row.get::<_, String>(1).map_err(|_| ())? == row.get::<_, String>(12).map_err(|_| ())?
        && row.get::<_, String>(2).map_err(|_| ())? == row.get::<_, String>(13).map_err(|_| ())?
        && row.get::<_, String>(3).map_err(|_| ())? == row.get::<_, String>(14).map_err(|_| ())?
        && row.get::<_, String>(4).map_err(|_| ())? == row.get::<_, String>(15).map_err(|_| ())?)
}

fn state_valid(state: &str, created: i64, attempt: Option<&str>, lease: Option<i64>) -> bool {
    match state {
        "pending" => attempt.is_none() && lease.is_none(),
        "in-flight" => {
            attempt.is_some_and(|value| !value.trim().is_empty())
                && lease.is_some_and(|value| value > created)
        }
        "committed" | "rejected" => attempt.is_none() && lease.is_none(),
        _ => false,
    }
}
