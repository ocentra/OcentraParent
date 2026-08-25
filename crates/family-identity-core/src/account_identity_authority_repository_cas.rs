use ocentra_schema::account_identity_authority::AccountIdentityCurrentMemberDeviceAuthorityHandoff;
use rusqlite::params;

use super::{
    account_identity_authority_repository_invariants::{
        provider_label, to_sql_generation, validate_next_handoff,
    },
    AccountIdentityAuthorityRepositoryError, AccountIdentityAuthorityService,
    SqliteAccountIdentityAuthorityRepository,
};
use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};

/// Account-owned monotonic CAS. The caller can only advance a row when it
/// presents the durable generation/session pair it just read; every advance
/// rotates the current session identity and increments both generations. No
/// selector or request header can mint a row.
impl SqliteAccountIdentityAuthorityRepository {
    pub(crate) fn compare_and_swap_current_authority(
        &self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        expected_authority_generation: Option<u64>,
        expected_session_generation: Option<u64>,
        next: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    ) -> Result<(), AccountIdentityAuthorityRepositoryError> {
        validate_next_handoff(provider, provider_subject, next)?;
        let next_authority_generation = to_sql_generation(next.member.authority_generation)?;
        let next_session_generation = to_sql_generation(next.member.session_generation)?;
        let authority_json = serde_json::to_string(next)
            .map_err(|_| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
        if expected_authority_generation.is_none()
            && expected_session_generation.is_none()
            && (next_authority_generation != 1 || next_session_generation != 1)
        {
            return Err(AccountIdentityAuthorityRepositoryError::CurrentnessConflict);
        }
        let provider = provider_label(provider);
        let subject = provider_subject.as_str();
        let changed = match (expected_authority_generation, expected_session_generation) {
            (None, None) => self
                .connection
                .execute(
                    "INSERT INTO account_identity_current_authority (
                         provider, provider_subject, mapping_status, authority_generation,
                         session_id, session_generation, authority_json
                     ) VALUES (?1, ?2, 'active', ?3, ?4, ?5, ?6)
                     ON CONFLICT(provider, provider_subject) DO NOTHING",
                    params![
                        provider,
                        subject,
                        next_authority_generation,
                        next.member.session_id.as_str(),
                        next_session_generation,
                        authority_json,
                    ],
                )
                .map_err(|_| AccountIdentityAuthorityRepositoryError::Unavailable)?,
            (Some(expected_authority_generation), Some(expected_session_generation)) => {
                let expected_authority_generation =
                    to_sql_generation(expected_authority_generation)?;
                let expected_session_generation = to_sql_generation(expected_session_generation)?;
                self.connection
                    .execute(
                        "UPDATE account_identity_current_authority
                         SET authority_generation = ?3,
                             session_id = ?4,
                             session_generation = ?5,
                             authority_json = ?6
                         WHERE provider = ?1
                           AND provider_subject = ?2
                           AND authority_generation = ?7
                           AND session_generation = ?8
                           AND mapping_status = 'active'
                           AND session_id <> ?4
                           AND ?3 = ?7 + 1
                           AND ?5 = ?8 + 1",
                        params![
                            provider,
                            subject,
                            next_authority_generation,
                            next.member.session_id.as_str(),
                            next_session_generation,
                            authority_json,
                            expected_authority_generation,
                            expected_session_generation,
                        ],
                    )
                    .map_err(|_| AccountIdentityAuthorityRepositoryError::Unavailable)?
            }
            _ => return Err(AccountIdentityAuthorityRepositoryError::CurrentnessConflict),
        };
        (changed == 1)
            .then_some(())
            .ok_or(AccountIdentityAuthorityRepositoryError::CurrentnessConflict)
    }
}

impl AccountIdentityAuthorityService {
    pub(crate) fn compare_and_swap_current_authority(
        &self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
        expected_authority_generation: Option<u64>,
        expected_session_generation: Option<u64>,
        next: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    ) -> Result<(), AccountIdentityAuthorityRepositoryError> {
        self.repository.compare_and_swap_current_authority(
            provider,
            provider_subject,
            expected_authority_generation,
            expected_session_generation,
            next,
        )
    }
}
