#![forbid(unsafe_code)]

use ocentra_schema::account_identity_authority::{
    AccountIdentityDeviceId, AccountIdentityMemberId, AccountIdentityProviderSubject,
    AccountIdentitySessionId,
};
use ocentra_schema::report_query_custody::{FamilyId, ParentAccountId};
use rusqlite::{params, OptionalExtension, Row, Transaction};

use crate::session_lifecycle::SessionActivityState;
use crate::session_lifecycle_custody::record::{SessionAuthorityBinding, SessionCredentialRecord};
use crate::session_lifecycle_custody::storage_values::{
    SessionAccessDigest, SessionRefreshDigest, SessionRefreshFamilyId,
};
use crate::session_lifecycle_record::SessionId;

use super::{invariants, labels, SessionLifecycleRepositoryError};

const SESSION_SELECT: &str = "SELECT credential_class, digest_algorithm, access_digest_domain,
            refresh_digest_domain, access_digest, refresh_digest, session_id,
            account_id, provider, provider_subject, household_id, member_id,
            device_id, authority_session_id, authority_session_generation,
            authority_generation, authority_expires_at_epoch_millis,
            refresh_family_id, refresh_generation, issued_at_epoch_millis,
            access_expires_at_epoch_millis, refresh_expires_at_epoch_millis,
            fresh_until_epoch_millis, activity_state, global_revoke_epoch,
            last_transition_at_epoch_millis
     FROM account_identity_session";

struct StoredSessionRow {
    credential_class: String,
    digest_algorithm: String,
    access_digest_domain: String,
    refresh_digest_domain: String,
    access_digest: String,
    refresh_digest: String,
    session_id: String,
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
    refresh_family_id: String,
    refresh_generation: i64,
    issued_at_epoch_millis: i64,
    access_expires_at_epoch_millis: i64,
    refresh_expires_at_epoch_millis: i64,
    fresh_until_epoch_millis: i64,
    activity_state: String,
    global_revoke_epoch: i64,
    last_transition_at_epoch_millis: i64,
}

enum SessionLookup<'a> {
    Access(&'a SessionAccessDigest),
    Refresh(&'a SessionRefreshDigest),
}

pub(crate) fn read_by_access_digest(
    transaction: &Transaction<'_>,
    digest: &SessionAccessDigest,
) -> Result<Option<SessionCredentialRecord>, SessionLifecycleRepositoryError> {
    read_one(transaction, SessionLookup::Access(digest))
}

pub(crate) fn read_by_refresh_digest(
    transaction: &Transaction<'_>,
    digest: &SessionRefreshDigest,
) -> Result<Option<SessionCredentialRecord>, SessionLifecycleRepositoryError> {
    read_one(transaction, SessionLookup::Refresh(digest))
}

pub(crate) fn read_active_for_account(
    transaction: &Transaction<'_>,
    account_id: &ParentAccountId,
) -> Result<Vec<SessionCredentialRecord>, SessionLifecycleRepositoryError> {
    let mut statement = transaction
        .prepare(&format!(
            "{SESSION_SELECT} WHERE account_id = ?1 AND activity_state = 'active' ORDER BY session_id"
        ))
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
    let rows = statement
        .query_map([account_id.to_string()], decode_stored_row)
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
    rows.map(|row| {
        row.map_err(|_| SessionLifecycleRepositoryError::Unavailable)
            .and_then(stored_row_into_record)
    })
    .collect::<Result<Vec<_>, _>>()
}

pub(crate) fn insert_record(
    transaction: &Transaction<'_>,
    record: &SessionCredentialRecord,
) -> Result<(), SessionLifecycleRepositoryError> {
    invariants::validate_record(record)?;
    let changed = transaction
        .execute(
            "INSERT INTO account_identity_session (
                 credential_class, digest_algorithm, access_digest_domain,
                 refresh_digest_domain, access_digest, refresh_digest, session_id,
                 account_id, provider, provider_subject, household_id, member_id,
                 device_id, authority_session_id, authority_session_generation,
                 authority_generation, authority_expires_at_epoch_millis,
                 refresh_family_id, refresh_generation, issued_at_epoch_millis,
                 access_expires_at_epoch_millis, refresh_expires_at_epoch_millis,
                 fresh_until_epoch_millis, activity_state, global_revoke_epoch,
                 last_transition_at_epoch_millis
             ) VALUES (
                 ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13,
                 ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26
             )",
            record_params(record)?,
        )
        .map_err(|_| SessionLifecycleRepositoryError::CurrentnessConflict)?;
    (changed == 1)
        .then_some(())
        .ok_or(SessionLifecycleRepositoryError::CurrentnessConflict)
}

