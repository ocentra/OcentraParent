use chrono::{DateTime, Utc};
use ocentra_family_identity_core::account_identity_authority::VerifiedAccountIdentityAuthority;
use ocentra_schema::account_identity_authority::{
    AccountIdentityAccountState, AccountIdentityAuditIdentity,
    AccountIdentityBindingLifecycleState, AccountIdentityBindingRevocationState,
    AccountIdentityChildDeviceId, AccountIdentityDeviceId, AccountIdentityDeviceTrustState,
    AccountIdentityInstallState, AccountIdentityMemberId, AccountIdentityMembershipState,
    AccountIdentityPairingState, AccountIdentityProvider, AccountIdentityProviderSubject,
    AccountIdentityRole, AccountIdentitySessionFreshnessState, AccountIdentitySessionId,
    AccountIdentitySupportIssuerId, AccountIdentitySupportScope,
};
use ocentra_schema::report_query_custody as contracts;

use super::{
    report_query_custody_request_validate, report_query_custody_row_validate,
    ReportQueryCustodyDerivationError, ReportQueryCustodyDerivationInput, ReportQueryCustodySignal,
};

#[path = "report_query_custody_state_projection.rs"]
mod report_query_custody_state_projection;
use report_query_custody_state_projection::ReportQueryCustodySourceStateProjection;

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReportQueryCustodyAuthorityBinding {
    account_id: contracts::ParentAccountId,
    household_id: contracts::FamilyId,
    member_id: AccountIdentityMemberId,
    role: AccountIdentityRole,
    account_state: AccountIdentityAccountState,
    membership_state: AccountIdentityMembershipState,
    device_id: AccountIdentityDeviceId,
    child_profile_id: contracts::ChildProfileId,
    child_device_id: AccountIdentityChildDeviceId,
    child_pairing_state: AccountIdentityPairingState,
    child_install_state: AccountIdentityInstallState,
    child_lifecycle_state: AccountIdentityBindingLifecycleState,
    child_revocation_state: AccountIdentityBindingRevocationState,
    device_trust_state: AccountIdentityDeviceTrustState,
    session_freshness_state: AccountIdentitySessionFreshnessState,
    session_id: AccountIdentitySessionId,
    session_generation: u64,
    session_expires_at: String,
    authority_generation: u64,
    provider: AccountIdentityProvider,
    provider_subject: AccountIdentityProviderSubject,
    support_scope: Option<AccountIdentitySupportScope>,
    support_issuer: Option<AccountIdentitySupportIssuerId>,
    support_audit_identity: Option<AccountIdentityAuditIdentity>,
}

impl ReportQueryCustodyAuthorityBinding {
    fn from_verified_authority(authority: &VerifiedAccountIdentityAuthority) -> Self {
        let (
            account_state,
            membership_state,
            device_trust_state,
            session_freshness_state,
            child_pairing_state,
            child_install_state,
            child_lifecycle_state,
            child_revocation_state,
        ) = authority.report_query_custody_states();
        Self {
            account_id: authority.account_id().clone(),
            household_id: authority.household_id().clone(),
            member_id: authority.member_id().clone(),
            role: authority.role(),
            account_state,
            membership_state,
            device_id: authority.device_id().clone(),
            child_profile_id: authority.child_profile_id().clone(),
            child_device_id: authority.child_device_id().clone(),
            child_pairing_state,
            child_install_state,
            child_lifecycle_state,
            child_revocation_state,
            device_trust_state,
            session_freshness_state,
            session_id: authority.session_id().clone(),
            session_generation: authority.session_generation(),
            session_expires_at: authority.session_expires_at().to_owned(),
            authority_generation: authority.authority_generation(),
            provider: authority.provider().clone(),
            provider_subject: authority.provider_subject().clone(),
            support_scope: authority.support_scope().copied(),
            support_issuer: authority.support_issuer().cloned(),
            support_audit_identity: authority.support_audit_identity().cloned(),
        }
    }

    fn is_current_at(&self, now: DateTime<Utc>) -> bool {
        DateTime::parse_from_rfc3339(&self.session_expires_at)
            .map(|expires_at| expires_at.with_timezone(&Utc) > now)
            .unwrap_or(false)
    }
}

