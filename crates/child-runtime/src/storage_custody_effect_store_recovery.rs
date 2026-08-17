use std::io;

use super::{StorageCustodyEffectStatus, StorageCustodyEffectStore};

const ORPHANED_APPLY_REASON: &str =
    "local delete apply lease was orphaned by a prior process; manual reconciliation is required";

impl StorageCustodyEffectStore {
    pub(super) fn mark_orphaned_apply_manual_required(
        &self,
        operation_ref: &str,
    ) -> io::Result<()> {
        self.update(operation_ref, |record| match record.status {
            StorageCustodyEffectStatus::Applying => {
                record.status = StorageCustodyEffectStatus::ManualRequired;
                record.apply_lease_id = None;
                record.manual_required_reason = Some(ORPHANED_APPLY_REASON.to_owned());
                Ok(())
            }
            StorageCustodyEffectStatus::ManualRequired => Ok(()),
            StorageCustodyEffectStatus::Applied => Ok(()),
            StorageCustodyEffectStatus::Prepared | StorageCustodyEffectStatus::Journaled => {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "only an Applying custody effect may be reconciled as orphaned",
                ))
            }
        })
    }
}
