use std::fmt;

use super::{LanHouseholdMeshIngressCustodyError, LanHouseholdMeshIngressReceiptStore};

impl fmt::Display for LanHouseholdMeshIngressCustodyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rejected { outcome } => write!(
                formatter,
                "household mesh ingress rejected and retained: {}",
                outcome.reason.as_str()
            ),
            Self::DuplicateReceipt { receipt_id } => write!(
                formatter,
                "household mesh receipt already reserved: {receipt_id}"
            ),
            Self::ReconciliationRequired { receipt_id } => write!(
                formatter,
                "household mesh receipt requires reconciliation: {receipt_id}"
            ),
            error => formatter.write_str(super::error_static_message::for_error(error)),
        }
    }
}

impl fmt::Debug for LanHouseholdMeshIngressReceiptStore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LanHouseholdMeshIngressReceiptStore")
            .field("connection", &"[redacted]")
            .finish()
    }
}
