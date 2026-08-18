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
pub enum ReportQueryCustodySignal {
    Fresh,
    Stale,
    PartiallyRedacted,
    Deleted,
    Conflict,
    CursorExpired,
    RateLimited,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportQueryCustodyDerivationInput {
    pub row_id: contracts::ReportQueryCustodySourceRef,
    pub source_data_class: contracts::ReportQueryCustodySourceDataClass,
    pub signal: ReportQueryCustodySignal,
    pub cursor_ref: contracts::ReportQueryCustodyCursorRef,
    pub source_cursor_ref: contracts::ReportQueryCustodyCursorRef,
    pub next_cursor_ref: Option<contracts::ReportQueryCustodyCursorRef>,
    pub page_index: u32,
    pub stable_sort_key: contracts::ReportQueryCustodySortKey,
    pub deleted_source_ref: Option<contracts::ReportQueryCustodyDeletedSourceRef>,
    pub deleted_source_at: Option<contracts::ParentTimestamp>,
    pub conflict_ref: Option<contracts::ReportQueryCustodyConflictRef>,
    pub cursor_expired_at: Option<contracts::ParentTimestamp>,
    pub rate_limited_until_at: Option<contracts::ParentTimestamp>,
    pub raw_child_evidence_included: bool,
    pub tombstone_confirmed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportQueryCustodyDerivationError {
    InvalidParentAuthority,
    ParentAuthorityActionRejected,
    ParentAuthorityIdentityMismatch,
    ParentAuthorityGenerationMismatch,
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
    DuplicateCursorRef,
    NonSequentialPageIndex,
    InvalidContractVersion,
}

pub fn derive_report_query_custody_row(
    request: &contracts::ReportQueryCustodyRequest,
    source: report_query_custody_source::ReportQueryCustodySourceResolution,
    authority: VerifiedAccountIdentityAuthority,
) -> Result<contracts::ReportQueryCustodyRow, ReportQueryCustodyDerivationError> {
    report_query_custody_row::derive_report_query_custody_row(request, source, authority)
}

pub fn build_report_query_custody_proof(
    request: &contracts::ReportQueryCustodyRequest,
    sources: Vec<report_query_custody_source::ReportQueryCustodySourceResolution>,
    updated_at: contracts::ParentTimestamp,
    authority: VerifiedAccountIdentityAuthority,
) -> Result<contracts::ReportQueryCustodyContractProof, ReportQueryCustodyDerivationError> {
    report_query_custody_proof::build_report_query_custody_proof(
        request, sources, updated_at, authority,
    )
}
