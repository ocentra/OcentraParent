use super::{support_invite::*, support_recovery::*, support_security::*, *};

pub(crate) fn ensure_current_authority(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    now: i64,
) -> Result<(), InviteRecoveryRepositoryError> {
    let expires_at = DateTime::parse_from_rfc3339(authority.session_expires_at())
        .map_err(|_| InviteRecoveryRepositoryError::AuthorityExpired)?
        .timestamp_millis();
    if expires_at <= now {
        return Err(InviteRecoveryRepositoryError::AuthorityExpired);
    }
    let row = transaction
        .query_row(
            "SELECT mapping_status, authority_generation, session_id,
                    session_generation, authority_json
             FROM account_identity_current_authority
             WHERE provider = ?1 AND provider_subject = ?2 LIMIT 1",
            params![
                provider_label(authority.provider()),
                authority.provider_subject().as_str()
            ],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        )
        .optional()
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?
        .ok_or(InviteRecoveryRepositoryError::AuthorityUnavailable)?;
    let handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff = serde_json::from_str(&row.4)
        .map_err(|_| InviteRecoveryRepositoryError::AuthorityNotCurrent)?;
    handoff
        .validate_shape()
        .map_err(|_| InviteRecoveryRepositoryError::AuthorityNotCurrent)?;
    if row.0 != "active"
        || handoff.mapping.status != AccountIdentityMappingStatus::Active
        || &handoff.mapping.provider != authority.provider()
        || &handoff.mapping.provider_subject != authority.provider_subject()
        || row.1 != authority.authority_generation() as i64
        || row.2 != authority.session_id().as_str()
        || row.3 != authority.session_generation() as i64
        || &handoff.member.account_id != authority.account_id()
        || &handoff.member.household_id != authority.household_id()
        || &handoff.member.member_id != authority.member_id()
        || &handoff.member.device_id != authority.device_id()
        || handoff.member.role != authority.role()
        || handoff.member.authority_generation != authority.authority_generation()
        || handoff.member.session_generation != authority.session_generation()
        || &handoff.member.session_id != authority.session_id()
        || &handoff.binding.child_profile_id != authority.child_profile_id()
        || &handoff.binding.child_device_id != authority.child_device_id()
    {
        return Err(InviteRecoveryRepositoryError::AuthorityNotCurrent);
    }
    Ok(())
}

pub(crate) fn next_transition_at(
    transaction: &Transaction<'_>,
    recovery_id: &RecoveryId,
    now: i64,
) -> Result<i64, InviteRecoveryRepositoryError> {
    let previous = transaction
        .query_row(
            "SELECT last_transition_at_epoch_millis
             FROM account_identity_recovery WHERE recovery_id = ?1 LIMIT 1",
            params![recovery_id.as_str()],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?
        .ok_or(InviteRecoveryRepositoryError::Missing)?;
    let next = previous
        .checked_add(1)
        .ok_or(InviteRecoveryRepositoryError::ClockUnavailable)?;
    Ok(now.max(next))
}

pub(crate) fn trusted_now_in_transaction(
    transaction: &Transaction<'_>,
) -> Result<(i64, String), InviteRecoveryRepositoryError> {
    let system_now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| InviteRecoveryRepositoryError::ClockUnavailable)?
        .as_millis();
    let system_now =
        i64::try_from(system_now).map_err(|_| InviteRecoveryRepositoryError::ClockUnavailable)?;
    let previous = transaction
        .query_row(
            "SELECT last_epoch_millis FROM account_identity_runtime_clock
             WHERE clock_id = 1 LIMIT 1",
            [],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
    const MAX_FORWARD_SKEW_MILLIS: i64 = 24 * 60 * 60 * 1_000;
    let now = match previous {
        None => system_now,
        Some(previous) => {
            let floor = previous
                .checked_add(1)
                .ok_or(InviteRecoveryRepositoryError::ClockUnavailable)?;
            let ceiling = previous
                .checked_add(MAX_FORWARD_SKEW_MILLIS)
                .ok_or(InviteRecoveryRepositoryError::ClockUnavailable)?;
            if system_now > ceiling {
                return Err(InviteRecoveryRepositoryError::ClockUnavailable);
            }
            system_now.max(floor)
        }
    };
    if now <= 0 {
        return Err(InviteRecoveryRepositoryError::ClockUnavailable);
    }
    transaction
        .execute(
            "INSERT INTO account_identity_runtime_clock (clock_id, last_epoch_millis)
             VALUES (1, ?1)
             ON CONFLICT(clock_id) DO UPDATE SET last_epoch_millis = excluded.last_epoch_millis",
            params![now],
        )
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
    Ok((now, timestamp(now)?))
}

fn timestamp(epoch_millis: i64) -> Result<String, InviteRecoveryRepositoryError> {
    DateTime::<Utc>::from_timestamp_millis(epoch_millis)
        .map(|value| value.to_rfc3339_opts(SecondsFormat::Millis, true))
        .ok_or(InviteRecoveryRepositoryError::InvalidInvite)
}
