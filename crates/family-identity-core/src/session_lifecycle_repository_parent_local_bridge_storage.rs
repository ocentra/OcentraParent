#![forbid(unsafe_code)]

//! Durable row encoding and fail-closed decoding for parent-local bridges.

use ocentra_schema::account_identity_authority::{
    AccountIdentityDeviceId, AccountIdentityMemberId, AccountIdentityProviderSubject,
    AccountIdentitySessionId,
};
use ocentra_schema::account_identity_parent_local_bridge::AccountIdentityParentLocalBridgeAudience;
use ocentra_schema::report_query_custody::{FamilyId, ParentAccountId};
use rusqlite::{OptionalExtension, Transaction};

use crate::session_lifecycle_custody::record::SessionAuthorityBinding;

use super::super::{labels, SessionLifecycleRepositoryError};
use super::{
    ParentLocalBridgeState, StoredParentLocalBridgeSession, ACTIVE_STATE, CONSUMED_STATE,
    DIGEST_ALGORITHM, REVOKED_STATE,
};
use crate::session_lifecycle_custody::parent_local_bridge::CAPABILITY_DIGEST_DOMAIN;

struct StoredParentLocalBridgeRow {
    capability_digest: String,
    digest_algorithm: String,
    capability_digest_domain: String,
    audience: String,
    connection_nonce_digest: String,
    account_id: String,
    provider: String,
    provider_subject: String,
    household_id: String,
    member_id: String,
    device_id: String,
    authority_session_id: String,
    authority_session_generation: i64,
    authority_generation: i64,
    authority_expires_at_epoch_millis: i64,
    issued_at_epoch_millis: i64,
    expires_at_epoch_millis: i64,
    global_revoke_epoch: i64,
    state: String,
    last_transition_at_epoch_millis: i64,
}

pub(super) fn read_record(
    transaction: &Transaction<'_>,
    capability_digest: &str,
) -> Result<Option<StoredParentLocalBridgeSession>, SessionLifecycleRepositoryError> {
    transaction
        .query_row(
            "SELECT capability_digest, digest_algorithm, capability_digest_domain,
                    audience, connection_nonce_digest, account_id, provider,
                    provider_subject, household_id, member_id, device_id,
                    authority_session_id, authority_session_generation,
                    authority_generation, authority_expires_at_epoch_millis,
                    issued_at_epoch_millis, expires_at_epoch_millis,
                    global_revoke_epoch, state, last_transition_at_epoch_millis
             FROM account_identity_parent_local_bridge_session
            WHERE capability_digest = ?1 LIMIT 1",
            [capability_digest],
            |row| {
                Ok(StoredParentLocalBridgeRow {
                    capability_digest: row.get(0)?,
                    digest_algorithm: row.get(1)?,
                    capability_digest_domain: row.get(2)?,
                    audience: row.get(3)?,
                    connection_nonce_digest: row.get(4)?,
                    account_id: row.get(5)?,
                    provider: row.get(6)?,
                    provider_subject: row.get(7)?,
                    household_id: row.get(8)?,
                    member_id: row.get(9)?,
                    device_id: row.get(10)?,
                    authority_session_id: row.get(11)?,
                    authority_session_generation: row.get(12)?,
                    authority_generation: row.get(13)?,
                    authority_expires_at_epoch_millis: row.get(14)?,
                    issued_at_epoch_millis: row.get(15)?,
                    expires_at_epoch_millis: row.get(16)?,
                    global_revoke_epoch: row.get(17)?,
                    state: row.get(18)?,
                    last_transition_at_epoch_millis: row.get(19)?,
                })
            },
        )
        .optional()
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?
        .map(decode_record)
        .transpose()
}

fn decode_record(
    row: StoredParentLocalBridgeRow,
) -> Result<StoredParentLocalBridgeSession, SessionLifecycleRepositoryError> {
    validate_record_shape(&row)?;
    let audience = decode_audience(&row.audience)?;
    let state = decode_state(&row.state)?;
    let binding = decode_binding(&row)?;
    Ok(StoredParentLocalBridgeSession {
        capability_digest: row.capability_digest,
        audience,
        connection_nonce_digest: row.connection_nonce_digest,
        binding,
        issued_at_epoch_millis: row.issued_at_epoch_millis,
        expires_at_epoch_millis: row.expires_at_epoch_millis,
        global_revoke_epoch: positive_generation(row.global_revoke_epoch)?,
        state,
        last_transition_at_epoch_millis: row.last_transition_at_epoch_millis,
    })
}

fn validate_record_shape(
    row: &StoredParentLocalBridgeRow,
) -> Result<(), SessionLifecycleRepositoryError> {
    if row.digest_algorithm != DIGEST_ALGORITHM
        || row.capability_digest_domain != CAPABILITY_DIGEST_DOMAIN
        || !is_hex_digest(&row.capability_digest)
        || !is_hex_digest(&row.connection_nonce_digest)
        || row.issued_at_epoch_millis <= 0
        || row.expires_at_epoch_millis <= row.issued_at_epoch_millis
        || row.expires_at_epoch_millis > row.authority_expires_at_epoch_millis
        || row.global_revoke_epoch <= 0
        || row.last_transition_at_epoch_millis < row.issued_at_epoch_millis
        || ((row.state == ACTIVE_STATE
            && row.last_transition_at_epoch_millis != row.issued_at_epoch_millis)
            || (row.state != ACTIVE_STATE
                && row.last_transition_at_epoch_millis <= row.issued_at_epoch_millis))
    {
        return Err(SessionLifecycleRepositoryError::InvalidStoredSession);
    }
    Ok(())
}

fn decode_audience(
    audience: &str,
) -> Result<AccountIdentityParentLocalBridgeAudience, SessionLifecycleRepositoryError> {
    match audience {
        "parent-desktop-agent-service" => {
            Ok(AccountIdentityParentLocalBridgeAudience::ParentDesktopAgentService)
        }
        _ => Err(SessionLifecycleRepositoryError::InvalidStoredSession),
    }
}

fn decode_state(state: &str) -> Result<ParentLocalBridgeState, SessionLifecycleRepositoryError> {
    match state {
        ACTIVE_STATE => Ok(ParentLocalBridgeState::Active),
        CONSUMED_STATE => Ok(ParentLocalBridgeState::Consumed),
        REVOKED_STATE => Ok(ParentLocalBridgeState::Revoked),
        _ => Err(SessionLifecycleRepositoryError::InvalidStoredSession),
    }
}

fn decode_binding(
    row: &StoredParentLocalBridgeRow,
) -> Result<SessionAuthorityBinding, SessionLifecycleRepositoryError> {
    Ok(SessionAuthorityBinding {
        account_id: ParentAccountId::parse(row.account_id.clone())
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        provider: labels::parse_provider(row.provider.as_bytes())?,
        provider_subject: AccountIdentityProviderSubject::parse(row.provider_subject.clone())
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        household_id: FamilyId::parse(row.household_id.clone())
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        member_id: AccountIdentityMemberId::parse(row.member_id.clone())
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        device_id: AccountIdentityDeviceId::parse(row.device_id.clone())
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        authority_session_id: AccountIdentitySessionId::parse(row.authority_session_id.clone())
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        authority_session_generation: positive_generation(row.authority_session_generation)?,
        authority_generation: positive_generation(row.authority_generation)?,
        authority_expires_at_epoch_millis: row.authority_expires_at_epoch_millis,
    })
}

fn is_hex_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn positive_generation(value: i64) -> Result<u64, SessionLifecycleRepositoryError> {
    u64::try_from(value)
        .ok()
        .filter(|value| *value > 0)
        .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)
}