pub(crate) fn rotate_record(
    transaction: &Transaction<'_>,
    current: &SessionCredentialRecord,
    next: &SessionCredentialRecord,
) -> Result<(), SessionLifecycleRepositoryError> {
    invariants::validate_record(current)?;
    invariants::validate_record(next)?;
    validate_rotation_transition(current, next)?;
    let changed = transaction
        .execute(
            "UPDATE account_identity_session SET
                 credential_class = ?1, digest_algorithm = ?2,
                 access_digest_domain = ?3, refresh_digest_domain = ?4,
                 access_digest = ?5, refresh_digest = ?6, session_id = ?7,
                 account_id = ?8, provider = ?9, provider_subject = ?10,
                 household_id = ?11, member_id = ?12, device_id = ?13,
                 authority_session_id = ?14, authority_session_generation = ?15,
                 authority_generation = ?16, authority_expires_at_epoch_millis = ?17,
                 refresh_family_id = ?18, refresh_generation = ?19,
                 issued_at_epoch_millis = ?20, access_expires_at_epoch_millis = ?21,
                 refresh_expires_at_epoch_millis = ?22, fresh_until_epoch_millis = ?23,
                 activity_state = ?24, global_revoke_epoch = ?25,
                 last_transition_at_epoch_millis = ?26
             WHERE access_digest = ?27 AND refresh_digest = ?28
               AND refresh_generation = ?29 AND activity_state = 'active'
               AND global_revoke_epoch = ?30 AND last_transition_at_epoch_millis = ?31",
            rusqlite::params_from_iter(record_param_values(next)?.into_iter().chain([
                rusqlite::types::Value::Text(current.access_digest.as_str().to_owned()),
                rusqlite::types::Value::Text(current.refresh_digest.as_str().to_owned()),
                rusqlite::types::Value::Integer(to_sql_generation(current.refresh_generation)?),
                rusqlite::types::Value::Integer(to_sql_generation(current.global_revoke_epoch)?),
                rusqlite::types::Value::Integer(current.last_transition_at_epoch_millis),
            ])),
        )
        .map_err(|_| SessionLifecycleRepositoryError::CurrentnessConflict)?;
    (changed == 1)
        .then_some(())
        .ok_or(SessionLifecycleRepositoryError::CurrentnessConflict)
}

pub(crate) fn transition_activity(
    transaction: &Transaction<'_>,
    current: &SessionCredentialRecord,
    activity_state: SessionActivityState,
    transitioned_at_epoch_millis: i64,
) -> Result<(), SessionLifecycleRepositoryError> {
    invariants::validate_record(current)?;
    let mut next = current.clone();
    next.activity_state = activity_state;
    next.last_transition_at_epoch_millis = transitioned_at_epoch_millis;
    invariants::validate_record(&next)?;
    let changed = transaction
        .execute(
            "UPDATE account_identity_session
             SET activity_state = ?2, last_transition_at_epoch_millis = ?3
             WHERE access_digest = ?1 AND activity_state = 'active'
               AND global_revoke_epoch = ?4 AND last_transition_at_epoch_millis = ?5",
            params![
                current.access_digest.as_str(),
                labels::activity_label(next.activity_state).0,
                transitioned_at_epoch_millis,
                to_sql_generation(current.global_revoke_epoch)?,
                current.last_transition_at_epoch_millis,
            ],
        )
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
    (changed == 1)
        .then_some(())
        .ok_or(SessionLifecycleRepositoryError::CurrentnessConflict)
}

