use super::*;

use super::*;

pub(crate) fn opaque_id(prefix: &str) -> Result<String, InviteRecoveryRepositoryError> {
    let mut bytes = [0_u8; 16];
    fill(&mut bytes).map_err(|_| InviteRecoveryRepositoryError::EntropyUnavailable)?;
    let mut value = String::with_capacity(prefix.len() + bytes.len() * 2);
    value.push_str(prefix);
    for byte in bytes {
        value.push_str(&format!("{byte:02x}"));
    }
    Ok(value)
}

pub(crate) fn digest_token(token: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(INVITE_TOKEN_DIGEST_DOMAIN);
    digest.update(token.as_bytes());
    digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub(crate) fn digest_email(email: &str) -> String {
    let email = email.trim().to_ascii_lowercase();
    let mut digest = Sha256::new();
    digest.update(b"ocentra-account-invitee-email-v1");
    digest.update(email.as_bytes());
    digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub(crate) fn enforce_recovery_rate_limit(
    transaction: &Transaction<'_>,
    provider: &AccountIdentityProvider,
    subject: &AccountIdentityProviderSubject,
    now: i64,
) -> Result<(), InviteRecoveryRepositoryError> {
    const WINDOW_MILLIS: i64 = 15 * 60 * 1_000;
    const MAX_ATTEMPTS: i64 = 5;
    let mut digest = Sha256::new();
    digest.update(b"ocentra-account-recovery-rate-v1");
    digest.update(provider_label(provider).as_bytes());
    digest.update(subject.as_str().as_bytes());
    let subject_digest = digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let existing = transaction
        .query_row(
            "SELECT window_started_at_epoch_millis, attempt_count
             FROM account_identity_recovery_rate_limit WHERE subject_digest = ?1",
            params![subject_digest],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
        )
        .optional()
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
    match existing {
        None => {
            transaction
                .execute(
                    "INSERT INTO account_identity_recovery_rate_limit
                     (subject_digest, window_started_at_epoch_millis, attempt_count)
                     VALUES (?1, ?2, 1)",
                    params![subject_digest, now],
                )
                .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        }
        Some((window_started, attempts)) if now.saturating_sub(window_started) >= WINDOW_MILLIS => {
            transaction
                .execute(
                    "UPDATE account_identity_recovery_rate_limit
                     SET window_started_at_epoch_millis = ?2, attempt_count = 1
                     WHERE subject_digest = ?1",
                    params![subject_digest, now],
                )
                .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        }
        Some((_window_started, attempts)) if attempts >= MAX_ATTEMPTS => {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected);
        }
        Some((_window_started, _attempts)) => {
            transaction
                .execute(
                    "UPDATE account_identity_recovery_rate_limit
                     SET attempt_count = attempt_count + 1 WHERE subject_digest = ?1",
                    params![subject_digest],
                )
                .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        }
    }
    Ok(())
}

pub(crate) fn recovery_requires_custody_handoff(kind: RecoveryKind) -> bool {
    matches!(
        kind,
        RecoveryKind::ForgotLogin
            | RecoveryKind::LostParentDevice
            | RecoveryKind::CompromisedAccount
            | RecoveryKind::ChildReinstall
            | RecoveryKind::HouseholdTransfer
    )
}

pub(crate) fn owner_effect(kind: RecoveryKind) -> RecoveryOwnerEffect {
    match kind {
        RecoveryKind::ForgotLogin => RecoveryOwnerEffect::ProviderCredentialSession,
        RecoveryKind::LostParentDevice | RecoveryKind::CompromisedAccount => {
            RecoveryOwnerEffect::DeviceTrustRevoke
        }
        RecoveryKind::ChildReinstall => RecoveryOwnerEffect::DeviceTrustReinstall,
        RecoveryKind::HouseholdTransfer => RecoveryOwnerEffect::HouseholdAuthorityMutation,
    }
}

pub(crate) fn owner_effect_label(effect: RecoveryOwnerEffect) -> &'static str {
    match effect {
        RecoveryOwnerEffect::ProviderCredentialSession => "provider-credential-session",
        RecoveryOwnerEffect::DeviceTrustRevoke => "device-trust-revoke",
        RecoveryOwnerEffect::DeviceTrustReinstall => "device-trust-reinstall",
        RecoveryOwnerEffect::HouseholdAuthorityMutation => "household-authority-mutation",
        RecoveryOwnerEffect::DataCustodyExportDelete => "data-custody-export-delete",
    }
}
