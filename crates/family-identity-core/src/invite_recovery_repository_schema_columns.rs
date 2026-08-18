use rusqlite::Connection;

const CLOCK_COLUMNS: &[(&str, &str, i64, i64)] = &[
    ("clock_id", "INTEGER", 0, 1),
    ("last_epoch_millis", "INTEGER", 1, 0),
];
const MUTATION_EFFECT_COLUMNS: &[(&str, &str, i64, i64)] = &[
    ("account_id", "TEXT", 1, 1),
    ("household_id", "TEXT", 1, 2),
    ("action", "TEXT", 1, 3),
    ("target_kind", "TEXT", 1, 4),
    ("target_id", "TEXT", 1, 5),
    ("idempotency_key", "TEXT", 1, 6),
    ("payload_digest", "TEXT", 1, 0),
    ("key_id", "TEXT", 1, 0),
    ("token_expires_at_epoch_millis", "INTEGER", 1, 0),
    ("status", "TEXT", 1, 0),
    ("result_code", "TEXT", 0, 0),
    ("created_at_epoch_millis", "INTEGER", 1, 0),
    ("updated_at_epoch_millis", "INTEGER", 1, 0),
    ("completed_at_epoch_millis", "INTEGER", 0, 0),
    ("retain_until_epoch_millis", "INTEGER", 1, 0),
];
const INVITE_COLUMNS: &[(&str, &str, i64, i64)] = &[
    ("invite_id", "TEXT", 0, 1),
    ("token_digest", "TEXT", 1, 0),
    ("household_id", "TEXT", 1, 0),
    ("inviter_account_id", "TEXT", 1, 0),
    ("inviter_member_id", "TEXT", 1, 0),
    ("inviter_device_id", "TEXT", 1, 0),
    ("inviter_authority_generation", "INTEGER", 1, 0),
    ("inviter_session_generation", "INTEGER", 1, 0),
    ("inviter_role", "TEXT", 1, 0),
    ("purpose", "TEXT", 1, 0),
    ("target_role", "TEXT", 1, 0),
    ("recipient_provider", "TEXT", 1, 0),
    ("recipient_provider_subject", "TEXT", 1, 0),
    ("recipient_account_id", "TEXT", 1, 0),
    ("invitee_email_digest", "TEXT", 1, 0),
    ("issued_at_epoch_millis", "INTEGER", 1, 0),
    ("expires_at_epoch_millis", "INTEGER", 1, 0),
    ("state", "TEXT", 1, 0),
    ("accepted_at_epoch_millis", "INTEGER", 0, 0),
    ("revoked_at_epoch_millis", "INTEGER", 0, 0),
    ("use_count", "INTEGER", 1, 0),
];
const PENDING_COLUMNS: &[(&str, &str, i64, i64)] = &[
    ("invite_id", "TEXT", 0, 1),
    ("household_id", "TEXT", 1, 0),
    ("recipient_provider", "TEXT", 1, 0),
    ("recipient_provider_subject", "TEXT", 1, 0),
    ("recipient_account_id", "TEXT", 1, 0),
    ("target_role", "TEXT", 1, 0),
    ("state", "TEXT", 1, 0),
    ("created_at_epoch_millis", "INTEGER", 1, 0),
    ("active_attempt_id", "TEXT", 0, 0),
    ("lease_expires_at_epoch_millis", "INTEGER", 0, 0),
    ("attempt_count", "INTEGER", 1, 0),
];
const RECOVERY_COLUMNS: &[(&str, &str, i64, i64)] = &[
    ("recovery_id", "TEXT", 0, 1),
    ("household_id", "TEXT", 1, 0),
    ("account_id", "TEXT", 1, 0),
    ("requester_member_id", "TEXT", 1, 0),
    ("requester_device_id", "TEXT", 1, 0),
    ("requester_role", "TEXT", 1, 0),
    ("kind", "TEXT", 1, 0),
    ("support_channel", "TEXT", 1, 0),
    ("identity_proof_id", "TEXT", 1, 0),
    ("identity_proof_provider", "TEXT", 1, 0),
    ("identity_proof_subject", "TEXT", 1, 0),
    ("identity_proof_expires_at_epoch_millis", "INTEGER", 1, 0),
    ("identity_proof_state", "TEXT", 1, 0),
    ("support_authorization_id", "TEXT", 0, 0),
    ("support_authorization_issuer", "TEXT", 0, 0),
    ("support_authorization_scope", "TEXT", 0, 0),
    (
        "support_authorization_expires_at_epoch_millis",
        "INTEGER",
        0,
        0,
    ),
    ("owner_effect_kind", "INTEGER", 1, 0),
    ("state", "TEXT", 1, 0),
    ("created_at_epoch_millis", "INTEGER", 1, 0),
    ("last_transition_at_epoch_millis", "INTEGER", 1, 0),
    ("reserved_owner_receipt_id", "TEXT", 0, 0),
    ("reserved_owner_transition_id", "TEXT", 0, 0),
    (
        "reserved_owner_receipt_expires_at_epoch_millis",
        "INTEGER",
        0,
        0,
    ),
];
const RATE_COLUMNS: &[(&str, &str, i64, i64)] = &[
    ("subject_digest", "TEXT", 0, 1),
    ("window_started_at_epoch_millis", "INTEGER", 1, 0),
    ("attempt_count", "INTEGER", 1, 0),
];
const HANDOFF_COLUMNS: &[(&str, &str, i64, i64)] = &[
    ("handoff_id", "TEXT", 0, 1),
    ("correlation_id", "TEXT", 1, 0),
    ("recovery_id", "TEXT", 1, 0),
    ("household_id", "TEXT", 1, 0),
    ("account_id", "TEXT", 1, 0),
    ("member_id", "TEXT", 1, 0),
    ("device_id", "TEXT", 1, 0),
    ("kind", "TEXT", 1, 0),
    ("requested_at_epoch_millis", "INTEGER", 1, 0),
    ("state", "TEXT", 1, 0),
    ("active_attempt_id", "TEXT", 0, 0),
    ("lease_expires_at_epoch_millis", "INTEGER", 0, 0),
    ("attempt_count", "INTEGER", 1, 0),
    ("owner_transition_id", "TEXT", 0, 0),
    ("owner_receipt_digest", "TEXT", 0, 0),
];

