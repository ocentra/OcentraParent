#![forbid(unsafe_code)]

use chrono::DateTime;
use ocentra_schema::account_identity_authority::{
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityMappingStatus,
    AccountIdentityProvider, AccountIdentityProviderSubject, AccountIdentityRole,
    AccountIdentitySessionFreshnessState,
};
use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::session_lifecycle_custody::record::{SessionAuthorityBinding, SessionCredentialRecord};

use super::{labels, SessionLifecycleRepositoryError};

struct CurrentAuthorityRow {
    handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    expires_at_epoch_millis: i64,
}

pub(crate) fn binding_from_verified(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    now_epoch_millis: i64,
) -> Result<SessionAuthorityBinding, SessionLifecycleRepositoryError> {
    if !role_can_hold_browser_session(authority.role()) {
        return Err(SessionLifecycleRepositoryError::WrongCredentialClass);
    }
    let current = load_current(
        transaction,
        authority.provider(),
        authority.provider_subject(),
        now_epoch_millis,
    )?;
    let member = &current.handoff.member;
    if &member.account_id != authority.account_id()
        || &member.household_id != authority.household_id()
        || &member.member_id != authority.member_id()
        || &member.device_id != authority.device_id()
        || member.role != authority.role()
        || &member.session_id != authority.session_id()
        || member.session_generation != authority.session_generation()
        || member.authority_generation != authority.authority_generation()
    {
        return Err(SessionLifecycleRepositoryError::CurrentnessConflict);
    }
    Ok(binding_from_handoff(
        &current.handoff,
        current.expires_at_epoch_millis,
    ))
}

pub(crate) fn parent_local_bridge_binding_from_verified(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    now_epoch_millis: i64,
) -> Result<SessionAuthorityBinding, SessionLifecycleRepositoryError> {
    if !role_can_hold_parent_local_bridge(authority.role()) {
        return Err(SessionLifecycleRepositoryError::WrongCredentialClass);
    }
    binding_from_verified(transaction, authority, now_epoch_millis)
}

pub(crate) fn parent_local_bridge_current_role(
    transaction: &Transaction<'_>,
    expected: &SessionAuthorityBinding,
    now_epoch_millis: i64,
) -> Result<AccountIdentityRole, SessionLifecycleRepositoryError> {
    let current = load_current(
        transaction,
        &expected.provider,
        &expected.provider_subject,
        now_epoch_millis,
    )?;
    if !role_can_hold_parent_local_bridge(current.handoff.member.role) {
        return Err(SessionLifecycleRepositoryError::WrongCredentialClass);
    }
    let current_binding = binding_from_handoff(&current.handoff, current.expires_at_epoch_millis);
    if current_binding != *expected {
        return Err(SessionLifecycleRepositoryError::CurrentnessConflict);
    }
    Ok(current.handoff.member.role)
}

pub(crate) fn binding_for_record_current(
    transaction: &Transaction<'_>,
    record: &SessionCredentialRecord,
    now_epoch_millis: i64,
) -> Result<SessionAuthorityBinding, SessionLifecycleRepositoryError> {
    let current = load_current(
        transaction,
        &record.binding.provider,
        &record.binding.provider_subject,
        now_epoch_millis,
    )?;
    let binding = binding_from_handoff(&current.handoff, current.expires_at_epoch_millis);
    if binding.account_id != record.binding.account_id
        || binding.provider != record.binding.provider
        || binding.provider_subject != record.binding.provider_subject
        || binding.household_id != record.binding.household_id
        || binding.member_id != record.binding.member_id
        || binding.device_id != record.binding.device_id
        || binding.authority_session_id != record.binding.authority_session_id
        || binding.authority_session_generation != record.binding.authority_session_generation
        || binding.authority_generation != record.binding.authority_generation
        || binding.authority_expires_at_epoch_millis
            != record.binding.authority_expires_at_epoch_millis
    {
        return Err(SessionLifecycleRepositoryError::CurrentnessConflict);
    }
    Ok(binding)
}

