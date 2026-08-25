use rusqlite::Connection;

pub(super) fn validate(connection: &Connection) -> Result<(), ()> {
    let invalid = connection
        .query_row(
            "SELECT COUNT(*) FROM account_identity_mutation_effect
             WHERE length(trim(account_id)) = 0
                OR length(trim(household_id)) = 0
                OR action NOT IN (
                    'revoke-child-device','revoke-setup-invite','revoke-recovery'
                )
                OR target_kind NOT IN ('child-device','setup-invite','recovery')
                OR length(trim(target_id)) = 0 OR length(target_id) > 256
                OR length(trim(idempotency_key)) = 0 OR length(idempotency_key) > 256
                OR length(payload_digest) != 71
                OR substr(payload_digest, 1, 7) != 'sha256:'
                OR substr(payload_digest, 8) GLOB '*[^0-9a-f]*'
                OR length(key_id) != 71
                OR substr(key_id, 1, 7) != 'sha256:'
                OR substr(key_id, 8) GLOB '*[^0-9a-f]*'
                OR token_expires_at_epoch_millis <= 0
                OR created_at_epoch_millis <= 0
                OR updated_at_epoch_millis < created_at_epoch_millis
                OR retain_until_epoch_millis <= token_expires_at_epoch_millis
                OR status NOT IN ('pending','completed')
                OR (status = 'pending' AND (
                    result_code IS NOT NULL OR completed_at_epoch_millis IS NOT NULL
                ))
                OR (status = 'completed' AND (
                    result_code IS NULL
                    OR result_code NOT IN ('setup-invite-revoked','recovery-revoked')
                    OR completed_at_epoch_millis != updated_at_epoch_millis
                ))",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| ())?;
    (invalid == 0).then_some(()).ok_or(())
}
