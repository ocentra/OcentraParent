use rusqlite::Connection;

use super::schema_recovery_state::{effect_matches_kind, state_valid};

pub(super) fn validate(connection: &Connection) -> Result<(), ()> {
    let mut statement = connection
        .prepare(
            "SELECT household_id, account_id, requester_member_id, requester_device_id,
                    requester_role, kind, support_channel, identity_proof_id,
                    identity_proof_provider, identity_proof_subject,
                    identity_proof_expires_at_epoch_millis, identity_proof_state,
                    support_authorization_id, support_authorization_issuer,
                    support_authorization_scope, support_authorization_expires_at_epoch_millis,
                    owner_effect_kind, state, created_at_epoch_millis,
                    last_transition_at_epoch_millis,
                    reserved_owner_receipt_id, reserved_owner_transition_id,
                    reserved_owner_receipt_expires_at_epoch_millis
             FROM account_identity_recovery",
        )
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    while let Some(row) = rows.next().map_err(|_| ())? {
        if !identity_valid(row)? || !support_valid(row)? || !effect_and_state_valid(row)? {
            return Err(());
        }
    }
    Ok(())
}

fn identity_valid(row: &rusqlite::Row<'_>) -> Result<bool, ()> {
    let strings = [
        row.get::<_, String>(0).map_err(|_| ())?,
        row.get::<_, String>(1).map_err(|_| ())?,
        row.get::<_, String>(2).map_err(|_| ())?,
        row.get::<_, String>(3).map_err(|_| ())?,
        row.get::<_, String>(7).map_err(|_| ())?,
        row.get::<_, String>(9).map_err(|_| ())?,
    ];
    Ok(strings.iter().all(|value| !value.trim().is_empty())
        && matches!(
            row.get::<_, String>(4).map_err(|_| ())?.as_str(),
            "parent-owner"
                | "co-parent-guardian"
                | "observer"
                | "child-device-agent"
                | "support-admin"
        )
        && matches!(
            row.get::<_, String>(5).map_err(|_| ())?.as_str(),
            "forgot-login"
                | "lost-parent-device"
                | "compromised-account"
                | "child-reinstall"
                | "household-transfer"
        )
        && matches!(
            row.get::<_, String>(6).map_err(|_| ())?.as_str(),
            "self-serve" | "household-owner-assisted" | "support-assisted"
        )
        && matches!(
            row.get::<_, String>(8).map_err(|_| ())?.as_str(),
            "authjs" | "firebase"
        )
        && row.get::<_, String>(11).map_err(|_| ())? == "verified")
}

fn support_valid(row: &rusqlite::Row<'_>) -> Result<bool, ()> {
    Ok(super::schema_recovery_support::valid(
        &row.get::<_, String>(5).map_err(|_| ())?,
        &row.get::<_, String>(6).map_err(|_| ())?,
        row.get::<_, Option<String>>(12).map_err(|_| ())?.as_deref(),
        row.get::<_, Option<String>>(13).map_err(|_| ())?.as_deref(),
        row.get::<_, Option<String>>(14).map_err(|_| ())?.as_deref(),
        row.get::<_, Option<i64>>(15).map_err(|_| ())?,
    ))
}

fn effect_and_state_valid(row: &rusqlite::Row<'_>) -> Result<bool, ()> {
    let kind = row.get::<_, String>(5).map_err(|_| ())?;
    let effect = row.get::<_, i64>(16).map_err(|_| ())?;
    let state = row.get::<_, String>(17).map_err(|_| ())?;
    let created = row.get::<_, i64>(18).map_err(|_| ())?;
    let transition = row.get::<_, i64>(19).map_err(|_| ())?;
    let receipt = row.get::<_, Option<String>>(20).map_err(|_| ())?;
    let owner_transition = row.get::<_, Option<String>>(21).map_err(|_| ())?;
    Ok(row.get::<_, i64>(10).map_err(|_| ())? > created
        && created > 0
        && transition >= created
        && (1..=4).contains(&effect)
        && effect_matches_kind(&kind, effect)
        && state_valid(&state, receipt.as_deref(), owner_transition.as_deref())
        && row
            .get::<_, Option<i64>>(22)
            .map_err(|_| ())?
            .is_none_or(|expiry| expiry > 0))
}
