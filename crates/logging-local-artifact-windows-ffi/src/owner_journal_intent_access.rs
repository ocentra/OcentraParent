use super::*;

impl IntentRecord {
    pub(crate) fn request_id(&self) -> JournalText<'_> {
        match self {
            Self::Append { request_id, .. }
            | Self::Replace { request_id, .. }
            | Self::Remove { request_id, .. }
            | Self::Transaction { request_id, .. }
            | Self::RemoveTree { request_id, .. } => JournalText(request_id),
        }
    }

    pub(crate) fn relative_path(&self) -> JournalText<'_> {
        match self {
            Self::Append { relative_path, .. }
            | Self::Replace { relative_path, .. }
            | Self::Remove { relative_path, .. } => JournalText(relative_path),
            Self::Transaction { .. } => JournalText(TRANSACTION_OPERATION),
            Self::RemoveTree {
                receipt_relative_path,
                ..
            } => JournalText(receipt_relative_path),
        }
    }
}