fn load_current(
    transaction: &Transaction<'_>,
    provider: &AccountIdentityProvider,
    provider_subject: &AccountIdentityProviderSubject,
    now_epoch_millis: i64,
) -> Result<CurrentAuthorityRow, SessionLifecycleRepositoryError> {
    let row = transaction
        .query_row(
            "SELECT mapping_status, authority_generation, session_id,
                    session_generation, authority_json
             FROM account_identity_current_authority
             WHERE provider = ?1 AND provider_subject = ?2 LIMIT 1",
            params![
                labels::provider_label(provider).0,
                provider_subject.as_str()
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
        .map_err(|_error| SessionLifecycleRepositoryError::Unavailable)?
        .ok_or(SessionLifecycleRepositoryError::AuthorityMissing)?;
    let (mapping_status, authority_generation, session_id, session_generation, authority_json) =
        row;
    let handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff =
        serde_json::from_str(&authority_json)
            .map_err(|_error| SessionLifecycleRepositoryError::InvalidAuthorityBinding)?;
    handoff
        .validate_shape()
        .map_err(|_error| SessionLifecycleRepositoryError::InvalidAuthorityBinding)?;
    let authority_generation = positive_generation(authority_generation)?;
    let session_generation = positive_generation(session_generation)?;
    let member = &handoff.member;
    if mapping_status != "active"
        || handoff.mapping.status != AccountIdentityMappingStatus::Active
        || &handoff.mapping.provider != provider
        || &handoff.mapping.provider_subject != provider_subject
        || member.authority_generation != authority_generation
        || handoff.binding.authority_generation != authority_generation
        || member.session_generation != session_generation
        || member.session_id.as_str() != session_id
        || member.session_freshness_state != AccountIdentitySessionFreshnessState::Fresh
        || !role_can_hold_browser_session(member.role)
    {
        return Err(SessionLifecycleRepositoryError::InvalidAuthorityBinding);
    }
    let expires_at_epoch_millis = DateTime::parse_from_rfc3339(&member.session_expires_at)
        .map_err(|_error| SessionLifecycleRepositoryError::InvalidAuthorityBinding)?
        .timestamp_millis();
    if expires_at_epoch_millis <= now_epoch_millis {
        return Err(SessionLifecycleRepositoryError::AuthorityExpired);
    }
    Ok(CurrentAuthorityRow {
        handoff,
        expires_at_epoch_millis,
    })
}

fn binding_from_handoff(
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    expires_at_epoch_millis: i64,
) -> SessionAuthorityBinding {
    SessionAuthorityBinding {
        account_id: handoff.member.account_id.clone(),
        provider: handoff.mapping.provider.clone(),
        provider_subject: handoff.mapping.provider_subject.clone(),
        household_id: handoff.member.household_id.clone(),
        member_id: handoff.member.member_id.clone(),
        device_id: handoff.member.device_id.clone(),
        authority_session_id: handoff.member.session_id.clone(),
        authority_session_generation: handoff.member.session_generation,
        authority_generation: handoff.member.authority_generation,
        authority_expires_at_epoch_millis: expires_at_epoch_millis,
    }
}

fn positive_generation(value: i64) -> Result<u64, SessionLifecycleRepositoryError> {
    u64::try_from(value)
        .ok()
        .filter(|value| *value > 0)
        .ok_or(SessionLifecycleRepositoryError::InvalidAuthorityBinding)
}

fn role_can_hold_browser_session(role: AccountIdentityRole) -> bool {
    matches!(
        role,
        AccountIdentityRole::ParentOwner
            | AccountIdentityRole::CoParentGuardian
            | AccountIdentityRole::Observer
            | AccountIdentityRole::ChildProfile
    )
}

fn role_can_hold_parent_local_bridge(role: AccountIdentityRole) -> bool {
    matches!(
        role,
        AccountIdentityRole::ParentOwner | AccountIdentityRole::CoParentGuardian
    )
}
