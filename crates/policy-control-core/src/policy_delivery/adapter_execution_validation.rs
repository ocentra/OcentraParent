#![forbid(unsafe_code)]

mod identity;
mod receipt_requirement;
mod rollback;
mod sequence;

use super::{
    EventingError, PolicyDeliveryExecutionReceipt, PolicyDeliveryRecord, PolicyDeliveryTransition,
};

pub(super) fn validate_policy_delivery_execution_receipt(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: Option<&PolicyDeliveryExecutionReceipt>,
) -> Result<(), EventingError> {
    receipt_requirement::validate_policy_delivery_execution_receipt(current, transition, receipt)
}