pub(crate) fn current_revoke_epoch(
    transaction: &Transaction<'_>,
    account_id: &ParentAccountId,
) -> Result<u64, SessionLifecycleRepositoryError> {
    let account_id = account_id.to_string();
    let epoch = transaction
        .query_row(
            "SELECT epoch FROM account_identity_session_revoke_epoch WHERE account_id = ?1",
            [account_id.as_str()],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
    match epoch {
        Some(epoch) => from_sql_generation(epoch),
        None => {
            let inserted = transaction
                .execute(
                    "INSERT INTO account_identity_session_revoke_epoch (account_id, epoch)
                     VALUES (?1, 1)",
                    [account_id.as_str()],
                )
                .map_err(|_| SessionLifecycleRepositoryError::CurrentnessConflict)?;
            (inserted == 1)
                .then_some(1)
                .ok_or(SessionLifecycleRepositoryError::CurrentnessConflict)
        }
    }
}

pub(crate) fn advance_revoke_epoch(
    transaction: &Transaction<'_>,
    account_id: &ParentAccountId,
    expected_epoch: u64,
) -> Result<u64, SessionLifecycleRepositoryError> {
    let next_epoch = expected_epoch
        .checked_add(1)
        .ok_or(SessionLifecycleRepositoryError::InvalidTransition)?;
    let changed = transaction
        .execute(
            "UPDATE account_identity_session_revoke_epoch SET epoch = ?2
             WHERE account_id = ?1 AND epoch = ?3",
            params![
                account_id.to_string(),
                to_sql_generation(next_epoch)?,
                to_sql_generation(expected_epoch)?,
            ],
        )
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
    if changed != 1 {
        let reloaded = current_revoke_epoch(transaction, account_id)?;
        return if reloaded != expected_epoch {
            Err(SessionLifecycleRepositoryError::CurrentnessConflict)
        } else {
            Err(SessionLifecycleRepositoryError::Unavailable)
        };
    }
    Ok(next_epoch)
}

pub(crate) fn register_consumed_refresh(
    transaction: &Transaction<'_>,
    record: &SessionCredentialRecord,
    consumed_at_epoch_millis: i64,
) -> Result<(), SessionLifecycleRepositoryError> {
    invariants::validate_record(record)?;
    let changed = transaction
        .execute(
            "INSERT INTO account_identity_session_refresh_replay (
                 digest_algorithm, refresh_digest_domain, consumed_refresh_digest,
                 session_id, refresh_family_id, consumed_generation,
                 consumed_at_epoch_millis
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                record.digest_algorithm,
                record.refresh_digest_domain,
                record.refresh_digest.as_str(),
                record.session_id.as_str(),
                record.refresh_family_id.as_str(),
                to_sql_generation(record.refresh_generation)?,
                consumed_at_epoch_millis,
            ],
        )
        .map_err(|_| SessionLifecycleRepositoryError::ReplayRejected)?;
    (changed == 1)
        .then_some(())
        .ok_or(SessionLifecycleRepositoryError::ReplayRejected)
}

pub(crate) fn refresh_was_consumed(
    transaction: &Transaction<'_>,
    digest: &SessionRefreshDigest,
) -> Result<bool, SessionLifecycleRepositoryError> {
    transaction
        .query_row(
            "SELECT 1 FROM account_identity_session_refresh_replay
             WHERE consumed_refresh_digest = ?1 LIMIT 1",
            [digest.as_str()],
            |_| Ok(()),
        )
        .optional()
        .map(|row| row.is_some())
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)
}

pub(crate) fn to_sql_generation(value: u64) -> Result<i64, SessionLifecycleRepositoryError> {
    i64::try_from(value).map_err(|_| SessionLifecycleRepositoryError::InvalidTransition)
}

