use chrono::{DateTime, Utc};
use ocentra_family_identity_core::{
    account_identity_authority::VerifiedAccountIdentityAuthority,
    account_identity_target_authority::{
        resolve_target_action_from_verified_authority, AccountIdentityTarget,
        AccountIdentityTargetActionRequest,
    },
    household_authority::{HouseholdAuthorityAction, HouseholdAuthorizationState},
};
use ocentra_schema::account_identity_authority::AccountIdentityRole;
use ocentra_schema::report_query_custody as contracts;

use super::ReportQueryCustodyDerivationError;

#[path = "report_query_custody_page_size_validate.rs"]
mod report_query_custody_page_size_validate;

pub(super) fn validate_report_query_custody_request(
    request: &contracts::ReportQueryCustodyRequest,
    authority: &VerifiedAccountIdentityAuthority,
) -> Result<(), ReportQueryCustodyDerivationError> {
    validate_report_query_custody_request_at(request, authority, Utc::now())
}

pub(super) fn validate_report_query_custody_request_at(
    request: &contracts::ReportQueryCustodyRequest,
    authority: &VerifiedAccountIdentityAuthority,
    now: DateTime<Utc>,
) -> Result<(), ReportQueryCustodyDerivationError> {
    validate_current_parent_authority(request, authority, now)?;
    let authority_reference = &request.parent_authority;
    if authority_reference.authority_generation == 0 {
        return Err(ReportQueryCustodyDerivationError::InvalidParentAuthority);
    }
    if authority_reference.family_id != request.family.family_id
        || authority_reference.parent_account_id != request.account.parent_account_id
        || authority_reference.device_id != request.device.device_id
        || authority_reference.child_profile_id != request.device.child_profile_id
    {
        return Err(ReportQueryCustodyDerivationError::ParentAuthorityIdentityMismatch);
    }
    if request.raw_child_evidence_requested {
        return Err(ReportQueryCustodyDerivationError::RawChildEvidenceRequested);
    }
    report_query_custody_page_size_validate::validate_report_query_custody_page_size(
        request.page_size,
    )?;
    if request.requested_data_classes.is_empty() || request.allowed_source_data_classes.is_empty() {
        return Err(ReportQueryCustodyDerivationError::EmptyRequestScope);
    }
    if request.source_citation_refs.is_empty() || request.assistant_citation_refs.is_empty() {
        return Err(ReportQueryCustodyDerivationError::MissingCitationRefs);
    }
    if request.notification_payload_boundary
        != contracts::ReportQueryCustodyBoundary::ParentOwnedCitationsOnly
    {
        return Err(ReportQueryCustodyDerivationError::InvalidNotificationBoundary);
    }
    if request
        .requested_data_classes
        .iter()
        .any(|data_class| !request.allowed_source_data_classes.contains(data_class))
    {
        return Err(ReportQueryCustodyDerivationError::DisallowedSourceDataClass);
    }
    if request
        .source_citation_refs
        .iter()
        .chain(request.assistant_citation_refs.iter())
        .any(|citation| citation.kind != contracts::ParentEvidenceReferenceKind::QueryStoreSummary)
    {
        return Err(ReportQueryCustodyDerivationError::InvalidCitationKind);
    }
    if request
        .source_citation_refs
        .iter()
        .chain(request.assistant_citation_refs.iter())
        .any(|citation| {
            citation.family_id != request.family.family_id
                || citation.child_profile_id != request.device.child_profile_id
        })
    {
        return Err(ReportQueryCustodyDerivationError::CitationIdentityMismatch);
    }
    if request
        .source_citation_refs
        .iter()
        .chain(request.assistant_citation_refs.iter())
        .any(|citation| {
            !request
                .requested_data_classes
                .contains(&citation.source_data_class)
                || !request
                    .allowed_source_data_classes
                    .contains(&citation.source_data_class)
        })
    {
        return Err(ReportQueryCustodyDerivationError::CitationSourceClassMismatch);
    }
    Ok(())
}

fn validate_current_parent_authority(
    request: &contracts::ReportQueryCustodyRequest,
    authority: &VerifiedAccountIdentityAuthority,
    now: DateTime<Utc>,
) -> Result<(), ReportQueryCustodyDerivationError> {
    let child_profile_id = request
        .device
        .child_profile_id
        .as_ref()
        .ok_or(ReportQueryCustodyDerivationError::ParentAuthorityIdentityMismatch)?;
    let target_request = AccountIdentityTargetActionRequest::new(
        HouseholdAuthorityAction::ViewChildStatus,
        Some(AccountIdentityTarget::child_profile(
            child_profile_id.clone(),
        )),
    );
    let action_decision = resolve_target_action_from_verified_authority(authority, &target_request)
        .map(|resolution| resolution.decision())
        .map_err(|_| ReportQueryCustodyDerivationError::ParentAuthorityActionRejected)?;
    (action_decision.authorization_state == HouseholdAuthorizationState::Authorized)
        .then_some(())
        .ok_or(ReportQueryCustodyDerivationError::ParentAuthorityActionRejected)?;

    let request_actor_id = request.parent_action.actor.actor_id.to_string();
    let request_device_id = request.device.device_id.to_string();
    let current_identity = (
        authority.household_id(),
        authority.account_id(),
        authority.member_id().as_str(),
        authority.device_id().as_str(),
        authority.child_profile_id(),
    );
    let requested_identity = (
        &request.family.family_id,
        &request.account.parent_account_id,
        request_actor_id.as_str(),
        request_device_id.as_str(),
        child_profile_id,
    );
    if current_identity != requested_identity {
        return Err(ReportQueryCustodyDerivationError::ParentAuthorityIdentityMismatch);
    }
    if !request_actor_role_matches_authority(request.parent_action.actor.role, authority.role()) {
        return Err(ReportQueryCustodyDerivationError::ParentAuthorityIdentityMismatch);
    }
    (authority.authority_generation() == request.parent_authority.authority_generation)
        .then_some(())
        .ok_or(ReportQueryCustodyDerivationError::ParentAuthorityGenerationMismatch)?;
    let session_expires_at = DateTime::parse_from_rfc3339(authority.session_expires_at())
        .map_err(|_| ReportQueryCustodyDerivationError::ParentAuthorityExpired)?
        .with_timezone(&Utc);
    (session_expires_at > now)
        .then_some(())
        .ok_or(ReportQueryCustodyDerivationError::ParentAuthorityExpired)?;
    Ok(())
}

fn request_actor_role_matches_authority(
    request_role: contracts::ParentActorRole,
    authority_role: AccountIdentityRole,
) -> bool {
    matches!(
        (request_role, authority_role),
        (
            contracts::ParentActorRole::Parent,
            AccountIdentityRole::ParentOwner
        ) | (
            contracts::ParentActorRole::Guardian,
            AccountIdentityRole::CoParentGuardian
        )
    )
}