/// Opaque owner evidence for a producer-owned query store.
///
/// A producer obtains this only from the verified family capability supplied
/// by its current authority path. It carries the complete account, household,
/// member, device, child, session, expiry, generation, provider, and support
/// provenance tuple. It is not serde data and cannot be replaced by a request
/// DTO or an availability boolean.
#[derive(Debug, Eq, PartialEq)]
pub struct ReportQueryCustodySourceOwnerEvidence {
    authority: ReportQueryCustodyAuthorityBinding,
}

impl ReportQueryCustodySourceOwnerEvidence {
    pub fn from_verified_current_authority(authority: &VerifiedAccountIdentityAuthority) -> Self {
        Self {
            authority: ReportQueryCustodyAuthorityBinding::from_verified_authority(authority),
        }
    }
}

/// Typed state evidence supplied by the owning query source.
///
/// The variants prevent a producer adapter from passing untyped signal,
/// tombstone, conflict, or timestamp booleans into custody. Terminal states
/// intentionally carry no continuation cursor.
#[derive(Debug, Eq, PartialEq)]
pub enum ReportQueryCustodySourceStateEvidence {
    Fresh {
        next_cursor_ref: contracts::ReportQueryCustodyCursorRef,
    },
    Stale {
        next_cursor_ref: contracts::ReportQueryCustodyCursorRef,
    },
    PartiallyRedacted {
        next_cursor_ref: contracts::ReportQueryCustodyCursorRef,
    },
    Deleted {
        tombstone: ReportQueryCustodyTombstoneEvidence,
    },
    Conflict {
        next_cursor_ref: contracts::ReportQueryCustodyCursorRef,
        conflict_ref: contracts::ReportQueryCustodyConflictRef,
    },
    CursorExpired {
        expired_at: contracts::ParentTimestamp,
    },
    RateLimited {
        until_at: contracts::ParentTimestamp,
    },
}

/// Opaque tombstone evidence issued by the current query-source owner.
#[derive(Debug, Eq, PartialEq)]
pub struct ReportQueryCustodyTombstoneEvidence {
    authority: ReportQueryCustodyAuthorityBinding,
    deleted_source_ref: contracts::ReportQueryCustodyDeletedSourceRef,
    deleted_source_at: contracts::ParentTimestamp,
}

impl ReportQueryCustodyTombstoneEvidence {
    pub fn from_current_owner(
        owner: &ReportQueryCustodySourceOwnerEvidence,
        deleted_source_ref: contracts::ReportQueryCustodyDeletedSourceRef,
        deleted_source_at: contracts::ParentTimestamp,
    ) -> Self {
        Self {
            authority: owner.authority.clone(),
            deleted_source_ref,
            deleted_source_at,
        }
    }
}

/// Typed, owner-derived row evidence accepted by the custody boundary.
#[derive(Debug, Eq, PartialEq)]
pub struct ReportQueryCustodySourceOwnerRow {
    authority: ReportQueryCustodyAuthorityBinding,
    input: ReportQueryCustodyDerivationInput,
}

impl ReportQueryCustodySourceOwnerRow {
    pub fn from_current_owner(
        owner: &ReportQueryCustodySourceOwnerEvidence,
        row_id: contracts::ReportQueryCustodySourceRef,
        source_data_class: contracts::ReportQueryCustodySourceDataClass,
        cursor_ref: contracts::ReportQueryCustodyCursorRef,
        source_cursor_ref: contracts::ReportQueryCustodyCursorRef,
        page_index: u32,
        stable_sort_key: contracts::ReportQueryCustodySortKey,
        state: ReportQueryCustodySourceStateEvidence,
    ) -> Result<Self, ReportQueryCustodyDerivationError> {
        let projection = ReportQueryCustodySourceStateProjection::from_state(owner, state)?;
        Ok(Self {
            authority: owner.authority.clone(),
            input: projection.into_input(
                row_id,
                source_data_class,
                cursor_ref,
                source_cursor_ref,
                page_index,
                stable_sort_key,
            ),
        })
    }
}

/// A source result that has crossed the custody boundary.
///
/// The fields deliberately remain private. The request/query transport can
/// describe a source, but only a producer row tied to opaque current-owner
/// evidence can mint this result.
#[derive(Debug, Eq, PartialEq)]
pub struct ReportQueryCustodySourceResolution {
    input: ReportQueryCustodyDerivationInput,
    authority: ReportQueryCustodyAuthorityBinding,
}