fn read_one(
    transaction: &Transaction<'_>,
    lookup: SessionLookup<'_>,
) -> Result<Option<SessionCredentialRecord>, SessionLifecycleRepositoryError> {
    let (sql, key) = match lookup {
        SessionLookup::Access(digest) => (
            format!("{SESSION_SELECT} WHERE access_digest = ?1 LIMIT 1"),
            digest.as_str(),
        ),
        SessionLookup::Refresh(digest) => (
            format!("{SESSION_SELECT} WHERE refresh_digest = ?1 LIMIT 1"),
            digest.as_str(),
        ),
    };
    transaction
        .query_row(&sql, [key], decode_stored_row)
        .optional()
        .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?
        .map(stored_row_into_record)
        .transpose()
}

fn validate_rotation_transition(
    current: &SessionCredentialRecord,
    next: &SessionCredentialRecord,
) -> Result<(), SessionLifecycleRepositoryError> {
    let expected_generation = current
        .refresh_generation
        .checked_add(1)
        .ok_or(SessionLifecycleRepositoryError::InvalidTransition)?;
    if current.activity_state != SessionActivityState::Active
        || next.activity_state != SessionActivityState::Active
        || next.session_id != current.session_id
        || next.binding != current.binding
        || next.refresh_family_id != current.refresh_family_id
        || next.refresh_generation != expected_generation
        || next.global_revoke_epoch != current.global_revoke_epoch
        || next.fresh_until_epoch_millis != current.fresh_until_epoch_millis
        || next.issued_at_epoch_millis <= current.last_transition_at_epoch_millis
        || next.access_digest == current.access_digest
        || next.refresh_digest == current.refresh_digest
    {
        return Err(SessionLifecycleRepositoryError::InvalidTransition);
    }
    Ok(())
}

fn decode_stored_row(row: &Row<'_>) -> rusqlite::Result<StoredSessionRow> {
    Ok(StoredSessionRow {
        credential_class: row.get(0)?,
        digest_algorithm: row.get(1)?,
        access_digest_domain: row.get(2)?,
        refresh_digest_domain: row.get(3)?,
        access_digest: row.get(4)?,
        refresh_digest: row.get(5)?,
        session_id: row.get(6)?,
        account_id: row.get(7)?,
        provider: row.get(8)?,
        provider_subject: row.get(9)?,
        household_id: row.get(10)?,
        member_id: row.get(11)?,
        device_id: row.get(12)?,
        authority_session_id: row.get(13)?,
        authority_session_generation: row.get(14)?,
        authority_generation: row.get(15)?,
        authority_expires_at_epoch_millis: row.get(16)?,
        refresh_family_id: row.get(17)?,
        refresh_generation: row.get(18)?,
        issued_at_epoch_millis: row.get(19)?,
        access_expires_at_epoch_millis: row.get(20)?,
        refresh_expires_at_epoch_millis: row.get(21)?,
        fresh_until_epoch_millis: row.get(22)?,
        activity_state: row.get(23)?,
        global_revoke_epoch: row.get(24)?,
        last_transition_at_epoch_millis: row.get(25)?,
    })
}

fn stored_row_into_record(
    row: StoredSessionRow,
) -> Result<SessionCredentialRecord, SessionLifecycleRepositoryError> {
    let record = SessionCredentialRecord {
        credential_class: labels::parse_credential_class(row.credential_class.as_bytes())?,
        digest_algorithm: row.digest_algorithm,
        access_digest_domain: row.access_digest_domain,
        refresh_digest_domain: row.refresh_digest_domain,
        access_digest: SessionAccessDigest::parse(row.access_digest)
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        refresh_digest: SessionRefreshDigest::parse(row.refresh_digest)
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        session_id: SessionId::parse(row.session_id)
            .map_err(SessionLifecycleRepositoryError::InvalidValue)?,
        binding: SessionAuthorityBinding {
            account_id: ParentAccountId::parse(row.account_id)
                .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
            provider: labels::parse_provider(row.provider.as_bytes())?,
            provider_subject: AccountIdentityProviderSubject::parse(row.provider_subject)
                .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
            household_id: FamilyId::parse(row.household_id)
                .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
            member_id: AccountIdentityMemberId::parse(row.member_id)
                .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
            device_id: AccountIdentityDeviceId::parse(row.device_id)
                .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
            authority_session_id: AccountIdentitySessionId::parse(row.authority_session_id)
                .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
            authority_session_generation: from_sql_generation(row.authority_session_generation)?,
            authority_generation: from_sql_generation(row.authority_generation)?,
            authority_expires_at_epoch_millis: row.authority_expires_at_epoch_millis,
        },
        refresh_family_id: SessionRefreshFamilyId::parse(row.refresh_family_id)
            .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)?,
        refresh_generation: from_sql_generation(row.refresh_generation)?,
        issued_at_epoch_millis: row.issued_at_epoch_millis,
        access_expires_at_epoch_millis: row.access_expires_at_epoch_millis,
        refresh_expires_at_epoch_millis: row.refresh_expires_at_epoch_millis,
        fresh_until_epoch_millis: row.fresh_until_epoch_millis,
        activity_state: labels::parse_activity_state(row.activity_state.as_bytes())?,
        global_revoke_epoch: from_sql_generation(row.global_revoke_epoch)?,
        last_transition_at_epoch_millis: row.last_transition_at_epoch_millis,
    };
    invariants::validate_record(&record)?;
    Ok(record)
}

