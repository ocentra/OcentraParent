use ocentra_family_identity_core::account_identity_authority_repository::invite_recovery_repository::
    {RecoveryOwnerEffect, INVITE_RECOVERY_SCHEMA_SQL};
use rusqlite::{params, Connection};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn effect_taxonomy_remains_explicit_and_non_collapsible() {
    let effects = [
        RecoveryOwnerEffect::ProviderCredentialSession,
        RecoveryOwnerEffect::DeviceTrustRevoke,
        RecoveryOwnerEffect::DeviceTrustReinstall,
        RecoveryOwnerEffect::HouseholdAuthorityMutation,
    ];
    let labels = effects
        .into_iter()
        .map(|effect| format!("{effect:?}"))
        .collect::<Vec<_>>();

    assert_eq!(
        labels,
        vec![
            "ProviderCredentialSession".to_owned(),
            "DeviceTrustRevoke".to_owned(),
            "DeviceTrustReinstall".to_owned(),
            "HouseholdAuthorityMutation".to_owned(),
        ]
    );
}

#[test]
fn owner_receipt_storage_accepts_only_lowercase_sha256_digests() -> TestResult {
    let connection = Connection::open_in_memory()?;
    connection.execute_batch(INVITE_RECOVERY_SCHEMA_SQL)?;
    let cases = [
        ("lowercase-a", "a".repeat(64), "accepted"),
        ("lowercase-mixed", "0123456789abcdef".repeat(4), "accepted"),
        ("uppercase", "A".repeat(64), "rejected"),
        ("short", "0".repeat(63), "rejected"),
        ("non-hex", format!("{}g", "0".repeat(63)), "rejected"),
    ];
    let actual = cases
        .iter()
        .map(|(label, digest, _)| insert_delivered_receipt(&connection, label, digest))
        .map(|result| receipt_storage_result(&result))
        .collect::<Vec<_>>();
    let expected = cases
        .iter()
        .map(|(_, _, expected)| *expected)
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
    Ok(())
}

fn insert_delivered_receipt(
    connection: &Connection,
    label: &str,
    receipt_digest: &str,
) -> rusqlite::Result<usize> {
    insert_recovery(connection, label)?;
    connection.execute(
        "INSERT INTO account_identity_recovery_custody_handoff (
             handoff_id, correlation_id, recovery_id, household_id, account_id,
             member_id, device_id, kind, requested_at_epoch_millis, state,
             active_attempt_id, lease_expires_at_epoch_millis, attempt_count,
             owner_transition_id, owner_receipt_digest
         ) VALUES (?1, ?2, ?3, 'household-1', 'account-1', 'member-1',
                   'device-1', 'forgot-login', 1, 'delivered', NULL, NULL, 1,
                   'transition-1', ?4)",
        params![
            format!("handoff-{label}"),
            format!("correlation-{label}"),
            format!("recovery-{label}"),
            receipt_digest,
        ],
    )
}

fn insert_recovery(connection: &Connection, label: &str) -> rusqlite::Result<usize> {
    connection.execute(
        "INSERT INTO account_identity_recovery (
             recovery_id, household_id, account_id, requester_member_id,
             requester_device_id, requester_role, kind, support_channel,
             identity_proof_id, identity_proof_provider, identity_proof_subject,
             identity_proof_expires_at_epoch_millis, identity_proof_state,
             owner_effect_kind, state, created_at_epoch_millis,
             last_transition_at_epoch_millis
         ) VALUES (?1, 'household-1', 'account-1', 'member-1', 'device-1',
                   'parent-owner', 'forgot-login', 'self-serve', ?2, 'authjs',
                   ?3, 2, 'verified', 1, 'completed', 1, 1)",
        params![
            format!("recovery-{label}"),
            format!("identity-proof-{label}"),
            format!("provider-subject-{label}"),
        ],
    )
}

fn receipt_storage_result(result: &rusqlite::Result<usize>) -> &'static str {
    match result {
        Ok(1) => "accepted",
        Ok(_) => "unexpected-row-count",
        Err(_) => "rejected",
    }
}
