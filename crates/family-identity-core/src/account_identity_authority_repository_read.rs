use ocentra_schema::account_identity_authority::{
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityMappingStatus,
    AccountIdentityProvider, AccountIdentityProviderSubject,
    ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION,
};
use rusqlite::{params, OptionalExtension};

use super::{
    account_identity_authority_repository_invariants::{from_sql_generation, provider_label},
    AccountIdentityAuthorityRepositoryError, SqliteAccountIdentityAuthorityRepository,
};
use crate::account_identity_authority::AccountIdentityAuthorityRepository;

impl SqliteAccountIdentityAuthorityRepository {
    fn read_row(
        &self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
    ) -> Result<
        Option<AccountIdentityCurrentMemberDeviceAuthorityHandoff>,
        AccountIdentityAuthorityRepositoryError,
    > {
        let row = self
            .connection
            .query_row(
                "SELECT mapping_status, authority_generation, session_id, session_generation, authority_json
                 FROM account_identity_current_authority
                 WHERE provider = ?1 AND provider_subject = ?2
                 LIMIT 1",
                params![provider_label(provider), provider_subject.as_str()],
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
            .map_err(|_| AccountIdentityAuthorityRepositoryError::Unavailable)?;
        let Some((
            mapping_status,
            authority_generation,
            session_id,
            session_generation,
            authority_json,
        )) = row
        else {
            return Ok(None);
        };
        let authority_generation = from_sql_generation(authority_generation)?;
        let session_generation = from_sql_generation(session_generation)?;
        if authority_generation > ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION
            || session_generation > ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION
        {
            return Err(AccountIdentityAuthorityRepositoryError::InvalidGeneration);
        }
        let handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff =
            serde_json::from_str(&authority_json)
                .map_err(|_| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
        handoff
            .validate_shape()
            .map_err(|_| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
        if mapping_status != "active"
            || handoff.mapping.status != AccountIdentityMappingStatus::Active
            || &handoff.mapping.provider != provider
            || &handoff.mapping.provider_subject != provider_subject
            || handoff.member.authority_generation != authority_generation
            || handoff.binding.authority_generation != authority_generation
            || handoff.member.session_generation != session_generation
            || handoff.member.session_id.as_str() != session_id
        {
            return Err(AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority);
        }
        Ok(Some(handoff))
    }
}

impl AccountIdentityAuthorityRepository for SqliteAccountIdentityAuthorityRepository {
    type Error = AccountIdentityAuthorityRepositoryError;

    fn read_current_member_device_authority(
        &self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
    ) -> Result<
        Option<(
            ocentra_schema::account_identity_authority::AccountIdentityProviderSubjectMapping,
            ocentra_schema::account_identity_authority::AccountIdentityCurrentMemberDeviceAuthority,
            ocentra_schema::account_identity_authority::AccountIdentityHouseholdChildDeviceBinding,
        )>,
        Self::Error,
    > {
        self.read_row(provider, provider_subject).map(|handoff| {
            handoff.map(|handoff| (handoff.mapping, handoff.member, handoff.binding))
        })
    }
}
