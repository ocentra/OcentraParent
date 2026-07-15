#![forbid(unsafe_code)]

use crate::policy_source::{ParentPolicySourceDocument, PolicyRollbackRef};
use ocentra_eventing::error::EventingError;

mod identity;
mod restoration;

pub(crate) fn assert_rollback_ref_matches_document(
    document: &ParentPolicySourceDocument,
    rollback_ref: &PolicyRollbackRef,
) -> Result<(), EventingError> {
    identity::assert_rollback_ref_identity(document, rollback_ref)?;
    restoration::assert_rollback_ref_restoration(document, rollback_ref)?;
    Ok(())
}
