use ocentra_family_identity_core::{
    account_identity_authority::{
        VerifiedAccountIdentityAuthority, authorize_household_action_from_verified_authority,
    },
    household_authority::{HouseholdAuthorityAction, HouseholdAuthorizationState},
};
use ocentra_schema::report_query_custody as contracts;

use super::ReportQueryCustodyDerivationError;

#[path = "report_query_custody_page_size_validate.rs"]
mod report_query_custody_page_size_validate;

pub(super) fn validate_report_query_custody_request(
    request: &contracts::ReportQueryCustodyRequest,
    authority: &VerifiedAccountIdentityAuthority,
) -> Result<(), ReportQueryCustodyDerivationError> {
    validate_current_parent_authority(request, authority)?;
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
) -> Result<(), ReportQueryCustodyDerivationError> {
    let action_decision = authorize_household_action_from_verified_authority(
        authority,
        HouseholdAuthorityAction::ViewChildStatus,
        false,
        None,
    );
    (action_decision.authorization_state == HouseholdAuthorizationState::Authorized)
        .then_some(())
        .ok_or(ReportQueryCustodyDerivationError::ParentAuthorityActionRejected)?;

    let child_profile_id = request
        .device
        .child_profile_id
        .as_ref()
        .ok_or(ReportQueryCustodyDerivationError::ParentAuthorityIdentityMismatch)?;
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
    (authority.authority_generation() == request.parent_authority.authority_generation)
        .then_some(())
        .ok_or(ReportQueryCustodyDerivationError::ParentAuthorityGenerationMismatch)?;
    Ok(())
}
