#![forbid(unsafe_code)]

use std::fmt;

use super::{
    PolicyDeliveryApplyOutcome, PolicyDeliveryAttemptId, PolicyDeliveryExecutionReceipt,
    PolicyDeliveryId, PolicyDeliveryRecord, PolicyDeliveryTarget, PolicyDeliveryTransition,
};

impl fmt::Debug for PolicyDeliveryId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("PolicyDeliveryId")
            .field(&"<redacted>")
            .finish()
    }
}

impl fmt::Debug for PolicyDeliveryAttemptId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("PolicyDeliveryAttemptId")
            .field(&"<redacted>")
            .finish()
    }
}

impl fmt::Debug for PolicyDeliveryTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PolicyDeliveryTarget")
            .field("child_profile_id", &"<redacted>")
            .field("device_id", &"<redacted>")
            .field("domain", &"<redacted>")
            .finish()
    }
}

impl fmt::Debug for PolicyDeliveryRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PolicyDeliveryRecord")
            .field("schema_version", &self.schema_version)
            .field("delivery_id", &"<redacted>")
            .field("household_id", &"<redacted>")
            .field("policy_version", &self.policy_version.value())
            .field("source_document_id", &"<redacted>")
            .field("target", &self.target)
            .field("state", &self.state)
            .field("last_sequence", &self.last_sequence.value())
            .field("last_attempt_id", &"<redacted>")
            .field("audit_reference_count", &self.audit_reference_ids.len())
            .field(
                "source_audit_reference_count",
                &self.source_audit_reference_ids.len(),
            )
            .field(
                "source_superseded_by_policy_version",
                &self.source_superseded_by_policy_version,
            )
            .field(
                "source_rollback_ref_present",
                &self.source_rollback_ref.is_some(),
            )
            .field("reason_code_present", &self.reason_code.is_some())
            .field(
                "superseded_by_policy_version",
                &self.superseded_by_policy_version,
            )
            .field("rollback_reference_state", &self.rollback_reference_state)
            .field(
                "execution_receipt_present",
                &self.execution_receipt.is_some(),
            )
            .finish()
    }
}

impl fmt::Debug for PolicyDeliveryTransition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PolicyDeliveryTransition")
            .field("attempt_id", &"<redacted>")
            .field("sequence", &self.sequence.value())
            .field("state", &self.state)
            .field("audit_reference_count", &self.audit_reference_ids.len())
            .field("reason_code_present", &self.reason_code.is_some())
            .field(
                "superseded_by_policy_version",
                &self.superseded_by_policy_version,
            )
            .field("rollback_reference_state", &self.rollback_reference_state)
            .finish()
    }
}

impl fmt::Debug for PolicyDeliveryExecutionReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PolicyDeliveryExecutionReceipt")
            .field("delivery_id", &"<redacted>")
            .field("household_id", &"<redacted>")
            .field("policy_version", &self.policy_version.value())
            .field("source_document_id", &"<redacted>")
            .field("target", &"<redacted>")
            .field("attempt_id", &"<redacted>")
            .field("sequence", &self.sequence.value())
            .field("state", &self.state)
            .field("audit_reference_count", &self.audit_reference_ids.len())
            .field("reason_code_present", &self.reason_code.is_some())
            .field("rollback_reference_state", &self.rollback_reference_state)
            .finish()
    }
}

impl fmt::Debug for PolicyDeliveryApplyOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Advanced(record) => formatter.debug_tuple("Advanced").field(record).finish(),
            Self::Duplicate(record) => formatter.debug_tuple("Duplicate").field(record).finish(),
            Self::Stale(record) => formatter.debug_tuple("Stale").field(record).finish(),
        }
    }
}