pub(super) fn validate_tables(connection: &Connection) -> Result<(), ()> {
    validate_table(connection, "account_identity_runtime_clock", CLOCK_COLUMNS)?;
    validate_table(
        connection,
        "account_identity_mutation_effect",
        MUTATION_EFFECT_COLUMNS,
    )?;
    validate_table(connection, "account_identity_setup_invite", INVITE_COLUMNS)?;
    validate_table(
        connection,
        "account_identity_pending_invite_membership",
        PENDING_COLUMNS,
    )?;
    validate_table(connection, "account_identity_recovery", RECOVERY_COLUMNS)?;
    validate_table(
        connection,
        "account_identity_recovery_rate_limit",
        RATE_COLUMNS,
    )?;
    validate_table(
        connection,
        "account_identity_invite_rate_limit",
        RATE_COLUMNS,
    )?;
    validate_table(
        connection,
        "account_identity_recovery_custody_handoff",
        HANDOFF_COLUMNS,
    )
}

fn validate_table(
    connection: &Connection,
    table: &str,
    expected: &[(&str, &str, i64, i64)],
) -> Result<(), ()> {
    let mut statement = connection
        .prepare(&format!("PRAGMA table_info('{table}')"))
        .map_err(|_| ())?;
    let mut rows = statement.query([]).map_err(|_| ())?;
    let mut index = 0;
    while let Some(row) = rows.next().map_err(|_| ())? {
        let Some(expected_column) = expected.get(index) else {
            return Err(());
        };
        if row.get::<_, String>(1).map_err(|_| ())? != expected_column.0
            || row
                .get::<_, String>(2)
                .map_err(|_| ())?
                .to_ascii_uppercase()
                != expected_column.1
            || row.get::<_, i64>(3).map_err(|_| ())? != expected_column.2
            || row.get::<_, i64>(5).map_err(|_| ())? != expected_column.3
        {
            return Err(());
        }
        index += 1;
    }
    (index == expected.len()).then_some(()).ok_or(())
}
