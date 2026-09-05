#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_schema::account_identity_authority::{
    AccountIdentityDeviceId, AccountIdentityMemberId, AccountIdentityProvider,
    AccountIdentityProviderSubject, AccountIdentitySessionId,
};
use ocentra_schema::report_query_custody::{FamilyId, ParentAccountId};

use crate::family_identity::SessionFreshnessState;
use crate::session_lifecycle::{
    authorize_session_token_action, SessionActivityState, SessionCredentialKind,
    SessionLifecycleAction, SessionTokenDecision, SessionTokenInput, TokenReplayState,
    TokenValidityWindowState,
};
use crate::session_lifecycle_record::SessionId;

use super::browser_credentials::IssuedBrowserSession;
use super::storage_values::{
    SessionAccessDigest, SessionCredentialMaterial, SessionRefreshDigest, SessionRefreshFamilyId,
    SESSION_ACCESS_DIGEST_DOMAIN, SESSION_DIGEST_ALGORITHM, SESSION_REFRESH_DIGEST_DOMAIN,
};
use super::SessionLifecyclePolicy;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SessionAuthorityBinding {
    pub(crate) account_id: ParentAccountId,
    pub(crate) provider: AccountIdentityProvider,
    pub(crate) provider_subject: AccountIdentityProviderSubject,
    pub(crate) household_id: FamilyId,
    pub(crate) member_id: AccountIdentityMemberId,
    pub(crate) device_id: AccountIdentityDeviceId,
    pub(crate) authority_session_id: AccountIdentitySessionId,
    pub(crate) authority_session_generation: u64,
    pub(crate) authority_generation: u64,
    pub(crate) authority_expires_at_epoch_millis: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SessionCredentialClass {
    BrowserUserSession,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SessionCredentialRecord {
    pub(crate) credential_class: SessionCredentialClass,
    pub(crate) digest_algorithm: String,
    pub(crate) access_digest_domain: String,
    pub(crate) refresh_digest_domain: String,
    pub(crate) session_id: SessionId,
    pub(crate) binding: SessionAuthorityBinding,
    pub(crate) access_digest: SessionAccessDigest,
    pub(crate) refresh_digest: SessionRefreshDigest,
    pub(crate) refresh_family_id: SessionRefreshFamilyId,
    pub(crate) refresh_generation: u64,
    pub(crate) issued_at_epoch_millis: i64,
    pub(crate) access_expires_at_epoch_millis: i64,
    pub(crate) refresh_expires_at_epoch_millis: i64,
    pub(crate) fresh_until_epoch_millis: i64,
    pub(crate) activity_state: SessionActivityState,
    pub(crate) global_revoke_epoch: u64,
    pub(crate) last_transition_at_epoch_millis: i64,
}

impl SessionCredentialRecord {
    pub(crate) fn issue(
        binding: SessionAuthorityBinding,
        material: &SessionCredentialMaterial,
        refresh_family_id: SessionRefreshFamilyId,
        now_epoch_millis: i64,
        global_revoke_epoch: u64,
        policy: &SessionLifecyclePolicy,
    ) -> Result<Self, EventingError> {
        let access_expires_at_epoch_millis = bounded_expiry(
            now_epoch_millis,
            policy.access_ttl_millis,
            binding.authority_expires_at_epoch_millis,
        )?;
        let refresh_expires_at_epoch_millis = bounded_expiry(
            now_epoch_millis,
            policy.refresh_ttl_millis,
            binding.authority_expires_at_epoch_millis,
        )?;
        let fresh_until_epoch_millis = bounded_expiry(
            now_epoch_millis,
            policy.freshness_ttl_millis,
            access_expires_at_epoch_millis,
        )?;
        Ok(Self {
            credential_class: SessionCredentialClass::BrowserUserSession,
            digest_algorithm: String::from(SESSION_DIGEST_ALGORITHM),
            access_digest_domain: String::from(SESSION_ACCESS_DIGEST_DOMAIN),
            refresh_digest_domain: String::from(SESSION_REFRESH_DIGEST_DOMAIN),
            session_id: SessionId::parse(binding.authority_session_id.as_str().to_owned())?,
            binding,
            access_digest: material.access_digest.clone(),
            refresh_digest: material.refresh_digest.clone(),
            refresh_family_id,
            refresh_generation: 1,
            issued_at_epoch_millis: now_epoch_millis,
            access_expires_at_epoch_millis,
            refresh_expires_at_epoch_millis,
            fresh_until_epoch_millis,
            activity_state: SessionActivityState::Active,
            global_revoke_epoch,
            last_transition_at_epoch_millis: now_epoch_millis,
        })
    }

    pub(crate) fn rotated(
        &self,
        binding: SessionAuthorityBinding,
        material: &SessionCredentialMaterial,
        now_epoch_millis: i64,
        policy: &SessionLifecyclePolicy,
    ) -> Result<Self, EventingError> {
        let mut next = Self::issue(
            binding,
            material,
            self.refresh_family_id.clone(),
            now_epoch_millis,
            self.global_revoke_epoch,
            policy,
        )?;
        next.refresh_generation =
            self.refresh_generation
                .checked_add(1)
                .ok_or(EventingError::InvalidValue {
                    field: "family_identity.session.refresh_generation",
                    value: String::from("overflow"),
                })?;
        next.fresh_until_epoch_millis = self.fresh_until_epoch_millis;
        Ok(next)
    }

    pub(crate) fn authorize(
        &self,
        action: SessionLifecycleAction,
        now_epoch_millis: i64,
        current_global_revoke_epoch: u64,
        policy: &SessionLifecyclePolicy,
    ) -> SessionTokenDecision {
        let activity_state = if current_global_revoke_epoch == self.global_revoke_epoch {
            self.activity_state
        } else {
            SessionActivityState::GloballyRevoked
        };
        authorize_session_token_action(SessionTokenInput {
            credential_kind: SessionCredentialKind::BrowserUserSession,
            action,
            activity_state,
            replay_state: TokenReplayState::Fresh,
            validity_window_state: self.access_validity(now_epoch_millis, policy),
            session_freshness_state: self.freshness(now_epoch_millis),
        })
    }

    pub(crate) fn refresh_is_current(
        &self,
        now_epoch_millis: i64,
        current_global_revoke_epoch: u64,
        policy: &SessionLifecyclePolicy,
    ) -> bool {
        self.activity_state == SessionActivityState::Active
            && current_global_revoke_epoch == self.global_revoke_epoch
            && now_epoch_millis
                <= self
                    .refresh_expires_at_epoch_millis
                    .saturating_add(policy.clock_skew_millis)
    }

    pub(crate) fn into_issued(self, material: SessionCredentialMaterial) -> IssuedBrowserSession {
        IssuedBrowserSession::new(
            self.session_id,
            material.issued_access,
            material.issued_refresh,
            self.access_expires_at_epoch_millis,
            self.refresh_expires_at_epoch_millis,
        )
    }

    fn access_validity(
        &self,
        now_epoch_millis: i64,
        policy: &SessionLifecyclePolicy,
    ) -> TokenValidityWindowState {
        if now_epoch_millis.saturating_add(policy.clock_skew_millis) < self.issued_at_epoch_millis {
            TokenValidityWindowState::NotYetValid
        } else if now_epoch_millis
            > self
                .access_expires_at_epoch_millis
                .saturating_add(policy.clock_skew_millis)
        {
            TokenValidityWindowState::Expired
        } else if now_epoch_millis > self.access_expires_at_epoch_millis {
            TokenValidityWindowState::ValidWithinClockSkewTolerance
        } else {
            TokenValidityWindowState::Valid
        }
    }

    fn freshness(&self, now_epoch_millis: i64) -> SessionFreshnessState {
        if now_epoch_millis <= self.fresh_until_epoch_millis {
            SessionFreshnessState::Fresh
        } else if now_epoch_millis <= self.access_expires_at_epoch_millis {
            SessionFreshnessState::Stale
        } else {
            SessionFreshnessState::Expired
        }
    }
}

fn bounded_expiry(
    now_epoch_millis: i64,
    ttl_millis: i64,
    hard_limit_epoch_millis: i64,
) -> Result<i64, EventingError> {
    let requested =
        now_epoch_millis
            .checked_add(ttl_millis)
            .ok_or(EventingError::InvalidValue {
                field: "family_identity.session.expiry",
                value: String::from("overflow"),
            })?;
    let expiry = requested.min(hard_limit_epoch_millis);
    if expiry <= now_epoch_millis {
        return Err(EventingError::InvalidValue {
            field: "family_identity.session.expiry",
            value: String::from("authority-expired"),
        });
    }
    Ok(expiry)
}
