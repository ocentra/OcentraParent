use super::SqliteAccountIdentityAuthorityRepository;
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_mutation_authority::{
    AccountIdentityMutationAuthority, AccountIdentityMutationAuthorityCustody,
    AccountIdentityMutationAuthorityRequest, AccountIdentityMutationOutcome,
};
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

#[path = "account_identity_mutation_authority_repository_apply.rs"]
mod apply;
#[path = "account_identity_mutation_authority_repository_consume.rs"]
mod consume;
#[path = "account_identity_mutation_authority_repository_current.rs"]
mod current;
#[path = "account_identity_mutation_authority_current_validation.rs"]
pub(super) mod current_validation;
#[path = "account_identity_mutation_authority_repository_effect.rs"]
mod effect;
#[path = "account_identity_mutation_authority_repository_issue.rs"]
mod issue;
#[path = "account_identity_mutation_authority_repository_target.rs"]
mod target;

impl SqliteAccountIdentityAuthorityRepository {
    pub(crate) fn issue_mutation_authority(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        request: &AccountIdentityMutationAuthorityRequest,
        custody: &dyn AccountIdentityMutationAuthorityCustody,
    ) -> Result<AccountIdentityMutationAuthority, AccountIdentityMutationAuthorityError> {
        issue::issue(&mut self.connection, authority, request, custody)
    }

    pub(crate) fn consume_and_apply_mutation_authority(
        &mut self,
        wire: &[u8],
        custody: &dyn AccountIdentityMutationAuthorityCustody,
    ) -> Result<AccountIdentityMutationOutcome, AccountIdentityMutationAuthorityError> {
        consume::consume_and_apply(&mut self.connection, wire, custody)
    }
}
