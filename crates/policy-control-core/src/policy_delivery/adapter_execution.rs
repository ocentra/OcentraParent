#![forbid(unsafe_code)]

use super::{
    adapter_execution_validation, EventingError, PolicyDeliveryExecutionReceipt,
    PolicyDeliveryRecord, PolicyDeliveryTransition,
};

pub(super) fn validate_policy_delivery_execution_receipt(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: Option<&PolicyDeliveryExecutionReceipt>,
) -> Result<(), EventingError> {
    adapter_execution_validation::validate_policy_delivery_execution_receipt(
        current, transition, receipt,
    )
}
