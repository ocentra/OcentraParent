use super::MutationReceipt;
use super::ReceiptOutcome;

impl MutationReceipt {
    pub(crate) fn new(
        request_id: String,
        operation: String,
        relative_path: String,
        outcome: ReceiptOutcome,
        replayed: bool,
    ) -> Self {
        Self {
            request_id,
            operation,
            relative_path,
            outcome,
            replayed,
        }
    }

    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    pub fn operation(&self) -> &str {
        &self.operation
    }

    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }

    pub fn outcome(&self) -> &ReceiptOutcome {
        &self.outcome
    }

    pub fn replayed(&self) -> bool {
        self.replayed
    }
}
