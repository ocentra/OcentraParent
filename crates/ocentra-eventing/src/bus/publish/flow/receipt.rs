use crate::{EventingError, JournalAppend};

use super::BeforeDispatchReceiptValidator;

const MISSING_RECEIPT_REASON: &str =
    "before-dispatch receipt validation requires a before-dispatch journal append";

pub(super) fn validate_before_dispatch_receipt(
    validator: Option<BeforeDispatchReceiptValidator>,
    append: Option<&JournalAppend>,
) -> Result<(), EventingError> {
    match (validator, append) {
        (Some(validator), Some(append)) => validator(append),
        (Some(_), None) => Err(EventingError::InvalidHandlerPolicy {
            reason: MISSING_RECEIPT_REASON.to_owned(),
        }),
        (None, _) => Ok(()),
    }
}
