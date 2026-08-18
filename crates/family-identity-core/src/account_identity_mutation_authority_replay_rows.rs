use rusqlite::Connection;

pub(super) fn validate(connection: &Connection) -> Result<(), ()> {
    let invalid = connection
        .query_row(
            "SELECT COUNT(*) FROM account_identity_mutation_authority_replay
             WHERE length(payload_digest) != 71
                OR substr(payload_digest, 1, 7) != 'sha256:'
                OR substr(payload_digest, 8) GLOB '*[^0-9a-f]*'
                OR length(trim(idempotency_key)) = 0
                OR length(idempotency_key) > 256
                OR length(key_id) != 71
                OR substr(key_id, 1, 7) != 'sha256:'
                OR substr(key_id, 8) GLOB '*[^0-9a-f]*'
                OR consumed_at_epoch_millis <= 0",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| ())?;
    (invalid == 0).then_some(()).ok_or(())
}
