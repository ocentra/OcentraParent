use rusqlite::Connection;

use super::hex_digest;

pub(super) fn validate(connection: &Connection) -> Result<(), ()> {
    let mut statement = connection
        .prepare(
            "SELECT invite_id, token_digest, household_id, inviter_account_id,
                    inviter_member_id, inviter_device_id, inviter_authority_generation,
                    inviter_session_generation, inviter_role, purpose, target_role,
                    recipient_provider, recipient_provider_subject, recipient_account_id,
                    invitee_email_digest, issued_at_epoch_millis, expires_at_epoch_millis,
                    state, accepted_at_epoch_millis, revoked_at_epoch_millis, use_count
             FROM account_identity_setup_invite",
        )
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    while let Some(row) = rows.next().map_err(|_| ())? {
        let state = row.get::<_, String>(17).map_err(|_| ())?;
        let issued = row.get::<_, i64>(15).map_err(|_| ())?;
        let expires = row.get::<_, i64>(16).map_err(|_| ())?;
        let accepted = row.get::<_, Option<i64>>(18).map_err(|_| ())?;
        let revoked = row.get::<_, Option<i64>>(19).map_err(|_| ())?;
        let use_count = row.get::<_, i64>(20).map_err(|_| ())?;
        if !identity_is_valid(row)?
            || issued <= 0
            || expires <= issued
            || !state_is_valid(&state, issued, expires, accepted, revoked, use_count)
        {
            return Err(());
        }
    }
    Ok(())
}

fn identity_is_valid(row: &rusqlite::Row<'_>) -> Result<bool, ()> {
    let strings = [
        row.get::<_, String>(0).map_err(|_| ())?,
        row.get::<_, String>(2).map_err(|_| ())?,
        row.get::<_, String>(3).map_err(|_| ())?,
        row.get::<_, String>(4).map_err(|_| ())?,
        row.get::<_, String>(5).map_err(|_| ())?,
        row.get::<_, String>(12).map_err(|_| ())?,
        row.get::<_, String>(13).map_err(|_| ())?,
    ];
    if strings.iter().any(|value| value.trim().is_empty()) {
        return Ok(false);
    }
    let provider = row.get::<_, String>(11).map_err(|_| ())?;
    let inviter_role = row.get::<_, String>(8).map_err(|_| ())?;
    let purpose = row.get::<_, String>(9).map_err(|_| ())?;
    let target = row.get::<_, String>(10).map_err(|_| ())?;
    let token_digest = row.get::<_, String>(1).map_err(|_| ())?;
    let email_digest = row.get::<_, String>(14).map_err(|_| ())?;
    Ok(matches!(provider.as_str(), "authjs" | "firebase")
        && matches!(inviter_role.as_str(), "parent-owner" | "co-parent-guardian")
        && purpose_matches_target(&purpose, &target)
        && hex_digest(&token_digest)
        && hex_digest(&email_digest)
        && row.get::<_, i64>(6).map_err(|_| ())? > 0
        && row.get::<_, i64>(7).map_err(|_| ())? > 0)
}

fn purpose_matches_target(purpose: &str, target_role: &str) -> bool {
    matches!(
        (purpose, target_role),
        ("co-parent-invite", "co-parent-guardian")
            | ("observer-invite", "observer")
            | ("child-device-pairing", "child-device-agent")
            | ("household-transfer", "parent-owner")
    )
}

fn state_is_valid(
    state: &str,
    issued: i64,
    expires: i64,
    accepted: Option<i64>,
    revoked: Option<i64>,
    use_count: i64,
) -> bool {
    match state {
        "pending" => accepted.is_none() && revoked.is_none() && use_count == 0,
        "accepted" => {
            accepted.is_some_and(|value| value >= issued && value <= expires)
                && revoked.is_none()
                && use_count == 1
        }
        "expired" => accepted.is_none() && revoked.is_none() && use_count == 0,
        "revoked" => {
            revoked.is_some_and(|value| value >= issued && value <= expires)
                && accepted.is_none()
                && use_count == 0
        }
        _ => false,
    }
}
