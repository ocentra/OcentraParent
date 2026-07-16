use ocentra_schema::report_query_custody as contracts;

#[path = "report_query_custody_proof.rs"]
mod report_query_custody_proof;
#[path = "report_query_custody_row.rs"]
mod report_query_custody_row;

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
    DisallowedSourceDataClass,
    RawChildEvidenceRequested,
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
    input: ReportQueryCustodyDerivationInput,
) -> Result<contracts::ReportQueryCustodyRow, ReportQueryCustodyDerivationError> {
    report_query_custody_row::derive_report_query_custody_row(request, input)
}

pub fn build_report_query_custody_proof(
    request: &contracts::ReportQueryCustodyRequest,
    inputs: Vec<ReportQueryCustodyDerivationInput>,
    updated_at: contracts::ParentTimestamp,
) -> Result<contracts::ReportQueryCustodyContractProof, ReportQueryCustodyDerivationError> {
    report_query_custody_proof::build_report_query_custody_proof(request, inputs, updated_at)
}
