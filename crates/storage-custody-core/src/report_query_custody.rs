use ocentra_family_identity_core::account_identity_authority::VerifiedAccountIdentityAuthority;
use ocentra_schema::report_query_custody as contracts;

#[path = "report_query_custody_proof.rs"]
mod report_query_custody_proof;
#[path = "report_query_custody_request_validate.rs"]
mod report_query_custody_request_validate;
#[path = "report_query_custody_row.rs"]
mod report_query_custody_row;
#[path = "report_query_custody_row_state.rs"]
mod report_query_custody_row_state;
#[path = "report_query_custody_row_validate.rs"]
mod report_query_custody_row_validate;
#[path = "report_query_custody_source.rs"]
pub mod report_query_custody_source;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReportQueryCustodySignal {
    Fresh,
    Stale,
    PartiallyRedacted,
    Deleted,
    Conflict,
    CursorExpired,
    RateLimited,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReportQueryCustodyDerivationInput {
    pub(crate) row_id: contracts::ReportQueryCustodySourceRef,
    pub(crate) source_data_class: contracts::ReportQueryCustodySourceDataClass,
    pub(crate) signal: ReportQueryCustodySignal,
    pub(crate) cursor_ref: contracts::ReportQueryCustodyCursorRef,
    pub(crate) source_cursor_ref: contracts::ReportQueryCustodyCursorRef,
    pub(crate) next_cursor_ref: Option<contracts::ReportQueryCustodyCursorRef>,
    pub(crate) page_index: u32,
    pub(crate) stable_sort_key: contracts::ReportQueryCustodySortKey,
    pub(crate) deleted_source_ref: Option<contracts::ReportQueryCustodyDeletedSourceRef>,
    pub(crate) deleted_source_at: Option<contracts::ParentTimestamp>,
    pub(crate) conflict_ref: Option<contracts::ReportQueryCustodyConflictRef>,
    pub(crate) cursor_expired_at: Option<contracts::ParentTimestamp>,
    pub(crate) rate_limited_until_at: Option<contracts::ParentTimestamp>,
    pub(crate) raw_child_evidence_included: bool,
    pub(crate) tombstone_confirmed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportQueryCustodyDerivationError {
    InvalidParentAuthority,
    ParentAuthorityActionRejected,
    ParentAuthorityIdentityMismatch,
    ParentAuthorityGenerationMismatch,
    ParentAuthorityExpired,
    RawChildEvidenceRequested,
    EmptyRequestScope,
    NonPositivePageSize,
    MissingCitationRefs,
    InvalidNotificationBoundary,
    InvalidCitationKind,
    CitationIdentityMismatch,
    CitationSourceClassMismatch,
    TrustedSourceResolutionUnavailable,
    DisallowedSourceDataClass,
    NonPositivePageIndex,
    MissingNextCursor,
    UnexpectedNextCursor,
    MissingDeletedSourceMetadata,
    TombstoneRequiredForDeletedSource,
    MissingConflictRef,
    MissingCursorExpiredAt,
    MissingRateLimitedUntilAt,
    PageSizeExceedsLimit,
    DuplicateCursorRef,
    DuplicateSourceRef,
    DuplicateStableSortKey,
    NonMonotonicStableSortKey,
    CursorContinuityMismatch,
    SourceCursorContinuityMismatch,
    NonSequentialPageIndex,
    InvalidContractVersion,
}

pub fn derive_report_query_custody_row(
    request: &contracts::ReportQueryCustodyRequest,
    source: report_query_custody_source::ReportQueryCustodySourceResolution,
    authority: &VerifiedAccountIdentityAuthority,
) -> Result<contracts::ReportQueryCustodyRow, ReportQueryCustodyDerivationError> {
    report_query_custody_row::derive_report_query_custody_row(request, source, authority)
}

pub fn build_report_query_custody_proof(
    request: &contracts::ReportQueryCustodyRequest,
    sources: Vec<report_query_custody_source::ReportQueryCustodySourceResolution>,
    updated_at: contracts::ParentTimestamp,
    authority: &VerifiedAccountIdentityAuthority,
) -> Result<contracts::ReportQueryCustodyContractProof, ReportQueryCustodyDerivationError> {
    report_query_custody_proof::build_report_query_custody_proof(
        request, sources, updated_at, authority,
    )
}
