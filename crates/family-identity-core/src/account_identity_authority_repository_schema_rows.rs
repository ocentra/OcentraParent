use ocentra_schema::account_identity_authority::{
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityMappingStatus,
    AccountIdentityProvider, AccountIdentityProviderSubject,
    ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION,
};

use super::super::AccountIdentityAuthorityRepositoryError;

pub(super) fn validate(
    connection: &rusqlite::Connection,
) -> Result<(), AccountIdentityAuthorityRepositoryError> {
    let mut statement = connection
        .prepare(
            "SELECT provider, provider_subject, mapping_status, authority_generation,
                    session_id, session_generation, authority_json
             FROM account_identity_current_authority",
        )
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?;
    let mut rows = statement
        .query([])
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?;
    while let Some(row) = rows
        .next()
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::Unavailable)?
    {
        validate_row(row)?;
    }
    Ok(())
}

fn validate_row(row: &rusqlite::Row<'_>) -> Result<(), AccountIdentityAuthorityRepositoryError> {
    let provider = row
        .get::<_, String>(0)
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
    let provider_subject = row
        .get::<_, String>(1)
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
    let status = row
        .get::<_, String>(2)
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
    let generation = row
        .get::<_, i64>(3)
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
    let session_id = row
        .get::<_, String>(4)
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
    let session_generation = row
        .get::<_, i64>(5)
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
    let authority_json = row
        .get::<_, String>(6)
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
    let parsed_provider = match provider.as_str() {
        "authjs" => AccountIdentityProvider::Authjs,
        "firebase" => AccountIdentityProvider::Firebase,
        _ => return Err(AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority),
    };
    let parsed_subject = AccountIdentityProviderSubject::parse(provider_subject)
        .ok_or(AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
    let handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff =
        serde_json::from_str(&authority_json)
            .map_err(|_error| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
    handoff
        .validate_shape()
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
    if !matches!(status.as_str(), "active" | "revoked")
        || generation <= 0
        || generation > ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION as i64
        || session_generation <= 0
        || session_generation > ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION as i64
        || session_id.trim().is_empty()
        || handoff.mapping.provider != parsed_provider
        || handoff.mapping.provider_subject != parsed_subject
        || (status == "active") != (handoff.mapping.status == AccountIdentityMappingStatus::Active)
        || handoff.member.authority_generation != generation as u64
        || handoff.member.session_generation != session_generation as u64
        || handoff.member.session_id.as_str() != session_id
    {
        return Err(AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority);
    }
    Ok(())
}