impl ReportQueryCustodySourceResolution {
    pub fn from_owned_source(
        request: &contracts::ReportQueryCustodyRequest,
        owner: &ReportQueryCustodySourceOwnerEvidence,
        source: ReportQueryCustodySourceOwnerRow,
        authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<Self, ReportQueryCustodyDerivationError> {
        Self::from_owned_source_at(request, owner, source, authority, Utc::now())
    }

    fn from_owned_source_at(
        request: &contracts::ReportQueryCustodyRequest,
        owner: &ReportQueryCustodySourceOwnerEvidence,
        source: ReportQueryCustodySourceOwnerRow,
        authority: &VerifiedAccountIdentityAuthority,
        resolved_at: DateTime<Utc>,
    ) -> Result<Self, ReportQueryCustodyDerivationError> {
        let current_authority =
            ReportQueryCustodyAuthorityBinding::from_verified_authority(authority);
        if !current_authority.is_current_at(resolved_at) {
            return Err(ReportQueryCustodyDerivationError::ParentAuthorityExpired);
        }
        if owner.authority != current_authority || source.authority != owner.authority {
            return Err(ReportQueryCustodyDerivationError::TrustedSourceResolutionUnavailable);
        }
        report_query_custody_request_validate::validate_report_query_custody_request_at(
            request,
            authority,
            resolved_at,
        )?;
        report_query_custody_row_validate::validate_report_query_custody_input_at(
            request,
            &source.input,
            authority,
            resolved_at,
        )?;
        Ok(Self {
            input: source.input,
            authority: current_authority,
        })
    }

    pub(super) fn into_input(self) -> ReportQueryCustodyDerivationInput {
        self.input
    }

    pub(super) fn matches_authority_at(
        &self,
        authority: &VerifiedAccountIdentityAuthority,
        resolved_at: DateTime<Utc>,
    ) -> bool {
        self.authority == ReportQueryCustodyAuthorityBinding::from_verified_authority(authority)
            && self.authority.is_current_at(resolved_at)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ReportQueryCustodySourceAdapterError<E> {
    Producer(E),
    Custody(ReportQueryCustodyDerivationError),
}

/// Producer-owned query sources enter custody through this explicit adapter.
///
/// An implementation must read its own governed source (for example the
/// agent-service ActivityStore) and return typed rows bound to the supplied
/// opaque owner evidence. It cannot return request DTO authority, raw child
/// evidence, or an availability boolean. The verified authority capability
/// does not cross this producer port; only the opaque owner token does. The
/// storage boundary remains the owner of resolution and derivation semantics.
pub trait ReportQueryCustodySourcePort {
    type Error;

    fn resolve(
        &self,
        request: &contracts::ReportQueryCustodyRequest,
        owner: &ReportQueryCustodySourceOwnerEvidence,
    ) -> Result<Vec<ReportQueryCustodySourceOwnerRow>, Self::Error>;
}

pub fn resolve_report_query_custody_sources<P>(
    request: &contracts::ReportQueryCustodyRequest,
    authority: &VerifiedAccountIdentityAuthority,
    producer: &P,
) -> Result<Vec<ReportQueryCustodySourceResolution>, ReportQueryCustodySourceAdapterError<P::Error>>
where
    P: ReportQueryCustodySourcePort,
{
    let resolved_at = Utc::now();
    report_query_custody_request_validate::validate_report_query_custody_request_at(
        request,
        authority,
        resolved_at,
    )
    .map_err(ReportQueryCustodySourceAdapterError::Custody)?;
    let owner = ReportQueryCustodySourceOwnerEvidence::from_verified_current_authority(authority);
    producer
        .resolve(request, &owner)
        .map_err(ReportQueryCustodySourceAdapterError::Producer)?
        .into_iter()
        .map(|source| {
            ReportQueryCustodySourceResolution::from_owned_source_at(
                request,
                &owner,
                source,
                authority,
                resolved_at,
            )
            .map_err(ReportQueryCustodySourceAdapterError::Custody)
        })
        .collect()
}
