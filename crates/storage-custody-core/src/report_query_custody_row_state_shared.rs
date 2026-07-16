use super::{ReportQueryCustodyDerivationError, ReportQueryCustodyDerivationInput};

pub(super) fn require_next_cursor(
    input: &ReportQueryCustodyDerivationInput,
) -> Result<(), ReportQueryCustodyDerivationError> {
    if input.next_cursor_ref.is_none() {
        return Err(ReportQueryCustodyDerivationError::MissingNextCursor);
    }
    Ok(())
}

pub(super) fn reject_next_cursor(
    input: &ReportQueryCustodyDerivationInput,
) -> Result<(), ReportQueryCustodyDerivationError> {
    if input.next_cursor_ref.is_some() {
        return Err(ReportQueryCustodyDerivationError::UnexpectedNextCursor);
    }
    Ok(())
}

pub(super) fn require_deleted_source_metadata(
    input: &ReportQueryCustodyDerivationInput,
) -> Result<(), ReportQueryCustodyDerivationError> {
    if input.deleted_source_ref.is_none() || input.deleted_source_at.is_none() {
        return Err(ReportQueryCustodyDerivationError::MissingDeletedSourceMetadata);
    }
    if !input.tombstone_confirmed {
        return Err(ReportQueryCustodyDerivationError::TombstoneRequiredForDeletedSource);
    }
    Ok(())
}