fn record_params(
    record: &SessionCredentialRecord,
) -> Result<impl rusqlite::Params, SessionLifecycleRepositoryError> {
    Ok(rusqlite::params_from_iter(record_param_values(record)?))
}

fn record_param_values(
    record: &SessionCredentialRecord,
) -> Result<Vec<rusqlite::types::Value>, SessionLifecycleRepositoryError> {
    Ok(vec![
        rusqlite::types::Value::Text(
            labels::credential_class_label(record.credential_class)
                .0
                .to_owned(),
        ),
        rusqlite::types::Value::Text(record.digest_algorithm.clone()),
        rusqlite::types::Value::Text(record.access_digest_domain.clone()),
        rusqlite::types::Value::Text(record.refresh_digest_domain.clone()),
        rusqlite::types::Value::Text(record.access_digest.as_str().to_owned()),
        rusqlite::types::Value::Text(record.refresh_digest.as_str().to_owned()),
        rusqlite::types::Value::Text(record.session_id.as_str().to_owned()),
        rusqlite::types::Value::Text(record.binding.account_id.to_string()),
        rusqlite::types::Value::Text(
            labels::provider_label(&record.binding.provider)
                .0
                .to_owned(),
        ),
        rusqlite::types::Value::Text(record.binding.provider_subject.as_str().to_owned()),
        rusqlite::types::Value::Text(record.binding.household_id.to_string()),
        rusqlite::types::Value::Text(record.binding.member_id.as_str().to_owned()),
        rusqlite::types::Value::Text(record.binding.device_id.as_str().to_owned()),
        rusqlite::types::Value::Text(record.binding.authority_session_id.as_str().to_owned()),
        rusqlite::types::Value::Integer(to_sql_generation(
            record.binding.authority_session_generation,
        )?),
        rusqlite::types::Value::Integer(to_sql_generation(record.binding.authority_generation)?),
        rusqlite::types::Value::Integer(record.binding.authority_expires_at_epoch_millis),
        rusqlite::types::Value::Text(record.refresh_family_id.as_str().to_owned()),
        rusqlite::types::Value::Integer(to_sql_generation(record.refresh_generation)?),
        rusqlite::types::Value::Integer(record.issued_at_epoch_millis),
        rusqlite::types::Value::Integer(record.access_expires_at_epoch_millis),
        rusqlite::types::Value::Integer(record.refresh_expires_at_epoch_millis),
        rusqlite::types::Value::Integer(record.fresh_until_epoch_millis),
        rusqlite::types::Value::Text(labels::activity_label(record.activity_state).0.to_owned()),
        rusqlite::types::Value::Integer(to_sql_generation(record.global_revoke_epoch)?),
        rusqlite::types::Value::Integer(record.last_transition_at_epoch_millis),
    ])
}

fn from_sql_generation(value: i64) -> Result<u64, SessionLifecycleRepositoryError> {
    u64::try_from(value)
        .ok()
        .filter(|value| *value > 0)
        .ok_or(SessionLifecycleRepositoryError::InvalidStoredSession)
}
