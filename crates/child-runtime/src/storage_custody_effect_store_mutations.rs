use std::io;

use super::{StorageCustodyEffectStatus, StorageCustodyEffectStore};

impl StorageCustodyEffectStore {
    pub(in crate::service::storage_custody_runtime) fn mark_journaled(
        &self,
        operation_ref: &str,
    ) -> io::Result<()> {
        self.update(operation_ref, |record| match record.status {
            StorageCustodyEffectStatus::Prepared => {
                record.status = StorageCustodyEffectStatus::Journaled;
                Ok(())
            }
            StorageCustodyEffectStatus::Journaled => Ok(()),
            StorageCustodyEffectStatus::Applying
            | StorageCustodyEffectStatus::Applied
            | StorageCustodyEffectStatus::ManualRequired => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "custody effect cannot return to Journaled after apply or recovery",
            )),
        })
    }
}
